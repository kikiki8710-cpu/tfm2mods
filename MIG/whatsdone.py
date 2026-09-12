#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""whatsdone.py — 「이거 전에 했나?」를 5초로 (CLAUDE.md §7 착수 전 grep 자동화 · 2026-09-13)

왜: 2026-09-10~13 세션에서 주소 탐색 4건·#13 회계 등 「이미 기록된 것」을 재조사했다. 원인은
    기록이 없어서가 아니라 기록처가 8곳(원장·DONE·RE 파일명·ANA §20/§22·TOOLS·METHOD_MAP·spec20 메모리·
    명세 JSON)이라 한 곳만 grep 하고 「없다」로 결론 낸 것. 이 도구는 그 8곳을 한 번에 훑는다.

사용:
  python -X utf8 MIG\\whatsdone.py <키워드> [키워드…]      OR 매칭(키워드 하나라도)
  python -X utf8 MIG\\whatsdone.py "#12"                   원장 §1 에서 #12 행을 찾아 함수명·RVA 로 확장 검색
  python -X utf8 MIG\\whatsdone.py 0xe595b0 --all           AND 매칭
  python -X utf8 MIG\\whatsdone.py midpin --max 40          출처당 최대 행(기본 15)
  환경변수 TFM2_REPORT=<mods_report 절대경로> 로 REPORT 루트를 고정할 수 있다(기본 = 원장 mtime 최신 worktree).

