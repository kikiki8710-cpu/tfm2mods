# -*- coding: utf-8 -*-
"""★크래시 수정 — 적용 체인의 동시 실행 경쟁(race).

증상: cfg 를 통째로 교체한 뒤 경기 중 AV(0xc0000005, access=write).
      faultAddr = exe+0xCA0008 = `th_skill_margin` 사이트 `0xca0006` + imm_off 2 의 **쓰기 대상 주소**.
      RIP 은 시스템 DLL(memcpy) — 즉 우리가 부른 복사가 폴트했다.

원인: `patch_imm_bytes` 는 VirtualProtect(쓰기허용) → 쓰기 → VirtualProtect(원복) 순서다.
      그런데 적용 체인의 게이트가 `cfg_gen != APPLY_GEN` **단순 비교**라, retreat 훅이 도는
      rayon 워커 여러 개가 **동시에 통과**해 같은 사이트를 함께 패치한다.
      A 가 원복한 직후 B 가 쓰면 그 페이지는 이미 읽기전용 → **B 의 쓰기가 AV**.
      기본값(-1)일 때도 체인은 원본값을 그대로 쓰므로 경쟁 자체는 늘 있었지만,
      cfg 교체로 CFG_GEN 이 올라 **경기 중에** 체인이 다시 돌면서 드러났다.

부수 증상: 체인이 중간에 죽으니 뒤쪽 묶음은 적용되지 않는다
          = "config 에서 특정 항목만 빼고 불러온다"로 보이던 것의 정체.

수정: 세대 게이트 안쪽을 CAS 로 한 스레드만 들어가게 잠근다.
      못 들어간 스레드는 그냥 지나가고 다음 발화 때 재시도한다(기존 재시도 의미 보존)."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/tfm2_ai_adjust/src/tfm2_ai_adjust.rs'
t = io.open(P, encoding='utf-8').read()
n = 0


def sub1(old, new, tag):
    global t, n
    if old not in t:
        print('  [건너뜀] %s' % tag); return
    t = t.replace(old, new, 1); n += 1
    print('  [ok] %s' % tag)


sub1('static APPLY_GEN: AtomicU64 = AtomicU64::new(0);',
'''static APPLY_GEN: AtomicU64 = AtomicU64::new(0);
/// ★[08-06 크래시수정] 적용 체인 배타 락. retreat 훅은 rayon 워커 여러 개에서 동시에 불린다 —
///   락이 없으면 두 스레드가 같은 사이트를 함께 패치하고, 한쪽이 VirtualProtect 로 보호를 되돌린
///   직후 다른 쪽이 쓰면서 **AV(write)** 가 난다(실사고: exe+0xCA0008 = th_skill_margin 사이트).
///   ⚠체인이 거기서 죽으면 **뒤쪽 묶음은 적용되지 않는다** — "일부 설정만 안 먹는다"로 보인다.
static APPLY_LOCK: AtomicBool = AtomicBool::new(false);''', 'APPLY_LOCK 선언')

sub1('''    if cfg_gen != APPLY_GEN.load(Ordering::Relaxed) {
        apply_call_ablate();''',
'''    if cfg_gen != APPLY_GEN.load(Ordering::Relaxed)
        && APPLY_LOCK.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok()
    {
        apply_call_ablate();''', '게이트 CAS 진입')

sub1('''        if exe_base() != 0 && READY_TICKS.load(Ordering::Relaxed) >= READY_MIN {
            APPLY_GEN.store(cfg_gen, Ordering::Relaxed);   // READY 상태서 체인 완주 = 이 세대 완료
        }
    }''',
'''        if exe_base() != 0 && READY_TICKS.load(Ordering::Relaxed) >= READY_MIN {
            APPLY_GEN.store(cfg_gen, Ordering::Relaxed);   // READY 상태서 체인 완주 = 이 세대 완료
        }
        APPLY_LOCK.store(false, Ordering::Release);   // ★[08-06] 락 해제 — 실패해도 반드시 푼다
    }''', '락 해제')

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n적용 %d건' % n)
