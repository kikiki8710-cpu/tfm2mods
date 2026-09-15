# -*- coding: utf-8 -*-
u"""mkpatch — 배치가 `patch.json`(정정 계약)을 **안전하게 만드는** 참조 구현. (2026-09-11 신설)

## 왜 만드나 — 6차에 네 배치가 **각자 따로** 만들었다
`_verify6/{A,B,C,D}/mkpatch.py` 가 넷 다 있었다. 같은 걸 한 라운드에서 **네 번 재발명**한 것이다.
계약(`applypatch.py` 머리말)은 있는데 **참조 구현이 없어서** 매번 새로 썼다.
⟹ 이 파일이 그 자리다. 배치는 이걸 `import` 해서 쓰고, 새로 만들지 마라.

## 왜 직접 JSON 을 쓰면 안 되나
`old` 는 **정본에 실제로 있는 문자열**이어야 한다. 없으면 `applypatch` 가 실패로 보고한다
(조용한 no-op 금지). 손으로 옮겨 적다 공백·따옴표 하나가 어긋나는 일이 실제로 잦다 —
그래서 이 도구는 **넣는 시점에 정본과 대조**하고, 안 맞으면 그 자리에서 알려준다.

## 쓰는 법
```python
import sys; sys.path.insert(0, r"C:\\tfm2mods\\MIG")
import mkpatch

p = mkpatch.Patch(round=7, batch="A")

# ①정정 — `old` 가 정본에 있는지 즉시 확인한다(없으면 예외)
p.fix("/specs[0]/logic",
      old="if let Some(e) = champ.attack_effect {",
      new="let attack_effect = champ.attack_effect().as_ref();",
      evidence="DWARF !56150=Option<&Effect> · as_ref@option.rs:742 프레임",
      behavior_change=True, found_by="reused")

# ②값 필드(정수·불리언)도 된다 — 키까지 지정하면 값 비교로 간다
p.fix("/specs[3]/consts[0]/src_line", old=21, new=22,
      evidence="!dbg inlinedAt 사슬 전개", found_by="reused")

# ③ev 상향 — 숫자를 직접 박지 마라. 근거를 주면 `applypatch` 가 문면에 붙이고 ev 가 파생된다
p.ev("/specs[16]/mem[0]", to=3, evidence="offset_of! 107/107 MISMATCH 0", found_by="inherited")

p.save()      # → `_verify<N>/<B>/patch.json` · 저장 전에 전건 재검증
```

## 규칙 (도구가 강제한다)
- `old` 가 정본에 없으면 **`save()` 가 거부한다.** 넘어가고 싶으면 `p.fix(..., force=True)`.
- `mem` 의 `ev` 상한은 **3**(오프셋의 정본은 tcx). `to=2` 로 줘도 3 으로 깎인다.
- `found_by` 는 `reused`/`inherited`/`new` 중 하나. 빠지면 실험 집계에서 빠진다.
- `new` 는 **긍정 서술**로 써라. 「옛것이 틀렸다」가 아니라 **「참인 것」**을 적는다
  (이력은 `history` 와 RE 기록에 있다 — 본문에 지층을 쌓지 마라).
"""
import io, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
V3 = os.path.join(HERE, "_spec", "specs20_v3.json")
# ★★경로 문법은 **`applypatch` 에서 가져온다 — 복사하지 마라.** (7차 배치B 적발)
#   여기에 베껴 둔 2단 정규식이 `applypatch` 의 3단(`/specs[i]/sig/params[j]`)을 못 받아
#   참조구현이 `ValueError` 로 죽었다. 6차가 `applypatch` 만 고치고 이쪽을 안 고쳤기 때문이다.
#   같은 사고가 `cleanspec.KEEP` 에서도 있었다 — **두 곳에 같은 규칙을 두면 반드시 갈라진다.**
try:
    from applypatch import PATH, parse_path        # noqa: F401  (계약의 단일 출처)
except Exception:                                  # 단독 실행 등 import 실패 시에만 폴백
    PATH = re.compile(r"^/specs\[(\d+)\]/([A-Za-z_]+)(?:/([A-Za-z_]+))?"
                      r"(?:\[(\d+)\])?(?:/([A-Za-z_]+))?$")
    parse_path = None
_SPEC = [None]


def spec():
    if _SPEC[0] is None:
        _SPEC[0] = json.load(io.open(V3, encoding="utf-8"))
    return _SPEC[0]


def _strings(o, out):
    if isinstance(o, dict):
        [_strings(v, out) for v in o.values()]
    elif isinstance(o, list):
        [_strings(v, out) for v in o]
    elif isinstance(o, str):
        out.append(o)


def locate(path):
    u"""경로가 가리키는 값을 돌려준다(없으면 None). 배치가 `old` 를 확인할 때도 쓴다."""
    pp = parse_path(path) if parse_path else None
    if not pp:
        raise ValueError(u"경로 형식 오류: %s" % path)
    i, outer, field, idx, key = pp
    r = _locate_in(spec()["specs"][i], outer, field, idx, key, v2=False)
    if r is None:
        # ★09-15(22차·23차 적발 · `sig/abi`·스칼라 `sig/ret` force 우회 6건): v3 에 없는 키(재생성 전 `abi` ·
        #   v2 이름 `returns`)는 **v2 정본으로 폴백**해 old 를 검증한다. 조회=v3 우선, 없을 때만 v2.
        try:
            r = _locate_in(spec2()["specs"][i], outer, field, idx, key, v2=True)
        except Exception:
            r = None
    return r


