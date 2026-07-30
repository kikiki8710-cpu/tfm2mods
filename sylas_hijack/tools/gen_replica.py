# -*- coding: utf-8 -*-
"""
gen_replica.py — 바닐라 챔프 궁 → 사일러스 강탈용 data_champion replica 자동 생성기.

입력(전부 정적):
  1) champion_info.champion_info_sheet   : 챔프별 정확한 수치(stat/growth/attack/skill/skill2/ult)
  2) {champ}#anim.fanim                  : 챔프별 애니 태그(비주얼)
필드명→effect 트리 휴리스틱으로 ult effect를 조립하고, 매핑 못한 필드는 리포트.

출력: <STAGE>/champion/{champ}_r.data_champion  +  통합 champion.i18n  +  gen_report.txt
"""
import json, os, glob, io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

GAME  = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
SHEET = os.path.join(GAME, r"bundle_unpacked_full\setting\champion_info.champion_info_sheet")
FANIM = os.path.join(GAME, r"bundle_unpacked_full\aseprite_resources\champions")
STAGE = r"C:\tfm2mods\sylas_hijack\replica_stage"
SPRITE = "asset/base/aseprite_resources/champions/{c}"
EFFECT_SHEET = "asset/base/aseprite_resources/skill_effect/{s}"
SYLAS_ICONS = ["asset/sylas/icons/sylas_skill1","asset/sylas/icons/sylas_skill2","asset/sylas/icons/sylas_ult"]
# 정적 에셋 DB(asset_db.py 산출물): 챔프시트 태그/이펙트시트 태그/sfx(sound_info basename)
ASSET_DB = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "assets_db.json"), encoding="utf-8"))

def num(v, default=0):
    """dict/None/문자 안전 int 변환."""
    if isinstance(v,(int,float)): return int(v)
    if isinstance(v,str):
        try: return int(float(v))
        except Exception: return default
    return default

def load_sheet():
    return json.load(open(SHEET, encoding="utf-8"))

def anim_tags(champ):
    p = os.path.join(FANIM, f"{champ}#anim.fanim")
    if not os.path.exists(p): return set()
    try: return set(json.load(open(p, encoding="utf-8")).get("anims", {}).keys())
    except Exception: return set()

# ---- effect 빌더 (실제 동방 data_champion 관례 준수) ----
def circle(r):      return {"Circle": {"radius": num(r)}}
def time_dur(t):    return {"Time": {"tick": num(t)}}

def dmg_effect(u, ap):
    """주 데미지 effect. ap=True면 ApAttack(주문력계수) 아니면 Attack(공격력계수)."""
    dmg   = u.get("attack", u.get("damage", 0))
    ratio = u.get("attack_ratio", u.get("damage_ratio", u.get("magic_ratio", 0)))
    t = "ApAttack" if ap else "Attack"
    return {"type": t, "damage": num(dmg), "attack_ratio": num(ratio)}

def heal_effect(u):
    return {"type": "Heal", "amount": num(u.get("heal", 0)),
            "ap_ratio": num(u.get("heal_ratio", u.get("heal_ap_ratio", 0))),
            "heal_type": "Ally"}

def shield_effect(u):
    return {"type": "Shield", "amount": num(u.get("shield", u.get("shield_amount", 0))),
            "ap_ratio": num(u.get("shield_ap_ratio", u.get("shield_hp_ratio", 0))),
            "tick": num(u.get("shield_duration", u.get("duration", 120)))}

