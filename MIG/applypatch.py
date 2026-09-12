# -*- coding: utf-8 -*-
u"""applypatch — 배치가 낸 **기계 판독 정정**(`patch.json`)을 v2 정본에 적용한다. (2026-09-11 신설)

## 왜 만드나 — 보고↔반영 경계에서 계속 샜다
| 라운드 | 유실 | 원인 |
|---|---|---|
| 1차 | 정정 13건 중 10건이 표에 안 들어감 | 내가 `resolved` 에만 적음 |
| 3차 | 정정이 **항목 이동만** 되고 본문은 옛 결론 | 손으로 옮기다 절반만 |
| 3차 | §7 ev 상향 8건 **전부 미반영** | 산문 권고라 아무도 실행 안 함 |
| 4차 | `patch4.py` 가 **배치 A 를 통째로 누락** | 손으로 옮기다 배치 하나를 빼먹음 |
| 5차 | ev 상향 **492행 중 317행 유실** | 배치가 집계표로 보고, 행 목록 없음 |

전부 **사람이 산문을 읽고 손으로 옮겨 적는 구간**에서 났다.
⟹ 배치는 **구조화 패치**를 내고, 적용은 **기계**가 한다. 사람은 승인만 한다.

## 계약 — `_verify<N>/<배치>/patch.json`
```json
{ "round": 6, "batch": "A",
  "errors": [
    { "path": "/specs[0]/logic",          // v3 경로 그대로 (v2 로는 이 도구가 옮긴다)
      "kind": "실오류",                    // 실오류 | 판정반전 | 분류오류 | 보강
      "old": "<정본에 실제로 있는 부분문자열>",
      "new": "<대체 문자열>",
      "evidence": "오라클 A5_o3.tsv 26/26 · IR m04.ll:44245",
      "behavior_change": true,            // ★이 명세**대로 재구현하면 틀린 동작**이 나오나
                                          //   (「지금 재현 코드가 바뀌나」가 아니다 — 7차에 네 배치가
                                          //    그렇게 읽어 전건 false 로 냈는데 실제 8건이 재구현을 틀리게 했다)
      "found_by": "reused" }              // reused | inherited | new  ★라운드 실험용
  ],
  "ev_up": [ { "path": "/specs[16]/mem[0]", "from": 4, "to": 2,
               "evidence": "D5_o16.tsv 122/122 MISMATCH 0" } ],
  "brief_errors": [ "…" ] }
```
`old` 가 정본에 **없으면 적용하지 않고 실패로 보고**한다(조용한 no-op 금지 — 4차 `closelist` 사고).

## ★`ev_up` 은 숫자를 직접 쓰지 않는다
`ev` 는 **근거 문면에서 파생**되는 설계다(`mkspec3.evtier`). 그래서 이 도구는 대상 행의 근거란에
`· 오라클 실행 확증(<라운드>차 배치<B>, <evidence>)` 를 **덧붙인다.** 그러면 다음 빌드에서 ev 가 내려온다.
숫자를 직접 박으면 `history` 에 "ev 4→2 하라"를 적어놓고 표는 안 따라간 5차 사고(G11)가 재발한다.

사용:
  python -X utf8 applypatch.py 6            # 전 배치
  python -X utf8 applypatch.py 6 --dry      # 적용 없이 검사만
  python -X utf8 applypatch.py 6 --only D
"""
import hashlib, io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
P2 = os.path.join(HERE, "_spec", "specs20.json")
# ★3단 경로도 받는다 — `/specs[9]/sig/params[0]/note` 처럼.
#   초판은 2단까지만 받아서 `signature.params[].note` 6조각을 **주소지정조차 못 했다**
#   (6차 정리 배치가 적발). 계약이 못 가리키는 자리는 영원히 안 고쳐진다.
PATH = re.compile(r"^/specs\[(\d+)\]/([A-Za-z_]+)(?:/([A-Za-z_]+))?"
                  r"(?:\[(\d+)\])?(?:/([A-Za-z_]+))?$")


def parse_path(p):
    u"""`/specs[i]/field[j]/key` 와 `/specs[i]/outer/field[j]/key` 를 함께 받는다.
    돌려주는 것 = (i, outer|None, field, idx|None, key|None)."""
    m = PATH.match(p)
    if not m:
        return None
    i, a, b, idx, key = m.groups()
    if b:                       # `/specs[i]/sig/params[j]/note`
        return int(i), a, b, idx, key
    return int(i), None, a, idx, key


def resolve(spec, field, idx, create=False):
    u"""v3 경로 → v2 (배열, 인덱스). v3 는 v2 의 두 배열을 이어 붙인 것이라 경계를 넘긴다.
    ⚠경계 계산이 틀리면 **엉뚱한 행을 고친다** — 길이를 세서 확인하고, 벗어나면 예외.

    ★★`create=True` 는 **삽입 전용**이다. (11차 배치B 적발 — 조용한 no-op **여섯 번째**)
      `spec.get("writes") or []` 는 **빈 배열이 falsy 라 새 객체**를 돌려준다.
      거기 `insert` 하면 그 임시 리스트에만 들어가고 **정본은 그대로**인데 `--dry` 는 성공으로 찍는다.
      10차에 `/specs[8]/mem at=21`(`writes` 가 0행)이 정확히 그렇게 **통째로 사라졌고**,
      게이트 19개는 0 이었고 보고서엔 「행 추가 완료」라고 적혔다.
      ⟹ 삽입할 때는 **정본의 그 키에 실제 리스트를 만들어** 돌려준다."""
    def _a(k):
        v = spec.get(k)
        if isinstance(v, list):
            return v
        if create:
            spec[k] = []
            return spec[k]
        return v or []
    if field == "mem":
        r, w = _a("reads"), _a("writes")
        return (r, idx) if idx < len(r) else (w, idx - len(r))
    if field == "knobs":
        k, n = _a("knobs"), _a("new_knobs")
        return (k, idx) if idx < len(k) else (n, idx - len(k))
    if field == "consts":
        return spec.get("constants") or [], idx
    if field == "history":
        return spec.get("resolved") or [], idx
    if field in ("open", "notes"):
        # ⚠open/notes 는 v3 가 `unknown` 을 **걸러 만든 것**이라 인덱스가 안 맞는다.
        #   경로로 찾지 말고 `old` 문면으로 찾아야 한다 → 호출부에서 처리.
        return None, None
    return spec.get(field), idx


