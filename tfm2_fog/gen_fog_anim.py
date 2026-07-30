# 12장 안개 애니메이션 생성 — 공간 이음매없음(x,y 주기) + 시간 이음매없음(12프레임 루프).
# 3D FFT 노이즈(kx,ky,kt 모두 정수 주파수 → 세 축 전부 주기적) → 매끄러운 타일링+무한루프.
import numpy as np
from PIL import Image

N = 512      # 프레임 한 장 크기(정사각, 타일)
T = 12       # 프레임 수
SEED = 20260704
rng = np.random.default_rng(SEED)

# 주파수 공간 랜덤 복소 계수
freq = rng.normal(size=(T, N, N)) + 1j * rng.normal(size=(T, N, N))

# 라디얼 진폭 감쇠(fbm 느낌). 저주파 강조 → 큰 뭉게구름.
kt = np.fft.fftfreq(T) * T          # -6..6
ky = np.fft.fftfreq(N) * N
kx = np.fft.fftfreq(N) * N
KT, KY, KX = np.meshgrid(kt, ky, kx, indexing='ij')
# 공간 주파수는 넓게, 시간 주파수는 느리게(±1~2 위주) 가중.
k_space = np.sqrt(KX**2 + KY**2)
k_time = np.abs(KT) * 6.0            # 시간축 가중 크게 → 느린 진화
k = np.sqrt(k_space**2 + k_time**2) + 1e-6
amp = 1.0 / (k ** 2.4)
# 너무 낮은/높은 주파수 컷(DC 제거, 초고주파 억제)
amp[0, 0, 0] = 0.0
amp[k_space < 2.0] *= 0.3            # 너무 큰 얼룩 억제
field = np.fft.ifftn(freq * amp).real

# 정규화 0..1
field = (field - field.min()) / (field.max() - field.min() + 1e-9)

# 대비 커브 — 전체적으로 뿌옇게(바닥 haze) + 그 위 뭉치. 넓은 smoothstep 로 균일하게.
lo, hi = 0.28, 0.92
a = np.clip((field - lo) / (hi - lo), 0.0, 1.0)
a = a * a * (3 - 2 * a)             # smoothstep
FLOOR, PEAK = 90, 205              # 바닥 알파 90(어디나 뿌옇게) ~ 최대 205
alpha = (FLOOR + a * (PEAK - FLOOR)).astype(np.uint8)

# 색: 흰빛 강한 청백(밝게)
base = np.array([232, 240, 255], dtype=np.float32)
for t in range(T):
    rgb = np.empty((N, N, 3), dtype=np.uint8)
    for c in range(3):
        rgb[..., c] = base[c]
    img = np.dstack([rgb, alpha[t][..., None]]).astype(np.uint8)
    Image.fromarray(img, 'RGBA').save(f'ui/fog_{t}.png')
    print(f'ui/fog_{t}.png  alpha[min,mean,max]={alpha[t].min()},{alpha[t].mean():.1f},{alpha[t].max()}')

print('done: 12 frames')
