# -*- coding: utf-8 -*-
"""transfer_fix 0.7.1 RVA 외 수정 — 바이트 치환(줄바꿈 보존). RVA·old_tgt 값은 repin apply 가 바꾼다."""
R = r'F:\claude\tfm2mods\tfm2_transfer_fix'
def edit(path, reps):
    b = open(path, 'rb').read()
    for o, n in reps:
        ob, nb = o.encode('utf-8'), n.encode('utf-8')
        assert b.count(ob) == 1, (path, o, b.count(ob))
        b = b.replace(ob, nb)
    open(path, 'wb').write(b)
    print('ok', path)
edit(R + r'\src\lib.rs', [
    ('const GAME_VER: &str = "0.7.0";', 'const GAME_VER: &str = "0.7.1";'),
    ('// ── ① s 식 (0.7.0) ──', '// ── ① s 식 (0.7.1 · c 레지스터 xmm6→xmm7) ──'),
    ('// maxsd xmm0, xmm6  (s 하한)', '// maxsd xmm0, xmm7  (s 하한)'),
    ('const ORIG_S_MAX: [u8; 4] = [0xf2, 0x0f, 0x5f, 0xc6];', 'const ORIG_S_MAX: [u8; 4] = [0xf2, 0x0f, 0x5f, 0xc7];'),
    ('// movsd xmm6,[c]          — s 식의 c (0.95)', '// movsd xmm7,[c]          — s 식의 c (0.95)'),
    ('prefix: [0xf2, 0x0f, 0x10, 0x35]', 'prefix: [0xf2, 0x0f, 0x10, 0x3d]'),
    ('// ── ② 10분위 보정 초기값 (0.7.0) ──', '// ── ② 10분위 보정 초기값 (0.7.1 — 게임이 호출 순서를 고쳐 초기값이 이미 1.0 · 패치는 결과 불변 · RE 2026-11-04_0.7.1-재핀판정-로직대조) ──'),
    ('// ── ③ 배율 함수 입구 (0.7.0) ──', '// ── ③ 배율 함수 입구 (0.7.1) ──'),
    ('# tfm2_transfer_fix 설정 (게임 0.7.0)', '# tfm2_transfer_fix 설정 (게임 0.7.1)'),
    ('# ② 첫 시즌 종료 때 연봉 보정이 두 번 곱해지는 오류 수정 (1 = 켬, 0 = 끔)', '# ② 첫 시즌 종료 때 연봉 보정이 두 번 곱해지는 오류 수정 (1 = 켬, 0 = 끔)\n#    0.7.1 부터 게임이 직접 고쳤다. 켜 둬도 결과는 같다.'),
    ('참고: 첫 연도 전환 직후 배율은 0.67 ~ 1.85 였다(② 수정 켬 · 2026-11 실측).', '참고: 0.7.0 첫 연도 전환 직후 배율은 0.67 ~ 1.85 였다(② 수정 켬 · 2026-11 실측).'),
])
edit(R + r'\Cargo.toml', [
    ('version = "0.2.0"', 'version = "0.2.1"'),
    ('게임 0.7.0 전용(패치마다 재핀)', '게임 0.7.1 전용(패치마다 재핀)'),
])
edit(R + r'\README.txt', [
    ('대상 게임 버전: Teamfight Manager 2  0.7.0 (모드 v0.2.0)', '대상 게임 버전: Teamfight Manager 2  0.7.1 (모드 v0.2.1)'),
    ('  double_apply_fix = 1      ② 이중 적용 수정 (0 이면 바닐라)', '  double_apply_fix = 1      ② 이중 적용 수정 (0.7.1 부터 게임이 직접 고쳐서 켜도 꺼도 결과가 같습니다)'),
    ('  참고로 첫 연도 전환 직후 배율은 0.67 ~ 1.85 였습니다(2026-11 실측).', '  참고로 0.7.0 첫 연도 전환 직후 배율은 0.67 ~ 1.85 였습니다(2026-11 실측).'),
    ('  · 게임 0.7.0 전용입니다.', '  · 게임 0.7.1 전용입니다.'),
])
