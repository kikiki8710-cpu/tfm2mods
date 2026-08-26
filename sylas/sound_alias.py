# -*- coding: utf-8 -*-
"""사일러스 사운드 alias 생성기 — mod.override_info 의 사운드 항목을 다시 쓴다.

왜 필요한가
  사일러스는 **사운드 애셋이 하나도 없다**(번들·모드 통틀어 `sylas_*` 0건).
  게임은 `asset/base/sound/sfx/<챔피언id>_<애니태그>` 키로 효과음을 찾으므로
  사일러스는 전 동작이 무음이었다(2026-08-26 규명, 유저 확인 "잘된다").

왜 파일이 아니라 alias 인가
  `sound_info` 는 raw 미보존 확장자라 **merge 불가, override 만 가능**하고,
  `override` 는 **대상 키 존재를 검사하지 않아 없던 키를 새로 만들 수 있다**
  ([[tfm2-asset-override-merge]] §2·§3, 확신 A = 디스어셈 직독).
  ⟹ mp3·sound_info 를 새로 만들 필요 없이 **바닐라 사운드를 가리키기만** 하면 된다.

무엇을 매핑하나
  ① 사일러스 본인 동작 = `PERSONA`(사슬 테마 = prisoner/whip_master 계열 고정)
  ② 강탈한 궁 동작   = `--donor` 챔피언의 같은 태그 (`ult_pre`/`ult_loop`/`ult_end`)
  ③ 접미사 이름(`ult_pre_archer` 등)도 같이 건다 — tag_swap 을 다시 켤 때를 대비.
  소스가 번들에 실재하는 항목만 쓴다(없는 소스를 걸면 로더가 통째로 스킵한다).

실행
  python sound_alias.py --donor archer            # 미리보기
  python sound_alias.py --donor archer --write    # 개발본+게임본에 기록
"""
import argparse, json, os, shutil, sys

MOD   = r"C:\tfm2mods\sylas"
GAME  = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\sylas"
VAN   = r"C:\Users\jungs\Desktop\claude\tfm2\bundle_unpacked_0826"
SFX   = "asset/base/sound/sfx/"
FANIM = os.path.join(MOD, "aseprite_resources", "champions", "sylas#anim.fanim")

# 사일러스 본인 동작 → 사슬 테마 바닐라 사운드
PERSONA = {
    "attack":            "prisoner_attack",
    "skill":             "prisoner_skill1",
    "skill2":            "prisoner_skill2",
    "skill2_cast":       "prisoner_skill2",
    "skill_dash":        "prisoner_ult_dash",
    "ult":               "prisoner_ult_end",      # 강탈 시전
    "ult_attack":        "whip_master_ult_attack",
    "unshackled_attack": "whip_master_ult_attack",
    "ult_dash":          "prisoner_ult_dash",
}
# 공여자에서 가져오는 태그(강탈한 궁의 연출)
DONOR_TAGS = ["ult_pre", "ult_loop", "ult_end", "ult_idle", "ult_heal",
              "ult_attack", "ult_dash", "ult"]


def have(name, pool):
    return name in pool


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--donor", required=True, help="궁을 빌려오는 챔피언 id (예: archer)")
    ap.add_argument("--write", action="store_true", help="개발본+게임본에 기록")
    a = ap.parse_args()

    pool = {f[:-11] for f in os.listdir(os.path.join(VAN, "sound", "sfx"))
            if f.endswith(".sound_info")}
    tags = set(json.load(open(FANIM, encoding="utf-8"))["anims"])

    alias, miss = {}, []

    # ① 본인 동작
    for tag, src in PERSONA.items():
        if tag not in tags:      continue
        if not have(src, pool):  miss.append((tag, src, "소스 없음")); continue
        alias["sylas_" + tag] = src

    # ② 공여자 궁 동작 — 본인 것보다 우선(강탈 중엔 공여자 소리가 맞다)
    for tag in DONOR_TAGS:
        src = "%s_%s" % (a.donor, tag)
        if not have(src, pool): miss.append((tag, src, "공여자에 그 사운드 없음")); continue
        if tag in tags:
            alias["sylas_" + tag] = src
        # ③ 접미사 이름(tag_swap 재도입 대비)
        suf = "%s_%s" % (tag, a.donor)
        if suf in tags:
            alias["sylas_" + suf] = src

    # override_info 재작성 — 사운드 외 항목(text merge 등)은 보존
    dev = os.path.join(MOD, "mod.override_info")
    d = json.load(open(dev, encoding="utf-8-sig"))
    d = {k: v for k, v in d.items() if not k.startswith(SFX)}
    for t, s in sorted(alias.items()):
        d[SFX + t] = {"remapping": SFX + s, "type": "override"}

    print("공여자 = %s / 사운드 alias %d건" % (a.donor, len(alias)))
    for t, s in sorted(alias.items()):
        kind = "공여자" if s.startswith(a.donor + "_") else "본인"
        print("   %-6s %-24s → %s" % (kind, t, s))
    if miss:
        print("건너뜀(소스 없음):")
        for t, s, why in miss: print("   %-20s %-28s %s" % (t, s, why))

    if not a.write:
        print("\n미리보기만 했다. 기록하려면 --write 를 붙여라.")
        return

    txt = json.dumps(d, ensure_ascii=False, indent=2)
    for p in (dev, os.path.join(GAME, "mod.override_info")):
        if os.path.exists(p): shutil.copy2(p, p + ".bak_sound")
        open(p, "w", encoding="utf-8", newline="\n").write(txt)   # ★BOM 없는 UTF-8 필수
        b = open(p, "rb").read(1)
        assert b == b"{", "BOM 이 붙었다: %s" % p
        print("[기록] %s  %dB  첫바이트=%s" % (p, os.path.getsize(p), b.hex()))
    print("게임 재시작해야 반영된다(애셋은 시작 시 1회 로드).")


if __name__ == "__main__":
    main()
