# 비(rain) 12장 — 가늘고 긴 대각선 빗줄기가 아래로 떨어지며 매끄럽게 루프.
# x축 고르게 분포(뭉침=굵은 띠 방지) + 세로 roll 낙하. roll mod N = 무한루프.
import numpy as np
from PIL import Image

N = 1024
T = 12
rng = np.random.default_rng(77)

base = np.zeros((N, N), dtype=np.float32)
NUM = 1200
ANG = np.deg2rad(13)
dxu, dyu = np.sin(ANG), np.cos(ANG)
for i in range(NUM):
    # x 를 균등 분포(뭉쳐서 굵은 띠 되는 것 방지) + 약간 지터
    cx = ((i + 0.5) / NUM * N + rng.uniform(-N/NUM, N/NUM)) % N
    cy = rng.uniform(0, N)
    length = rng.uniform(50, 110)
    peak = rng.uniform(55, 120)
    width = rng.uniform(0.4, 0.8)
    steps = int(length)
    for s in range(steps):
        t = s / max(steps - 1, 1)
        fade = np.sin(t * np.pi) ** 0.7
        px = cx + (t - 0.5) * length * dxu
        py = cy + (t - 0.5) * length * dyu
        ix = int(px) % N; iy = int(py) % N
        a = peak * fade
        for w in (-1, 0, 1):
            xx = (ix + w) % N
            ww = np.exp(-(w * w) / (2 * width * width))
            base[iy, xx] += a * ww

base = np.clip(base, 0, 255)
FLOOR = 10
base_color = np.array([200, 218, 255], dtype=np.float32)
for t in range(T):
    shift = int(round(t / T * N))
    frame = np.roll(base, shift, axis=0)
    alpha = np.clip(FLOOR + frame, 0, 200).astype(np.uint8)
    rgb = np.empty((N, N, 3), dtype=np.uint8)
    for c in range(3): rgb[..., c] = base_color[c]
    Image.fromarray(np.dstack([rgb, alpha[..., None]]).astype(np.uint8), 'RGBA').save(f'ui/rain_{t}.png')
print(f'done rain 12 @ {N}px, {NUM} streaks')