# ★★**v3 키 → v2 키 매핑.** (8차 배치C 적발)
#   배치는 **v3 경로**로 쓰고(`mkdossier`·`mkspec3_md` 가 v3 를 보여주니까),
#   이 도구는 **v2 에 적용**한다. 그런데 배열 원소 안의 **키 이름이 다른 경우**가 있어
#   `/specs[i]/sig/params[j]/role` 이 v2 에서 **`note`** 를 찾아야 하는데 `role` 을 찾아 실패했다.
#   실측(배치C): role 판 7/10 vs note 판 10/10.
#   ⟹ 바깥 컨테이너만 매핑(`sig`→`signature`)하고 **키는 안 하던 것**이 구멍이었다.
# ★산문이 인덱스로 표를 가리키는 이름들. v2 키와 v3 키를 둘 다 받는다.
#   (`history` 가 `consts[2]` 라고 적으면 v2 에선 `constants[2]` 다 — 배치가 쓰는 말은 v3 다.)
_PROSE_FIELDS = ("resolved", "unknown", "still_unknown", "logic", "one_line", "no_map_reason")
_ALIAS = {"constants": ("consts", "constants"), "reads": ("mem", "reads"),
          "writes": ("mem", "writes"), "knobs": ("knobs",), "new_knobs": ("new_knobs",),
          "params": ("params", "sig.params")}


def _shift_prose(cont, which, pos):
    u"""`insert` 로 밀린 배열을 **산문이 인덱스로 가리키던 것**까지 같이 민다.

    ★왜 필요한가 (12차 배치A) — `--restamp` 는 도장만 다시 잡는다. 표를 가리키는
      **문자열**(`history` 의 「`consts[2]` 를 ev 4→2 로」)은 그대로 남아 다른 행을 가리키고,
      G11·G17 이 그 문자열을 파싱하므로 **없는 오류를 영구히 적발**한다.
    ⚠`mem` 은 v3 에서 `reads`+`writes` 를 이어 붙인 것이라 v2 `writes` 삽입은
      v3 `mem` 인덱스를 `len(reads)+pos` 부터 민다. 그 보정까지 한다."""
    names = _ALIAS.get(which)
    if not names:
        return 0
    base = {"writes": len(cont.get("reads") or []) - 1}.get(which, 0)  # 이미 삽입된 뒤라 -1
    n = 0
    for f in _PROSE_FIELDS:
        v = cont.get(f)
        if not isinstance(v, (str, type(u""))):
            continue
        out = v
        for nm in names:
            shift = base if (nm == "mem" and which == "writes") else 0

            def _bump(m, _nm=nm, _sh=shift):
                k = int(m.group(1))
                return u"%s[%d]" % (_nm, k + 1) if k >= pos + _sh else m.group(0)

            out = re.sub(re.escape(nm) + r"\[(\d+)\]", _bump, out)
        if out != v:
            n += sum(1 for _ in re.finditer(r"\[\d+\]", v))
            cont[f] = out
    return n


V2KEY = {"role": "note", "ty": "type"}

# ★경고 — **실패가 아니다.** 별도 통에 넣는다. (10차 배치B·C 가 둘 다 지적)
WARN = []

_V3CACHE = [None]


def _ev_now(path):
    u"""그 경로의 **현재 `ev`**(v3 파생값). `ev_up.from` 검산용."""
    try:
        if _V3CACHE[0] is None:
            _V3CACHE[0] = json.load(io.open(
                os.path.join(HERE, "_spec", "specs20_v3.json"), encoding="utf-8"))
        m = re.match(r"^/specs\[(\d+)\]/([A-Za-z_]+)(?:/([A-Za-z_]+))?\[(\d+)\]", path or u"")
        if not m:
            return None
        i, a, b, j = int(m.group(1)), m.group(2), m.group(3), int(m.group(4))
        sp = _V3CACHE[0]["specs"][i]
        arr = (sp.get(a) or {}).get(b) if b else sp.get(a)
        if isinstance(arr, list) and j < len(arr):
            return (arr[j] or {}).get("ev")
    except Exception:
        pass
    return None


_MISS = object()          # ★「경로가 가리키는 칸이 없다」 — spec 전체와 구별해야 한다


def _subtree(spec, path):
    u"""경로가 가리키는 **그 항목**만 돌려준다. 칸이 없으면 `_MISS`.

    ★★**없는 칸을 「spec 전체」로 폴백하면 안 된다.** (9차 배치C 적발 — `already` 의 **다섯 번째** 구멍)
      `old: null` 로 **없는 키를 새로 채우는** 패치에서 대상이 None 이라 spec 전체를 훑었고,
      `new` 가 명세 어딘가에 우연히 겹치면 **아직 안 채운 값이 「이미 적용」으로 조용히 버려졌다**
      (배치C 의 첫 dry-run 이 실제로 `(이미 적용 2)` 를 냈다).
      칸이 없으면 **미적용이 확실**하다 — 폴백할 이유가 없다."""
    pp = parse_path(path or u"")
    if not pp:
        return spec
    i, outer, field, idx, key = pp
    t = spec
    if outer:
        t = t.get({"sig": "signature"}.get(outer, outer)) or {}
    if idx is not None:
        arr, jj = resolve(t, field, int(idx))
        if arr is None or jj is None or jj >= len(arr):
            return _MISS
        t = arr[jj]
        if key and isinstance(t, dict):
            t = t.get(V2KEY.get(key, key), t.get(key, _MISS))
    else:
        t = t.get(field, _MISS)
    return _MISS if t is None else t


