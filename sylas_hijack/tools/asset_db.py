# -*- coding: utf-8 -*-
"""
asset_db.py — 정적 비주얼/사운드 에셋 인벤토리.
- 챔프 시트 태그: champions/{champ}#anim.fanim
- 이펙트 시트 태그: skill_effect/*#anim.fanim
- 사운드: sound/sfx/*.sound_info (Sfx name = basename)
출력: assets_db.json (gen_replica가 소비)
"""
import json, os, glob, io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
B    = os.path.join(GAME, "bundle_unpacked_full")
OUT  = os.path.join(os.path.dirname(os.path.abspath(__file__)), "assets_db.json")

def fanim_tags(path):
    try: return sorted(json.load(open(path, encoding="utf-8")).get("anims", {}).keys())
    except Exception: return []

def main():
    db = {"champ_tags": {}, "effect_sheets": {}, "sfx": []}
    # 챔프 시트
    for f in glob.glob(os.path.join(B, "aseprite_resources", "champions", "*#anim.fanim")):
        name = os.path.basename(f).split("#")[0]
        db["champ_tags"][name] = fanim_tags(f)
    # 이펙트 시트 (skill_effect)
    for f in glob.glob(os.path.join(B, "aseprite_resources", "skill_effect", "*#anim.fanim")):
        name = os.path.basename(f).split("#")[0]
        db["effect_sheets"][name] = fanim_tags(f)
    # 사운드 (sound_info basename = Sfx name)
    for f in glob.glob(os.path.join(B, "sound", "sfx", "*.sound_info")):
        db["sfx"].append(os.path.basename(f)[:-len(".sound_info")])
    db["sfx"].sort()
    json.dump(db, open(OUT, "w", encoding="utf-8"), ensure_ascii=False, indent=0)
    print(f"챔프시트 {len(db['champ_tags'])} / 이펙트시트 {len(db['effect_sheets'])} / sfx {len(db['sfx'])} → {OUT}")

if __name__ == "__main__":
    main()
