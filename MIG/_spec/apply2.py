# -*- coding: utf-8 -*-
u"""apply2 — 13·14 의 `is_top_side` 극성 반전 + `target_bush_v41` 누락을 본 표에 반영.

근거 = 2026-09-11 검증배치 C (`_verify/REPORT_C.md`).
`is_top_side` 는 vis=pub·mir=1 이라 **SDK 오라클로 실행 확정**했다:
표본 9개 전부 `ry_lt_x == !is_top_side` (ry = height - y), 대각선(x+y==height)은 top/bottom 양쪽 true.
⟹ `is_top_side  <=>  x + y <= height  <=>  ry >= x  <=>  !(ry < x)`
IR 의 `%c = icmp ult ry, x` 는 **`!is_top_side`** 다. 동작 서술은 우연히 맞았지만 뜻이 정확히 뒤집혀 있었다.
"""
import io, json, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
P = "_spec/specs20.json"
D = json.load(io.open(P, encoding="utf-8"))
S = D["specs"]
N = [0]

POLAR = (u"`is_top_side` = `champ.x + champ.y <= height` = `!(ry < champ.x)` "
         u"(ry = height - champ.y). IR 의 `icmp ult ry, champ.x` 는 **그 부정**이다 "
         u"— 2026-09-11 검증배치 C 가 SDK 오라클 실행으로 확정(표본 9개 전부 일치, "
         u"대각선 x+y==height 는 top/bottom 양쪽 true)")


def rep(obj, key, old, new, label):
    s = obj[key]
    assert old in s, u"못 찾음: %s" % label
    obj[key] = s.replace(old, new)
    N[0] += 1
    print(u"  OK %s" % label)


def setv(obj, key, val, label):
    assert obj.get(key) != val, label
    obj[key] = val
    N[0] += 1
    print(u"  OK %s" % label)


def setl(lst, i, val, label):
    lst[i] = val
    N[0] += 1
    print(u"  OK %s" % label)


# -- 13 target_bush_v30 ----------------------------------------------
print(u"\n[13] target_bush_v30")
rep(S[13], "logic",
    u"//        본문식 = (ctx.setting.height - champ.y) <u champ.x   (map_regions.rs:22,24)",
    u"//        IR 술어 = (ctx.setting.height - champ.y) <u champ.x = **!is_top_side** "
    u"(~~is_top_side 본문식~~ 정정, 배치C 오라클). 원식 = x + y <= height",
    u"13 logic 극성 주석")
for j in (8, 12):
    c = S[13]["constants"][j]
    setv(c, "meaning", c["meaning"] +
         u" [= **!is_top_side**(봇 사이드) 일 때. ~~is_top_side 참~~ 아님 — 2026-09-11 배치C]",
         u"13 constants[%d].meaning" % j)
setl(S[13]["unknown"], 1,
     u"~~map_regions::is_top_side 의 극성 — 전 .ll 에 define 이 없어(항상 인라인) "
     u"`ry < x` 인지 `x <= ry` 인지 확정 못 함~~ -> **확정**(2026-09-11 검증배치 C). " + POLAR +
     u". 범위정정: 이건 「재료 부재」가 아니라 **미탐색**이었다 — `tcxq grep game_core is_top_side` "
     u"한 번이면 `vis=pub, mir=1` 이 보이고 오라클·MIR 두 경로가 열려 있었다",
     u"13 unknown[1] 극성 확정")

# -- 14 LineGankerPlan::update ---------------------------------------
print(u"\n[14] LineGankerPlan::update")
rep(S[14], "logic",
    u"//   ry = context.setting.height - champ.y ; 결과 = (ry < champ.x)",
    u"//   ry = context.setting.height - champ.y ; IR 술어 = (ry < champ.x) = **!is_top_side**"
    u"\n              //   (~~결과 = is_top_side~~ 정정, 2026-09-11 배치C 오라클. 원식 = x + y <= height)",
    u"14 logic 극성 주석")
rep(S[14], "logic", u"if is_top_side {",
    u"if ry_lt_x /* = !is_top_side */ {", u"14 logic if 변수명")
# (bush 삼항은 위 replace 가 전 occurrence 를 잡으므로 별도 처리 불필요)
FLIP = {13: (u"is_top_side 참", u"**is_top_side 거짓**(= 봇 사이드)"),
        14: (u"is_top_side 거짓", u"**is_top_side 참**(= 탑 사이드)"),
        15: (u"is_top_side 참", u"**is_top_side 거짓**(= 봇 사이드)"),
        16: (u"is_top_side 참", u"**is_top_side 거짓**(= 봇 사이드)"),
        17: (u"is_top_side 거짓", u"**is_top_side 참**(= 탑 사이드)"),
        18: (u"is_top_side 거짓", u"**is_top_side 참**(= 탑 사이드)")}
for j, (o, n) in FLIP.items():
    c = S[14]["constants"][j]
    assert o in c["meaning"], j
    setv(c, "meaning", c["meaning"].replace(o, n) +
         u" [극성 정정 2026-09-11 배치C: IR 술어 `ry < x` 가 **!is_top_side** 였다. 부시 ID 자체는 불변]",
         u"14 constants[%d] 극성 뒤집기(value=%s)" % (j, c["value"]))
kb = S[14]["knobs"][3]
setv(kb, "where",
     u"map_regions.rs:22~24 `is_top_side` — ry = GameSetting.height - champ.y, "
     u"**is_top_side = !(ry < champ.x) = (champ.x + champ.y <= height)**. "
     u"~~결과 = ry < champ.x~~ 는 극성 반전(2026-09-11 배치C 오라클 확정)",
     u"14 knobs[3].where")
