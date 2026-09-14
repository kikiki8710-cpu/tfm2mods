# 113 i32 wrap 케이스 탐색: shot = tatk - tatk//101 (실측 4점 정합), edge_ticks=32000(ms<=1), cool=3, hp=1
# v = (floor(shot*32000/3)*30) mod 2^32 가 [2^31, 2^31+50] 이면 umin(v+2,60)=60 vs 부호있는 min 은 음수.
found = []
for tatk in range(1, 20_000_000):
    shot = tatk - tatk // 101
    edge = shot * 32000 // 3
    v = (edge * 30) % (1 << 32)
    if (1 << 31) <= v <= (1 << 31) + 50:
        found.append((tatk, shot, v, v - (1 << 31)))
        if len(found) >= 5:
            break
print(found)
# 또 하나: v 가 [0, 57] 이면 wrap 뒤 작은 값(2..59)이 나온다 — 재구현이 64비트로 계산하면 60 이 나와 갈린다
found2 = []
for tatk in range(100_000, 20_000_000):
    shot = tatt = tatk - tatk // 101
    edge = shot * 32000 // 3
    v = (edge * 30) % (1 << 32)
    if 1 <= v <= 57 and edge * 30 >= (1 << 32):
        found2.append((tatk, shot, v))
        if len(found2) >= 5:
            break
print(found2)