출력 규칙: 출처별로 `경로:줄  스니펫` · 원장이 맨 앞 · 마지막 줄에 총 히트. 히트 0 = 「기록 없음」이 아니라
「이 8곳엔 없음」 — 그 다음은 prior-work 에이전트.
"""
import io, os, re, sys, glob, subprocess

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

TFM2 = r"C:\Users\jungs\Desktop\claude\tfm2"
MEM = r"C:\Users\jungs\.claude\projects\C--Users-jungs-Desktop-claude-tfm2\memory"
ANA = os.path.join(TFM2, u"팀파매2모드 분석")
MIG = os.path.dirname(os.path.abspath(__file__))
LEDGER = u"00_상태원장.md"


def report_root():
    u"""REPORT 루트 = 환경변수 > 원장(00_상태원장.md) mtime 이 가장 최신인 worktree/base."""
    env = os.environ.get("TFM2_REPORT")
    if env and os.path.isdir(env):
        return env, "env"
    cands = [os.path.join(TFM2, "mods_report")]
    try:
        out = subprocess.run(["git", "-C", TFM2, "worktree", "list", "--porcelain"],
                             capture_output=True, text=True, encoding="utf-8", errors="replace").stdout
        for ln in out.splitlines():
            if ln.startswith("worktree "):
                cands.append(os.path.join(ln[9:].strip().replace("/", os.sep), "mods_report"))
    except Exception:
        pass
    best, bt = None, -1
    for c in cands:
        p = os.path.join(c, "tfm2_judge_verify", LEDGER)
        if os.path.exists(p):
            t = os.path.getmtime(p)
            if t > bt:
                best, bt = c, t
    if best is None:
        best = cands[0]
    return best, "ledger-mtime"


def read_lines(path):
    try:
        with io.open(path, encoding="utf-8", errors="replace") as f:
            return f.read().splitlines()
    except Exception:
        return []


def expand_fn(term, ledger_lines):
    u"""`#12` → 원장 §1 행의 명세 id 마지막 토큰 + RVA 들을 검색어에 추가."""
    m = re.match(r"^#(\d{1,2})$", term)
    if not m:
        return [term]
    idx = int(m.group(1))
    extra = [term]
    for ln in ledger_lines:
        if re.match(r"^\|\s*%02d\s*\|" % idx, ln):
            cells = [c.strip() for c in ln.strip("|").split("|")]
            if len(cells) >= 3:
                name = cells[1].strip("`")
                extra.append(name.split("::")[-1])           # handle_chat
                extra += re.findall(r"0x[0-9a-f]{5,7}", cells[2])  # RVA 들
            break
    return extra


def grep(path, terms, mode_all, maxn, header_only=False, label=None):
    hits = []
    for i, ln in enumerate(read_lines(path), 1):
        if header_only and not ln.startswith("#"):
            continue
        low = ln.lower()
        ok = all(t in low for t in terms) if mode_all else any(t in low for t in terms)
        if ok:
            hits.append((i, ln.strip()))
    return hits


def show(title, path, hits, maxn, total):
    if not hits:
        return total
    print(u"\n## %s  (%d건)  %s" % (title, len(hits), path))
    for i, ln in hits[:maxn]:
        s = ln if len(ln) <= 170 else ln[:167] + u"…"
        print(u"  %5d: %s" % (i, s))
    if len(hits) > maxn:
        print(u"  … +%d (--max 로 늘림)" % (len(hits) - maxn))
    return total + len(hits)


def main():
    args = [a for a in sys.argv[1:]]
    mode_all = "--all" in args
    maxn = 15
    if "--max" in args:
        k = args.index("--max"); maxn = int(args[k + 1]); del args[k:k + 2]
    args = [a for a in args if not a.startswith("--")]
    if not args:
        print(__doc__); return 2

    rep, how = report_root()
    jv = os.path.join(rep, "tfm2_judge_verify")
    ledger = os.path.join(jv, LEDGER)
    ledger_lines = read_lines(ledger)

    terms = []
    for a in args:
        terms += expand_fn(a, ledger_lines)
    terms = sorted(set(t.lower() for t in terms if t), key=len, reverse=True)
    print(u"검색어: %s   (매칭=%s · REPORT=%s [%s])" % (u" | ".join(terms), "AND" if mode_all else "OR", rep, how))

    total = 0
    # 1 원장
    total = show(u"원장 00_상태원장.md", ledger, grep(ledger, terms, mode_all, maxn), maxn, total)
    # 2 MEM
    for f in ["DONE.md", "CURRENT.md", "INDEX.md", "tfm2-judge-spec20.md", "tfm2-judge-layer.md",
              "tfm2-analysis-method-map.md", "tfm2-ir-mining-toolkit.md"]:
        p = os.path.join(MEM, f)
        total = show(u"MEM " + f, p, grep(p, terms, mode_all, maxn), maxn, total)
    # 3 RE 파일명
    for sub in ["tfm2_judge_verify", "tfm2_ai_adjust"]:
        d = os.path.join(rep, sub, "RE")
        names = sorted(os.listdir(d)) if os.path.isdir(d) else []
        hits = [(0, n) for n in names if (all(t in n.lower() for t in terms) if mode_all else any(t in n.lower() for t in terms))]
        total = show(u"RE 파일명 " + sub, d, hits, maxn, total)
    # 4 REPORT 본문(judge_verify 01~04 · ai_adjust 03/05)
    for f in [os.path.join(jv, x) for x in [u"01_구조.md", u"02_구현정보.md", u"03_시행착오.md", u"04_분석방법_정리.md"]] + \
             [os.path.join(rep, "tfm2_ai_adjust", x) for x in [u"03_시행착오.md", u"05_judge_계층_설계.md"]]:
        total = show(u"REPORT " + os.path.relpath(f, rep), f, grep(f, terms, mode_all, maxn), maxn, total)
    # 5 ANA 정본 (절 제목 우선 → 본문)
    for f in [u"감사도구-방법론-사례정본.md", u"판단함수20-명세-확정사실-0.5.8.md", u"judge-계층-사례정본.md", u"reimpl-tracker.md"]:
        p = os.path.join(ANA, f)
        h = grep(p, terms, mode_all, maxn, header_only=True)
        total = show(u"ANA(절제목) " + f, p, h, maxn, total)
        b = grep(p, terms, mode_all, maxn)
        b = [x for x in b if x not in h]
        total = show(u"ANA(본문) " + f, p, b, maxn, total)
    # 6 MIG 문서·생성기
    for f in ["TOOLS.md", "METHOD_MAP.md", "SPEC_RUNBOOK.md", "gensweep20.py", "probe20.py"]:
        p = os.path.join(MIG, f)
        total = show(u"MIG " + f, p, grep(p, terms, mode_all, maxn), maxn, total)
    # 7 명세 JSON (건수만 — 1.2MB)
    p = os.path.join(MIG, "_spec", "specs20_v3.json")
    n = sum(1 for _ in grep(p, terms, mode_all, 10**9))
    if n:
        print(u"\n## 명세 specs20_v3.json  (%d줄 매칭 — 내용은 mkdossier/specgate --only 로)" % n); total += n
    # 8 검증 모드 소스
    src = os.path.join(r"C:\tfm2mods\tfm2_judge_verify", "src")
    for f in sorted(glob.glob(os.path.join(src, "*.rs"))):
        total = show(u"SRC " + os.path.basename(f), f, grep(f, terms, mode_all, maxn), maxn, total)

    print(u"\n총 %d 히트. 0 이면 「이 8곳엔 없음」 → prior-work 에이전트." % total)
    return 0 if total else 1


if __name__ == "__main__":
    sys.exit(main())
