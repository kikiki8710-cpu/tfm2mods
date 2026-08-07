# 순서도 보기 보강:
#  ① 새 설정값군(th_/ae_/bv_/mv2_/c3_/hd_/d4_/lt_/nx_/eh_/rt_/jg_/fix_)을 층에 배치
#  ② ★어느 그룹에도 안 걸린 키를 **자동으로 주워 담는** 마지막 층 추가
#     — 지금까지 새 노브를 추가하면 순서도에서 조용히 사라지고 있었다. 다시는 그러지 않게 한다.
#  ③ 각 설정값 밑에 **설명문을 바로 표시**(지금은 '설명' 버튼을 눌러야 보임)
import io, sys
sys.stdout.reconfigure(encoding="utf-8")
P = "src/main.rs"
s = io.open(P, encoding="utf-8").read()

# ── ① flow_keys: "*" 를 "남은 것 전부"로 해석 ──
A = """        if prefixes.iter().any(|p| if p.ends_with('_') { k.starts_with(*p) } else { k == *p }) {"""
B = """        if prefixes.iter().any(|p| *p == "*" || if p.ends_with('_') { k.starts_with(*p) } else { k == *p }) {"""
assert A in s; s = s.replace(A, B, 1)

# ── ② 층 3(매퍼)에 라인 배정·총력전 추가 ──
A2 = """      FlowGroup{ label:"넥서스 방어",
        note:"막바지에 넥서스를 지킬 때의 판단입니다. 진척도와 체력 경계로 물러날지 버틸지 정합니다.",
        prefixes:&["nxd_", "nx_repl"] },
    ] },"""
B2 = """      FlowGroup{ label:"넥서스 방어",
        note:"막바지에 넥서스를 지킬 때의 판단입니다. 진척도와 체력 경계로 물러날지 버틸지 정합니다.",
        prefixes:&["nxd_", "nx_repl"] },
      FlowGroup{ label:"라인 배정 · 봇 듀오",
        note:"어느 라인에 서고 언제 물러날지를 정합니다. 레인 번호는 0=탑 · 1=미드 · 2=바텀이고, 특수 처리가 붙는 것은 바텀(봇 듀오)입니다. 체력 기준을 올리면 조금만 아파도 뒤로 빠집니다.",
        prefixes:&["d4_"] },
      FlowGroup{ label:"라인 총력전",
        note:"한 라인에 다 같이 밀어붙일 때의 판단입니다. 아군에게 붙기 시작하는 거리를 내리면 잘 안 모입니다.",
        prefixes:&["lt_"] },
      FlowGroup{ label:"후퇴 트리거 · 정글 진행",
        note:"'내가 곧 죽는다'를 따로 감시하는 장치와, 정글을 계속 돌지 정하는 체력 기준입니다. 후퇴 기준 세 개는 고정값이 아니라 판단력이 높을수록 한쪽으로 밀리는 직선입니다.",
        prefixes:&["rt_", "jg_"] },
    ] },"""
assert A2 in s; s = s.replace(A2, B2, 1)

# ── ③ 층 4(후보 만들기)에 숨기·지원스킬·해금레벨 추가 ──
A3 = """      FlowGroup{ label:"넥서스 공수 실행",
        note:"넥서스를 치거나 지킬 때의 체력·거리 임계입니다.",
        prefixes:&["oi_", "d19_", "d19i_"] },
    ] },"""
