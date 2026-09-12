# -*- coding: utf-8 -*-
u"""G20 `shared` — **명세 간 공유 사실 대조** 게이트.

## 이 축이 G1~G19 와 다른 점
기존 게이트는 전부 **명세 1개 안**을 본다(자기모순·IR 대조·표↔산문). 그래서 두 명세가
같은 사실을 **서로 다르게** 적어도 양쪽 다 자기 안에서는 무모순이라 아무도 안 잡는다.
20개 명세는 같은 구조체를 반복해서 읽으므로(`PlayerState+0x930` 을 15개 명세가 적는다)
이 축은 **IR 을 다시 안 보고도** 「최소한 하나는 틀렸다」를 말할 수 있다.

## 규칙 4개
| 규칙 | 키 | 비교 | 이게 왜 결함인가 |
|---|---|---|---|
| `R1` | (base, offset) | 필드명 | 같은 자리를 다른 이름으로 부른다 |
| `R2` | (base, 필드명) | offset | ★표기로 설명이 안 된다 — 반드시 한쪽이 틀렸다 |
| `R3` | (base, offset) | dir 미기재 | 한쪽만 `-` 면 그 한쪽이 안 적은 것이다 |
| `R4` | 노브 이름 | value | 같은 노브가 두 값을 가질 수 없다 |

## ★오탐 억제 — 측정해서 알아낸 두 가지
### ⓐ enum 페이로드 중첩 (R1 의 최대 오염원)
`TeamPlan+0x420` 을 `08` 은 `objective.Morgard.phase`, `18` 은 `objective.Serpen.phase`
라고 적는다. **둘 다 맞다** — Rust enum 은 variant 페이로드가 같은 자리에 겹친다.
`SubPlan+0x8`(`LineDefenseSubPlan.style` / `Hide.__0.bush` / `EpicHunt.__0.need_recall`) 도 같다.
⟹ 두 이름이 **대문자로 시작하는 성분(=variant)에서 갈리면** 불일치로 세지 않는다.

### ⓑ 정밀도 차이 ≠ 모순
`game` / `game.data` / `game.data_ptr` 은 같은 것을 **다른 깊이로** 적은 것이다.
한쪽이 다른 쪽의 접두이면 「모순」이 아니라 **`정밀도`** 로 따로 센다(errors 아님).
⚠단 `12`·`18` 처럼 **fat pointer 의 양쪽 절반을 똑같이 `game` 이라고** 부르면 그건
R2 가 잡는다 — 같은 이름이 0x0 과 0x8 두 곳에 있으므로.

★그리고 이 억제는 **기각에만** 쓴다(7차 규칙). 억제된 건은 `정밀도`/`중첩` 으로 보이게 남긴다.
"""
import io, json, os, re, sys, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"


def norm_base(b):
    u"""`PlayerState(=GamePlayer info)` → `PlayerState`"""
    b = re.sub(u"[（(].*?[)）]", u" ", b or u"")
    return re.sub(u"\\s+", u" ", b).strip()


def norm_off(o):
    o = (o or u"").strip().lower()
    try:
        return "0x%x" % (int(o, 16) if o.startswith("0x") else int(o))
    except Exception:
        return o


_HANG = re.compile(u"[가-힣]")


