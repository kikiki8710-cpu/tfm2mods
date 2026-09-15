"""poslock_math.py — tfm2_champ_pos_lock 의 라인 매칭 수학(lib.rs:986~1136)을 파이썬으로 그대로 옮긴 것.
시뮬레이터(bpsim.py)가 모드 층을 재현할 때 쓴다. 원문과 1:1 (정렬·재귀 순서까지).
"""
MASK_ALL = 0b11111


def max_match(masks):
    def rec(i, used):
        if i == len(masks):
            return 0
        m = masks[i]
        best = rec(i + 1, used)
        for p in range(5):
            b = 1 << p
            if m & b and not used & b:
                v = 1 + rec(i + 1, used | b)
                if v > best:
                    best = v
        return best
    return rec(0, 0)


def helps(pinned, cand):
    return max_match(list(pinned) + [cand]) > max_match(list(pinned))


def cover_sets(masks):
    reach = 1
    for m in masks:
        nxt = reach
        for s in range(32):
            if not reach & (1 << s):
                continue
            for p in range(5):
                if m & (1 << p) and not s & (1 << p):
                    nxt |= 1 << (s | (1 << p))
        reach = nxt
    return reach


def pool_cover_sets(pool):
    by = [0] * 32
    for m in pool:
        by[m & MASK_ALL] += 1
    avail = [0] * 32
    for t in range(1, 32):
        avail[t] = sum(by[m] for m in range(1, 32) if m & t)
    ok = 1
    for s in range(1, 32):
        good = True
        t = s
        while t > 0:
            if avail[t] < bin(t).count("1"):
                good = False
                break
            t = (t - 1) & s
        if good:
            ok |= 1 << s
    return ok


def achievable(pinned, pool, slots):
    pin = cover_sets(pinned)
    pl = pool_cover_sets(pool)
    best = 0
    for a in range(32):
        if not pin & (1 << a):
            continue
        rest = MASK_ALL & ~a
        b = rest
        while True:
            if pl & (1 << b) and bin(b).count("1") <= slots:
                v = bin(a).count("1") + bin(b).count("1")
                if v > best:
                    best = v
            if b == 0:
                break
            b = (b - 1) & rest
    return best


def free_left(pinned, pool, team):
    slots = max(team - len(pinned), 0)
    target = achievable(pinned, pool, slots)
    budget = max(team - target, 0)
    used = max(len(pinned) - max_match(pinned), 0)
    return max(budget - used, 0)


def feasible(masks):
    ms = sorted(masks, key=lambda m: bin(m).count("1"))
    if len(ms) > 5:
        return False

    def rec(i, used):
        if i == len(ms):
            return True
        m = ms[i]
        for p in range(5):
            b = 1 << p
            if m & b and not used & b and rec(i + 1, used | b):
                return True
        return False
    return rec(0, 0)


def best_assignment(masks):
    """최대 매칭 하나(포지션→인덱스). 스왑 축 검증용(적합 수 산출)."""
    n = len(masks)
    best = (-1, None)

    def rec(i, used, assign):
        nonlocal best
        if i == n:
            cnt = sum(1 for a in assign if a is not None)
            if cnt > best[0]:
                best = (cnt, list(assign))
            return
        rec(i + 1, used, assign + [None])
        for p in range(5):
            b = 1 << p
            if masks[i] & b and not used & b:
                rec(i + 1, used | b, assign + [p])
    rec(0, 0, [])
    return best


if __name__ == "__main__":
    # 자체 검사
    assert max_match([0b00001, 0b00001]) == 1
    assert helps([0b00001], 0b00010) and not helps([0b00001], 0b00001)
    assert feasible([0b00001, 0b00010, 0b00100, 0b01000, 0b10000])
    assert not feasible([0b00001, 0b00001])
    assert free_left([], [0b00001] * 5, 5) == 4  # 풀이 서폿뿐 → 4자리 자유
    print("poslock_math OK")