def already(spec, e):
    u"""★**이미 적용됐나**를 먼저 본다 — 재실행 안전성.

    치환식 정정은 두 번 돌리면 `old` 가 없어서 「실패」로 찍힌다. 그러면 재실행이 두려워지고,
    두려우면 안 돌리게 되고, 안 돌리면 반영이 샌다(이 프로젝트가 6라운드 겪은 그 고리).
    ⟹ `new` 가 이미 있으면 **성공도 실패도 아닌 「이미 적용」**으로 센다.
    """
    # ★★**정수·불리언 패치는 여기서 판정하지 않는다.** (7차 배치D 적발)
    #   `new` 가 int 면 `[:80]` 이 `TypeError: 'int' object is not subscriptable` 로 죽어
    #   **그 배치의 보고가 통째로 날아간다.** `mkpatch.fix` 는 `old` 를 int 로 내라고 하고
    #   여기서는 int 면 죽었으니 **두 도구의 계약이 정면으로 상충**하고 있었다
    #   (7차 배치D 의 최대 수확 2건이 전부 `src_line` 정수 패치였다).
    #   값 필드는 아래 `apply_error` 가 **현재 값과 직접 비교**하므로 판정이 불필요하다.
    nv = e.get("new")
    ov = e.get("old")
    if not isinstance(nv, str):
        return False

    # ★★★**부분문자열 판정 자체를 버린다.** (8차 배치B 적발 — 이 함수의 **세 번째** 결함)
    #
    #   `new[:80]` 이 대상 필드 안에 있으면 「이미 적용」으로 셌다. 그런데 정정이
    #   **긴 `meaning` 의 꼬리만 고치는 것**이면 앞 80자는 당연히 그대로 있으므로
    #   **아직 안 고친 정정이 조용히 버려진다.** 8차 배치B 의 가장 확실한 2건
    #   (`06 consts[0]`·`07 consts[1]`)이 실제로 그렇게 삼켜졌다(`정정 47/47 … (이미 적용 2)`).
    #
    #   ⟹ 「이미 적용」은 **`old` 가 사라졌고 `new` 가 통째로 들어 있을 때**만이다.
    #      `old` 가 아직 있으면 그건 **안 고쳐진 것**이고, `apply_error` 가 정상 처리한다.
    #      (이 도구의 존재 이유가 「조용한 no-op 금지」인데 이 함수가 세 번이나 그걸 뚫었다:
    #       ①명세 전체를 훑음 ②정수에서 TypeError ③여기 — 전부 **판정을 느슨하게 해서** 생겼다.)
    sub = _subtree(spec, e.get("path") or u"")
    if sub is _MISS:
        return False              # ★칸이 아예 없다 = 미적용이 확실하다(9차 배치C)
    if isinstance(ov, str) and ov:
        if ov in json.dumps(sub, ensure_ascii=False):
            return False          # 옛 값이 아직 있다 = 안 고쳐졌다
    new = nv

    # ★★**대상 필드만 본다.** (7차 배치A 적발)
    #   전에는 명세 **전체**를 훑어서, 다른 항목에 우연히 같은 문구가 있으면
    #   아직 안 고친 정정이 「이미 적용」으로 **조용히 버려졌다**(실측 2건).
    #   이 도구의 존재 이유가 「조용한 no-op 금지」인데 그걸 이 함수가 뚫고 있었다.
    #   ⟹ 경로가 가리키는 **그 항목**(배열 원소면 그 원소, 필드면 그 필드)만 훑는다.
    #   ⚠**같은 계산을 두 벌 두면 한쪽만 고쳐진다.** 초판은 여기에 경로 해석을 인라인으로
    #     복사해 두었고, 9차에 `_subtree` 쪽만 `_MISS` 로 고쳐서 **이 블록은 여전히
    #     spec 전체로 폴백**하고 있었다(`mkpatch`↔`applypatch` 경로 문법 때와 같은 형태).
    tgt = sub
    blob = []

    def walk(o):
        if isinstance(o, dict):
            [walk(v) for v in o.values()]
        elif isinstance(o, list):
            [walk(v) for v in o]
        elif isinstance(o, str):
            blob.append(o)
    walk(tgt)
    if any(new in s for s in blob):
        return True
    # ★`cleanspec` 이 라운드 끝에 돌면 본문의 출처 표기가 사라져 `patch.json` 문자열과 어긋난다.
    #   그러면 이미 반영된 정정이 「미반영」으로 보인다(실측 12/34).
    #   ⟹ **양쪽에 같은 정리를 걸어** 다시 본다. ⚠조각이 아니라 **필드 전체**에 걸어야 한다 —
    #      문맥 의존 정규식을 60자 조각에 걸었더니 오히려 오탐이 늘었다(auditrounds 실측 0→3).
    try:
        import cleanspec as CS
        nc = CS.clean(e.get("new") or u"", [])[:80]
        if nc and any(nc in CS.clean(s, []) for s in blob):
            return True
    except Exception:
        pass
    return False


def field_now(D, path):
    u"""그 경로가 가리키는 **현재 값**을 문자열로. 도장(stamp)용."""
    pp = parse_path(path)
    if not pp:
        return None
    i, outer, field, idx, key = pp
    spec = D["specs"][i]
    if outer:
        spec = spec.get({"sig": "signature"}.get(outer, outer)) or {}
    if idx is None:
        v = spec.get(field)
        if isinstance(v, str):
            return v
        for sname in (("unknown", "still_unknown") if field in ("open", "notes") else (field,)):
            arr = spec.get(sname)
            if isinstance(arr, list):
                return json.dumps(arr, ensure_ascii=False)
        return None
    if field in ("open", "notes"):
        return json.dumps((spec.get("unknown") or []) + (spec.get("still_unknown") or []),
                          ensure_ascii=False)
    arr, jj = resolve(spec, field, int(idx))
    if arr is None or jj is None or jj >= len(arr):
        return None
    row = arr[jj]
    return json.dumps(row.get(key) if key else row, ensure_ascii=False)