def norm_name(n):
    u"""타입 주석·설명 괄호·**꼬리 한글 주석**을 벗기고 첨자는 자리표시자로 남긴다.

    ⚠`[` 부터 통째로 버리면 안 된다 — `twin_towers[0].len` 이 `twin_towers` 가 되어
      `twin_towers[아군팀]`(다른 오프셋)과 같은 이름이 되고 **R2 오탐**이 된다(실측 1건).
    ⚠★**소문자로 낮추지 않는다** — variant 판정(`^[A-Z]`)이 죽는다. 초판이 그래서
      `objective.Morgard.phase` / `objective.Serpen.phase` 의 중첩 억제를 놓쳤다.
      대소문자 무시 비교가 필요한 곳에서만 호출부가 `.lower()` 한다.
    ⚠꼬리 한글은 이름이 아니라 설명이다 — `Tower.nearest_enemy 판별자` 와
      `ty.Tower.nearest_enemy 태그` 는 같은 자리를 가리킨다.
    """
    n = (n or u"")
    # 타입 주석 `name: Ty` 를 자른다.
    # ⚠`[:：]` 로 자르면 **경로 구분자 `::`** 까지 먹는다 — `Chat::Battle.__0` 이 `Chat` 이 되어
    #   0x0/0x8/0x10 세 자리가 전부 같은 이름이 되고 R2 가 오탐을 낸다(실측 1건).
    n = re.split(u"(?<!:):(?!:)|：", n)[0]
    n = re.sub(u"[（(].*", u" ", n)                   # 설명 괄호
    n = re.sub(u"\\[[^\\]]*\\]", u"[]", n)            # 첨자는 유지하되 값은 무시
    n = u" ".join(w for w in n.split() if not _HANG.search(w))
    return re.sub(u"\\s+", u" ", n).strip()


def names_of(r):
    u"""한 칸에 ` / ` 로 **여러 variant 를 함께 적은 행**을 갈라 낸다.
    `16` 의 `ty.Champion.0.ult_cooldown / ty.Bear.info.attack_cooldown` 이 그 형태다."""
    raw = (r.get("name") or u"")
    out = [norm_name(p) for p in re.split(u"\\s+/\\s+", raw)]
    return [x for x in out if x]


_VAR = re.compile(u"^[A-Z]")


def variant_split(a, b):
    u"""두 필드명이 **variant 성분에서 갈리는가**. 갈리면 enum 중첩이라 모순이 아니다."""
    pa, pb = a.split(u"."), b.split(u".")
    for x, y in zip(pa, pb):
        if x == y:
            continue
        return bool(_VAR.match(x) or _VAR.match(y))
    return False


_SUF = re.compile(u"(?:_ptr|_id|\\[\\])+$")


def skel(n):
    u"""**표기 접미**만 벗긴 뼈대. `game.data_ptr`→`game.data` · `player_champion[][]`→`player_champion`

    ★**숫자 단독 성분(튜플 variant 첨자)도 뺀다.** (12차 배치D 적발 — R1 오탐 3건)
      `ty.Champion.0.skill_cooldown`(`16`) 과 `ty.Champion.skill_cooldown`(`03`) 은
      **모순이 아니다** — `EntityType::Champion` 만 튜플 variant 라 페이로드 필드명이 `0` 이고,
      `03` 은 그 첨자를 생략했을 뿐이다(tcx `--enum EntityType` + rustc `E0164` 로 확정).
      `variant_split` 은 갈리는 성분(`0` vs `skill_cooldown`)이 대문자가 아니라 통과시키고,
      `prefix_of` 도 성분 수가 어긋나 통과시켜 **R1 5건 중 3건이 이 형태**였다."""
    return u".".join(_SUF.sub(u"", s) for s in n.split(u".") if not s.isdigit())


def spelling_of(a, b):
    u"""같은 것을 **다르게 표기**했을 뿐인가. 뼈대가 같거나 뼈대끼리 접두/접미 관계."""
    sa, sb = skel(a), skel(b)
    return sa == sb or prefix_of(sa, sb)


def prefix_of(a, b):
    u"""한쪽이 다른 쪽의 **경로 접두 또는 접미**인가(정밀도 차이).

    접미도 봐야 한다 — `Tower.ty` 와 `ty.Tower.ty` 는 뿌리를 어디로 잡았는지만 다르다."""
    pa, pb = a.split(u"."), b.split(u".")
    n = min(len(pa), len(pb))
    if len(pa) == len(pb):
        return False
    return pa[:n] == pb[:n] or pa[-n:] == pb[-n:]


