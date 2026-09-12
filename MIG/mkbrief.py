# -*- coding: utf-8 -*-
u"""mkbrief — 반증검증 라운드 브리핑의 **사실 절을 데이터에서 생성**한다. (2026-09-11 신설)

## 왜 만드나 — 내 지시가 병목이었다
유저 지적: *"가끔 지시 자체를 잘못할때가 있어. 이러면 엄청 낭비니까."* 실제 이력:

| 라운드 | 내 브리핑 오류 | 결과 |
|---|---|---|
| 3차 | 「무엇이 불가능한지」 3건 과잉 단정 | 4차에 **전부 뒤집힘** — 배치들이 시도조차 안 할 뻔했다 |
| 4차 | `tcxaudit` 기준선 수치 | **네 배치 전부**가 지적(인자 다른 실행값을 기준선으로 옮겨 적음) |
| 5차 | ev 집계 **스코프 미기재** | 배치 B 가 다른 분모로 세어 「브리핑이 틀렸다」 **오적발** |
| 5차 | `game_ai::is_ignored_well_enemy` **존재하지 않는 경로** | 배치 D 가 rustc E0425 로 적발. 내가 승격하며 모듈 경로를 잘랐다 |
| 5차 | ev 보고 **형식 미지정** | 상향 492행 중 **317행 유실**(집계표만 받음) |

전부 **내가 기억으로 썼기 때문**이다. ⟹ 사실은 손으로 쓰지 않는다:
 - 수치(행 수·ev 분포·open/notes)는 **정본 JSON 에서 센다**
 - 심볼 경로는 **`_tcx` 에 실재 확인 후**에만 적는다(없으면 브리핑 생성을 **실패시킨다**)
 - 게이트 기준선은 **숫자 대신 명령**을 넣는다(수치는 명세가 자라면 변한다)

사용:
  python -X utf8 mkbrief.py 6            → `_verify6/BRIEF_FACTS.md` 생성
  python -X utf8 mkbrief.py 6 --check    → 생성 없이 검증만(심볼 실재·수치)
"""
import hashlib, io, json, os, re, subprocess, sys, time

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
V3 = os.path.join(HERE, "_spec", "specs20_v3.json")
D = json.load(io.open(V3, encoding="utf-8"))
S = D["specs"]

# ★집계 스코프 — **반드시 브리핑에 박는다.** 5차에 이걸 안 적어서 배치 B 가 오적발했다.
EV_FIELDS = ("mem", "consts", "knobs")
BATCH = (("A", 0, 4), ("B", 5, 9), ("C", 10, 14), ("D", 15, 19))


# ── 심볼 실재 검증 (5차 `is_ignored_well_enemy` 사고 봉쇄) ────────────
_TCX = [None]


def tcx_paths():
    if _TCX[0] is None:
        p = set()
        for cr in ("game_ai", "game_core", "game_view"):
            f = os.path.join(HERE, "_tcx", "%s.json" % cr)
            if not os.path.exists(f):
                continue
            for x in json.load(io.open(f, encoding="utf-8"))["items"]:
                if x.get("p"):
                    p.add(x["p"])
        _TCX[0] = p
    return _TCX[0]


def verify_symbols(text):
    u"""브리핑 본문에서 `crate::path::name` 꼴을 뽑아 tcx 에 실재하는지 본다.
    없으면 **생성을 실패시킨다** — 없는 경로를 배치에 넘기면 그 배치는 rustc 에러로 시간을 버린다."""
    pat = re.compile(r"`(game_ai|game_core|game_view)((?:::[A-Za-z_][A-Za-z0-9_]*)+)`")
    all_p = tcx_paths()
    bad = []
    for m in pat.finditer(text):
        full = m.group(1) + m.group(2)
        if full in all_p:
            continue
        leaf = full.rsplit("::", 1)[-1]
        cands = sorted(x for x in all_p if x.rsplit("::", 1)[-1] == leaf)
        bad.append((full, cands[:3]))
    return bad


def run(cmd):
    try:
        e = dict(os.environ, PYTHONIOENCODING="utf-8")
        out = subprocess.run([sys.executable, "-X", "utf8"] + cmd, cwd=HERE,
                             capture_output=True, text=True, encoding="utf-8",
                             errors="replace", env=e, timeout=900)
        return (out.stdout or u"") + (out.stderr or u"")
    except Exception as ex:
        return u"(실행 실패: %s)" % ex


def evdist(lo, hi):
    c = {}
    for i in range(lo, hi + 1):
        for f in EV_FIELDS:
            for x in S[i][f]:
                e = x.get("ev")
                c[e] = c.get(e, 0) + 1
    return c