setl(S[14]["unknown"], 2,
     u"~~ganker.rs:279 소스 조건의 극성 — 별도 define 이 없어(전량 인라인) 확정 불가~~ -> "
     u"**확정**(2026-09-11 검증배치 C). " + POLAR +
     u". 소스는 `if !is_top_side`(또는 is_bottom_side 계열)이 맞고 그래서 줄 280 이 먼저 온다. "
     u"⚠판정 어휘 범위정정: **「확정 불가」가 아니라 「미탐색(오라클·MIR)」이었다**",
     u"14 unknown[2] 극성 확정")

# target_bush_v41 -- 통째 누락분
S[14]["new_knobs"].extend([
    {"what": u"**갱커 실제 은신 목표 = `target_bush_v41` (이 명세에 통째로 빠져 있었다)**",
     "where": u"`target_bush_v41`(ganker.rs:310, `_gaibc/m08.ll:94136~94353`)를 "
              u"`LineGankerPlan::sub_plan`(m08.ll:94960)이 **무조건** 호출해 "
              u"`SubPlan::Hide{bush, out_line=Outline(1), check_move=0, enemy_spotted_me=0}`(태그 9)로 내보낸다. "
              u"반면 `update`(이 함수)는 `target_bush_v30` 을 인라인해 **도착 판정**에만 쓴다",
     "value": u"인덱스 = `*_lead[team]`, 범위검사 `lead < 7`",
     "effect": u"★**v30 != v41 인 구간에서는 `update` 의 도착 판정이 성립할 수 없어 "
               u"`CancelReason::TargetMissing` 취소가 원리적으로 발화하지 않는다.** "
               u"명세 knob 주석이 '가정'으로 걱정하던 것이 **현행 코드의 실제 상태**다. "
               u"구체 예: Top·team0·top_lead=0 이면 v41 은 부시 **16** 으로 보내는데 "
               u"v30 의 Top·team0 반환집합은 {2,3,6} 이라 16 이 없다. "
               u"⚠v30/v41 은 **버전 게이트가 아니라 호출자별 하드와이어**(양쪽 다 version 분기 없음) "
               u"— 2026-09-11 검증배치 C"},
    {"what": u"★갱커 매복 덤불을 실제로 지배하는 세 필드 (라인 통제도)",
     "where": u"`AbstractGameWithCache` `top_lead@0x21c0` / `mid_lead@0x21d0` / `bottom_lead@0x21e0` "
              u"(`[usize;2]`, IR 리터럴 8640/8656/8672) — `update` 의 reads 에는 당연히 없다",
     "value": u"target_bush_v41 전표 (m08.ll:94136~94353 실측)\n"
              u"  Top(0)    team0: [16, 6, 3, 3, 3, 2, 2]   team1: [2, 3, 6, 6, 6, 16, 16]\n"
              u"  Bottom(2) team0: [21, 20, 15, 15, 15, 9, 7]  team1: [9, 15, 20, 20, 20, 21, 23]\n"
              u"  Mid(1) 은 챔피언 좌표를 쓴다 — s = !is_top_side(봇 사이드) 라 할 때\n"
              u"    table[0] = team0 ? (s?21:17) : (s?9:4) · table[1..4] = s?14:11 · "
              u"table[5],[6] = team0 ? (s?9:4) : (s?21:17)",
     "effect": u"**라인 통제가 전진할수록(lead 0->6) 갱커 매복 덤불이 우리 진영 -> 적 진영으로 한 칸씩 밀린다** "
               u"— 부시 중심좌표로 검산해 단조임을 확인했다(team0 Top: lead0 부시16 (16000,656000) "
               u"= 아군 넥서스쪽 … lead5/6 부시2 (656000,16000) = 적 넥서스쪽). "
               u"이 세 필드를 조정하면 갱커 동선 전체가 바뀐다. "
               u"⚠**Mid 만 `player_champion[team][pos]` 를 unwrap 한다 = None 이면 패닉** "
               u"(Top/Bottom 은 챔피언을 안 본다)"},
])
N[0] += 1
print(u"  OK 14 new_knobs += 2 (target_bush_v41 · *_lead 3필드)")
S[14]["unknown"].append(
    u"⚠**일반화 금지**(2026-09-11 배치C): `_version` 미사용은 **이 함수(update) 본문에 한해** 맞다. "
    u"형제 `sub_plan` 이 `target_bush_v41` 을 부르므로 '이 플랜에 버전 게이트가 없다'로 "
    u"플랜 전체에 일반화하면 안 된다. 미탐색 = `target_bush_v41` 의 Mid 분기 IR 재확인 · "
    u"`*_lead` 를 누가 갱신하는지")
N[0] += 1
print(u"  OK 14 unknown += 1 (일반화 금지)")

D["meta"]["corrections"].append(
    u"2026-09-11 배치C 정정을 본 표에 반영(apply2.py): 13·14 `is_top_side` 극성 반전 "
    u"— IR 술어 `ry < x` 는 **!is_top_side** 다(오라클 실행 확정). "
    u"14 에 `target_bush_v41`(실제 은신 목표)·`*_lead` 3필드 전표 추가 "
    u"— v30!=v41 구간에서 TargetMissing 취소가 원리적으로 발화하지 않는다.")
with io.open(P, "w", encoding="utf-8", newline="\r\n") as f:
    f.write(json.dumps(D, ensure_ascii=False, indent=1))
print(u"\n총 %d곳 반영 -> %s" % (N[0], P))