def check(D):
    u"""반환 = (errors, soft)
    errors = [(규칙, 키, [(i, 값, 원문)…], 사유)] · soft = 억제된 것(참고용)"""
    NM = [s.get("name") for s in D]
    err, soft = [], []
    mem = [(i, j, r) for i, s in enumerate(D) for j, r in enumerate(s.get("mem") or [])]

    # ── R1 / R3 : (base, offset) 로 묶는다 ──────────────────────────────
    g = collections.defaultdict(list)
    for i, j, r in mem:
        b, o = norm_base(r.get("base")), norm_off(r.get("offset"))
        if b and o:
            g[(b, o)].append((i, j, r))
    for k, v in sorted(g.items()):
        if len(set(x[0] for x in v)) < 2:
            continue                                   # 한 명세 안의 중복은 G1 담당
        # ★행마다 이름이 **여럿**일 수 있다(` / ` 로 적은 다중 variant).
        #   한 이름이라도 모든 명세에 공통으로 있으면 그 자리는 합의된 것이다.
        per = [set(x.lower() for x in names_of(r)) for (_, _, r) in v if names_of(r)]
        if per and set.intersection(*per):
            per = None                                 # 공통 이름이 있다 → 합의
        names = sorted(set(x for (_, _, r) in v for x in names_of(r)))
        if per is not None and len(names) > 1:
            pairs = [(a, b) for x, a in enumerate(names) for b in names[x + 1:]]
            if all(variant_split(a, b) for a, b in pairs):
                soft.append((u"R1중첩", k, names, u"enum variant 에서 갈림 — 페이로드 중첩이라 양립"))
            elif all(variant_split(a, b) or prefix_of(a, b) for a, b in pairs):
                soft.append((u"R1정밀도", k, names, u"한쪽이 다른 쪽의 경로 접두/접미 — 깊이만 다르다"))
            elif all(variant_split(a, b) or prefix_of(a, b) or spelling_of(a, b)
                     for a, b in pairs):
                soft.append((u"R1표기", k, names,
                             u"`_ptr`·`[]` 같은 **표기 접미**만 다르다 — 사실 주장은 같다"))
            else:
                err.append((u"R1", k,
                            [(i, u"/".join(names_of(r)), (r.get("name") or u""))
                             for (i, _, r) in v],
                            u"같은 (base,offset) 을 %d가지 이름으로 부른다: %s"
                            % (len(names), u" / ".join(names))))
        # ⛔R3(`dir` 미기재)은 **폐기했다.** (12차 배치C 가 IR 로 뒤집음)
        #   `dir` 은 **cross-spec 사실이 아니다** — 함수마다 실제로 다르다.
        #   `13 target_bush_v30` 은 fastcc 인자승격이라 본문에 gep 가 **0건**이고
        #   `PlayerState+0x930/0x9c0` 의 로드는 **호출부**(`LineGankCoverPlan::sub_plan`,
        #   m10.ll:11762~11791)에 있다. 복제본 `14 update` 는 본문에 gep 가 있어 `r` 이 맞다.
        #   ⟹ 「다른 14개 명세가 `r` 이니 누락」이라고 채웠으면 **IR 에 없는 로드가 있다고 주장**하게 된다.
        #   이 프로젝트에서 `dir="-"` = 「이 함수 본문은 읽지도 쓰지도 않는다(참조용 행)」이고
        #   G14 도 `dir=-` 를 의도적으로 스킵한다. 축 자체가 잘못 놓였던 규칙이다.
        #   ★그리고 이건 **내(메인) 지시 오류**이기도 하다 — 도시에에 「기계적으로 확실한 누락」이라
        #     단정해서 보냈고, 배치가 IR 로 반증했다.

    # ── R2 : (base, 필드명) → offset ───────────────────────────────────
    g = collections.defaultdict(list)
    for i, j, r in mem:
        b, n = norm_base(r.get("base")), (names_of(r) or [u""])[0].lower()
        if b and n and norm_off(r.get("offset")):
            g[(b, n)].append((i, j, r))
    for k, v in sorted(g.items()):
        offs = sorted(set(norm_off(r.get("offset")) for (_, _, r) in v))
        if len(offs) > 1 and len(set(x[0] for x in v)) > 1:
            err.append((u"R2", k,
                        [(i, norm_off(r.get("offset")), (r.get("name") or u""))
                         for (i, _, r) in v],
                        # ★사유를 고쳤다. (12차 배치D 적발) 구판은 「**반드시 한쪽이 틀렸다**」였는데
                        #   `18`·`12` 는 괄호로 팻포인터 두 절반을 정확히 갈라 적었고 **둘 다 맞다**.
                        #   발화 원인은 `norm_name` 이 괄호를 버리는 것이고, 여기선 괄호가 유일한 판별자였다.
                        #   ⟹ 「값이 틀렸다」로 오도하지 말고 **「이름이 판별력을 잃었다」도 답**임을 적는다
                        #      (실제로 한 팻포인터를 부르는 이름이 20명세 안에서 5가지였다).
                        u"같은 이름이 오프셋 %s 에 동시에 있다 — **값이 틀렸거나, 이름이 "
                        u"두 자리를 구별하지 못한다**(후자면 이름을 갈라라)"
                        % u" / ".join(offs)))

    # ── R4 : 노브 이름 → value ─────────────────────────────────────────
    g = collections.defaultdict(list)
    for i, s in enumerate(D):
        for j, r in enumerate(s.get("knobs") or []):
            w = re.sub(u"\\s+", u" ", (r.get("what") or u"")).strip().lower()
            if w:
                g[w].append((i, j, r))
    for k, v in sorted(g.items()):
        vals = sorted(set(json.dumps(r.get("value"), ensure_ascii=False) for (_, _, r) in v))
        if len(vals) > 1 and len(set(x[0] for x in v)) > 1:
            err.append((u"R4", k,
                        [(i, json.dumps(r.get("value"), ensure_ascii=False), u"")
                         for (i, _, r) in v],
                        u"같은 노브가 값 %s 를 동시에 갖는다" % u" / ".join(vals)))
    return err, soft