def apply_error(D, e, log, skipped=None):
    pp = parse_path(e["path"])
    if not pp:
        log.append((u"경로 형식 오류", e["path"], u"")); return False
    i, outer, field, idx, key = pp
    spec = D["specs"][i]

    # ★★**행 추가·삭제** — 8차 배치C 가 「도구에 이 연산이 없어 못 고친다」로 막혔다.
    #   `sig.params` 에 sret out-ptr 행을 넣어야 `G16` 이 닫히는데, 스칼라·문면·키 치환뿐이라
    #   배치가 **인접 행 `note` 에 사실을 적어 두는 것으로 갈음**해야 했다(판정 `보류`).
    #   ⟹ `op` 를 받는다: `insert`(`at` 위치에 `new` 객체 삽입) / `delete`(`idx` 행 제거).
    #   ⚠`insert` 는 **되돌리기 어렵다.** `old` 가 아니라 **`guard`**(그 자리에 이미 없어야 할 표식)를
    #     요구해 재실행 시 중복 삽입을 막는다.
    op = e.get("op")
    if op in ("insert", "delete"):
        cont = spec
        if outer:
            cont = cont.get({"sig": "signature"}.get(outer, outer))
            if cont is None:
                log.append((u"바깥 컨테이너 없음", e["path"], u"")); return False
        # ★★**`at` 은 v3 인덱스다 — 그걸로 대상 배열을 골라야 한다.** (10차 배치D 적발)
        #   초판은 `resolve(cont, field, 0)` 이라 **항상 첫 배열**(`mem`→`reads` · `knobs`→`knobs`)에 꽂혔다.
        #   v3 `mem` 은 `reads`+`writes` 를 이어 붙인 것이라, 쓰기 행을 넣으려 해도 읽기 쪽에 들어간다.
        #   배치D 가 `/specs[18]/writes` 로 **경로를 우회**해야 했다(계약을 도구가 강제하지 못한 것).
        at0 = int(e.get("at", 0))
        arr, jj = resolve(cont, field, at0, create=(op == "insert"))
        if not isinstance(arr, list):
            log.append((u"배열이 아니다", e["path"], u"")); return False
        if op == "insert":
            # ★guard 는 **행 전체 문자열이 아니라 「그 행을 식별하는 키」**로 본다.
            #   초판은 `json.dumps(row)` 에 부분문자열이 있으면 「이미 있다」로 판정했는데,
            #   8차 배치C 가 **인접 행 `note` 에 「이 표는 sret 를 빠뜨렸다」고 적어 둔 것**을
            #   삽입 완료로 오판했다 — 느슨한 판정이 조용한 no-op 을 만드는 네 번째 사례다.
            g = e.get("guard")
            gk = e.get("guard_key", "name")          # 기본: 그 키의 값이 정확히 일치하는 행
            if g and any((r.get(gk) if isinstance(r, dict) else None) == g for r in arr):
                if skipped is not None:
                    skipped.append(e["path"])
                    return None                      # 이미 들어 있다(재실행 안전)
                log.append((u"guard 가 이미 있다 — 중복 삽입 거부", e["path"], g[:60]))
                return False
            row = e.get("new")
            if not isinstance(row, dict):
                log.append((u"insert 의 `new` 는 객체여야 한다", e["path"], u"")); return False
            pos = max(0, min(int(jj if jj is not None else at0), len(arr)))
            arr.insert(pos, row)

            # ★★★**적용했다고 믿지 말고 「정본에서 다시 읽어」 확인한다.** (11차 배치B)
            #   `resolve` 가 사본을 돌려주던 탓에 10차 삽입 1건이 **통째로 사라졌는데**
            #   `--dry` 는 성공, 게이트는 0, 보고서엔 「완료」라고 적혔다.
            #   ⟹ 「고쳤다」를 도구가 스스로 재측정한다(`selftest` 가 도구에 한 일을 정정에도).
            g2 = e.get("guard")
            gk2 = e.get("guard_key", "name")
            back = json.dumps(D["specs"][i], ensure_ascii=False)
            if g2 and g2 not in back:
                log.append((u"★삽입이 정본에 반영되지 않았다 — 사본에 꽂혔을 수 있다",
                            e["path"], str(g2)[:60]))
                return False
            # 어느 v2 배열에 들어갔는지 결과에 찍는다(배치가 `at` 을 고를 때 필요하다)
            which = u"?"
            for k2 in ("reads", "writes", "knobs", "new_knobs", "constants", "resolved",
                       "params"):
                if cont.get(k2) is arr:
                    which = k2
                    break
            # ★★**삽입은 뒤 인덱스를 전부 밀어 「인덱스 경로」를 무효화한다.** (9차 실측)
            #   `/specs[15]/mem[12]` 를 넣었더니 5차·7차가 찍어 둔 같은 경로의 도장이
            #   **다른 행을 가리키게 돼** `auditrounds` 가 STALE 1 · ev되돌아감 1 을 냈다
            #   (내용은 멀쩡한데 지표만 빨간불 — 「지표가 무엇을 세는지」 문제의 재발).
            #   ⟹ 삽입한 라운드는 **반드시 `--restamp` 로 기준선을 다시 잡아야 한다.**
            # ★**경고를 「적용 실패」 통에 넣지 마라.** (10차 배치B·C 가 둘 다 지적)
            #   `log` 는 실패 목록이라 삽입 성공이 「★적용 실패 8건」으로 찍혔고,
            #   바로 다음 줄의 「정정 14 성공 / 0 실패」와 모순돼 **되돌릴 뻔했다**.
            #   「빨간불이 상수가 되면 아무도 안 본다」(§S5-f)가 **이 도구 자신에게** 생긴 사례다.
            # ★★★**산문 속 인덱스 참조도 같이 민다.** (12차 배치A 적발)
            #   `--restamp` 는 도장만 다시 잡고 `resolved`/`unknown` **본문의 `consts[N]`**
            #   문자열은 손대지 않는데, G11·G17 은 바로 그 문자열을 파싱한다.
            #   ⟹ 11차가 `/specs[3]/consts` 에 한 행을 넣자 `history[9]` 의
            #      「`consts[2]`·`consts[3]` 를 ev 4→2 로」가 **다른 행을 가리키게 돼**
            #      12차 G11·G17 이 **없는 오류 2건**을 적발했다.
            #   이것이 11차 결론 「정정은 오류를 옮기지, 없애지 않는다」의 기계적 실체다.
            n_shift = _shift_prose(cont, which, pos)
            if n_shift:
                WARN.append((u"산문 속 인덱스 참조 %d곳을 같이 밀었다" % n_shift,
                             e["path"], u"`%s[k]`(k≥%d) → `[k+1]`" % (which, pos)))
            WARN.append((u"삽입 → **v2 `%s`[%d]** 에 들어갔다 · 뒤 인덱스가 밀렸다"
                         u"(끝나고 `--restamp`)" % (which, pos),
                         e["path"], u"at=%d · v2 기준 이후 %d행 이동"
                         u"(⚠v3 는 두 배열을 이어 붙이므로 더 밀릴 수 있다 — "
                         u"`mem` 의 `at` 이 `len(reads)` 이상이면 **`writes` 로 간다**)"
                         % (at0, len(arr) - pos - 1)))
            return True
        jj = int(idx) if idx is not None else None
        if jj is None or jj >= len(arr):
            log.append((u"delete 인덱스 범위 밖", e["path"], u"")); return False
        g = e.get("guard")
        if g and g not in json.dumps(arr[jj], ensure_ascii=False):
            log.append((u"delete guard 불일치 — 엉뚱한 행을 지울 뻔했다", e["path"], g[:60]))
            return False
        arr.pop(jj)
        return True

    if outer:
        # v3 `sig` = v2 `signature`. 바깥 컨테이너로 내려간다.
        spec = spec.get({"sig": "signature"}.get(outer, outer)) or {}
    old, new = e["old"], e["new"]
    if skipped is not None and already(spec, e):
        skipped.append(e["path"]); return None        # None = 이미 적용

    # ① 스칼라 필드 (logic / one_line …)
    if idx is None and key is None:
        cur = spec.get(field)
        # ★**값을 「없음」으로 만들거나, 없던 객체를 새로 채우는** 두 경우. (13차 RVA2 배치 신설)
        #   쓰임: `exe.addr` 을 **null** 로 내리기(= 「이 주소는 이 함수가 아니다」를 기록) ·
        #        `exe` 자체가 null 인 항목에 **객체를 만들어 넣기**(`#13` 은 exe 가 아예 없었다).
        #   ⚠가드: `old` 가 **현재 값과 정확히 일치**할 때만 적용한다(부분일치·타입 추측 금지).
        #     이걸 느슨하게 하면 「덮어썼는데 성공으로 찍히는」 조용한 오적용이 생긴다.
        if e.get("new") is None and not isinstance(e.get("new"), str):
            if cur != e.get("old"):
                log.append((u"`old` 가 정본과 다르다(현재 %r) — null 화 거부" % (cur,),
                            e["path"], repr(e.get("old"))[:60]))
                return False
            spec[field] = None
            return True
        if isinstance(e.get("new"), dict):
            if cur not in (None, {}):
                log.append((u"그 필드가 이미 채워져 있다 — 객체 통째 교체 거부(현재 %r)" % (cur,),
                            e["path"], u""))
                return False
            spec[field] = e["new"]
            return True
        # ★★**dict 안의 정수·불리언 필드도 고칠 수 있어야 한다.** (13차 RVA 배치 적발)
        #   6차 배치A 에서 「정수도 고쳐야 한다」를 배워 **③ 배열 원소 분기에만** conv 를 넣었고,
        #   여기(① 스칼라)는 문자열 전용으로 남았다. 그래서 `/specs[4]/exe/bytes`(337→1295) 가
        #   「필드가 문자열이 아니고 문면도 못 찾음」으로 **실패**했다.
        #   ⟹ ★같은 교훈을 **한쪽 축에만 적용**한 것이다 — 규칙을 배웠다는 것과
        #      그 규칙이 닿는 모든 자리에 적용됐다는 것은 다르다(이 세션 네 번째 재발).
        #   ⚠list 는 아래 문면 탐색(open/notes)이 처리하므로 여기서 건드리지 않는다.
        if cur is not None and not isinstance(cur, (str, list, dict)):
            def _conv(v, like):
                if isinstance(like, bool):
                    return str(v).strip().lower() in ("true", "1")
                if isinstance(like, int):
                    return int(str(v).strip(), 0)
                if isinstance(like, float):
                    return float(str(v).strip())
                return v
            try:
                want = _conv(old, cur)
            except Exception:
                log.append((u"`old` 를 그 필드 타입으로 못 읽는다", e["path"], str(old)[:60]))
                return False
            if cur != want:
                log.append((u"`old` 값이 정본과 다르다(현재 %r)" % cur, e["path"], str(old)[:60]))
                return False
            try:
                spec[field] = _conv(new, cur)
            except Exception:
                log.append((u"`new` 를 그 필드 타입으로 못 읽는다", e["path"], str(new)[:60]))
                return False
            return True
        # ★**리스트 필드 전체 교체**(13차 RVA 배치 신설) — `exe.callers`/`exe.callees` 처럼
        #   방향이 뒤바뀐 필드는 **원소 치환으로는 고칠 수 없다**(키 사이를 옮겨야 한다).
        #   `old`/`new` 가 JSON 배열이면 **현재 값과 완전일치할 때만** 통째로 바꾼다.
        #   ⚠완전일치를 요구하는 이유 = 부분일치 허용은 「엉뚱한 줄을 고치고도 성공으로 찍는」
        #     조용한 오적용 경로를 만든다(이 도구가 존재하는 이유 그 자체).
        if isinstance(cur, list) and isinstance(old, str) and old.strip().startswith("["):
            try:
                want, repl = json.loads(old), json.loads(new)
            except Exception:
                log.append((u"`old`/`new` 를 JSON 배열로 못 읽는다", e["path"], str(old)[:60]))
                return False
            if cur != want:
                log.append((u"`old` 배열이 정본과 다르다(현재 %r)" % (cur,), e["path"], str(old)[:60]))
                return False
            spec[field] = repl
            return True
        if not isinstance(cur, str):
            # open/notes 처럼 배열이면 문면으로 찾는다
            srcs = (("unknown", "still_unknown") if field in ("open", "notes") else (field,))
            for sname in srcs:
                arr = spec.get(sname)
                if isinstance(arr, list):
                    for j, v in enumerate(arr):
                        if isinstance(v, str) and old in v:
                            arr[j] = v.replace(old, new); return True
            log.append((u"필드가 문자열이 아니고 문면도 못 찾음", e["path"], old[:60])); return False
        if old not in cur:
            log.append((u"`old` 가 정본에 없다", e["path"], old[:60])); return False
        spec[field] = cur.replace(old, new); return True

    # ② open/notes — 인덱스 무시하고 문면으로
    if field in ("open", "notes"):
        # ★v3 의 `open`/`notes` 는 v2 의 `unknown` **과 `still_unknown`** 을 합친 것이다.
        #   초판이 `unknown` 만 뒤져서 `still_unknown` 의 2조각을 못 고쳤다(6차 정리 배치 적발).
        for src in ("unknown", "still_unknown"):
            arr = spec.get(src) or []
            for j, v in enumerate(arr):
                if isinstance(v, str) and old in v:
                    arr[j] = v.replace(old, new); return True
        log.append((u"open/notes 문면 못 찾음", e["path"], old[:60])); return False

    # ③ 배열 원소
    arr, jj = resolve(spec, field, int(idx))
    if arr is None or jj is None or jj >= len(arr):
        log.append((u"인덱스 범위 밖(v3→v2 경계 확인)", e["path"], u"")); return False
    row = arr[jj]
    if key:
        # ★**v3 키를 v2 키로 옮긴다.** (8차 배치C 적발)
        #   배치는 v3 경로로 쓰는데(`/…/params[j]/role`) v2 의 그 칸 이름은 `note` 다.
        #   바깥 컨테이너만 매핑하고 키는 안 해서 `sig.params` 정정이 3/10 실패했다.
        #   ⚠원래 키가 그 행에 실제로 있으면 그쪽을 우선한다(오검출 방지).
        if key not in row and V2KEY.get(key) in row:
            key = V2KEY[key]
        cur = row.get(key)
        # ★**정수·불리언 필드도 고칠 수 있어야 한다.** (6차 배치A 적발)
        #   초판은 문자열 치환만 해서 그 배치 최대 수확인 `consts[].src_line` 정수 4건을
        #   구조로 못 고쳤다. `old`/`new` 가 숫자 문자열이면 원래 타입으로 되돌려 넣는다.
        if not isinstance(cur, str):
            def conv(v, like):
                if isinstance(like, bool):
                    return str(v).strip().lower() in ("true", "1")
                if isinstance(like, int):
                    return int(str(v).strip())
                if isinstance(like, float):
                    return float(str(v).strip())
                return v
            try:
                want = conv(old, cur)
            except Exception:
                log.append((u"`old` 를 그 필드 타입으로 못 읽는다", e["path"] + u"/" + key, str(old)[:60]))
                return False
            if cur != want:
                log.append((u"`old` 값이 정본과 다르다(현재 %r)" % cur, e["path"] + u"/" + key, str(old)[:60]))
                return False
            try:
                row[key] = conv(new, cur)
            except Exception:
                log.append((u"`new` 를 그 필드 타입으로 못 읽는다", e["path"] + u"/" + key, str(new)[:60]))
                return False
            return True
        if old not in cur:
            log.append((u"`old` 가 그 키에 없다", e["path"] + u"/" + key, old[:60])); return False
        row[key] = cur.replace(old, new); return True
    hit = [k for k, v in row.items() if isinstance(v, str) and old in v]
    if not hit:
        log.append((u"`old` 가 그 행 어느 키에도 없다", e["path"], old[:60])); return False
    row[hit[0]] = row[hit[0]].replace(old, new); return True


