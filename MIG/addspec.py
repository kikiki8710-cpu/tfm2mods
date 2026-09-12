#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""addspec.py — 명세 정본(`_spec\\specs20.json` v2)에 **함수 1개를 추가**한다. (2026-09-13 신설)

왜 따로 있나: `mkspec20.py` 는 `_spec\\r1..r6` 에서 정본을 **통째로 재생성**한다. 그런데 20함수의
1~12차 정정은 `applypatch.py` 가 v2 에 직접 누적했고 r1..r6 엔 없다 ⟹ 21번째 함수를 넣겠다고
`mkspec20.py` 를 다시 돌리면 **12라운드 정정이 r6 원본으로 덮여 사라진다**. 그래서 신규 함수는
이 도구로 v2 에 append 하고, `mkspec3.py` 로 v3 를 재생성한다(기존 20함수는 손대지 않는다).

입력 = 라운드 JSON 1개(에이전트가 `SPEC_GUIDE.md` 규격으로 쓴 것 · `_spec\\rN\\<id>.json` 과 같은 키):
  name · sym · src · src_line · ir_file · ir_from · ir_to · one_line · signature · logic ·
  reads[] · writes[] · constants[] · calls[] · knobs[] · unknown[]

사용:
  python -X utf8 MIG\\addspec.py <id> <라운드JSON> [--exe 0xRVA] [--evidence ghidra|fp|runtime] [--dry]
  예) python -X utf8 MIG\\addspec.py 20_lph_update _spec\\r7\\20_lph_update.json --exe 0xe4c5c0 --evidence ghidra
  → 백업 `_spec\\specs20.bak-<날짜>-addspec.json` 생성 → v2 끝에 i=<N> 으로 append → 안내:
     ① python -X utf8 MIG\\mkspec3.py   (v3 재생성)   ② specgate.py --only <N>   ③ probe20.py / gensweep20.py 재생성 시 idx <N> 반영 확인

⚠ `--exe` 없이 넣으면 `exe=None`(`no_map_reason` 기록) — 주소는 rvaverify/ghidra-re 로 확정한 뒤
  `applypatch` 로 채운다. `--evidence fp`(지문) 는 addrgate.py 게이트를 통과하기 전엔 **추정**이다.
"""
import io, json, os, sys, shutil, time

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
V2 = os.path.join(HERE, "_spec", "specs20.json")
REQ = ["name", "sym", "src", "src_line", "ir_file", "ir_from", "ir_to", "one_line", "signature", "logic"]
LISTS = ["reads", "writes", "constants", "calls", "knobs", "unknown"]


def main():
    a = sys.argv[1:]
    if len(a) < 2:
        print(__doc__); return 2
    dry = "--dry" in a
    exe = a[a.index("--exe") + 1] if "--exe" in a else None
    evd = a[a.index("--evidence") + 1] if "--evidence" in a else None
    sid, src = a[0], a[1]
    j = json.load(io.open(src, encoding="utf-8"))
    miss = [k for k in REQ if k not in j]
    if miss:
        print(u"✗ 라운드 JSON 필수 키 누락: %s  (SPEC_GUIDE.md 규격)" % miss); return 1
    D = json.load(io.open(V2, encoding="utf-8"))
    if any(s["id"] == sid for s in D["specs"]):
        print(u"✗ id %s 는 이미 정본에 있다 — 정정은 applypatch.py 로" % sid); return 1
    if any(s.get("sym") == j.get("sym") for s in D["specs"]):
        print(u"✗ sym %s 이 이미 정본에 있다(다른 id) — 중복 등록 금지" % j.get("sym")); return 1
    from mkspec20 import layer_of
    rec = dict(
        id=sid, name=j["name"], sym=j.get("sym"), src=j.get("src"), src_line=j.get("src_line"),
        layer=layer_of(j.get("src")),
        ir=dict(file=j.get("ir_file"), frm=j.get("ir_from"), to=j.get("ir_to")),
        base_round=os.path.basename(os.path.dirname(os.path.abspath(src))) or "r?", rounds=1,
        one_line=j.get("one_line"), signature=j.get("signature"), logic=j.get("logic"),
        **{k: (j.get(k) or []) for k in LISTS})
    if exe:
        rec["exe"] = dict(addr="%x" % int(exe, 16),   # 기존 20함수와 같은 16진 문자열(0x 없음) — int 로 넣으면 rvaverify 가 죽는다(09-13) module=j.get("module"), bytes=None, instrs=None,
                          evidence=evd or "manual", callers=[], callees=[])
    else:
        rec["exe"] = None
        rec["no_map_reason"] = u"addspec 등록 시 주소 미확정 — rvaverify/ghidra-re 확정 후 applypatch 로 채울 것"
    rec["resolved"], rec["new_knobs"], rec["still_unknown"] = [], [], []
    n = len(D["specs"])
    print(u"append: i=%d id=%s sym=%s exe=%s  (reads %d · writes %d · consts %d · calls %d · knobs %d · unknown %d)" % (
        n, sid, rec["sym"], exe, *[len(rec[k]) for k in LISTS]))
    if dry:
        print(u"(--dry) 쓰지 않음"); return 0
    bak = V2.replace(".json", ".bak-%s-addspec.json" % time.strftime("%Y-%m-%d-%H%M"))
    shutil.copy2(V2, bak)
    D["specs"].append(rec)
    D["meta"]["counts"]["functions"] = len(D["specs"])
    D["meta"].setdefault("addspec_log", []).append(dict(i=n, id=sid, date=time.strftime("%Y-%m-%d"), src=os.path.abspath(src)))
    io.open(V2, "w", encoding="utf-8", newline="").write(json.dumps(D, ensure_ascii=False, indent=1))
    print(u"OK → %s (백업 %s)\n다음: ① python -X utf8 MIG\\mkspec3.py  ② python -X utf8 MIG\\specgate.py --only %d  ③ 원장 §1 에 행 추가" % (V2, os.path.basename(bak), n))
    return 0


if __name__ == "__main__":
    sys.exit(main())