_V2KEY = {"sig": "signature", "role": "note", "ty": "type", "ret": "returns"}
_SPEC2 = [None]
V2 = os.path.join(HERE, "_spec", "specs20.json")


def spec2():
    if _SPEC2[0] is None:
        _SPEC2[0] = json.load(io.open(V2, encoding="utf-8"))
    return _SPEC2[0]


def _locate_in(sp, outer, field, idx, key, v2):
    k = (lambda x: _V2KEY.get(x, x)) if v2 else (lambda x: x)
    if outer:
        # ⚠v3 조회에서는 v2 매핑(`sig`→`signature`)을 하지 않는다 — 경로의 이름이 곧 키다(조회=v3 / 적용=v2).
        sp = sp.get(k(outer)) or {}
    if idx is None:
        return sp.get(k(field))
    arr = sp.get(k(field))
    if not isinstance(arr, list) or int(idx) >= len(arr):
        return None
    row = arr[int(idx)]
    return (row.get(k(key)) if isinstance(row, dict) else None) if key else row


class Patch(object):
    def __init__(self, round, batch):
        self.round, self.batch = str(round), batch
        self.errors, self.ev_up, self.brief_errors = [], [], []
        self.warn = []

    # ── 정정 ──────────────────────────────────────────────────────────
    def fix(self, path, old, new, evidence, behavior_change=False,
            found_by="reused", kind=u"실오류", force=False):
        tgt = locate(path)
        ok = False
        if isinstance(tgt, str):
            ok = str(old) in tgt
        elif isinstance(tgt, dict):
            ok = any(isinstance(v, str) and str(old) in v for v in tgt.values())
        elif isinstance(tgt, list):
            # ★09-16(24차 C·D·F 적발): v3 `open`/`notes` 는 dict 리스트라 `==` 비교가 항상 거짓 → force 로만 통과했다.
            #   리스트면 그 안 문자열 잎을 전부 훑어 old 포함 여부로 판정한다.
            blob = []
            _strings(tgt, blob)
            ok = any(str(old) in x for x in blob)
        elif tgt is not None:
            ok = (tgt == old)            # 값 필드는 동등 비교
        elif u"/open" in path or u"/notes" in path:
            blob = []
            _strings(spec()["specs"][int(PATH.match(path).group(1))], blob)
            ok = any(str(old) in s for s in blob)
        if not ok and not force:
            raise AssertionError(
                u"`old` 가 정본에 없다 — %s\n   찾은 값: %r\n   준 old : %r\n"
                u"   (정본을 다시 보고 정확한 문면을 복사하라. 정말 맞다면 force=True)"
                % (path, (tgt if not isinstance(tgt, str) else tgt[:120]), old))
        if not ok:
            self.warn.append(path)
        self.errors.append({"path": path, "kind": kind, "old": old, "new": new,
                            "evidence": evidence, "behavior_change": bool(behavior_change),
                            "found_by": found_by})
        return self

    # ── ev 상향 ───────────────────────────────────────────────────────
    def ev(self, path, evidence, to=2, frm=4, found_by="reused"):
        # ★2026-09-13 정정(15차 4배치 전부 적발): 옛 코드는 `m.group(3) is None` 을 「배열 원소가 아니다」로 읽었는데
        #   PATH 의 group(3) 은 **바깥 세그먼트(outer)** 이고 인덱스는 group(4) 다 ⟹ `/specs[i]/mem[j]` 같은 2단 경로가
        #   전부 거부됐다(배치들이 dict 직접 삽입으로 우회). parse_path 로 (i, outer, field, idx, key) 를 받아 idx 로 판정한다.
        pp = parse_path(path)
        if not pp or pp[3] is None:
            raise ValueError(u"ev 상향은 배열 원소만: %s" % path)
        if locate(path) is None:
            raise AssertionError(u"그 행이 없다 — %s" % path)
        if pp[2] == "mem" and int(to) < 3:
            to = 3                        # 오프셋의 정본은 tcx — ev3 이 상한
        self.ev_up.append({"path": path, "from": int(frm), "to": int(to),
                           "evidence": evidence, "found_by": found_by})
        return self

    def brief_error(self, text):
        self.brief_errors.append(text)
        return self

    # ── 저장 ──────────────────────────────────────────────────────────
    def save(self, where=None):
        out = where or os.path.join(HERE, "_verify%s" % self.round, self.batch, "patch.json")
        d = os.path.dirname(out)
        if not os.path.isdir(d):
            os.makedirs(d)
        miss = [e["found_by"] for e in self.errors if e.get("found_by") not in
                ("reused", "inherited", "new")]
        if miss:
            raise AssertionError(u"`found_by` 가 잘못됐다(%s) — reused/inherited/new 중 하나" % miss[:3])
        io.open(out, "w", encoding="utf-8").write(json.dumps(
            {"round": int(self.round), "batch": self.batch, "errors": self.errors,
             "ev_up": self.ev_up, "brief_errors": self.brief_errors},
            ensure_ascii=False, indent=1))
        print(u"%s  (정정 %d · ev상향 %d · 브리핑오류 %d%s)"
              % (out, len(self.errors), len(self.ev_up), len(self.brief_errors),
                 u" · ⚠force %d" % len(self.warn) if self.warn else u""))
        print(u"⟹ 제출 전 반드시: python -X utf8 applypatch.py %s --only %s --dry"
              % (self.round, self.batch))
        return out


if __name__ == "__main__":
    print(__doc__)
