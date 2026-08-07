# 「기타」탭 신설 + 게임 버그 수정 스위치(fix_skill2_dmg) 배선
import io, sys
sys.stdout.reconfigure(encoding="utf-8")
P = "src/main.rs"
s = io.open(P, encoding="utf-8").read()

# ① 체크박스로 렌더되도록 토글 목록에 추가
A = '|"nx_repl"|"d12_repl"|"d14_repl"|"d15_repl"|"skip_untuned"|"sp_seen")'
B = '|"nx_repl"|"d12_repl"|"d14_repl"|"d15_repl"|"skip_untuned"|"sp_seen"\n    |"fix_skill2_dmg")'
assert A in s; s = s.replace(A, B, 1)

# ② 「기타」탭 — [공통] 엔진 탭 앞에 둔다
NEW = '''  Tab{ id:"misc", title:"• [기타] 게임 원본의 결함 고치기", keys:&[
      "§적의 두 번째 스킬 피해를 무시하는 문제","fix_skill2_dmg",], note:
    "게임 원본에 있는 <b>계산 결함</b>을 선택적으로 고칩니다. <b>전부 기본 꺼짐</b>이라, 켜지 않으면 원본과 완전히 같습니다.<br>\\
     <br>\\
     <b>■ 적의 두 번째 스킬 피해를 무시하는 문제</b><br>\\
     AI가 &quot;저 자리로 가면 얼마나 아플까&quot;를 계산할 때, 적의 <b>기본공격 · 첫 번째 스킬 · 궁극기</b>는 제대로 세면서 \\
     <b>두 번째 스킬만 첫 번째 스킬 값으로 대신</b> 넣습니다. 코드에서 값을 옮기는 줄이 잘못 쓰여 있어서 생긴 문제이고, \\
     <b>아군을 볼 때는 정상</b>입니다.<br>\\
     그래서 AI는 <b>두 번째 스킬이 강한 적을 실제보다 덜 무서워하고</b>, 반대로 두 번째 스킬이 약한 적은 과하게 피합니다. \\
     체감으로는 &quot;딜러 옆에 겁 없이 서 있다&quot; 쪽으로 나타납니다.<br>\\
     <b>켜면</b> 두 번째 스킬 피해를 제대로 세도록 바꿉니다 — 전체적으로 <b>조금 더 몸을 사리는</b> 판단이 됩니다.<br>\\
     ⚠원본 게임과 다르게 동작하므로, 원본 그대로를 원하면 꺼 두세요. 적용확인 = <b>fix_imm.txt</b>." },

'''
ANCH = '  Tab{ id:"engine",'
assert ANCH in s; s = s.replace(ANCH, NEW + ANCH, 1)

# ③ 설명 + 원본값
D = '    "fix_skill2_dmg" => "켜면 적의 두 번째 스킬 피해를 위험 계산에 제대로 반영합니다. 기본 꺼짐 = 게임 원본 그대로",\n'
a = 'fn desc_static(k: &str) -> Option<&\'static str> {\n  Some(match k {\n'
assert a in s; s = s.replace(a, a + D, 1)
O = '    "fix_skill2_dmg" => "0",\n'
b = 'fn orig_val(k: &str) -> Option<&\'static str> {\n  Some(match k {\n'
assert b in s; s = s.replace(b, b + O, 1)

io.open(P, "w", encoding="utf-8").write(s)
print("기타 탭 + fix_skill2_dmg 배선 완료")