EVMARK = {1: u"런타임 실측 DIFF=0", 2: u"오라클 실행 확증", 3: u"tcx 정본 대조"}


def apply_evup(D, u, rnd, batch, log):
    pp = parse_path(u["path"])
    if not pp:
        log.append((u"경로 형식 오류", u["path"], u"")); return False
    i, outer, field, idx, _k = pp
    if idx is None:
        log.append((u"ev_up 은 배열 원소만", u["path"], u"")); return False
    # ★**3단 경로(`/specs[i]/sig/params[j]`)를 받는다.** (7차 배치B 적발)
    #   `parse_path` 는 `outer` 를 돌려주는데 여기서 **버리고** 있어서
    #   `sig.params` ev 상향이 **항상 실패**했다 — `apply_error` 는 같은 경로를 제대로 처리하니
    #   **한 파일 안에서 두 함수의 계약이 달랐다.** 그 탓에 담당 5함수만 해도 26행이
    #   「도구가 막아서 아무도 못 내리는」 상태로 잠겨 있었다.
    base = D["specs"][i]
    if outer:
        base = base.get({"sig": "signature"}.get(outer, outer)) or {}
    arr, jj = resolve(base, field, int(idx))
    if arr is None or jj is None or jj >= len(arr):
        log.append((u"인덱스 범위 밖", u["path"], u"")); return False
    row = arr[jj]

    # ★★**`guard` 로 「그 행이 맞는지」 확인한다.** (10차 배치C 적발)
    #   `ev_up` 은 `errors` **뒤에** 적용되는데, 같은 배열에 `op:insert` 가 있었으면
    #   인덱스가 밀려 **조용히 엉뚱한 행에 도장이 찍힌다.** `--dry` 는 `2/2 성공`으로 통과시켰고,
    #   배치C 가 「정본 사본에 적용해 대조하는」 스크립트를 따로 만들어서야 잡았다.
    #   ⟹ 선택적이지만 **주면 검사한다**. 안 맞으면 적용하지 않는다.
    g = u.get("guard")
    if g and g not in json.dumps(row, ensure_ascii=False):
        log.append((u"ev_up guard 불일치 — 인덱스가 밀렸을 수 있다(삽입 뒤 경로를 다시 세라)",
                    u["path"], str(g)[:60]))
        return False

    # ★`from` 이 거짓이어도 통과하던 것을 막는다(10차 배치B 적발).
    #   `ev` 는 v2 에 없는 파생값이라 v3 에서 읽는다. 다르면 **경고만** 한다 —
    #   파생이라 라운드 중에 정당하게 바뀔 수 있어 거부까지 하면 오탐이 된다.
    fr = u.get("from")
    if isinstance(fr, int):
        cur = _ev_now(u["path"])
        if cur is not None and cur != fr:
            WARN.append((u"ev_up `from` 이 현재값과 다르다(현재 ev%s)" % cur,
                         u["path"], u"from=%s 로 적혀 있다 — 경로가 밀렸는지 확인하라" % fr))

    want = int(u.get("to", 2))
    # ★★`mem`(오프셋·필드명)은 **ev3 이 상한**이다 — 배치 A↔B 충돌의 판정을 도구에 박는다.
    #   `METHOD_MAP:190` 신뢰서열 = 런타임 > **tcx(컴파일러 정본)** > 오라클 > IR 이고,
    #   `:192` 가 「**타입·오프셋·판별자는 tcx 가 정본**」이라고 못 박는다.
    #   그런데 `ev` 축은 `2 오라클 < 3 tcx` 라 **두 문서가 서로 반대**다(5차 배치B 적발).
    #   ⟹ 주장의 종류마다 강한 증거가 다르다: **오프셋은 tcx 가, 동작·임계는 오라클이** 강하다.
    #      오프셋을 ev2 로 올리면 **정본에서 파생으로 강등**된다. 그래서 여기서 막는다.
    #      `offset_of!` 실행 대조는 tcx 와 **같은 등급의 교차검증**이므로 ev3 + 근거 보강이 정답이다.
    if field == "mem" and want < 3:
        want = 3
        u = dict(u, evidence=(u.get("evidence", u"?") +
                              u" ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)"))
    tag = u" · %s(%s차 배치%s: %s)" % (EVMARK.get(want, u"실행"), rnd, batch, u.get("evidence", u"?"))
    # 근거는 **그 행의 설명란**에 붙인다 — ev 는 거기서 파생된다
    for k in ("meaning", "effect", "note", "value", "what", "name"):
        if isinstance(row.get(k), str):
            if tag in row[k]:
                return True            # 이미 적용됨(재실행 안전)
            row[k] += tag
            return True
    log.append((u"설명란을 못 찾음", u["path"], u"")); return False