# ── specgate 연동 진입점 ─────────────────────────────────────────────
def check_all(specs):
    u"""`specgate.py` 가 부른다. 반환 = [(대표 명세 index, 메시지, 상세)]"""
    err, _ = check(specs)
    out = []
    for (rule, key, rows, why) in err:
        i = rows[0][0]
        out.append((i, u"[%s] %s — %s" % (rule, u" ".join(str(x) for x in key), why),
                    u" · ".join(u"[%02d]%s" % (a, b) for a, b, _ in rows[:8])))
    return out


def main():
    D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"),
                          encoding="utf-8"))["specs"]
    NM = [s.get("name") for s in D]
    err, soft = check(D)
    tal = collections.Counter(e[0] for e in err)
    for (rule, key, rows, why) in err:
        print(u"\n[%s] %s" % (rule, u"  ".join(str(x) for x in key)))
        print(u"      %s" % why)
        seen = set()
        for (i, val, raw) in rows:
            if (i, val) in seen:
                continue
            seen.add((i, val))
            print(u"        [%02d %-27s] %-22s %s" % (i, (NM[i] or u"")[:27], str(val)[:22], raw[:44]))
    print(u"\n" + u"-" * 92)
    print(u"[억제됨 — 결함 아님] %d건" % len(soft))
    for (tag, key, names, why) in soft:
        print(u"  %-9s %-34s %s" % (tag, u" ".join(str(x) for x in key), why))
        print(u"            %s" % u" / ".join(names))
    print(u"=" * 92)
    print(u"G20 shared = %d건  (%s)" % (len(err), u" · ".join(
        u"%s %d" % (k, v) for k, v in sorted(tal.items()))))


if __name__ == "__main__":
    main()
