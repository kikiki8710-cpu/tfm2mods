#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""run.py — ★마이그레이션 단일 진입점. **모든 축을 순서대로 돌리고, 하나라도 건너뛸 수 없게 한다.**

왜 이 파일이 있나 (2026-09-02 0.5.8):
  0.5.8 마이그는 RVA 1,454건을 전부 재핀하고 `check` 전건 PASS·`coverage` 클린을 받았는데
  **게임이 크래시**했다. 원인은 RVA 가 아니라 **구조체 오프셋이 0x10 밀린 것**이었고,
  그 축은 README ⑦ 에 "별도 확인" 이라고 글로만 적혀 있었다.
  → 글로 적힌 절차는 지켜지지 않는다. **명령 하나로 전 축을 돌리고 종료코드로 막는다.**

  놓쳤던 것 전부가 여기 축으로 들어가 있다:
    RVA(check/coverage/dups) · 구조체 오프셋(offsets) · 환경(deps·버전게이트·SDK·apply누락·stale dll)

사용:
  python MIG\run.py --exe <신exe> --pkl <신fnidx.pkl> [--sdk sdk_058] [--ver 0.5.8]
종료코드: 0 = 전 축 클린(= "마이그 완료"라고 말해도 되는 유일한 상태) / 1 = 남은 작업 있음
"""
import sys, os, subprocess, argparse

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
import mig_verify as MV  # noqa: E402

PY = sys.executable


def run(title, args, must_pass=True):
    print('\n' + '=' * 78)
    print('■ %s' % title)
    print('  $ ' + ' '.join(os.path.basename(a) if a.endswith('.py') else a for a in args[1:]))
    print('=' * 78)
    rc = subprocess.call(args)
    print('-- 종료코드 %d %s' % (rc, '' if rc == 0 else '<= 이 축에 남은 작업이 있다'))
    return (rc == 0) or (not must_pass)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--exe', default=MV.GAME_EXE)
    ap.add_argument('--pkl', default=os.path.join(MV.ROOT, '_fnidx_%s.pkl'
                                                  % MV.GAME_VER.replace('.', '')[-3:]))
    ap.add_argument('--ver', default=MV.GAME_VER)
    ap.add_argument('--sdk', default='sdk_' + MV.GAME_VER.replace('.', '')[-3:])
    a = ap.parse_args()

    ok = []
    ok.append(('① RVA — 매니페스트 대조',
               run('① RVA: 매니페스트 각 엔트리가 새 exe 에서 그대로인가 (mig_verify check)',
                   [PY, os.path.join(HERE, 'mig_verify.py'), 'check', '--exe', a.exe])))
    ok.append(('② RVA — 커버리지(치환 누락)',
               run('② RVA: 소스의 RVA 리터럴 중 매니페스트 미등록 = 치환 누락 (mig_verify coverage)\n'
                   '   ★apply 직후 필수. 누락은 diff 로 안 보이고 여기서만 잡힌다(0.5.8 serpen 5건).',
                   [PY, os.path.join(HERE, 'mig_verify.py'), 'coverage'])))
    ok.append(('③ RVA — 연동 그룹(dups)',
               run('③ RVA: 같은 값을 여러 모드가 들고 있는가 = 한쪽만 고치면 사고 (mig_verify dups)',
                   [PY, os.path.join(HERE, 'mig_verify.py'), 'dups'])))
    ok.append(('④ 구조체 오프셋',
               run('④ 구조체 오프셋: 우리가 의존하는 함수가 쓰는 필드 오프셋이 움직였는가 (offsets check)\n'
                   '   ★0.5.8 크래시의 진범 축. RVA 가 다 맞아도 여기가 밀리면 게임이 죽는다.',
                   [PY, os.path.join(HERE, 'offsets.py'), 'check', '--exe', a.exe, '--pkl', a.pkl])))
    ok.append(('⑤ 환경(deps·게이트·SDK·apply·stale)',
               run('⑤ 환경: mod_info 무결성/deps 대역 · 버전게이트 상수 · 빌드 SDK 경로 ·\n'
                   '   apply 누락(매니페스트↔소스) · 배포 dll stale (env)',
                   [PY, os.path.join(HERE, 'env.py'), '--exe', a.exe, '--ver', a.ver, '--sdk', a.sdk])))

    print('\n' + '#' * 78)
    print('# 마이그 축 요약')
    for name, good in ok:
        print('#   %-32s %s' % (name, 'OK' if good else '★남은 작업 있음'))
    bad = [n for n, g in ok if not g]
    if bad:
        print('#\n# ⛔ "마이그 완료"라고 하면 안 된다. 남은 축: %s' % ', '.join(bad))
        print('#   ④가 걸렸으면 → python MIG\\offsets.py sources   (고칠 소스 위치를 찍어준다)')
        print('#   ⑤가 걸렸으면 → 출력의 !! 줄이 그대로 작업 목록이다')
    else:
        print('#\n# ✅ 전 축 클린. 남은 것은 **인게임 검증**뿐이다(정적 검사는 그것을 대신하지 못한다).')
    print('#' * 78)
    return 1 if bad else 0


if __name__ == '__main__':
    sys.exit(main())