def per_fn(lo, hi):
    u"""★`sig.vis` 를 같이 낸다(5차 배치D 요청). `pub` 가 아니면 오라클 **직접 진입이 막힌다** —
    5차에 `18`(`in:game_ai`)이 47행을 그대로 남겼고, 그 사실을 미리 알았다면
    그 배치는 처음부터 우회 경로(상위 pub 래퍼·형제 복제본)를 노렸을 것이다."""
    out = []
    for i in range(lo, hi + 1):
        n = sum(1 for f in EV_FIELDS for x in S[i][f] if (x.get("ev") or 9) >= 4)
        sig = S[i].get("sig") or {}
        vis = sig.get("vis") or u"?"
        out.append((i, S[i]["name"], n, vis))
    return sorted(out, key=lambda t: -t[2])


def fresh_or_die():
    u"""★**생성물이라도 생성 시각이 정본보다 이르면 틀린다.** (6차 배치C 적발)

    5차 반영을 15:51/15:53 에 끝냈는데 브리핑을 15:49 에 만들어 놓고 그대로 넘겼다.
    배치들이 읽은 표는 **반영 전 수치**였고(배치 D ev≥4 가 89.8% 로 찍혔지만 실제는 37.5%),
    배치 C 가 mtime 을 대조해 잡아냈다.
    ⟹ 「생성했으니 맞다」가 아니라 **「정본보다 나중에 생성됐나」까지 봐야** 맞다.
    """
    v2 = os.path.join(HERE, "_spec", "specs20.json")
    # ★**해시까지 박는다**(6차 배치D 요청 — mtime 만으론 부족하다).
    #   `mkbrief --check` 가 mtime 만 봐서, 생성물이 낡았는데도 통과시킨 적이 있다.
    #   생성물 머리에 정본 해시를 넣으면 **읽는 쪽이 스스로 확인**할 수 있다.
    if os.path.getmtime(V3) < os.path.getmtime(v2):
        print(u"★`specs20_v3.json` 이 `specs20.json` 보다 낡았다 — `mkspec3.py` 를 먼저 돌려라.")
        print(u"   v3 %s  <  v2 %s" % (time.strftime("%H:%M:%S", time.localtime(os.path.getmtime(V3))),
                                       time.strftime("%H:%M:%S", time.localtime(os.path.getmtime(v2)))))
        sys.exit(2)


