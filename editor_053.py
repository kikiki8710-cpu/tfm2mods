# -*- coding: utf-8 -*-
# editor_053.py — 설정편집기(ai_adjust_editor) 를 0.5.3 실측 상태로 갱신.
#   ① numbers_threat_sp16/17 = 코드가 읽지 않음(감사 실측) → 死레버 섹션으로 이동
#   ② disc7(Recall) 재가동 반영 — scan2 경로만 원본위임, 나머지 DIFF=0
#   ③ note 의 0.5.2 서술을 0.5.3 실측으로 갱신(재핀 완료·사이트 수 변화·극성 반전)
import io, sys, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
P = r"C:\tfm2mods\ai_adjust_editor\src\main.rs"
s = open(P, encoding="utf-8").read()
n = 0


def rep(a, b, why):
    global s, n
    if a in s:
        s = s.replace(a, b, 1)
        n += 1
        print("  ✅ %s" % why)
    else:
        print("  ⚠ 패턴 불일치(건너뜀): %s" % why)


# ① numbers_threat_sp16/17 → 死레버 (감사: 코드에 read site 없음)
rep('"§◆ [16·17]세르펜 전력회피 (-1=공통·혼자 불리하면 후퇴·같이면 안걸림)","numbers_threat_sp16","numbers_threat_sp17",',
    '"§⛔ 死레버 (0.5.3 감사: 코드에 read site 없음 — 값 바꿔도 무반영)","numbers_threat_sp16","numbers_threat_sp17",',
    "numbers_threat_sp16/17 → 死레버 이동")

# ② disc7 Recall 재가동
rep('"§🆕 disc7 귀환 이동판단 (d7_repl=1 켜야 반영 — movepri Recall 리졸버)",',
    '"§✅ disc7 귀환 이동판단 [0.5.3 재가동] — d7_repl=1 기본ON. scan2 경로만 원본위임",',
    "disc7 섹션 라벨 = 재가동")

# ③ 넥서스 탭 note: 0.5.2 → 0.5.3
rep('★<b>0.5.2 재핀 완료(2026-07-23)</b>',
    '★<b>0.5.3 재핀 완료(2026-07-30)</b> — 전 사이트 실측 재핀(obj 14/14·d19 10/10). '
    '<b>oi_an_cull_dist</b> 는 0.5.3에서 게임이 후보를 <b>세 묶음으로 나눠 훑도록</b> 바뀌어 패치 자리가 1곳→3곳이 됐고, '
    '비교 방향도 뒤집혔다(모드가 내부에서 보정하므로 <b>설정값 의미·체감은 동일</b>). / 구 <b>0.5.2 재핀(2026-07-23)</b>',
    "넥서스 탭 note 0.5.3 갱신")

# ④ gb 탭 note: 사이트 수 10→9
rep('✅[07-16 경로A] <b>gb_enable=1</b>',
    '✅<b>[0.5.3 재핀 2026-07-30]</b> 적용 사이트 <b>10곳 → 9곳</b>(<b>gb_scout_radius</b> 가 루프 앞뒤 2곳에서 '
    '<b>루프 안 1곳으로 병합</b>). 값이 줄어든 게 아니라 자리가 합쳐진 것이고, 비교 방향 반전도 모드가 보정하므로 '
    '<b>설정값 의미·체감은 동일</b>. 적용확인=gb_imm.txt(applied=N/<b>9</b>).<br>[07-16 경로A] <b>gb_enable=1</b>',
    "gb 탭 note 0.5.3 갱신")

# ⑤ desc 갱신 — d7_repl
rep('"numbers_threat_sp16" => "세르펜 사냥(16) 전용 전력회피 임계.',
    '"numbers_threat_sp16" => "⛔DEAD(0.5.3 감사) — 코드에 read site 없음. 값 바꿔도 무반영. (구 설명) 세르펜 사냥(16) 전용 전력회피 임계.',
    "numbers_threat_sp16 desc = DEAD 표기")

# ⑥ ★sim_unchunk = 0.5.3 신규 死레버 (실측: sim_unchunk.txt 가 `ABORT bytes=90e9`)
#    패치 사이트 12B 시그가 0.5.3 exe 전역 0건 = rayon 브리지 코드 자체가 바뀜.
#    코드가 원본바이트 재검증 후에만 패치하므로 ABORT = fail-safe(게임 무영향), 노브만 죽음.
rep('"§◆ 백그sim 병렬도 개선 (실험)","sim_unchunk",',
    '"§⛔ 死레버 (0.5.3에서 무효 — 패치 시그 소멸, 값 바꿔도 무반영)","sim_unchunk",',
    "sim_unchunk → 死레버 이동 (0.5.3 ABORT 실측)")

# desc 도 DEAD 표기
rep('"sim_unchunk" => "✅[07-16 실험]',
    '"sim_unchunk" => "⛔DEAD(0.5.3) — 패치 사이트 12B 시그가 0.5.3 exe 전역 0건(rayon 브리지 코드 변경). '
    '원본바이트 재검증 후에만 패치하므로 ABORT=fail-safe(게임 무영향)이나 노브는 무반영. 실측=sim_unchunk.txt. (구 설명) [07-16 실험]',
    "sim_unchunk desc = DEAD 표기")

open(P, "w", encoding="utf-8").write(s)
print("\n적용 %d건" % n)