def main():
    rnd = sys.argv[1] if len(sys.argv) > 1 else "6"
    only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else None
    dry = "--dry" in sys.argv
    D = json.load(io.open(P2, encoding="utf-8"))
    grand = {"err_ok": 0, "err_ng": 0, "ev_ok": 0, "ev_ng": 0, "behav": 0}
    by_found, by_found_ev = {}, {}
    log = []
    # ★배치 폴더를 **스캔**한다 — 고정 A~D 였을 때 `_clean` 같은 특수 배치가
    #   `--only` 로도 안 잡혀서, 정리 배치가 A 슬롯을 빌려 쓰는 우회를 해야 했다.
    rdir = os.path.join(HERE, "_verify%s" % rnd)
    batches = sorted(d for d in (os.listdir(rdir) if os.path.isdir(rdir) else [])
                     if os.path.exists(os.path.join(rdir, d, "patch.json")))
    for b in batches:
        if only and b != only:
            continue
        f = os.path.join(HERE, "_verify%s" % rnd, b, "patch.json")
        if not os.path.exists(f):
            print(u"[%s] patch.json 없음 — 건너뜀  (%s)" % (b, f)); continue
        pj = json.load(io.open(f, encoding="utf-8"))

        # ★★모르는 최상위 키는 **시끄럽게 거부**한다. (7차 배치A 적발)
        #   내 도시에 §5 가 `{"entries":[{path,old,new,why}]}` 라는 **존재하지 않는 계약**을
        #   예시로 실었다. 그대로 낸 배치가 있었다면 `pj.get("errors")` 가 빈 리스트라
        #   **0건 적용에 성공 메시지**가 떴을 것이다 — 이 도구가 막으려는 「조용한 no-op」
        #   그 자체를, 지시 오류를 통해 재현할 뻔했다.
        KNOWN = {"round", "batch", "errors", "ev_up", "brief_errors", "note", "notes"}
        unknown = [k for k in pj if k not in KNOWN]
        if unknown:
            print(u"[%s] ★모르는 최상위 키 %s — **적용을 거부한다**" % (b, unknown))
            print(u"     계약: {\"round\":N, \"batch\":\"X\", "
                  u"\"errors\":[{path,kind,old,new,evidence,behavior_change,found_by}], "
                  u"\"ev_up\":[{path,from,to,evidence,found_by}], \"brief_errors\":[...]}")
            if "entries" in unknown:
                print(u"     `entries` → `errors` 로 옮기고 `why`→`evidence` · "
                      u"`kind`(실오류/오탐/보강) · `behavior_change`(bool) 를 채워라.")
            grand["schema_ng"] = grand.get("schema_ng", 0) + 1
            continue
        if not (pj.get("errors") or pj.get("ev_up")):
            print(u"[%s] ★`errors`·`ev_up` 이 **둘 다 비었다** — 보고가 유실됐는지 확인하라." % b)

        eo = en = vo = vn = 0
        skipped = []
        for e in pj.get("errors") or []:
            r = apply_error(D, e, log, skipped)
            if r is None:
                continue                      # 이미 적용됨 — 성공도 실패도 아니다
            if r:
                eo += 1
                if e.get("behavior_change"):
                    grand["behav"] += 1
                k = e.get("found_by") or u"미분류"
                by_found[k] = by_found.get(k, 0) + 1
            else:
                en += 1
        for u2 in pj.get("ev_up") or []:
            if apply_evup(D, u2, rnd, b, log):
                vo += 1
                # ★`ev_up` 도 `found_by` 를 센다(6차 배치B 적발 — 그 라운드 최대 물량인
                #   `mem` 94행이 `inherited` 였는데 실험 집계에서 통째로 빠졌다).
                k2 = u2.get("found_by") or u"미분류"
                by_found_ev[k2] = by_found_ev.get(k2, 0) + 1
            else:
                vn += 1
        print(u"[%s] 정정 %d/%d · ev상향 %d/%d%s"
              % (b, eo, eo + en, vo, vo + vn,
                 (u"  (이미 적용 %d)" % len(skipped)) if skipped else u""))
        # ★반영 표식 — `auditrounds` 가 「반영됨」과 「반영 대기」를 가르는 근거다.
        #   이게 없으면 아직 안 붙인 패치가 "유실"로 보여 감사가 시끄러워진다(6차에 실제로 그랬다).
        if not dry:
            # ★**적용 결과를 도장으로 남긴다.** 뒤에 정리 패스(`cleanspec`·재작성)가 같은 필드를
            #   다시 쓰면 `patch.json` 의 문자열로는 확인이 안 된다(실측 18/34).
            #   ⟹ 회귀 감사는 「그때 무엇이 들어갔나」가 아니라 **「지금 그 자리가 무엇인가」**를 본다.
            stamps = {}
            for e2 in pj.get("errors") or []:
                cur = field_now(D, e2.get("path") or u"")
                if cur is not None:
                    stamps[e2["path"]] = hashlib.sha256(cur.encode("utf-8")).hexdigest()[:16]
            io.open(os.path.join(HERE, "_verify%s" % rnd, b, "applied.json"), "w",
                    encoding="utf-8").write(json.dumps(
                        {"round": rnd, "batch": b, "errors_applied": eo,
                         "ev_applied": vo, "failed": en + vn, "stamps": stamps},
                        ensure_ascii=False, indent=1))
        grand["err_ok"] += eo; grand["err_ng"] += en
        grand["ev_ok"] += vo; grand["ev_ng"] += vn

    if WARN:
        print(u"\n⚠주의 %d건 — **실패가 아니다**" % len(WARN))
        for why, pth, det in WARN:
            print(u"   %-44s %s" % (pth, why))
            if det:
                print(u"        %s" % det)
    if log:
        print(u"\n★적용 실패 %d건 — **조용히 넘기지 않는다**" % len(log))
        for why, path, frag in log[:40]:
            print(u"   %-32s %s  %s" % (why, path, frag))

    print(u"\n" + u"=" * 92)
    print(u"정정 %d 성공 / %d 실패 · ev상향 %d 성공 / %d 실패 · **동작 변경 %d건**"
          % (grand["err_ok"], grand["err_ng"], grand["ev_ok"], grand["ev_ng"], grand["behav"]))
    if by_found:
        print(u"발견 경위(정정): " + u" · ".join(u"%s=%d" % (k, v) for k, v in sorted(by_found.items())))
    if by_found_ev:
        print(u"발견 경위(ev상향): " + u" · ".join(u"%s=%d" % (k, v) for k, v in sorted(by_found_ev.items())))
        print(u"  ★`reused`(그 배치가 이미 쓰던 기법) 가 0 에 가까우면 **명세가 안정적**이라는 뜻이고,")
        print(u"    여러 건이면 **명세가 진짜로 불안정**하다는 뜻이다 — 라운드를 더 도는 게 답이 아니다.")
    print(u"=" * 92)
    if "--restamp" in sys.argv:
        D2 = json.load(io.open(P2, encoding="utf-8"))
        for b in batches:
            f = os.path.join(HERE, "_verify%s" % rnd, b, "patch.json")
            ap = os.path.join(HERE, "_verify%s" % rnd, b, "applied.json")
            if not os.path.exists(ap):
                continue
            pj = json.load(io.open(f, encoding="utf-8"))
            aj = json.load(io.open(ap, encoding="utf-8"))
            st = {}
            for e2 in pj.get("errors") or []:
                cur = field_now(D2, e2.get("path") or u"")
                if cur is not None:
                    st[e2["path"]] = hashlib.sha256(cur.encode("utf-8")).hexdigest()[:16]
            aj["stamps"] = st
            aj["restamped"] = True
            io.open(ap, "w", encoding="utf-8").write(json.dumps(aj, ensure_ascii=False, indent=1))
            print(u"[%s] 도장 %d개 재기록" % (b, len(st)))
        print(u"⟹ 정리 패스(`cleanspec`·재작성) 뒤에는 이걸 돌려 기준선을 다시 잡는다.")
        return 0
    if dry:
        print(u"(--dry: 파일을 쓰지 않았다)")
        # ★`--dry` 도 **스키마 실패는 exit 1** 이어야 한다. 배치가 제출 전에 돌리는 게 이건데
        #   여기서 0 을 돌려주면 「사전 검증 통과」로 읽고 그대로 낸다.
        return 1 if grand.get("schema_ng") else 0
    if grand["err_ok"] or grand["ev_ok"]:
        io.open(P2, "w", encoding="utf-8").write(json.dumps(D, ensure_ascii=False, indent=1))
        print(u"-> %s  (이제 `mkspec3.py` 로 v3 재생성)" % P2)
    return 1 if (grand["err_ng"] or grand["ev_ng"] or grand.get("schema_ng")) else 0


if __name__ == "__main__":
    sys.exit(main())
