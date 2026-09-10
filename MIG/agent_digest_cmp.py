#!/usr/bin/env python3
"""agent_digest_cmp.py — agent_link 모드 1(원본 에이전트) vs 모드 2(내 사본 에이전트) 의 get_input 다이제스트 대조.

입력: 두 디렉토리(각각 judge_agent_link.txt + agent_digest_modeN_<seed>.bin …)
원리: 타이틀 화면의 백그라운드 리그 시뮬 때문에 전역 시퀀스는 소음이다. 다이제스트는 **게임 시드별**로 저장되고
      (tick, team<<8|id, presim, h) 로 정렬된 정준 시퀀스라 스레드 순서와 무관하다.
      두 디렉토리에 **공통으로 존재하는 시드**(= 사용자가 두 번 돌린 같은 리플레이)만 레코드 단위로 대조한다.
사용: python MIG\agent_digest_cmp.py <dir_mode1> <dir_mode2>
"""
import glob, io, os, re, struct, sys

REC = struct.Struct('<IHBBQ')   # tick u32 · pl u16 · exp u8 · pad u8 · h u64


def load(d):
    txt = io.open(os.path.join(d, 'judge_agent_link.txt'), encoding='utf-8', errors='ignore').read()
    head = txt.split('\n')[0]
    seeds = {}
    for f in glob.glob(os.path.join(d, 'agent_digest_mode*_*.bin')):
        seed = os.path.basename(f).rsplit('_', 1)[1][:-4]
        b = open(f, 'rb').read()
        seeds[seed] = [REC.unpack_from(b, i) for i in range(0, len(b) - len(b) % REC.size, REC.size)]
    return head, seeds


def main(a, b):
    ha, sa = load(a); hb, sb = load(b)
    print('mode1:', ha[:160]); print('mode2:', hb[:160])
    common = sorted(set(sa) & set(sb), key=lambda s: -len(sa[s]))
    print('\n시드: mode1 %d개 · mode2 %d개 · 공통 %d개 %s' % (len(sa), len(sb), len(common), common[:4]))
    if not common:
        print('공통 시드 없음 — 같은 리플레이를 두 모드로 각각 한 번씩 돌렸는지 확인'); return 2
    ok_all = True
    for s in common:
        A, B = sa[s], sb[s]
        n = min(len(A), len(B))
        first = next((i for i in range(n) if A[i] != B[i]), None)
        print('\n[seed %s] 레코드 %d vs %d · tick %d..%d vs %d..%d' % (s, len(A), len(B), A[0][0] if A else 0, A[-1][0] if A else 0, B[0][0] if B else 0, B[-1][0] if B else 0))
        # 순서무관 지표
        sumA = sum(r[4] for r in A) & (2**64 - 1); sumB = sum(r[4] for r in B) & (2**64 - 1)
        print('   sum %016x vs %016x %s' % (sumA, sumB, 'OK' if sumA == sumB else '← 다름'))
        if first is None and len(A) == len(B):
            print('   정준 시퀀스 %d개 전부 동일 → 이 시드 DIFF=0' % n)
        else:
            ok_all = False
            if first is not None:
                ra, rb = A[first], B[first]
                print('   첫 불일치 #%d/%d (%.2f%%): tick %d pl %04x exp %d h %016x  vs  tick %d pl %04x exp %d h %016x'
                      % (first, n, 100.0 * first / max(n, 1), ra[0], ra[1], ra[2], ra[4], rb[0], rb[1], rb[2], rb[4]))
                diff = sum(1 for i in range(n) if A[i] != B[i])
                print('   공통 구간 불일치 %d / %d' % (diff, n))
                # 첫 불일치 틱 이전까지는 동일한가 → "어느 틱부터 갈라졌나"
                t0 = min(ra[0], rb[0])
                print('   갈라진 틱 = %d (그 전 틱까지 %d개 동일)' % (t0, sum(1 for r in A[:first])))
            else:
                print('   공통 구간 동일 · 길이만 다름(한쪽이 덜 진행됐거나 덤프 시점 차)')
    print('\n판정:', 'DIFF=0 — 공통 시드 전부에서 에이전트 결정이 비트동일' if ok_all else '차이 있음 — 위 항목 확인')
    return 0 if ok_all else 1


if __name__ == '__main__':
    sys.exit(main(sys.argv[1], sys.argv[2]))