def main():
    rnd = sys.argv[1] if len(sys.argv) > 1 else "6"
    fresh_or_die()
    L = []
    A = L.append
    A(u"<!-- ★이 파일은 `mkbrief.py` 가 생성한다. 손으로 고치지 마라 —"
      u" 다음 생성에 날아가고, 손으로 쓴 수치가 5차까지 반복된 브리핑 오류의 원인이었다. -->")
    A(u"# %s차 반증검증 — **생성된 사실 절** (게임 %s)\n" % (rnd, D["meta"].get("game")))
    # ★해시까지 박는다(6차 배치D 요청). mtime 은 파일을 만졌는지만 알려주고,
    #   **내용이 같은지는 말해주지 않는다.** 읽는 쪽이 한 줄로 스스로 확인하게 한다.
    _h = hashlib.sha256(io.open(V3, "rb").read()).hexdigest()[:16]
    A(u"> **정본 스탬프** — `specs20_v3.json` sha256[:16] = `%s` · mtime `%s` · 이 파일 생성 `%s`"
      % (_h, time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(os.path.getmtime(V3))),
         time.strftime("%Y-%m-%d %H:%M:%S")))
    A(u"> ★**읽기 전에 스스로 확인하라.** 아래 명령의 출력이 위 해시와 다르면 **이 표는 낡았다** — "
      u"`python -X utf8 mkbrief.py <N>` 을 다시 돌려라.")
    A(u"> ```bash")
    A(u"> cd /c/tfm2mods/MIG && python -c \"import hashlib,io;"
      u"print(hashlib.sha256(io.open('_spec/specs20_v3.json','rb').read()).hexdigest()[:16])\"")
    A(u"> ```")
    A(u"> 수치는 전부 `_spec/specs20_v3.json` 에서 **센 값**이고, 심볼 경로는 `_tcx` 에 "
      u"**실재 확인**을 통과한 것만 적혀 있다. 브리핑의 판단·지침 절은 이 파일 **뒤에** 붙는다.\n")

    # ── ①집계 스코프 (5차 오적발의 원인) ─────────────────────────────
    A(u"## ① 집계 스코프 — **이 정의로 센 값이다**\n")
    A(u"이 문서의 「행」 = **`mem` + `consts` + `knobs`** 세 배열의 원소만이다. "
      u"`sig.params` · `callees` · `open`/`notes` 는 **포함하지 않는다.**")
    A(u"⚠5차에 이 정의를 안 적어서 한 배치가 다른 분모(296행)로 세고 「브리핑이 틀렸다」고 "
      u"보고했다. 네 분모가 다르면 **그건 불일치가 아니라 스코프 차이다** — 먼저 이 정의로 다시 세라.\n")

    tot = evdist(0, 19)
    ttl = sum(tot.values())
    A(u"| 배치 | 함수 | ev1 | ev2 실행 | ev3 tcx | ev4 IR | ev5 추론 | 계 | **ev≥4** |")
    A(u"|---|---|---|---|---|---|---|---|---|")
    for b, lo, hi in BATCH:
        c = evdist(lo, hi)
        t = sum(c.values())
        hi4 = c.get(4, 0) + c.get(5, 0)
        A(u"| **%s** | %02d~%02d | %d | %d | %d | %d | %d | %d | **%.1f%%** |"
          % (b, lo, hi, c.get(1, 0), c.get(2, 0), c.get(3, 0), c.get(4, 0), c.get(5, 0),
             t, 100.0 * hi4 / t if t else 0))
    A(u"| 계 | 00~19 | %d | %d | %d | %d | %d | %d | **%.1f%%** |"
      % (tot.get(1, 0), tot.get(2, 0), tot.get(3, 0), tot.get(4, 0), tot.get(5, 0), ttl,
         100.0 * (tot.get(4, 0) + tot.get(5, 0)) / ttl))
    A(u"")

    # ── ②표적: 미실행 행이 많은 함수 + 진입 가능성 ───────────────────
    A(u"## ② 표적 — `ev≥4`(미실행) 가 많은 함수\n")
    A(u"`sig.tcx` 가 `pub` 인지 함께 적었다(5차 배치D 요청). "
      u"**`pub` 가 아니면 오라클 진입 자체가 막힐 수 있다** — 5차에 `18` 이 47행을 그대로 남겼다.\n")
    for b, lo, hi in BATCH:
        rows = per_fn(lo, hi)
        A(u"- **배치 %s**: " % b + u" · ".join(u"`%02d %s` %d행(%s)" % (i, n[:26], k, p)
                                              for i, n, k, p in rows if k))
    A(u"")

    # ── ③open / notes ─────────────────────────────────────────────────
    op = [(o["i"], x) for o in S for x in o["open"]]
    nt = [(o["i"], x) for o in S for x in (o.get("notes") or [])]
    A(u"## ③ `open` %d건 · `notes` %d건\n" % (len(op), len(nt)))
    A(u"`open` = 아직 답이 없는 것. `notes` = **확정된 사실 서술이라 물음이 아니다** — "
      u"파지 말고, 틀렸다고 보면 **반증**하라.\n")
    A(u"| # | 분류 | ev | 물음 |")
    A(u"|---|---|---|---|")
    for i, x in op:
        A(u"| %02d | %s | %s | %s |" % (i, x.get("class"), x.get("ev"),
                                        (x["q"] or u"").replace(u"\n", u" ")[:110]))
    A(u"\n<details><summary>`notes` %d건 (파지 말 것)</summary>\n" % len(nt))
    for i, x in nt:
        A(u"- `%02d` %s" % (i, (x["q"] or u"").replace(u"\n", u" ")[:130]))
    A(u"</details>\n")

    # ── ④게이트 — 숫자가 아니라 명령과 **지금 실측** ─────────────────
    A(u"## ④ 게이트 — 숫자를 외우지 말고 **직접 재라**\n")
    A(u"```bash\ncd /c/tfm2mods/MIG\n"
      u"PYTHONIOENCODING=utf-8 python -X utf8 specgate.py        # G1~G8·G10·G11 = 0\n"
      u"PYTHONIOENCODING=utf-8 python -X utf8 _spec/audit4.py    # 과거 라운드 회귀\n"
      u"PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose <내보고서.md>\n```")
    A(u"**불변 게이트는 총 건수가 아니라 `오귀속 0 · 밀림 0` 이다.** 총 건수는 명세가 자라면 변한다.\n")
    g = run(["specgate.py"])
    line = [l for l in g.split(u"\n") if u"G1 자기모순" in l]
    A(u"생성 시점 실측(참고용 — 네가 다시 재라):\n```\n%s\n```\n"
      % (line[0].strip() if line else u"(측정 실패)"))

    txt = u"\n".join(L)
    bad = verify_symbols(txt)
    out = os.path.join(HERE, "_verify%s" % rnd, "BRIEF_FACTS.md")
    if bad:
        print(u"★심볼 실재 검증 실패 %d건 — 브리핑을 만들지 않는다:" % len(bad))
        for f, c in bad:
            print(u"   없음: %s   후보: %s" % (f, c))
        return 1
    if "--check" in sys.argv:
        print(u"검증만 수행 — 심볼 실재 OK · 행 %d · open %d · notes %d" % (ttl, len(op), len(nt)))
        return 0
    if not os.path.isdir(os.path.dirname(out)):
        os.makedirs(os.path.dirname(out))
    io.open(out, "w", encoding="utf-8").write(txt + u"\n")
    print(u"%s  (%d줄 · 행 %d · open %d · notes %d · 심볼 실재 OK)"
          % (out, len(L), ttl, len(op), len(nt)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
