# -*- coding: utf-8 -*-
# _done_append.py — DONE.md 에 07-30 세션 판정 추가(4컬럼 표 형식 유지).
import io, os, sys

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
P = r"C:\Users\dev\.claude\projects\C--Users-dev-Desktop-claude-tfm2\memory\DONE.md"
ROWS = [
    ("★asset-get clone family 3형제 확정 = layout `0x2e1550`/텍스처 `0x143d50`/애님 `0x888fd0`(앵커맵 콜러-대응 양방향 투표·역순도 100%) — `0x91ab0` 파급 전건 종결",
     "DONE(재조사금지)", "0.5.3(07-30)", "CURRENT.md + _MIGRATE_053 §1c"),
    ("0.5.3 `__rust_dealloc` = **인라인화로 소멸**(0.5.2 `0x25c4d90` 형태 부재) ⟹ `HeapFree(GetProcessHeap(),0,ptr)` 직접 호출이 정본(align=1 이면 `ptr-8` 보정 불요). alloc `0x28f7df0` = exe 내 유일한 HeapAlloc 참조 함수",
     "DONE(재탐색금지)", "0.5.3(07-30)", "_MIGRATE_053 §1c"),
    ("draft_overlay 구 RVA(`LOADER 0x40f3d0`·`ANIM_GET 0x40e250`)는 **0.5.2 시점에 이미 죽어 있었다**(함수 시작 아님·콜러 0) + 밴픽 asset-get **copy #2 는 존재하지 않음**(문자열이 copy #1 로 ×19 수렴)",
     "DONE(재조사금지)", "0.5.2~0.5.3", "_MIGRATE_053 §1c"),
    ("세이브 포맷 **0.5.2 = 0.5.3 무변경** 실증(실 세이브 full-load·salvage 아님) ⟹ 대시보드 빌더·프론트 무수정 유효. 4연속 무변경(0.5.0hf2=0.5.1=0.5.2=0.5.3)",
     "DONE", "0.5.3(07-30)", "tfm2gg-dashboard-save-probe"),
    ("릴리스 zip cfg 는 **라이브 복사 금지 = 배포 기본값으로 정규화**(실측: ai_adjust 라이브 cfg 에 `log = 1` 진단 ON·illust/comptest 는 유저 튜닝값) ⟹ dll·mod_info 만 라이브 반영",
     "DONE(규칙)", "버전무관", "tfm2-release-zip-location"),
    ("⚠**PII 검사 정규식을 bash heredoc 에 넣으면 백슬래시 소실로 가짜 음성(0건)** — 반드시 `.py` 파일로 실행(`MODS\\pii_check.py`)",
     "DONE(함정)", "버전무관", "CURRENT.md §릴리스"),
    ("ai_adjust 설정편집기 PII 414건(cargo 레지스트리 경로) = `--remap-path-prefix` 재빌드로 **0건**(8,375,808B). 기존 0.5.2 릴리스본도 동일 414건이었음(회귀 아님)",
     "DONE", "0.5.3(07-30)", "CURRENT.md §릴리스"),
]
s = open(P, encoding="utf-8").read()
if not s.endswith("\n"):
    s += "\n"
add = "".join(f"| {a} | {b} | {c} | {d} |\n" for a, b, c, d in ROWS)
open(P, "w", encoding="utf-8").write(s + add)
print(f"DONE.md +{len(ROWS)}행 → {os.path.getsize(P):,}B")