# CC: 시트 필드명 → 검증된 effect 타입(ALLOWED만). (tick vs duration 구분 실확인)
CC_MAP = [
    ("stun_duration",     lambda u,v: {"type":"Stun","duration":num(v)}),
    ("airborne_time",     lambda u,v: {"type":"Airborne","duration":num(v)}),
    ("airborne_tick",     lambda u,v: {"type":"Airborne","duration":num(v)}),
    ("airborne",          lambda u,v: {"type":"Airborne","duration":num(v) if num(v)>1 else 30}),
    ("fear_duration",     lambda u,v: {"type":"Fear","tick":num(v)}),   # Fear=tick(실확인)
    ("fear_tick",         lambda u,v: {"type":"Fear","tick":num(v)}),
    ("charm_duration",    lambda u,v: {"type":"Charm","tick":num(v)}),  # Charm=tick(실확인)
    ("bind_duration",     lambda u,v: {"type":"Bind","duration":num(v)}),
    ("seal_duration",     lambda u,v: {"type":"BlockSkill","tick":num(v)}),  # 봉인≈스킬차단
    ("block_skill_tick",  lambda u,v: {"type":"BlockSkill","tick":num(v)}),
    ("banish_duration",   lambda u,v: {"type":"Banish","duration":num(v)}),
    # Taunt=검증 화이트리스트에 없음 → omit(taunt_duration은 unmapped). 넉백: knockback_speed + knockback_tick
    ("knockback_speed",   lambda u,v: {"type":"Knockback","speed":num(v),"tick":num(u.get("knockback_tick",8))}),
    ("push_speed",        lambda u,v: {"type":"Knockback","speed":num(v),"tick":num(u.get("push_distance",5000))//max(1,num(v))+1}),
]
CC_CONSUMED = {"stun_duration","airborne_time","airborne_tick","airborne","fear_duration","fear_tick",
    "charm_duration","bind_duration","seal_duration","block_skill_tick","banish_duration",
    "knockback_speed","knockback_tick","push_speed","push_distance"}

def slow_debuff(u):
    # 감속/방깎 등 적 디버프 → AddBuff(적에게). move_speed_mult 음수.
    bs = {"name":"steal_slow","duration": time_dur(u.get("slow_duration", u.get("debuff_duration",120)))}
    if "slow" in u: bs["move_speed_mult"] = -num(u["slow"])
    if "move_speed_reduce" in u: bs["move_speed_mult"] = -num(u["move_speed_reduce"])
    if "defence_reduce" in u: bs["defence_mult"] = -num(u["defence_reduce"])
    if "magic_resistance_reduce" in u: bs["magic_resistance_mult"] = -num(u["magic_resistance_reduce"])
    if "attack_reduce_ratio" in u: bs["attack_mult"] = -num(u["attack_reduce_ratio"])
    return {"type":"AddBuff","buff_state":bs}
SLOW_CONSUMED = {"slow","slow_duration","debuff_duration","move_speed_reduce","defence_reduce",
    "magic_resistance_reduce","attack_reduce_ratio"}

def bleed_dot(u):
    # 도트: bleed(dmg)/bleed_ratio/bleed_duration/bleed_tick → 주기 ApAttack(Delayed 스택).
    dur = num(u.get("bleed_duration", u.get("poison_duration",120)))
    tk  = max(1, num(u.get("bleed_tick", u.get("poison_tick",30))))
    dmg = num(u.get("bleed", 0)); ratio = num(u.get("bleed_ratio",0))
    out = []
    for k in range(1, min(dur//tk, 8)+1):
        out.append({"type":"Delayed","tick":k*tk,"effects":[{"type":"ApAttack","damage":dmg,"attack_ratio":ratio}]})
    return out
BLEED_CONSUMED = {"bleed","bleed_ratio","bleed_duration","bleed_tick","poison_duration","poison_tick"}

# 검증된 buff_state 스탯 키(validate BUFF_KEYS). 시트필드 → buff키(부호). reduce류=음수.
BUFF_KEY = {  # (buff_state 키, 부호)
    "attack_boost":("attack_mult",1),"attack_increase":("attack_mult",1),"attack_mult":("attack_mult",1),
    "attack_speed_boost":("attack_speed_mult",1),"attack_speed_increase":("attack_speed_mult",1),"attack_speed_mult":("attack_speed_mult",1),
    "move_speed":("move_speed_mult",1),"move_speed_increase":("move_speed_mult",1),"move_speed_mult":("move_speed_mult",1),
    "magic_power_boost":("magic_power_mult",1),"magic_power_mult":("magic_power_mult",1),
    "defence":("defence",1),"defence_mult":("defence_mult",1),
    "range_increase":("range",1),"size_ratio":("radius_mult",1),"max_hp_ratio":("hp_mult",1),
    "skill_cooldown_reduce":("skill_cooldown_mult",-1),"damage_reduce":("damaged_reduce",1),
    "damage_reduce_hp_ratio":("damaged_reduce",1),
}
def self_buff(champ, u):
    """buff_duration + 스탯증가류 → AddCasterBuff. 시트필드→검증된 buff_state키만."""
    bs = {"name": f"{champ}_r_buff", "duration": time_dur(u.get("buff_duration", u.get("ult_duration", u.get("duration",180))))}
    for sk,(bk,sign) in BUFF_KEY.items():
        if sk in u: bs[bk] = sign*num(u[sk])
    if u.get("undying"): bs["undying"] = True
    if u.get("cc_immune"): bs["cc_immune"] = True
    return {"type":"AddCasterBuff","buff_state":bs}
BUFF_CONSUMED = set(BUFF_KEY) | {"buff_duration","ult_duration","undying","cc_immune"}

def range_shape(u):
    # 범위: range/attack_range/area_range/splash_range 우선순위
    for k in ("area_range","splash_range","attack_range","range","cast_range","ult_range"):
        if k in u and u[k]: return circle(u[k])
    return circle(120000)

def build_ult_effect(champ, u, tags, category="Range"):
    """필드 존재 기반으로 ult effect(Combine) 조립 + 매핑못한 필드 반환. (검증 vocab 내 최대 정밀)"""
    used = set(["range","cooltime","duration","start_timing","cancelable","speed",
                "attack_range","cast_range","ult_range","area_range","name","width",
                "half_angle_deg","projectile_radius","projectile_range","projectile_width","y_offset"])
    effs = []
    # AD/AP: Magician/Util 또는 magic_ratio 있으면 AP, 그 외 물리(AD).
    ap = (category in ("Magician","Util")) or ("magic_ratio" in u)
    def mk_dmg():
        dmg = num(u.get("attack", u.get("damage", 0)))
        ratio = num(u.get("attack_ratio", u.get("damage_ratio", u.get("magic_ratio", 0))))
        return {"type": "ApAttack" if ap else "Attack", "damage": dmg, "attack_ratio": ratio}
    has_dmg = ("attack" in u and "attack_ratio" in u) or ("damage" in u and ("damage_ratio" in u or "attack_ratio" in u)) or ("attack" in u and "magic_ratio" in u)

    # --- 적 페이로드(데미지 + CC + 도트 + 디버프) ---
    payload = []
    if has_dmg:
        payload.append(mk_dmg())
        for k in ("attack","damage","attack_ratio","damage_ratio","magic_ratio"): used.add(k)
    for fld, mk in CC_MAP:
        if fld in u: payload.append(mk(u, u[fld]))
    used |= (CC_CONSUMED & set(u))
    if any(k in u for k in ("slow","move_speed_reduce","defence_reduce","magic_resistance_reduce","attack_reduce_ratio")):
        payload.append(slow_debuff(u)); used |= (SLOW_CONSUMED & set(u))
    if "bleed" in u:
        payload += bleed_dot(u); used |= (BLEED_CONSUMED & set(u))

    shape = range_shape(u)
    dmg_r = max([num(u.get(k,0)) for k in ("range","cast_range","ult_range","area_range","attack_range")] + [100000])
    dmg_shape = circle(dmg_r)
    total   = u.get("total_shots", u.get("hit_count"))
    term    = u.get("term", u.get("interval", u.get("period")))
    channel = u.get("channel_duration", u.get("sweep_duration"))
    n_hits = 1; per_t = max(1, num(term)) if term else 8

    def periodic(target, eff_list, count):
        nonlocal n_hits
        n_hits = min(max(1, count), 15)
        for k in range(n_hits):
            one = {"type":"RangeEffect","shape":dmg_shape,"target":target,"apply_type":"AroundCaster","effects":eff_list}
            effs.append(one if k == 0 else {"type":"Delayed","tick":k*per_t,"effects":[one]})

    # --- 데미지/CC 전달 ---
    if payload:
        if total and term:                       # 주기 타격(archer/pythoness): total회
            periodic("EnemyChampion", payload, num(total)); used |= {"total_shots","hit_count","term","interval","period"}
        elif channel and term:                    # 채널 주기(werewolf/bard): channel/period회
            periodic("EnemyChampion", payload, num(channel)//per_t); used |= {"channel_duration","sweep_duration","period","term"}
        else:                                     # 단발 즉발 범위
            effs.append({"type":"RangeEffect","shape":dmg_shape,"target":"EnemyChampion","apply_type":"AroundCaster","effects":payload})

    # --- 힐(아군, 주기 힐 존 지원: priest) ---
    if "heal" in u:
        heal_e = heal_effect(u)
        htick = u.get("tick", u.get("heal_duration")); hper = u.get("heal_period", u.get("period"))
        if htick and hper:
            nh = min(max(1, num(htick)//max(1,num(hper))), 12)
            for k in range(nh):
                one = {"type":"RangeEffect","shape":shape,"target":"AllyChampion","apply_type":"AroundCaster","effects":[heal_e]}
                effs.append(one if k == 0 else {"type":"Delayed","tick":k*num(hper),"effects":[one]})
            used |= {"tick","heal_duration","heal_period","heal_speed","heal_delay"}
        else:
            effs.append({"type":"RangeEffect","shape":shape,"target":"AllyChampion","apply_type":"AroundCaster","effects":[heal_e]})
        for k in ("heal","heal_ratio","heal_ap_ratio"): used.add(k)

    # --- 실드(자신/팀) ---
    if "shield" in u or "shield_amount" in u:
        team = num(u.get("range",0)) > 1000
        effs.append({"type":"RangeEffect","shape":shape if team else circle(1),
                     "target":"AllyChampion" if team else "AllyOnlySelf",
                     "apply_type":"AroundCaster","effects":[shield_effect(u)]})
        for k in ("shield","shield_amount","shield_ap_ratio","shield_hp_ratio","shield_duration"): used.add(k)

    # --- 아군 버프 오라(bard/monk 팀버프, 적 디버프/데미지와 독립) ---
    ally_flds = [k for k in ("attack_boost","attack_speed_boost","magic_power_boost","move_speed",
                             "attack_increase","attack_speed_increase","range_increase","size_ratio") if k in u]
    team_buff = bool(ally_flds) and num(u.get("range",0)) > 1000
    if team_buff:
        bs = {"name":f"{champ}_r_teambuff","duration":time_dur(u.get("channel_duration",u.get("buff_duration",u.get("duration",180))))}
        for sk,(bk,sign) in BUFF_KEY.items():
            if sk in u: bs[bk] = sign*num(u[sk])
        effs.append({"type":"RangeEffect","shape":shape,"target":"AllyChampion","apply_type":"AroundCaster",
                     "effects":[{"type":"AddBuff","buff_state":bs}]})
        used |= (BUFF_CONSUMED & set(u)) | {"channel_duration"}

    # --- 순수 자기버프(버프덕만, 위 payload/힐/실드/팀버프 없음) ---
    if (("buff_duration" in u) or any(k in u for k in BUFF_KEY)) and not payload and "heal" not in u and "shield" not in u and not team_buff:
        effs.append(self_buff(champ, u)); used |= (BUFF_CONSUMED & set(u))

    if not effs:  # 완전 미매핑 → 최소 데미지라도(빈 궁 방지)
        effs.append({"type":"RangeEffect","shape":shape,"target":"EnemyChampion",
                     "apply_type":"AroundCaster","effects":[{"type":"ApAttack","damage":100,"attack_ratio":50}]})

    # ═══ v2 정적 비주얼 배선: sfx(60/60) + 본체시트 ult태그 시퀀스 + 전용 이펙트시트 ═══
    db = ASSET_DB
    view_effects = []; view_projectiles = []
    pre = []  # Combine 선두에 붙을 연출들
    dur = num(u.get("duration", 42)); stt = num(u.get("start_timing", 10))

    # ① Sfx: {champ}_ult 정확명 우선, 없으면 최단 {champ}_ult* (sound_info basename)
    sfx_cands = sorted([s for s in db["sfx"] if s.startswith(champ + "_ult")], key=len)
    if sfx_cands:
        main_sfx = champ + "_ult" if (champ + "_ult") in sfx_cands else sfx_cands[0]
        pre.append({"type":"Sfx","name":main_sfx})

    # 연출 스폰: RangeEffect(AllyOnlySelf)→ViewEffect 패턴 = 시전자 CC 맞아도 안 끊김(무녀 토리이 검증)
    def ve_at(tag, name, tick, anim_path, follow=True, z=1):
        view_effects.append({"type":"Animation","name":name,"z":z,
                             "anim":anim_path,"tag":tag,"is_follow":follow})
        spawn = {"type":"RangeEffect","shape":circle(1),"target":"AllyOnlySelf",
                 "apply_type":"AroundCaster","effects":[{"type":"ViewEffect","name":name}]}
        pre.append({"type":"Delayed","tick":num(tick),"effects":[spawn]} if tick > 0 else spawn)

    # ② 본체 시트 ult 태그 시퀀스: pre(0) → effect/on(시전점) → loop 반복 → end(종료 직전)
    spr = SPRITE.format(c=champ)
    if "ult_pre" in tags: ve_at("ult_pre", f"{champ}_r_vpre", 0, spr)
    for t in ("ult_effect","ult_on"):
        if t in tags:
            ve_at(t, f"{champ}_r_von", stt if "ult_pre" in tags else 0, spr); break
    if "ult_loop" in tags:
        step = 18
        n_loop = min(5, max(1, (dur - stt) // step if dur > stt else 1))
        for k in range(int(n_loop)):
            ve_at("ult_loop", f"{champ}_r_vloop{k}", stt + k * step, spr)
    if "ult_end" in tags: ve_at("ult_end", f"{champ}_r_vend", max(0, dur - 6), spr)

    # ③ 전용 이펙트 시트(skill_effect/): ult* 태그가 있는 시트만 채택(idle 등 오재생 방지)
    for sheet_name in (f"{champ}_ult_effect", f"{champ}_effect", f"{champ}_skill_effect"):
        st_tags = db["effect_sheets"].get(sheet_name)
        if not st_tags: continue
        good = [t for t in st_tags if t.startswith("ult")]
        if not good: continue
        ve_at(good[0], f"{champ}_r_sheetfx", stt, EFFECT_SHEET.format(s=sheet_name), z=2)
        break

    # ④ 순수 연출 투사체: speed 있는 궁만. 데미지 미포함(applied_effects=[]) → 안 떠도 게임플레이 무손상.
    #    태그 선택: "fly/spin/projectile/bullet/arrow/shot" 우선(“attack/idle”보다) — gambler=horizontal_spin 등.
    PROJ_KW = ("fly", "spin", "projectile", "bullet", "arrow", "shot", "throw", "laser")
    def pick_proj_tag(tag_list):
        for kw in PROJ_KW:
            for t in tag_list:
                if kw in t: return t
        return None
    def proj_visual():
        # 본체 시트 우선(ult_projectile 등) → 전용 이펙트/투사체 시트
        bt = pick_proj_tag(tags)
        if bt: return (spr, bt)
        for sn in (f"{champ}_ult_projectile", f"{champ}_projectile", f"{champ}_projectile_ani",
                   f"{champ}_dice", f"{champ}_effect"):
            st = db["effect_sheets"].get(sn)
            if st:
                pt = pick_proj_tag(st) or st[0]
                return (EFFECT_SHEET.format(s=sn), pt)
        return None
    if "speed" in u:
        pv = proj_visual()
        if pv:
            pj = f"{champ}_r_proj"
            view_projectiles.append({"type":"Animated","name":pj,"repeat":True,"z":3,
                                     "anim":pv[0],"tag":pv[1]})
            # 타격 횟수만큼(≤6) 연출 발사, 데미지 타이밍과 동기
            nvis = min(n_hits, 6)
            for k in range(nvis):
                shot = {"type":"TargetProjectile","speed":num(u["speed"]),
                        "name":pj,"applied_target":"Enemy","applied_effects":[]}
                pre.append(shot if k == 0 else {"type":"Delayed","tick":k*per_t,"effects":[shot]})
            used.add("speed")

    effs = pre + effs
    unmapped = sorted(set(u.keys()) - used)
    return {"type":"Combine","effects":effs}, view_effects, view_projectiles, unmapped

def action(u, name, default_range, ctype, ctgt, eff):
    # ⚠ 기본공격은 반드시 attack_type="BaseAttack" (동방20종+sylas 전수 실측).
    #   전부 "Skill"로 넣으면 파싱은 되지만 런타임 액션 슬롯이 틀어져 궁 실행이 조용히 죽음(2026-07-03 규명).
    at = "BaseAttack" if name == "attack" else "Skill"
    return {"action_name":name,"duration":num(u.get("duration",20)),
            "cooltime":num(u.get("cooltime",60)),"start_timing":num(u.get("start_timing",10)),
            "cancelable":bool(u.get("cancelable",True)),"range":num(u.get("range",default_range)),
            "casting_type":ctype,"casting_target":ctgt,"attack_type":at,"effect":eff}

def build_champion(champ, c):
    tags_anim = anim_tags(champ)
    ult = c["ult"]
    ult_eff, ve, vp, unmapped = build_ult_effect(champ, ult, tags_anim, c.get("category","Range"))
    # 공/스1/스2 는 강탈본이라 단순 복제(수치만) — 강탈 소스는 궁만 중요
    atk = c.get("attack", {}); sk = c.get("skill", {}); sk2 = c.get("skill2", {})
    doc = {
        "id": f"{champ}_r", "name": f"{champ.title()}_R",
        "category": c.get("category","Range"), "tags": c.get("tags",["AP"]),
        "sprite": SPRITE.format(c=champ), "anim_prefix": "",
        "skill_icons": SYLAS_ICONS,
        "stat": c.get("stat",{}), "growth": c.get("growth",{}),
        "attack": action(atk,"attack",32000,"Targeting","Enemy",
                         {"type":"Attack","damage":num(atk.get("attack",0)),"attack_ratio":num(atk.get("attack_ratio",100))}),
        "skill":  action(sk,"skill",60000,"Targeting","Enemy",
                         {"type":"ApAttack","damage":num(sk.get("attack",sk.get("damage",40))),"attack_ratio":num(sk.get("attack_ratio",60))}),
        "skill2": action(sk2,"skill2",50000,"Targeting","EnemyChampion",
                         {"type":"ApAttack","damage":num(sk2.get("attack",sk2.get("damage",50))),"attack_ratio":num(sk2.get("attack_ratio",60))}),
        "ult": {
            "action_name":"ult",
            "description": f"#asset/base/text/champion?description.{champ}_r.ult",
            "duration": num(ult.get("duration",42)), "cooltime": num(ult.get("cooltime",2400)),
            "start_timing": num(ult.get("start_timing",12)), "cancelable": bool(ult.get("cancelable",False)),
            "range": num(ult.get("range", ult.get("cast_range", ult.get("ult_range", ult.get("attack_range",120000))))),
            "casting_type":"Targeting", "casting_target":"EnemyChampion", "attack_type":"Skill",
            "effect": ult_eff,
        },
    }
    if ve: doc["view_effects"] = ve
    if vp: doc["view_projectiles"] = vp
    return doc, unmapped

def main():
    sheet = load_sheet()
    vanilla = {k:v for k,v in sheet.items() if k!="mod_champions" and isinstance(v,dict) and "ult" in v}
    os.makedirs(os.path.join(STAGE,"champion"), exist_ok=True)
    os.makedirs(os.path.join(STAGE,"text"), exist_ok=True)
    i18n = {"ko":{"description":{}}, "en":{"description":{}}}
    report = []
    only = sys.argv[1:] if len(sys.argv)>1 else None
    for champ, c in sorted(vanilla.items()):
        if only and champ not in only: continue
        doc, unmapped = build_champion(champ, c)
        outp = os.path.join(STAGE,"champion",f"{champ}_r.data_champion")
        with open(outp,"w",encoding="utf-8") as f: json.dump(doc,f,ensure_ascii=False,indent=1)
        nm = f"{champ.title()}(강탈본)"
        i18n["ko"]["description"][f"{champ}_r"] = {"name":nm,"skill":"강탈용 복제.","skill2":"강탈용 복제.","ult":f"{champ} 궁 강탈 복제."}
        i18n["en"]["description"][f"{champ}_r"] = {"name":f"{champ.title()}(Steal)","skill":"replica.","skill2":"replica.","ult":f"{champ} ult steal replica."}
        report.append((champ, unmapped))
    with open(os.path.join(STAGE,"text","champion.i18n"),"w",encoding="utf-8") as f:
        json.dump(i18n,f,ensure_ascii=False,indent=1)
    # 리포트
    lines = [f"생성 {len(report)}챔프. 매핑 못한 필드(=수동/덤프 보강 필요):\n"]
    for champ, un in report:
        flag = "  ⚠ "+", ".join(un) if un else "  ✓ 전필드 매핑"
        lines.append(f"[{champ}]{flag}")
    open(os.path.join(STAGE,"gen_report.txt"),"w",encoding="utf-8").write("\n".join(lines))
    print("\n".join(lines))
    print(f"\n→ {STAGE}\\champion\\*.data_champion  ({len(report)}개)")

if __name__=="__main__":
    main()
