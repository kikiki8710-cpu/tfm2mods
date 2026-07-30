# -*- coding: utf-8 -*-
"""
validate_replicas.py — 생성된 data_champion의 모든 effect 노드를 '실확인 스키마'와 대조.
무효 타입/무효 필드 = 게임 JSON 파서 실패 = 모드 전체 강제 비활성 → 배포 전 필수 게이트.
확인 출처: 워크샵 동방 데이터챔프 20종 + 작동 검증된 pythoness_r 수제본.
"""
import json, glob, os, io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
STAGE = r"C:\tfm2mods\sylas_hijack\replica_stage\champion"

# 타입 → 허용 키(‘type’ 제외). None=자유(미검증이나 관례상 허용). opt는 선택.
ALLOWED = {
    "Combine":               {"effects"},
    "WithSelf":              {"effects"},
    "Delayed":               {"tick","effects"},
    "Sfx":                   {"name"},
    "ViewEffect":            {"name"},
    "CasterViewEffect":      {"name"},
    "CasterAnimation":       {"name","tick"},
    "RemoveCasterAnimation": {"name"},
    "RangeEffect":           {"shape","target","apply_type","effects"},
    "RangePeriodProjectile": {"name","tick","period","first_delay","shape","applied_target","applied_effects","end_effects"},
    "TargetProjectile":      {"speed","name","applied_target","applied_effects","end_effects"},
    "ParabolicProjectile":   {"name","travel_time","range","speed","shape","applied_target","applied_effects","end_effects","range_effect_name"},
    "RangeProjectile":        {"name","length","speed","shape","applied_target","applied_effects","end_effects","range_effect_name"},
    "LineRangeProjectile":   {"name","width","length","delay","apply","applied_target","applied_effects"},
    "TargetSfx":             {"name"},
    "ApAttack":              {"damage","attack_ratio"},
    "Attack":                {"damage","attack_ratio"},
    "FixedAttack":           {"damage","attack_ratio","target_hp_ratio"},
    "Heal":                  {"heal_type","amount","ap_ratio"},
    "Shield":                {"amount","attack_ratio","ap_ratio","tick"},
    "AddBuff":               {"buff_state"},
    "AddCasterBuff":         {"buff_state"},
    "RemoveCasterBuff":      {"name"},
    "Stun":                  {"duration"},
    "Airborne":              {"duration"},
    "Bind":                  {"duration"},
    "Banish":                {"duration","lock_effect_name","end_effect_name"},
    "Fear":                  {"tick"},
    "Charm":                 {"tick"},
    "BlockSkill":            {"tick"},
    "Knockback":             {"speed","tick"},
    "Pull":                  {"speed","tick"},
    "AddCasted":             {"duration","period","casted_type","effects"},
    "Teleport":              set(),
}
BUFF_KEYS = {"attack_mult","attack_speed_mult","base_attack_enemy_max_hp_damage","cc_immune",
    "damage_reflect","damaged_amplify","damaged_reduce","defence","defence_mult","heal_reduce",
    "hp_mult","ignore_wall","magic_power_mult","magic_resistance","magic_resistance_mult",
    "move_speed_mult","radius_mult","range","skill_cooldown_mult","toughness","ult_cooldown_mult",
    "undying","vamp","name","duration"}
HEAL_TYPES = {"Ally","Any","Caster"}
TARGETS = {"Ally","AllyChampion","AllyNotSelf","AllyOnlySelf","Enemy","EnemyChampion",
    "EnemyChampionInCC","EnemyWithoutTower"}