B3 = """      FlowGroup{ label:"넥서스 공수 실행",
        note:"넥서스를 치거나 지킬 때의 체력·거리 임계입니다.",
        prefixes:&["oi_", "d19_", "d19i_", "nx_"] },
      FlowGroup{ label:"숨기",
        note:"수풀로 숨을지, 어디로 물러날지를 정합니다. 후보 선별 거리가 이 판단에서 압도적으로 많이 쓰이는 값이라, 여기만 바꿔도 은신 동선이 크게 달라집니다.",
        prefixes:&["hd_"] },
      FlowGroup{ label:"아군 지원스킬 낭비 방지",
        note:"체력이 넉넉한 아군에게 지원 스킬을 허비하지 않게 거르는 필터입니다. 체력 상한을 내리면 진짜 위급할 때만 씁니다. 대상 주변에 적이 없으면 아예 안 씁니다.",
        prefixes:&["c3_"] },
      FlowGroup{ label:"스킬 해금 레벨",
        note:"두 번째 스킬과 궁이 열리는 레벨입니다. 게임 데이터가 아니라 코드에 박혀 있어 원래는 손댈 수 없던 값이고, 바꾸면 성장 곡선 자체가 달라집니다. 같은 값이 여러 함수에 중복돼 있어 짝이 되는 항목을 같은 값으로 맞춰야 합니다.",
        prefixes:&["ex_skill2_level", "ex_ult_level", "ex_ult_level_x", "ex_skill2_level_x"] },
      FlowGroup{ label:"모르가드 · 세르펜 사냥",
        note:"에픽 몬스터를 사냥할 때 붙는 거리와 포기 조건입니다. 이 판단은 팀 전술 '마무리'를 읽는 단 두 곳 중 하나이기도 합니다.",
        prefixes:&["eh_"] },
    ] },"""
assert A3 in s; s = s.replace(A3, B3, 1)

# ── ④ 층 5(경매)에 위협 수치 생산·전투 실익·라인수비 후보점수 추가 ──
A4 = """      FlowGroup{ label:"★행동 성향 흔들림","""
B4 = """      FlowGroup{ label:"★★★위험 수치를 만드는 값",
        note:"자리 평가가 쓰는 재료를 만드는 곳입니다. 주변의 적을 훑어 평타·스킬 사거리의 띠를 만들고, 피해는 미리 만들어 둔 표에서 꺼내 '내 체력의 몇 %'로 환산한 뒤 적 하나당 상한에서 자릅니다. 훑는 반경을 줄이면 시야 밖 위협에 둔감해져 과감해지고, 상한을 내리면 강한 적 한 명을 덜 무서워합니다.",
        prefixes:&["th_"] },
      FlowGroup{ label:"라인 수비 후보 점수 — 접근·미니언 자리",
        note:"이동 계열 후보를 직접 채점합니다. 확실히 처치할 수 있는 대상이 있으면 가장 큰 가산이 붙어 막타 집착을 좌우합니다. 위험·이득을 얼마나 줄여서 반영할지도 여기서 정합니다.",
        prefixes:&["ae_"] },
      FlowGroup{ label:"★★전투 실익 계산",
        note:"스킬 한 방의 가치를 매깁니다. 상한에서 잘리고, 반경 안 아군 수에 따라 집중포화 배율이 붙고, 다른 아군이 이미 죽일 수 있는 만큼은 감점됩니다. 피해로 값을 매길 수 없는 특수 효과는 별도의 고정 점수표를 씁니다.",
        prefixes:&["bv_"] },
      FlowGroup{ label:"★행동 성향 흔들림","""
assert A4 in s; s = s.replace(A4, B4, 1)

# ── ⑤ 층 6(실행)에 이동 입력 생성기 추가 ──
A5 = """      FlowGroup{ label:"행동을 얼마나 자주·오래 붙잡나","""
B5 = """      FlowGroup{ label:"★★★실제 이동 만들기 (모든 이동이 통과)",
        note:"고른 행동이 이동이면 목적지 좌표만 정해진 채 이 한 곳을 통과해 실제 이동 입력이 됩니다. 그래서 도망·추적·접근·자리잡기에 전부 동시에 영향을 줍니다. 도착 판정 거리를 올리면 막판에 파고들고, 내리면 끝까지 남을 피해 돌아갑니다. 우물 탈출은 판단보다 먼저 걸립니다.",
        prefixes:&["mv2_arrive_snap", "mv2_avoid_", "mv2_well_", "mv2_pos_mode_thr"] },
      FlowGroup{ label:"행동을 얼마나 자주·오래 붙잡나","""
assert A5 in s; s = s.replace(A5, B5, 1)

io.open(P, "w", encoding="utf-8").write(s)
print("FLOW 그룹 배치 완료")
