# -*- coding: utf-8 -*-
u"""`config\\game\\mods.json` 의 `enabled_mods` 에 `tfm2_judge_verify` 를 넣는다. (2026-09-12)

★왜 필요한가 — 게임 `mods\\` 폴더에 dll 을 넣는 것만으로는 **로드되지 않는다.**
  `enabled_mods` 목록에 있어야 한다. 실측: 30개 목록에 `tfm2_ai_adjust` 가 **없고**
  (= 이미 비활성 상태 · 폴더명 변경 불요) `tfm2_judge_verify` 도 당연히 없다.
  ⟹ 이 단계를 빼먹으면 「배포했는데 아무 로그도 안 나온다」가 되고, 그걸
     「발화 0」으로 오독하면 1단계 결론이 통째로 틀린다.

★백업 규약은 이 프로젝트의 선례를 따른다 — `mods.json.bak_<날짜>_<무엇>`
  (실재: `bak_20260911_legacyoff`·`bak_20260911_riotoff`·`bak_20260911_itembuild`).

⚠BOM 금지 — 게임이 읽는 json 이다(CLAUDE.md §2: BOM 이면 파서 실패 → 모드 강제비활성).
  원본 인코딩·들여쓰기를 보존하고 목록에 한 항목만 더한다.
"""
import io
import json
import os
import shutil
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

P = (r"C:\Program Files (x86)\Steam\steamapps\common"
     r"\Teamfight Manager2\config\game\mods.json")
MOD = "tfm2_judge_verify"
BAK = P + ".bak_20260912_judgeverify"

raw = io.open(P, "rb").read()
if raw[:3] == b"\xef\xbb\xbf":
    print(u"⚠원본에 BOM 이 있다 — 건드리지 않고 멈춘다(게임 파서 실패 경로)")
    sys.exit(1)

d = json.loads(raw.decode("utf-8"))
em = d.get("enabled_mods")
if not isinstance(em, list):
    print(u"⛔`enabled_mods` 가 리스트가 아니다 — 구조가 바뀌었다. 멈춘다.")
    sys.exit(1)

print(u"전: enabled_mods %d개 · %s 포함=%s · tfm2_ai_adjust 포함=%s"
      % (len(em), MOD, MOD in em, "tfm2_ai_adjust" in em))

if MOD in em:
    print(u"이미 들어 있다 — 변경 없음")
    sys.exit(0)

if not os.path.exists(BAK):
    shutil.copy2(P, BAK)
    print(u"백업 → %s" % os.path.basename(BAK))

em.append(MOD)
# ★`accepted_code_mod_warnings` — 코드 모드는 경고 수락 목록에 없으면 게임이 물어본다.
#   미리 넣어 두면 리플레이 중 팝업으로 멈추지 않는다(실측 목록에 `tfm2_ai_adjust` 등이 있다).
acc = d.get("accepted_code_mod_warnings")
if isinstance(acc, list) and MOD not in acc:
    acc.append(MOD)
    print(u"`accepted_code_mod_warnings` 에도 추가(경고 팝업 방지)")

out = json.dumps(d, ensure_ascii=False, indent=1)
io.open(P, "w", encoding="utf-8", newline="\n").write(out)

# ★쓴 뒤 **다시 읽어** 확인한다(「고쳤다」를 도구가 재측정 — 이 프로젝트의 규율)
chk = io.open(P, "rb").read()
d2 = json.loads(chk.decode("utf-8"))
print(u"후: enabled_mods %d개 · %s 포함=%s · BOM=%s"
      % (len(d2["enabled_mods"]), MOD, MOD in d2["enabled_mods"],
         u"있음(불량!)" if chk[:3] == b"\xef\xbb\xbf" else u"없음"))
print(u"   첫 바이트 = %s" % u" ".join(u"%02x" % b for b in chk[:3]))