def check_node(e, champ, path, errs):
    if isinstance(e, list):
        for i,v in enumerate(e): check_node(v, champ, f"{path}[{i}]", errs)
        return
    if not isinstance(e, dict): return
    t = e.get("type")
    if t is not None:
        if t not in ALLOWED:
            errs.append(f"[{champ}] {path}: 미확인 타입 '{t}'"); return
        allowed = ALLOWED[t]
        for k in e:
            if k=="type": continue
            if k not in allowed:
                errs.append(f"[{champ}] {path}({t}): 무효 필드 '{k}' (허용={sorted(allowed)})")
        if t=="Heal" and e.get("heal_type") not in HEAL_TYPES:
            errs.append(f"[{champ}] {path}: heal_type '{e.get('heal_type')}' 무효")
        if t in ("AddBuff","AddCasterBuff"):
            bs = e.get("buff_state",{})
            for k in bs:
                if k not in BUFF_KEYS: errs.append(f"[{champ}] {path}: buff_state 무효키 '{k}'")
        if "target" in e and isinstance(e["target"],str) and e["target"] not in TARGETS:
            errs.append(f"[{champ}] {path}: target '{e['target']}' 무효")
        if "applied_target" in e and e["applied_target"] not in TARGETS:
            errs.append(f"[{champ}] {path}: applied_target '{e['applied_target']}' 무효")
    for k,v in e.items():
        if isinstance(v,(dict,list)): check_node(v, champ, f"{path}.{k}", errs)

def check_assets(d, champ, errs):
    """v2: view_effects/view_projectiles의 anim경로+tag, Sfx name 실존 검증."""
    dbp = os.path.join(os.path.dirname(os.path.abspath(__file__)), "assets_db.json")
    db = json.load(open(dbp, encoding="utf-8"))
    def tags_of(anim):
        if anim.startswith("asset/base/aseprite_resources/champions/"):
            return db["champ_tags"].get(anim.rsplit("/",1)[1])
        if anim.startswith("asset/base/aseprite_resources/skill_effect/"):
            return db["effect_sheets"].get(anim.rsplit("/",1)[1])
        return None  # 모드 자체 에셋 등 — 검증 불가(통과)
    names = set()
    for sec in ("view_effects","view_projectiles"):
        for v in d.get(sec, []):
            names.add(v.get("name"))
            t = tags_of(v.get("anim",""))
            if t is not None and v.get("tag") not in t:
                errs.append(f"[{champ}] {sec} '{v.get('name')}': tag '{v.get('tag')}' 없음 in {v.get('anim')}")
            if t is None and v.get("anim","").startswith("asset/base/"):
                errs.append(f"[{champ}] {sec} '{v.get('name')}': anim 경로 미존재 {v.get('anim')}")
    # ViewEffect/투사체 name 참조 무결성 + Sfx 실존
    def walk(e):
        if isinstance(e, dict):
            if e.get("type")=="ViewEffect" and e.get("name") not in names:
                errs.append(f"[{champ}] ViewEffect '{e.get('name')}' → view_effects에 정의 없음")
            if e.get("type")=="Sfx" and e.get("name") not in db["sfx"]:
                # 모드 자체 sfx 가능성 있으나 replica는 base만 씀 → 오류 처리
                errs.append(f"[{champ}] Sfx '{e.get('name')}' → base sound_info에 없음")
            if e.get("type") in ("TargetProjectile","RangePeriodProjectile","LineRangeProjectile") \
               and e.get("name") and e["name"] not in names:
                pass  # 투사체 name은 view 미정의여도 게임이 무시(비주얼만 없음) — 오류 아님
            for v in e.values(): walk(v)
        elif isinstance(e, list):
            for v in e: walk(v)
    for act in ("attack","skill","skill2","ult"):
        if act in d: walk(d[act].get("effect",{}))

def main():
    errs=[]; n=0
    for f in sorted(glob.glob(os.path.join(STAGE,"*.data_champion"))):
        champ=os.path.basename(f).replace(".data_champion","")
        d=json.load(open(f,encoding="utf-8")); n+=1
        for act in ("attack","skill","skill2","ult"):
            if act in d: check_node(d[act].get("effect",{}), champ, act, errs)
        check_assets(d, champ, errs)
    print(f"검증 {n}챔프.")
    if errs:
        print(f"❌ 위반 {len(errs)}건 — 배포 금지(모드 비활성 위험):")
        for e in errs: print("  "+e)
        sys.exit(1)
    print("✅ 전 replica 확정스키마+에셋실존 통과 — 배포 안전.")

if __name__=="__main__":
    main()
