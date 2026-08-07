# -*- coding: utf-8 -*-
"""중복 사이트 정리 — `ex_wait_dist`/`ex_wait_back` 와 `lw_wait_dist`/`lw_back` 가
**같은 두 주소를 각각 패치**하고 있었다(0xe721d3 · 0xe727c4).

증상: 나중에 적용되는 쪽이 이긴다 ⟹ 한쪽 값을 바꿔도 다른 쪽이 덮으면 **조용히 무효**.
     applied=N/N · blocked=0 이라 지표로는 전혀 드러나지 않는다(08-06 사고와 같은 계열).

조치: `lw_*` 를 정본으로 남기고(`lw_radius` 와 같은 "라인 대기" 묶음이라 일관됨),
     `ex_wait_*` 는 **알리아스**로 돌려 기존 cfg 를 깨지 않는다. exec 쪽 중복 패치는 제거.

설명문: 두 노브의 설명이 서로 달랐다(`아군 경로의 끝` vs `적까지의 경로 길이`).
     디스어셈으로 확인된 것은 **"경로를 따라간 누적 길이에서 이만큼 뺀 지점"** 까지다
     (0xe727bf `add rax,r12` → `sub rax,0x2bf20` → `cmovae` 0클램프).
     대상이 적인지 아군인지는 `rsi` 출처가 `lea rsi,[rbx+rdx*8]` 이라 미확정 —
     **확인된 사실만** 적고 미확정은 미확정이라고 적는다."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

D = 'C:/tfm2mods/tfm2_ai_adjust/src/detour.rs'
t = io.open(D, encoding='utf-8').read()
n = 0

# ── ① exec 쪽 중복 패치 2개 제거 ──
OLD = '''    // ── ② 대기 위치 (line_wait) ──
    ok += patch_imm_bytes(base + 0xe721d3, &[0x48,0xb9], 2, 8,
                          if wdst < 0 { 0x78B30C401 } else { sq1(wdst) }) as u32;            // movabs rcx, 180000²+1
    ok += patch_imm_bytes(base + 0xe727c4, &[0x48,0x2d], 2, 4, b4(wbak, 0x02bf20)) as u32;    // sub rax, 180000
'''
NEW = '''    // ── ② 대기 위치 (line_wait) ──
    // ⛔[08-07 중복 제거] 이 두 사이트(0xe721d3 · 0xe727c4)는 **apply_score_imm 의 lw_wait_dist / lw_back
    //    과 같은 주소**였다. 두 묶음이 같은 바이트를 각각 패치해 **나중 것이 이기는** 상태였고,
    //    한쪽 값을 바꿔도 다른 쪽이 덮으면 조용히 무효가 됐다(applied=N/N 이라 지표로 안 드러남).
    //    ⟹ lw_* 를 정본으로 두고 여기선 패치하지 않는다. ex_wait_* 는 알리아스로 계속 동작한다.
'''
assert OLD in t, 'exec 중복 블록 원문 불일치'
t = t.replace(OLD, NEW, 1); n += 1
print('  [ok] apply_exec_imm 중복 패치 2개 제거')

# 쓰이지 않게 된 tune 2개는 남기되(설정 파싱 호환) _ 로 소비
OLD2 = '''    let wdst = tune("ex_wait_dist", -1);     // 대기 경로 전환 거리(유닛, 원본 180000)
    let wbak = tune("ex_wait_back", -1);     // 목적지에서 뒤로 물러날 거리(원본 180000)'''
NEW2 = '''    // ⛔[08-07] 아래 둘은 lw_wait_dist / lw_back 의 **알리아스**가 됐다(중복 사이트 제거).
    //    tune() 호출을 남겨두면 sig 에 섞여 무의미한 재적용을 유발하므로 읽지 않는다.'''
assert OLD2 in t
t = t.replace(OLD2, NEW2, 1); n += 1
print('  [ok] ex_wait_* tune() 제거')

# sig 배열에서 wdst/wbak 제거
import re
m = re.search(r'for v in \[([^\]]*wdst[^\]]*)\]', t)
if m:
    inner = m.group(1)
    new_inner = ', '.join(x.strip() for x in inner.split(',')
                          if x.strip() not in ('wdst', 'wbak'))
    t = t[:m.start(1)] + new_inner + t[m.end(1):]
    n += 1
    print('  [ok] sig 배열에서 wdst/wbak 제거')
else:
    print('  [건너뜀] sig 배열 — 수동 확인 필요')

io.open(D, 'w', encoding='utf-8', newline='\n').write(t)

# ── ② 알리아스 등록 ──
P = 'C:/tfm2mods/tfm2_ai_adjust/src/tfm2_ai_adjust.rs'
s = io.open(P, encoding='utf-8').read()
A = '                "sr_near_dist"  => "nxd_near_dist",'
NEWA = ('                // ★[08-07] 중복 사이트 정리 — ex_wait_* 와 lw_* 가 같은 주소를 각각 패치하고 있었다.\n'
        '                //   lw_* 를 정본으로 삼고 옛 이름은 알리아스로 살린다(기존 cfg 무손실).\n'
        '                "ex_wait_dist" => "lw_wait_dist", "ex_wait_back" => "lw_back",\n' + A)
if '"ex_wait_dist" => "lw_wait_dist"' not in s:
    assert A in s
    s = s.replace(A, NEWA, 1); n += 1
    io.open(P, 'w', encoding='utf-8', newline='\n').write(s)
    print('  [ok] 알리아스 2개 등록')

print('\n적용 %d건' % n)
