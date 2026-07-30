import gzip
import argparse
import configparser
import json
import math
import os
import re
import shutil
import struct
from collections import Counter, defaultdict
from datetime import datetime, timedelta
from pathlib import Path


def detect_game_root():
    env_root = os.environ.get("TFM2_GAME_ROOT")
    if env_root:
        candidate = Path(env_root).resolve()
        if (candidate / "TeamfightManager2.exe").exists() and (candidate / "config" / "game").exists():
            return candidate

    current = Path(__file__).resolve()
    for parent in [current.parent, *current.parents]:
        if (parent / "TeamfightManager2.exe").exists() and (parent / "config" / "game").exists():
            return parent.resolve()

    for parent in [current.parent, *current.parents]:
        if parent.name.lower() == "steamapps":
            candidate = parent / "common" / "Teamfight Manager2"
            if (candidate / "config" / "game").exists():
                return candidate.resolve()

    default_root = Path(os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)")) / "Steam" / "steamapps" / "common" / "Teamfight Manager2"
    if (default_root / "config" / "game").exists():
        return default_root.resolve()

    return Path(__file__).resolve().parents[2].resolve()


def detect_workshop_content_dir():
    env_dir = os.environ.get("TFM2_WORKSHOP_CONTENT_DIR")
    if env_dir:
        return Path(env_dir).resolve()
    return ROOT.parents[1] / "workshop" / "content" / "3009300"


ROOT = detect_game_root()
DASHBOARD = Path(__file__).resolve().parents[1]
OUT = DASHBOARD / "data" / "meta-data.js"
CORE_ITEM_BUILDS_OUT = DASHBOARD / "data" / "core-item-builds.json"
CORE_ITEM_BUILDS_MOD_OUT = ROOT / "mods" / "tfm2_meta_item_delegate" / "core-item-builds.json"
CORE_ITEM_BUILDS_MOD_DATA_OUT = ROOT / "mods" / "tfm2_meta_item_delegate" / "data" / "core-item-builds.json"
AI_CHAMPION_POLICY_OUT = DASHBOARD / "data" / "ai_champion_policy.tsv"
AI_CHAMPION_POLICY_MOD_OUT = ROOT / "mods" / "tfm2_ai_banpick_probe" / "ai_champion_policy.tsv"
CHAMPION_TIER_POLICY_MOD_OUT = ROOT / "mods" / "tfm2_meta_champion_tiers" / "champion_tier_policy.tsv"
BANPICK_DATA = DASHBOARD / "data" / "banpick-data.js"
SAVE_PROBE_SNAPSHOT_DIR = DASHBOARD / "data" / "save_probe_snapshot"
META_CHUNKS_DIR = DASHBOARD / "data" / "meta-chunks"
ITEM_SETTING_PATHS = [
    DASHBOARD / "data" / "item_setting.item_setting",
    ROOT / "mods" / "base_unpacked" / "setting" / "item_setting.item_setting",
    ROOT / "_modding_downloads" / "base_current" / "setting" / "item_setting.item_setting",
]
ITEM_I18N_PATHS = [
    DASHBOARD / "data" / "item.i18n",
    ROOT / "mods" / "base_unpacked" / "text" / "item.i18n",
    ROOT / "_modding_downloads" / "base_current" / "text" / "item.i18n",
]
CHAMPION_I18N_PATHS = [
    DASHBOARD / "data" / "champion.i18n",
    ROOT / "mods" / "base_unpacked" / "text" / "champion.i18n",
    ROOT / "_modding_downloads" / "base_current" / "text" / "champion.i18n",
]
ITEM_ICON_DIR = DASHBOARD / "assets" / "items"
MOD_CHAMPION_ASSET_DIR = DASHBOARD / "assets" / "mod-champions"
MOD_SKILL_ASSET_DIR = DASHBOARD / "assets" / "mod-skills"
GAME_MODS_CONFIG = ROOT / "config" / "game" / "mods.json"
WORKSHOP_CONTENT_DIR = detect_workshop_content_dir()
DASHBOARD_DETAILED_RECENT_DAYS = int(os.environ.get("TFM2_DASHBOARD_DETAILED_RECENT_DAYS", "183"))
DASHBOARD_DETAILED_PATCH_LIMIT = int(os.environ.get("TFM2_DASHBOARD_DETAILED_PATCH_LIMIT", "3"))
POLICY_TARGET_CONFIG_SECTIONS = ("policy_targets", "paths")
POLICY_TARGET_ALIASES = {
    "ai_champion_policy": ("ai_champion_policy", "ai_champion_policy_tsv"),
    "champion_tier_policy": ("champion_tier_policy", "champion_tier_policy_tsv"),
    "core_item_builds": ("core_item_builds", "core_item_builds_json"),
    "core_item_builds_data": ("core_item_builds_data", "core_item_builds_data_json"),
}
_POLICY_TARGET_CACHE = None

DATE_VERSION_RE = re.compile(r"^\d{4}\.\d+\.\d+$")
POSITION_NAMES = ["top", "jungle", "mid", "bot", "support"]
POSITION_FIELD_NAMES = {"top": "top", "jungle": "jungle", "mid": "mid", "bot": "bottom", "support": "support"}
BUILD_DIRECTIONS = {"AD", "Magic", "AttackSpeed", "Defense", "MagicResistance", "Hp", "Auto"}
LEAGUE_REGIONS = [
    ("kr", "한국"),
    ("cn", "중국"),
    ("eu", "유럽"),
    ("na", "북미"),
    ("sa", "남미"),
    ("jp", "일본"),
]
LEAGUE_DIVISIONS = [("div1", "1부"), ("div2", "2부")]
LEAGUE_KEY_META = {
    "tack": ("kr", "div1"),
    "tacc": ("cn", "div1"),
    "tace": ("eu", "div1"),
    "taca": ("na", "div1"),
    "tacs": ("sa", "div1"),
    "tacj": ("jp", "div1"),
    "tack2": ("kr", "div2"),
    "tacc2": ("cn", "div2"),
    "tace2": ("eu", "div2"),
    "taca2": ("na", "div2"),
    "tacs2": ("sa", "div2"),
    "tacj2": ("jp", "div2"),
}
LEAGUE_ID_KEYS = ["tack", "tacc", "tace", "taca", "tacs", "tacj", "tack2", "tacc2", "tace2", "taca2", "tacs2", "tacj2"]
REGION_LABELS = dict(LEAGUE_REGIONS)
DIVISION_LABELS = dict(LEAGUE_DIVISIONS)
SOLO_REGION_KEYS = {index: key for index, (key, _label) in enumerate(LEAGUE_REGIONS)}


def version_sort_key(version):
    parts = re.findall(r"\d+", str(version))
    return tuple(int(part) for part in parts) if parts else (0,)


def recent_detail_patches(patch_versions, matches=None, game_day=None):
    if not patch_versions:
        return []
    recent_days = max(0, DASHBOARD_DETAILED_RECENT_DAYS)
    if recent_days and game_day:
        try:
            threshold = datetime.fromisoformat(str(game_day)).date() - timedelta(days=recent_days)
            version_latest_day = {}
            for match in matches or []:
                version = match.get("version")
                day = str(match.get("dateKey") or "")
                if version not in patch_versions or not re.match(r"^\d{4}-\d{2}-\d{2}$", day):
                    continue
                if day > version_latest_day.get(version, ""):
                    version_latest_day[version] = day
            recent_versions = [
                version
                for version in patch_versions
                if version_latest_day.get(version) and datetime.fromisoformat(version_latest_day[version]).date() >= threshold
            ]
            if recent_versions:
                return recent_versions
        except ValueError:
            pass
    limit = max(0, DASHBOARD_DETAILED_PATCH_LIMIT)
    return list(patch_versions[-limit:]) if limit else []


def prune_patch_map(mapping, keep_versions):
    keep = set(keep_versions or [])
    if not keep:
        return {}
    return {version: rows for version, rows in mapping.items() if version in keep}


def patch_chunk_file_name(version):
    safe = re.sub(r"[^0-9A-Za-z_.-]+", "_", str(version)).strip("._") or "unknown"
    return f"patch-{safe}.js"


def match_chunk_file_name(version):
    safe = re.sub(r"[^0-9A-Za-z_.-]+", "_", str(version)).strip("._") or "unknown"
    return f"matches-{safe}.js"


def write_patch_detail_chunks(chunks):
    META_CHUNKS_DIR.mkdir(parents=True, exist_ok=True)
    for old in META_CHUNKS_DIR.glob("patch-*.js"):
        old.unlink()
    written = {}
    for version, chunk in chunks.items():
        file_name = patch_chunk_file_name(version)
        path = META_CHUNKS_DIR / file_name
        script = (
            "window.TFM2_META_PATCH_CHUNKS=window.TFM2_META_PATCH_CHUNKS||{};"
            f"window.TFM2_META_PATCH_CHUNKS[{json.dumps(str(version), ensure_ascii=False)}]="
            f"{json.dumps(chunk, ensure_ascii=False, separators=(',', ':'))};\n"
        )
        path.write_text(script, encoding="utf-8")
        written[version] = {
            "file": f"data/meta-chunks/{file_name}",
            "bytes": path.stat().st_size,
        }
    return written


def write_match_analysis_chunks(chunks):
    META_CHUNKS_DIR.mkdir(parents=True, exist_ok=True)
    for old in META_CHUNKS_DIR.glob("matches-*.js"):
        old.unlink()
    written = {}
    for version, rows in chunks.items():
        if not rows:
            continue
        file_name = match_chunk_file_name(version)
        path = META_CHUNKS_DIR / file_name
        script = (
            "window.TFM2_META_MATCH_CHUNKS=window.TFM2_META_MATCH_CHUNKS||{};"
            f"window.TFM2_META_MATCH_CHUNKS[{json.dumps(str(version), ensure_ascii=False)}]="
            f"{json.dumps(rows, ensure_ascii=False, separators=(',', ':'))};\n"
        )
        path.write_text(script, encoding="utf-8")
        written[version] = {
            "file": f"data/meta-chunks/{file_name}",
            "bytes": path.stat().st_size,
            "rows": len(rows),
        }
    return written


def match_analysis_row_key(row):
    return f"{row.get('source') or 'match'}:{row.get('id')}"


def load_js_json(path: Path):
    text = path.read_text(encoding="utf-8")
    raw = text[text.find("=") + 1 :].strip()
    if raw.endswith(";"):
        raw = raw[:-1]
    return json.loads(raw)


def load_json_file(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None


def safe_file_stem(value):
    return re.sub(r"[^0-9A-Za-z_.-]+", "_", str(value)).strip("._") or "asset"


def png_dimensions(path: Path):
    try:
        with path.open("rb") as handle:
            header = handle.read(24)
    except OSError:
        return None
    if len(header) < 24 or header[:8] != b"\x89PNG\r\n\x1a\n":
        return None
    width = int.from_bytes(header[16:20], "big")
    height = int.from_bytes(header[20:24], "big")
    return width, height


def clean_game_text(value):
    text = str(value or "")
    text = re.sub(r"<i#[^>]+>", "", text)
    text = re.sub(r"<#[^>]+>", "", text)
    text = text.replace("<>", "")
    return re.sub(r"\s+", " ", text).strip()


def mod_id_from_info(mod_root: Path):
    info = load_json_file(mod_root / "mod.mod_info") or {}
    return info.get("mod_id") or info.get("id") or mod_root.name


def configured_mod_ids():
    config = load_json_file(GAME_MODS_CONFIG) or {}
    ids = set()
    for key in ("enabled_mods", "known_workshop_mods"):
        for mod_id in config.get(key) or []:
            if mod_id:
                ids.add(str(mod_id))
    return ids


def discover_mod_roots():
    configured = configured_mod_ids()
    roots = []
    seen = set()

    def maybe_add(path: Path):
        if not path.exists() or not path.is_dir():
            return
        mod_id = mod_id_from_info(path)
        if configured and mod_id not in configured and path.name not in configured:
            return
        if not any(path.rglob("*.data_champion")):
            return
        key = str(path.resolve())
        if key in seen:
            return
        seen.add(key)
        roots.append((path, mod_id))

    mods_dir = ROOT / "mods"
    if mods_dir.exists():
        for path in mods_dir.iterdir():
            if path.is_dir():
                maybe_add(path)
    if WORKSHOP_CONTENT_DIR.exists():
        for path in WORKSHOP_CONTENT_DIR.iterdir():
            if path.is_dir():
                maybe_add(path)
    return roots


def workshop_mod_policy_paths(mod_id, relative_path):
    targets = []
    if not WORKSHOP_CONTENT_DIR.exists():
        return targets
    rel = Path(relative_path)
    for path in WORKSHOP_CONTENT_DIR.iterdir():
        if not path.is_dir():
            continue
        if mod_id_from_info(path) == mod_id:
            targets.append(path / rel)
    return targets


def dashboard_config_paths():
    paths = []
    env_path = os.environ.get("TFM2_META_DASHBOARD_CONFIG")
    if env_path:
        paths.append(Path(env_path))
    appdata = os.environ.get("APPDATA")
    if appdata:
        paths.append(Path(appdata) / "TeamSamoyed" / "TeamfightManager2" / "tfm2_meta_dashboard_config.ini")
        paths.append(Path(appdata) / "TeamSamoyed" / "Teamfight Manager2" / "tfm2_meta_dashboard_config.ini")
    paths.append(DASHBOARD / "config.ini")

    out = []
    seen = set()
    for path in paths:
        try:
            resolved = Path(path).expanduser().resolve()
        except OSError:
            continue
        key = str(resolved).lower()
        if key in seen:
            continue
        seen.add(key)
        out.append(resolved)
    return out


def split_config_path_value(value):
    values = []
    for raw in re.split(r"[\n;]+", str(value or "")):
        text = raw.strip().strip('"').strip("'")
        if text:
            values.append(text)
    return values


def load_policy_target_config():
    targets = defaultdict(list)
    for config_path in dashboard_config_paths():
        if not config_path.exists():
            continue
        parser = configparser.ConfigParser(interpolation=None)
        try:
            parser.read(config_path, encoding="utf-8-sig")
        except configparser.Error as exc:
            print(f"WARNING: skipped invalid dashboard config {config_path}: {exc}")
            continue
        for section in POLICY_TARGET_CONFIG_SECTIONS:
            if not parser.has_section(section):
                continue
            for target_key, aliases in POLICY_TARGET_ALIASES.items():
                for alias in aliases:
                    if not parser.has_option(section, alias):
                        continue
                    for path_text in split_config_path_value(parser.get(section, alias)):
                        expanded = os.path.expandvars(path_text)
                        path = Path(expanded).expanduser()
                        if not path.is_absolute():
                            path = DASHBOARD / path
                        targets[target_key].append(path.resolve())
    return dict(targets)


def configured_policy_target_paths(target_key):
    global _POLICY_TARGET_CACHE
    if _POLICY_TARGET_CACHE is None:
        _POLICY_TARGET_CACHE = load_policy_target_config()
    return list(_POLICY_TARGET_CACHE.get(target_key, []))


def active_dashboard_config_paths():
    return [path for path in dashboard_config_paths() if path.exists()]


def load_mod_champion_texts(mod_root: Path):
    texts = {}
    for path in mod_root.rglob("*.i18n"):
        raw = load_json_file(path)
        if not isinstance(raw, dict):
            continue
        for lang in ("en", "ko"):
            descriptions = ((raw.get(lang) or {}).get("description") or {})
            if not isinstance(descriptions, dict):
                continue
            for champion_id, values in descriptions.items():
                if isinstance(values, dict):
                    texts.setdefault(champion_id, {}).setdefault(lang, {}).update(values)
    return texts


def text_from_champion_ref(ref, champion_id, field, texts, lang="ko"):
    text_key = None
    ref = str(ref or "")
    if "?description." in ref:
        text_key = ref.split("?description.", 1)[1]
    if text_key and "." in text_key:
        prefix, suffix = text_key.rsplit(".", 1)
        row = texts.get(prefix) or {}
        value = (row.get(lang) or {}).get(suffix) if isinstance(row.get(lang), dict) else row.get(suffix)
        if value:
            return clean_game_text(value)
    row = texts.get(champion_id) or {}
    value = (row.get(lang) or {}).get(field) if isinstance(row.get(lang), dict) else row.get(field)
    return clean_game_text(value or "")


def load_champion_translations():
    path = first_existing(CHAMPION_I18N_PATHS)
    if not path:
        return {}
    raw = load_json_file(path)
    if not isinstance(raw, dict):
        return {}
    translations = {}
    for lang in ("ko", "en"):
        descriptions = ((raw.get(lang) or {}).get("description") or {})
        if not isinstance(descriptions, dict):
            continue
        for champion_id, values in descriptions.items():
            if isinstance(values, dict):
                cleaned = {}
                for key, value in values.items():
                    text = clean_game_text(value)
                    if text:
                        cleaned[key] = text
                if cleaned:
                    translations.setdefault(champion_id, {})[lang] = cleaned
    return translations


def apply_champion_translations(champions, translations):
    if not translations:
        return 0
    updated = 0
    skill_id_map = {"attack": "attack", "skill": "skill", "skill2": "skill2", "ult": "ult"}
    for champ in champions:
        champion_id = champ.get("id")
        rows = translations.get(champion_id) or {}
        if not rows:
            continue
        ko = rows.get("ko") or {}
        en = rows.get("en") or {}
        if ko.get("name") and not champ.get("name"):
            champ["name"] = ko["name"]
        if en.get("name"):
            champ["nameEn"] = en["name"]
        description = champ.setdefault("description", {})
        description_en = champ.setdefault("descriptionEn", {})
        for field in skill_id_map:
            if ko.get(field):
                description[field] = ko[field]
            if en.get(field):
                description_en[field] = en[field]
        skill_by_id = {skill.get("id"): skill for skill in champ.get("skills", [])}
        for field, skill_id in skill_id_map.items():
            skill = skill_by_id.get(skill_id)
            if not skill:
                continue
            if ko.get(field):
                skill["description"] = ko[field]
            if en.get(field):
                skill["descriptionEn"] = en[field]
        updated += 1
    return updated


def number_value(row, *keys, default=0):
    if not isinstance(row, dict):
        return default
    for key in keys:
        value = row.get(key)
        if isinstance(value, (int, float)):
            return value
    return default


def mod_asset_base_path(asset_ref, mod_root: Path, mod_id):
    ref = str(asset_ref or "")
    prefix = f"asset/{mod_id}/"
    if ref.startswith(prefix):
        return mod_root / ref[len(prefix) :]
    parts = ref.split("/")
    if len(parts) >= 3 and parts[0] == "asset":
        return mod_root / "/".join(parts[2:])
    return None


def first_anim_frame(sprite_base: Path):
    if not sprite_base:
        return None
    fanim = Path(str(sprite_base) + "#anim.fanim")
    raw = load_json_file(fanim)
    anims = (raw or {}).get("anims") or {}
    candidates = []
    if isinstance(anims, dict):
        candidates.extend(anims.get("idle", {}).get("frames") or [])
        for anim in anims.values():
            candidates.extend(anim.get("frames") or [])
    for frame in candidates:
        data = frame.get("data") if isinstance(frame, dict) else None
        if isinstance(data, dict) and all(key in data for key in ("x", "y", "w", "h")):
            return {key: int(data.get(key) or 0) for key in ("x", "y", "w", "h")}
    return None


def copy_mod_champion_sprite(champion_id, sprite_ref, mod_root: Path, mod_id):
    sprite_base = mod_asset_base_path(sprite_ref, mod_root, mod_id)
    if not sprite_base:
        return None
    sheet = Path(str(sprite_base) + "#sheet.png")
    size = png_dimensions(sheet)
    if not size:
        return None
    MOD_CHAMPION_ASSET_DIR.mkdir(parents=True, exist_ok=True)
    target = MOD_CHAMPION_ASSET_DIR / f"{safe_file_stem(champion_id)}.png"
    try:
        if not target.exists() or target.stat().st_size != sheet.stat().st_size:
            shutil.copy2(sheet, target)
    except OSError:
        return None
    frame = first_anim_frame(sprite_base)
    if not frame:
        frame = {"x": 0, "y": 0, "w": min(size[0], 64), "h": min(size[1], 64)}
    return {
        "sheet": f"assets/mod-champions/{target.name}",
        "sheetWidth": size[0],
        "sheetHeight": size[1],
        "frame": frame,
    }


def mod_asset_image_path(asset_ref, mod_root: Path, mod_id):
    asset_base = mod_asset_base_path(asset_ref, mod_root, mod_id)
    if not asset_base:
        return None
    candidates = []
    if asset_base.suffix:
        candidates.append(asset_base)
    candidates.append(Path(str(asset_base) + ".png"))
    candidates.append(Path(str(asset_base) + "#sheet.png"))
    seen = set()
    for candidate in candidates:
        key = str(candidate).lower()
        if key in seen:
            continue
        seen.add(key)
        if png_dimensions(candidate):
            return candidate
    return None


def copy_mod_skill_icon(champion_id, field, icon_ref, mod_root: Path, mod_id):
    icon_path = mod_asset_image_path(icon_ref, mod_root, mod_id)
    if not icon_path:
        return None
    MOD_SKILL_ASSET_DIR.mkdir(parents=True, exist_ok=True)
    target = MOD_SKILL_ASSET_DIR / f"{safe_file_stem(champion_id)}_{safe_file_stem(field)}.png"
    try:
        if not target.exists() or target.stat().st_size != icon_path.stat().st_size:
            shutil.copy2(icon_path, target)
    except OSError:
        return None
    return f"assets/mod-skills/{target.name}"


def normalize_mod_stats(stat, attack_action=None):
    attack_cooltime = number_value(attack_action, "cooltime", default=60) or 60
    move_speed = number_value(stat, "move_speed", "moveSpeed", default=1000)
    return {
        "attack": number_value(stat, "attack"),
        "magicPower": number_value(stat, "magic_power", "magicPower"),
        "hp": number_value(stat, "hp"),
        "defence": number_value(stat, "defence", "defense"),
        "magicResistance": number_value(stat, "magic_resistance", "magicResistance"),
        "moveSpeed": move_speed,
        "moveSpeedDisplay": round(move_speed * 0.06, 2),
        "attackSpeed": round(60 / max(1, attack_cooltime), 2),
        "range": round(number_value(attack_action, "range", default=0) / 2000, 2),
    }


def normalize_mod_growth(growth):
    return {
        "attack": number_value(growth, "attack"),
        "magicPower": number_value(growth, "magic_power", "magicPower"),
        "hp": number_value(growth, "hp"),
        "defence": number_value(growth, "defence", "defense"),
        "magicResistance": number_value(growth, "magic_resistance", "magicResistance"),
        "moveSpeedDisplay": round(number_value(growth, "move_speed", "moveSpeed") * 0.06, 2),
        "attackSpeed": 0,
        "range": 0,
    }


def mod_action_cooltime(action):
    cooltime = number_value(action, "cooltime", default=0)
    return f"{cooltime / 60:.2f}" if cooltime else None


def infer_mod_role_fit(category, tags):
    tags = set(tags or [])
    category = category or ""
    role_fit = {"top": 30, "jungle": 30, "mid": 30, "bot": 30, "support": 30}
    if "Tank" in tags or category in {"Tank", "Melee"}:
        role_fit.update({"top": 75, "jungle": 55, "support": 45})
    if "AP" in tags or "Magic" in tags or category in {"Magician", "Mage"}:
        role_fit.update({"mid": max(role_fit["mid"], 85), "support": max(role_fit["support"], 45)})
    if "AD" in tags or "Range" in tags or category in {"Ranged", "Marksman"}:
        role_fit.update({"bot": max(role_fit["bot"], 85), "mid": max(role_fit["mid"], 55)})
    if "Assassin" in tags or "Mobility" in tags:
        role_fit.update({"jungle": max(role_fit["jungle"], 75), "mid": max(role_fit["mid"], 65)})
    if "Heal" in tags or "Shield" in tags or "Util" in tags or category in {"Util", "Support"}:
        role_fit.update({"support": max(role_fit["support"], 90), "top": max(role_fit["top"], 45)})
    return role_fit


def infer_mod_metrics(stats, tags):
    tags = set(tags or [])
    damage = stats.get("attack", 0) + stats.get("magicPower", 0) * 1.4
    durability = stats.get("hp", 0) / 10 + stats.get("defence", 0) * 2 + stats.get("magicResistance", 0) * 2
    utility = 30
    if "CC" in tags:
        utility += 25
    if "Heal" in tags:
        utility += 20
    if "Shield" in tags:
        utility += 15
    if "Mobility" in tags:
        utility += 10
    return {
        "damage": round(damage, 1),
        "durability": round(durability, 1),
        "utility": round(utility, 1),
        "scaling": round(stats.get("attack", 0) + stats.get("magicPower", 0), 1),
        "mobility": 40.0 if "Mobility" in tags else 20.0,
        "cc": 50.0 if "CC" in tags else 0.0,
        "heal": 50.0 if "Heal" in tags else 0.0,
        "shield": 50.0 if "Shield" in tags else 0.0,
        "damageNorm": 50.0,
        "durabilityNorm": 50.0,
        "utilityNorm": 50.0,
        "scalingNorm": 50.0,
        "mobilityNorm": 50.0,
    }


def build_mod_champion(champion_path: Path, mod_root: Path, mod_id, texts):
    raw = load_json_file(champion_path)
    if not isinstance(raw, dict) or not raw.get("id"):
        return None
    champion_id = raw["id"]
    text_rows = texts.get(champion_id) or {}
    text_row = text_rows.get("ko") or {}
    text_row_en = text_rows.get("en") or {}
    raw_tags = [str(tag) for tag in raw.get("tags") or []]
    category = str(raw.get("category") or "Custom")
    tags = sorted({*raw_tags, category})
    stats = normalize_mod_stats(raw.get("stat") or {}, raw.get("attack") or {})
    growth = normalize_mod_growth(raw.get("growth") or {})
    role_fit = infer_mod_role_fit(category, tags)
    best_role = max(role_fit, key=role_fit.get)
    descriptions = {
        field: text_from_champion_ref((raw.get(field) or {}).get("description"), champion_id, field, texts, "ko")
        for field in ("attack", "skill", "skill2", "ult")
    }
    descriptions_en = {
        field: text_from_champion_ref((raw.get(field) or {}).get("description"), champion_id, field, texts, "en")
        for field in ("attack", "skill", "skill2", "ult")
    }
    skills = []
    skill_icon_refs = raw.get("skill_icons") or []
    for index, (field, level) in enumerate((("skill", 1), ("skill2", 3), ("ult", 5))):
        action = raw.get(field) or {}
        icon_asset = copy_mod_skill_icon(
            champion_id,
            field,
            skill_icon_refs[index] if index < len(skill_icon_refs) else None,
            mod_root,
            mod_id,
        )
        skills.append(
            {
                "id": field,
                "level": level,
                "iconKey": None,
                "iconAsset": icon_asset,
                "cooltime": mod_action_cooltime(action),
                "description": descriptions.get(field) or field,
                "descriptionEn": descriptions_en.get(field) or descriptions.get(field) or field,
            }
        )
    name = clean_game_text(text_row.get("name")) or clean_game_text(text_row_en.get("name")) or champion_id.replace("_", " ").title()
    name_en = clean_game_text(text_row_en.get("name")) or name
    return {
        "id": champion_id,
        "name": name,
        "nameEn": name_en,
        "category": category,
        "tags": tags,
        "rawTags": raw_tags,
        "description": descriptions,
        "descriptionEn": descriptions_en,
        "stats": stats,
        "growth": growth,
        "skills": skills,
        "metrics": infer_mod_metrics(stats, tags),
        "roleFit": role_fit,
        "bestRole": best_role,
        "asset": copy_mod_champion_sprite(champion_id, raw.get("sprite"), mod_root, mod_id),
        "overall": 0,
        "tier": "-",
        "customChampion": True,
        "customSource": {
            "modId": mod_id,
            "path": str(champion_path),
        },
    }


def load_mod_champions(existing_ids):
    champions = []
    seen = set(existing_ids or [])
    roots = discover_mod_roots()
    for mod_root, mod_id in roots:
        texts = load_mod_champion_texts(mod_root)
        for champion_path in sorted(mod_root.rglob("*.data_champion")):
            champion = build_mod_champion(champion_path, mod_root, mod_id, texts)
            if not champion or champion["id"] in seen:
                continue
            seen.add(champion["id"])
            champions.append(champion)
    return champions, roots


def first_existing(paths):
    for path in paths:
        if path.exists():
            return path
    return None


def load_item_translations():
    path = first_existing(ITEM_I18N_PATHS)
    if not path:
        return {"ko": {}, "en": {}}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {"ko": {}, "en": {}}

    names = {"ko": {}, "en": {}}
    for lang in ["ko", "en"]:
        section = data.get(lang, {})
        for key, value in section.items():
            if isinstance(value, dict) and value.get("name") and key not in names[lang]:
                names[lang][key] = clean_game_text(value["name"])
    return names


def item_icon_asset_path(icon):
    if not icon:
        return None
    path = ITEM_ICON_DIR / f"{icon}.png"
    return f"assets/items/{icon}.png" if path.exists() else None


def load_item_catalog():
    path = first_existing(ITEM_SETTING_PATHS)
    if not path:
        return {"source": None, "byId": {}, "byIcon": {}}
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {"source": str(path), "byId": {}, "byIcon": {}}

    names = load_item_translations()
    ko_names = names.get("ko", {})
    en_names = names.get("en", {})
    by_id = {}
    by_icon = {}
    item_index = 0
    for setting_id, row in raw.items():
        if setting_id == "mod_items" or not isinstance(row, dict):
            continue
        key = row.get("key") or setting_id
        icon = row.get("icon") or ""
        fallback_name = str(key).replace("_", " ").title()
        name = ko_names.get(key) or ko_names.get(setting_id) or en_names.get(key) or en_names.get(setting_id) or fallback_name
        name_en = en_names.get(key) or en_names.get(setting_id) or name
        direction = normalize_build_direction(row.get("category"))
        item = {
            "id": item_index,
            "settingId": setting_id,
            "key": key,
            "icon": icon,
            "iconPath": item_icon_asset_path(icon),
            "name": name,
            "nameEn": name_en,
            "tier": row.get("tier"),
            "category": row.get("category"),
            "direction": direction,
        }
        by_id[str(item_index)] = item
        if icon:
            by_icon[icon] = item
        item_index += 1
    return {"source": str(path), "byId": by_id, "byIcon": by_icon}


def normalize_build_direction(category):
    value = str(category or "").strip()
    if value in BUILD_DIRECTIONS:
        return value
    aliases = {
        "Defence": "Defense",
        "defense": "Defense",
        "defence": "Defense",
        "MR": "MagicResistance",
        "MagicResist": "MagicResistance",
        "HP": "Hp",
        "Health": "Hp",
        "AS": "AttackSpeed",
        "Attack_Speed": "AttackSpeed",
    }
    return aliases.get(value, "Auto")


def compact_item(item):
    if not item:
        return None
    return {
        key: item.get(key)
        for key in ["id", "key", "icon", "iconPath", "name", "nameEn", "category", "direction", "unknown"]
        if item.get(key) is not None
    }


def fallback_item_from_id(item_id):
    try:
        number = int(item_id)
    except (TypeError, ValueError):
        return None
    if number < 0:
        return {"id": number, "key": str(number), "name": f"미확인 아이템 #{number}", "nameEn": f"Unknown Item #{number}", "direction": "Auto", "unknown": True}
    return {"id": number, "key": f"item:{number}", "name": f"미확인 아이템 #{number}", "nameEn": f"Unknown Item #{number}", "direction": "Auto", "unknown": True}


def describe_item_ids(item_ids, item_catalog):
    by_id = (item_catalog or {}).get("byId", {})
    out = []
    for item_id in item_ids or []:
        item = compact_item(by_id.get(str(item_id))) or fallback_item_from_id(item_id)
        if item:
            out.append(item)
    return out


def describe_item_icons(icon_keys, item_catalog):
    by_icon = (item_catalog or {}).get("byIcon", {})
    out = []
    for icon in icon_keys or []:
        item = compact_item(by_icon.get(str(icon)))
        if item:
            out.append(item)
        elif icon:
            out.append({"key": icon, "icon": icon, "name": icon, "nameEn": icon})
    return out


def with_item_order(items):
    return [{**item, "order": index + 1} for index, item in enumerate(items or [])]


def item_summary_key(item):
    if not isinstance(item, dict):
        return None
    if item.get("icon"):
        return str(item["icon"])
    if item.get("id") is not None:
        return f"id:{item['id']}"
    return item.get("key")


def item_from_summary_key(key, item_catalog):
    if not key:
        return None
    by_icon = (item_catalog or {}).get("byIcon", {})
    by_id = (item_catalog or {}).get("byId", {})
    if key in by_icon:
        return compact_item(by_icon[key])
    if str(key).startswith("id:"):
        item_id = str(key)[3:]
        return compact_item(by_id.get(item_id)) or fallback_item_from_id(item_id)
    return {"key": key, "icon": key if re.match(r"^t\d+_\d+$", str(key)) else None, "name": str(key), "nameEn": str(key), "direction": "Auto"}


def item_top_list(item_counts, item_catalog, limit=8):
    rows = []
    for key, count in Counter(item_counts or {}).most_common(limit):
        item = item_from_summary_key(key, item_catalog)
        if item:
            item["count"] = count
            rows.append(item)
    return rows


def item_build_signature(item):
    if not isinstance(item, dict):
        return None
    if item.get("id") is not None:
        return f"id:{item['id']}"
    if item.get("icon"):
        return f"icon:{item['icon']}"
    if item.get("key"):
        return f"key:{item['key']}"
    return None


def item_build_payload(item):
    if not isinstance(item, dict):
        return {}
    return {
        key: item.get(key)
        for key in ["id", "key", "icon", "iconPath", "name", "category", "direction"]
        if item.get(key) is not None
    }


def item_direction(item):
    if not isinstance(item, dict):
        return "Auto"
    return normalize_build_direction(item.get("direction") or item.get("category"))


def build_directions(items, slots=3):
    directions = [item_direction(item) for item in (items or [])[:slots]]
    while len(directions) < slots:
        directions.append("Auto")
    return directions


def core_item_catalog_payload(item_catalog):
    out = {}
    for item_id, item in sorted((item_catalog or {}).get("byId", {}).items(), key=lambda pair: int(pair[0])):
        out[str(item_id)] = item_build_payload(item)
    return out


def item_build_score(wins, games):
    games = float(games or 0)
    wins = float(wins or 0)
    if games <= 0:
        return 0.0
    # Wilson lower bound keeps one-off 100% builds below larger, reliable samples.
    z = 1.28
    phat = wins / games
    denom = 1 + z * z / games
    centre = phat + z * z / (2 * games)
    margin = z * math.sqrt((phat * (1 - phat) + z * z / (4 * games)) / games)
    return round(max(0, (centre - margin) / denom) * 100, 2)


def empty_core_item_builds(generated_at, save_path=None, patch_versions=None):
    return {
        "generatedAt": generated_at,
        "save": {"path": str(save_path) if save_path else None},
        "latestPatch": patch_versions[-1] if patch_versions else None,
        "rules": {
            "coreSizes": [2, 3],
            "directionSlots": 3,
            "directionValues": ["AD", "Magic", "AttackSpeed", "Defense", "MagicResistance", "Hp", "Auto"],
            "topPerGroup": 1,
            "score": "wilson_lower_bound_80pct",
            "recommendedMinGames": 5,
            "hybridWeights": {"soloActual": 1.0, "tournamentPlan": 0.25},
            "fallbackOrder": [
                "hybrid current patch + champion + position + 3 core",
                "hybrid current patch + champion + all positions + 3 core",
                "hybrid all patches + champion + position + 3 core",
                "hybrid all patches + champion + all positions + 3 core",
                "2 core fallback",
            ],
        },
        "sources": {
            "tournamentMatches": 0,
            "soloMatches": 0,
            "primaryRecommendation": "hybridRecommendation",
            "compatBuildsTournament": "hybridRecommendation",
            "soloItemKind": "solo_rank_match_items",
            "tournamentItemKind": "saved_replay_item_build_slots",
            "payload": "compact_mod_recommendations",
            "note": "SoloRankAthlete.items are treated as actual final held items. Tournament MatchReplayAthlete.items are kept separately as saved 3-slot item/build plans because the save does not expose purchase timestamps.",
        },
        "itemCatalog": {},
        "builds": {"tournament": {}},
    }


def core_item_group_row():
    return {
        "games": 0,
        "wins": 0,
        "weightedGames": 0.0,
        "weightedWins": 0.0,
        "soloGames": 0,
        "soloWins": 0,
        "tournamentPlanGames": 0,
        "tournamentPlanWins": 0,
        "items": [],
    }


def add_core_item_combo(groups, scope, patch, champion, position, core_size, items, won, weight=1.0, evidence_source=None):
    patch_key = patch or "unknown"
    pos_key = position or "all"
    signatures = tuple(item_build_signature(item) for item in items[:core_size])
    if len(signatures) < core_size or any(not key for key in signatures):
        return
    item_payloads = tuple(tuple(sorted(item_build_payload(item).items())) for item in items[:core_size])
    for patch_bucket in [patch_key, "all"]:
        for position_bucket in [pos_key, "all"]:
            key = (scope, patch_bucket, champion, position_bucket, core_size, signatures)
            row = groups[key]
            row["games"] += 1
            row["wins"] += 1 if won else 0
            row["weightedGames"] += float(weight or 0)
            row["weightedWins"] += float(weight or 0) if won else 0.0
            if evidence_source == "solo":
                row["soloGames"] += 1
                row["soloWins"] += 1 if won else 0
            elif evidence_source == "tournament":
                row["tournamentPlanGames"] += 1
                row["tournamentPlanWins"] += 1 if won else 0
            if not row["items"]:
                row["items"] = [dict(payload) for payload in item_payloads]


def compact_core_item_builds(groups, top_per_group=5, weighted=False):
    nested = {}
    grouped_rows = defaultdict(list)
    for (scope, patch, champion, position, core_size, signatures), row in groups.items():
        if row["games"] <= 0:
            continue
        if weighted:
            weighted_games = row.get("weightedGames", 0.0)
            weighted_wins = row.get("weightedWins", 0.0)
            games = max(1, int(math.ceil(weighted_games))) if weighted_games > 0 else 0
            wins = int(round(weighted_wins))
            win_rate = round(weighted_wins / weighted_games * 100, 1) if weighted_games else None
            score = item_build_score(weighted_wins, weighted_games)
        else:
            wins = row["wins"]
            games = row["games"]
            win_rate = round(wins / games * 100, 1) if games else None
            score = item_build_score(wins, games)
        payload = {
            "itemKeys": list(signatures),
            "itemIds": [item.get("id") for item in row["items"] if item.get("id") is not None],
            "itemCategories": [item_direction(item) for item in row["items"]],
            "directions": build_directions(row["items"], 3),
            "games": games,
            "wins": wins,
            "winRate": win_rate,
            "score": score,
        }
        if weighted:
            payload.update(
                {
                    "rawGames": row["games"],
                    "rawWins": row["wins"],
                    "soloGames": row.get("soloGames", 0),
                    "soloWins": row.get("soloWins", 0),
                    "tournamentPlanGames": row.get("tournamentPlanGames", 0),
                    "tournamentPlanWins": row.get("tournamentPlanWins", 0),
                    "weightedGames": round(row.get("weightedGames", 0.0), 2),
                    "weightedWins": round(row.get("weightedWins", 0.0), 2),
                }
            )
        grouped_rows[(scope, patch, champion, position, core_size)].append(payload)

    for (scope, patch, champion, position, core_size), rows in grouped_rows.items():
        rows.sort(key=lambda row: (row["score"], row["games"], row["winRate"] or 0), reverse=True)
        scope_node = nested.setdefault(scope, {})
        patch_node = scope_node.setdefault(patch, {})
        champion_node = patch_node.setdefault(champion, {})
        position_node = champion_node.setdefault(position, {})
        position_node[f"core{core_size}"] = rows[:top_per_group]
    return nested


def compact_core_item_candidate(row):
    payload = {
        "directions": row.get("directions") or [],
        "games": row.get("games", 0),
        "wins": row.get("wins", 0),
        "winRate": row.get("winRate"),
        "score": row.get("score", 0),
    }
    if row.get("itemIds"):
        payload["itemIds"] = row["itemIds"]
    if row.get("itemKeys"):
        payload["itemKeys"] = row["itemKeys"]
    return payload


def compact_core_recommendation_tree(builds, latest_patch=None, top_per_group=1):
    out = {}
    patch_keys = []
    if latest_patch:
        patch_keys.append(latest_patch)
    patch_keys.append("all")

    for patch_key in dict.fromkeys(patch_keys):
        patch_node = (builds or {}).get(patch_key)
        if not isinstance(patch_node, dict):
            continue
        patch_out = {}
        for champion, champion_node in patch_node.items():
            champion_out = {}
            if not isinstance(champion_node, dict):
                continue
            for position, position_node in champion_node.items():
                position_out = {}
                if not isinstance(position_node, dict):
                    continue
                for core_key in ["core3", "core2"]:
                    rows = position_node.get(core_key)
                    if not isinstance(rows, list) or not rows:
                        continue
                    position_out[core_key] = [
                        compact_core_item_candidate(row)
                        for row in rows[:top_per_group]
                    ]
                if position_out:
                    champion_out[position] = position_out
            if champion_out:
                patch_out[champion] = champion_out
        if patch_out:
            out[patch_key] = patch_out
    return out


def build_core_item_builds(match_analysis, generated_at, save_path=None, patch_versions=None, item_catalog=None):
    payload = empty_core_item_builds(generated_at, save_path, patch_versions)
    payload["itemCatalog"] = core_item_catalog_payload(item_catalog)
    source_groups = defaultdict(core_item_group_row)
    hybrid_groups = defaultdict(core_item_group_row)
    tournament_matches = 0
    solo_matches = 0
    for match in match_analysis or []:
        version = match.get("version") or "unknown"
        source = match.get("source") or "tournament"
        if source == "tournament":
            tournament_matches += 1
            source_scope = "tournamentPlan"
            hybrid_weight = 0.25
        elif source == "solo":
            solo_matches += 1
            source_scope = "soloActual"
            hybrid_weight = 1.0
        else:
            continue
        for side in ["blue", "red"]:
            team = match.get(side) or {}
            won = match.get("winner") == side
            for player in team.get("players") or []:
                champion = player.get("champion")
                if not champion:
                    continue
                items = [item for item in player.get("items") or [] if item and not item.get("unknown")]
                for core_size in [2, 3]:
                    if len(items) >= core_size:
                        add_core_item_combo(
                            source_groups,
                            source_scope,
                            version,
                            champion,
                            player.get("position") or "all",
                            core_size,
                            items,
                            won,
                            evidence_source=source,
                        )
                        add_core_item_combo(
                            hybrid_groups,
                            "hybridRecommendation",
                            version,
                            champion,
                            player.get("position") or "all",
                            core_size,
                            items,
                            won,
                            weight=hybrid_weight,
                            evidence_source=source,
                        )
    payload["sources"]["tournamentMatches"] = tournament_matches
    payload["sources"]["soloMatches"] = solo_matches
    hybrid_builds = compact_core_item_builds(hybrid_groups, top_per_group=1, weighted=True)
    payload["builds"] = {
        "tournament": compact_core_recommendation_tree(
            hybrid_builds.get("hybridRecommendation", {}),
            payload.get("latestPatch"),
            top_per_group=1,
        )
    }
    return payload


def write_core_item_builds(core_item_builds, export_mod_files=False):
    text = json.dumps(core_item_builds, ensure_ascii=False, separators=(",", ":")) + "\n"
    written = []
    paths = [CORE_ITEM_BUILDS_OUT]
    if export_mod_files:
        paths.extend([CORE_ITEM_BUILDS_MOD_OUT, CORE_ITEM_BUILDS_MOD_DATA_OUT])
        paths.extend(workshop_mod_policy_paths("tfm2_meta_item_delegate", "core-item-builds.json"))
        paths.extend(workshop_mod_policy_paths("tfm2_meta_item_delegate", Path("data") / "core-item-builds.json"))
        paths.extend(configured_policy_target_paths("core_item_builds"))
        paths.extend(configured_policy_target_paths("core_item_builds_data"))
    for path in unique_paths(paths):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
        written.append(path)
    return written


def select_ai_policy_stats(champions, combined_stats, stats_by_patch, patch_versions):
    candidates = []
    for version in reversed(patch_versions or []):
        patch_stats = (stats_by_patch.get(version) or {}).get("overall")
        candidates.append((patch_stats, f"overall latest patch {version}", version))
    candidates.append((combined_stats, "overall all patches", None))

    fallback = (combined_stats, "overall neutral fallback", None, 0)
    for stats, label, version in candidates:
        if not stats:
            continue
        scored = 0
        for champ in champions:
            meta = (stats.get(champ["id"]) or {}).get("metaScore") or {}
            if meta.get("eligible") and meta.get("score") is not None:
                scored += 1
        if scored:
            return stats, label, version, scored
        fallback = (stats, label, version, scored)
    return fallback


def existing_ai_policy_fallback(export_mod_files=False):
    paths = [AI_CHAMPION_POLICY_OUT]
    if export_mod_files:
        paths.insert(0, AI_CHAMPION_POLICY_MOD_OUT)
        paths.extend(configured_policy_target_paths("ai_champion_policy"))
    for path in unique_paths(paths):
        try:
            text = path.read_text(encoding="utf-8")
        except OSError:
            continue
        if "EXTREME_BAIT_TEST_DO_NOT_RELEASE" in text:
            continue
        rows = []
        for line in text.splitlines():
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            fields = line.split("\t")
            if len(fields) < 3:
                continue
            try:
                overall = float(fields[2])
            except ValueError:
                continue
            rows.append((fields[1], overall))
        if any(tier != "C" or overall != 50.0 for tier, overall in rows):
            neutral = sum(1 for tier, overall in rows if tier == "C" and overall == 50.0)
            if not text.endswith("\n"):
                text += "\n"
            return {
                "text": text,
                "path": path,
                "champions": len(rows),
                "neutralChampions": neutral,
            }
    return None


def build_ai_champion_policy(champions, combined_stats, stats_by_patch, patch_versions, generated_at, save_path=None, export_mod_files=False):
    stats, source_label, source_patch, scored = select_ai_policy_stats(
        champions,
        combined_stats,
        stats_by_patch,
        patch_versions,
    )
    if scored == 0:
        fallback = existing_ai_policy_fallback(export_mod_files=export_mod_files)
        if fallback:
            return {
                "text": fallback["text"],
                "source": {
                    "label": f"existing policy fallback from {fallback['path'].name} (no eligible scores)",
                    "patch": source_patch,
                    "scoredChampions": 0,
                    "neutralChampions": fallback["neutralChampions"],
                    "champions": fallback["champions"],
                },
            }
        source_label = f"{source_label} (neutral: no eligible scores)"
    rows = []
    for champ in champions:
        cid = champ["id"]
        stat = stats.get(cid) or {}
        meta = stat.get("metaScore") or {}
        score = meta.get("score") if meta.get("eligible") else None
        if score is None:
            tier = "C"
            overall = 50.0
            eligible = False
        else:
            overall = round(float(score), 1)
            tier = meta_score_grade(overall)
            eligible = True
        rows.append(
            {
                "champion": cid,
                "tier": tier if tier in {"S", "A", "B", "C", "D"} else "C",
                "overall": overall,
                "eligible": eligible,
            }
        )

    rows.sort(key=lambda row: (-row["overall"], row["champion"]))
    lines = [
        "# AUTO_GENERATED_BY_TFM2_META_DASHBOARD",
        "# Do not hand-edit unless you intentionally want to override dashboard meta scoring.",
        f"# Generated: {generated_at}",
        f"# Save: {save_path if save_path else 'not found'}",
        f"# Source: {source_label}",
        "# Format: champion_id<TAB>tier<TAB>overall",
        "# Non-eligible or low-sample champions are emitted as neutral C/50.0 so native AI keeps its base score.",
        "# champion_id\ttier\toverall",
    ]
    lines.extend(f"{row['champion']}\t{row['tier']}\t{row['overall']:.1f}" for row in rows)
    return {
        "text": "\n".join(lines) + "\n",
        "source": {
            "label": source_label,
            "patch": source_patch,
            "scoredChampions": scored,
            "neutralChampions": sum(1 for row in rows if not row["eligible"]),
            "champions": len(rows),
        },
    }


def write_ai_champion_policy(policy, export_mod_files=False):
    written = []
    text = policy["text"]
    paths = [AI_CHAMPION_POLICY_OUT]
    if export_mod_files:
        paths.extend([AI_CHAMPION_POLICY_MOD_OUT, CHAMPION_TIER_POLICY_MOD_OUT])
        paths.extend(workshop_mod_policy_paths("tfm2_ai_banpick_probe", "ai_champion_policy.tsv"))
        paths.extend(workshop_mod_policy_paths("tfm2_meta_champion_tiers", "champion_tier_policy.tsv"))
        paths.extend(configured_policy_target_paths("ai_champion_policy"))
        paths.extend(configured_policy_target_paths("champion_tier_policy"))
    for path in unique_paths(paths):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
        written.append(path)
    return written


def unique_paths(paths):
    seen = set()
    out = []
    for path in paths:
        try:
            resolved = Path(path).expanduser().resolve()
        except OSError:
            continue
        key = str(resolved).lower()
        if key not in seen:
            seen.add(key)
            out.append(resolved)
    return out


def tfm2_data_roots():
    raw_roots = []
    if os.environ.get("TFM2_APPDATA"):
        raw_roots.append(Path(os.environ["TFM2_APPDATA"]))
    if os.environ.get("APPDATA"):
        raw_roots.append(Path(os.environ["APPDATA"]))
    if os.environ.get("USERPROFILE"):
        raw_roots.append(Path(os.environ["USERPROFILE"]) / "AppData" / "Roaming")
    raw_roots.append(Path.home() / "AppData" / "Roaming")

    candidates = []
    for root in unique_paths(raw_roots):
        if root.name.lower() == "teamfightmanager2":
            candidates.append(root)
        else:
            candidates.append(root / "TeamSamoyed" / "TeamfightManager2")
            candidates.append(root / "TeamSamoyed" / "Teamfight Manager2")
    return unique_paths(candidates)


APPDATA_ROOTS = tfm2_data_roots()
APPDATA = next((root for root in APPDATA_ROOTS if (root / "data").exists()), APPDATA_ROOTS[0])
SAVE_DIRS = [root / "data" for root in APPDATA_ROOTS]
DIAG_DIRS = [root / "diagnostics" for root in APPDATA_ROOTS]
DIAG_DIR = next((path for path in DIAG_DIRS if path.exists()), APPDATA / "diagnostics")
if os.environ.get("TFM2_META_EXPORT_DIR"):
    EXPORT_DIR = Path(os.environ["TFM2_META_EXPORT_DIR"]).expanduser().resolve()
else:
    EXPORT_DIR = next(
        (
            path
            for path in [SAVE_PROBE_SNAPSHOT_DIR, *(diag / "meta_export" for diag in DIAG_DIRS)]
            if path.exists()
        ),
        DIAG_DIR / "meta_export",
    )


def looks_like_save(path: Path):
    if path.suffix.lower() != ".data":
        return False
    try:
        with path.open("rb") as f:
            tail = b""
            while True:
                chunk = f.read(1024 * 1024)
                if not chunk:
                    return False
                if b"\x1f\x8b\x08" in tail + chunk:
                    return True
                tail = chunk[-2:]
    except OSError:
        return False


def scan_save_dir(save_dir: Path):
    saves = []
    if not save_dir.exists():
        return saves
    direct = list(save_dir.glob("*.data"))
    recursive = [path for path in save_dir.rglob("*.data") if path.is_file() and path not in direct]
    for path in direct + recursive:
        if looks_like_save(path):
            saves.append(path)
    return saves


def resolve_manual_save_path(raw):
    if not raw:
        return None, []
    text = str(raw).strip().strip('"')
    if not text:
        return None, []
    path = Path(text).expanduser()
    if path.is_file():
        return path if looks_like_save(path) else None, [path]
    if path.is_dir():
        dirs = [path]
        if (path / "data").is_dir():
            dirs.insert(0, path / "data")
        saves = []
        for directory in dirs:
            saves.extend(scan_save_dir(directory))
        saves = sorted(set(saves), key=lambda p: p.stat().st_mtime, reverse=True)
        return (saves[0] if saves else None), dirs
    return None, [path]


def latest_save(manual_path=None):
    manual_save, manual_roots = resolve_manual_save_path(manual_path or os.environ.get("TFM2_SAVE_PATH"))
    if manual_save:
        return manual_save, manual_roots

    saves = []
    for save_dir in SAVE_DIRS:
        saves.extend(scan_save_dir(save_dir))
    saves = sorted(set(saves), key=lambda p: p.stat().st_mtime, reverse=True)
    return (saves[0] if saves else None), manual_roots


def decompress_save(path: Path):
    data = path.read_bytes()
    start = data.find(b"\x1f\x8b\x08")
    if start < 0:
        return b""
    return gzip.decompress(data[start:])


def readable_text(blob: bytes):
    chars = []
    for byte in blob:
        if 32 <= byte < 127:
            chars.append(chr(byte))
        else:
            chars.append("\t")
    return re.sub(r"\t+", "\t", "".join(chars))


def looks_like_team_name(token):
    token = (token or "").strip()
    if not 2 <= len(token) <= 64:
        return False
    if any(ch in token for ch in ":/#{}[]\\|"):
        return False
    if not re.search(r"[A-Za-z0-9가-힣]", token):
        return False
    lower = token.lower()
    blocked = [
        "asset",
        "custom_team_logo",
        "furniture",
        "wallpaper",
        "window",
        "partition",
        "chair",
        "desk",
        "plain_",
        "premium_",
        "clean_",
    ]
    return not any(word in lower for word in blocked)


ATHLETE_NAME_STOPWORDS = {
    "Position",
    "Champion",
    "PickCount",
    "WinRate",
    "ChampionStats",
    "Team",
    "Region",
    "Country",
    "League",
    "Season",
    "Year",
    "Month",
    "Day",
    "ContentId",
    "LocalizedName",
    "LocalizedDesc",
    "True",
    "False",
    "None",
    "Game",
    "Data",
    "Asset",
    "Player",
    "Coach",
    "Staff",
    "Ratio",
    "Rating",
    "Rank",
    "Score",
}


def looks_like_athlete_name(token):
    token = (token or "").strip()
    if not 2 <= len(token) <= 28:
        return False
    if token in ATHLETE_NAME_STOPWORDS:
        return False
    if any(ch in token for ch in ":/#{}[]\\|"):
        return False
    if not re.search(r"[A-Za-z가-힣]", token):
        return False
    lower = token.lower()
    blocked = [
        "asset",
        "custom_team_logo",
        "furniture",
        "wallpaper",
        "window",
        "partition",
        "chair",
        "desk",
        "plain_",
        "premium_",
        "clean_",
    ]
    if any(word in lower for word in blocked):
        return False
    if re.fullmatch(r"[A-Z]", token):
        return False
    if re.fullmatch(r"[a-z_]+", token) and len(token) > 10:
        return False
    return True


def extract_team_names_from_text(text):
    teams = {}
    for match in re.finditer(r"custom:custom_team_logo/(\d+)", text):
        prefix = text[max(0, match.start() - 240) : match.start()]
        tokens = [token.strip() for token in prefix.split("\t") if token.strip()]
        for token in reversed(tokens):
            if looks_like_team_name(token):
                teams[str(int(match.group(1)))] = token
                break
    return teams


def extract_athlete_names_from_text(text):
    # The save starts with global/team data and later stores athlete contract rows.
    # Scanning after the first megabyte avoids many non-player date fields.
    start_pos = min(1_000_000, len(text))
    tokens = [
        (match.group(0).strip(), start_pos + match.start())
        for match in re.finditer(r"[^\t]+", text[start_pos:])
    ]
    names = []
    for index, (token, _pos) in enumerate(tokens):
        if not re.fullmatch(r"20\d{2}-\d{2}-\d{2}", token):
            continue
        if index + 1 >= len(tokens) or tokens[index + 1][0] != "00:00:00":
            continue
        has_end_date = any(
            cursor < len(tokens) and re.fullmatch(r"20\d{2}-\d{2}-\d{2}", tokens[cursor][0])
            for cursor in range(index + 2, min(index + 8, len(tokens)))
        )
        if not has_end_date:
            continue
        candidates = []
        for cursor in range(max(0, index - 50), index):
            candidate = tokens[cursor][0]
            if looks_like_athlete_name(candidate):
                candidates.append(candidate)
        if candidates:
            name = candidates[-1]
            if not names or names[-1] != name:
                names.append(name)
    return {str(index): name for index, name in enumerate(names)}


def extract_save_lookup(blob):
    if not blob:
        return {"teams": {}, "athletes": {}}
    text = readable_text(blob)
    return {
        "teams": extract_team_names_from_text(text),
        "athletes": extract_athlete_names_from_text(text),
    }


def iter_struct_blocks_with_prefix(text, struct_name):
    search = f"{struct_name} {{"
    offset = 0
    while True:
        start = text.find(search, offset)
        if start < 0:
            break
        open_at = text.find("{", start)
        block, end = read_balanced(text, open_at)
        if not block:
            break
        yield block, text[max(0, start - 80) : start]
        offset = end + 1


def parse_debug_name_lookup(path: Path, struct_names, name_fields, validator):
    if not path.exists():
        return {}
    text = path.read_text(encoding="utf-8", errors="ignore")
    out = {}
    for struct_name in struct_names:
        for block, prefix in iter_struct_blocks_with_prefix(text, struct_name):
            item_id = parse_first_int(block, "id")
            if item_id is None:
                key_match = re.search(r"(\d+)\s*:\s*$", prefix)
                if key_match:
                    item_id = int(key_match.group(1))
            if item_id is None:
                continue

            name = None
            for field in name_fields:
                name = parse_quoted_field(block, field)
                if name:
                    break
            if name and validator(name):
                out[str(item_id)] = name
    return out


def extract_exporter_lookup(export_dir: Path):
    return {
        "teams": parse_debug_name_lookup(
            export_dir / "teams.debug.txt",
            ["Team"],
            ["name", "team_name", "display_name", "localized_name"],
            looks_like_team_name,
        ),
        "athletes": parse_debug_name_lookup(
            export_dir / "athletes.debug.txt",
            ["Athlete"],
            ["name", "nickname", "nick_name", "display_name", "localized_name"],
            looks_like_athlete_name,
        ),
    }


def league_filter_payload():
    return {
        "regions": [{"id": "all", "label": "전체 지역"}]
        + [{"id": key, "label": label} for key, label in LEAGUE_REGIONS],
        "divisions": [{"id": "all", "label": "전체 부"}]
        + [{"id": key, "label": label} for key, label in LEAGUE_DIVISIONS],
    }


def league_stat_bucket_keys():
    keys = ["all"] + [key for key, _ in LEAGUE_DIVISIONS]
    keys.extend(f"region:{region}" for region, _ in LEAGUE_REGIONS)
    for region, _ in LEAGUE_REGIONS:
        for division, _ in LEAGUE_DIVISIONS:
            keys.append(f"region:{region}:{division}")
    return keys


def league_meta_from_key(key, league_id=None):
    if key not in LEAGUE_KEY_META:
        return None
    region, division = LEAGUE_KEY_META[key]
    region_label = REGION_LABELS.get(region, region.upper())
    division_label = DIVISION_LABELS.get(division, division)
    return {
        "id": league_id,
        "key": key,
        "region": region,
        "regionLabel": region_label,
        "division": division,
        "divisionLabel": division_label,
        "label": f"{region_label} {division_label}",
    }


def parse_team_league_lookup(export_dir: Path):
    path = export_dir / "teams.debug.txt"
    if not path.exists():
        return {}
    text = path.read_text(encoding="utf-8", errors="ignore")
    teams = {}
    for block, prefix in iter_struct_blocks_with_prefix(text, "Team"):
        team_id = parse_first_int(block, "id")
        if team_id is None:
            key_match = re.search(r"(\d+)\s*:\s*$", prefix)
            if key_match:
                team_id = int(key_match.group(1))
        if team_id is None:
            continue
        league_id = parse_first_int(block, "league_id")
        league_key = None
        key_match = re.search(r"#asset/base/text/ui\?league\.([a-z0-9]+)", block)
        if key_match:
            league_key = key_match.group(1)
        if league_key is None and league_id is not None and 0 <= league_id < len(LEAGUE_ID_KEYS):
            league_key = LEAGUE_ID_KEYS[league_id]
        meta = league_meta_from_key(league_key, league_id)
        if meta:
            teams[str(team_id)] = meta
    return teams


def league_bucket_keys(region, division):
    keys = ["all"]
    if division in DIVISION_LABELS:
        keys.append(division)
    if region in REGION_LABELS:
        keys.append(f"region:{region}")
        if division in DIVISION_LABELS:
            keys.append(f"region:{region}:{division}")
    return keys


def solo_region_meta(region_id):
    region = SOLO_REGION_KEYS.get(region_id)
    if not region:
        return {"id": region_id, "key": None, "region": None, "regionLabel": None, "bucketKeys": ["all"]}
    return {
        "id": region_id,
        "key": region,
        "region": region,
        "regionLabel": REGION_LABELS.get(region, region.upper()),
        "bucketKeys": league_bucket_keys(region, None),
    }


def match_bucket_keys(match):
    if (match or {}).get("source") == "solo":
        return ((match or {}).get("region") or {}).get("bucketKeys", ["all"])
    return ((match or {}).get("league") or {}).get("bucketKeys", ["all"])


def match_league_meta(match, team_leagues):
    blue = team_leagues.get(str(match.get("blueTeamId")))
    red = team_leagues.get(str(match.get("redTeamId")))
    if not blue and not red:
        return {"key": None, "region": None, "division": None, "label": "리그 미확인", "bucketKeys": ["all"]}
    if blue and red and blue.get("key") == red.get("key"):
        meta = dict(blue)
        meta["bucketKeys"] = league_bucket_keys(meta.get("region"), meta.get("division"))
        return meta

    region = blue.get("region") if blue and red and blue.get("region") == red.get("region") else None
    division = blue.get("division") if blue and red and blue.get("division") == red.get("division") else None
    label_parts = []
    if region:
        label_parts.append(REGION_LABELS.get(region, region.upper()))
    if division:
        label_parts.append(DIVISION_LABELS.get(division, division))
    meta = {
        "key": None,
        "region": region,
        "regionLabel": REGION_LABELS.get(region) if region else None,
        "division": division,
        "divisionLabel": DIVISION_LABELS.get(division) if division else None,
        "label": " ".join(label_parts) if label_parts else "혼합/국제",
    }
    meta["bucketKeys"] = league_bucket_keys(region, division)
    return meta


def attach_match_league_meta(matches, team_leagues):
    for match in matches or []:
        match["league"] = match_league_meta(match, team_leagues)
    return matches


def parse_patch_version_dates(export_dir: Path):
    path = export_dir / "teams.debug.txt"
    if not path.exists():
        return {}
    text = path.read_text(encoding="utf-8", errors="ignore")
    out = {}
    for block, _prefix in iter_struct_blocks_with_prefix(text, "News"):
        if "ty: Patch" not in block:
            continue
        date_key = date_key_from_datetime(parse_datetime_field(block, "date"))
        if date_key == "unknown":
            continue
        version_match = re.search(r'\("Version",\s*"([^"]+)"\)', block)
        if not version_match:
            continue
        version = version_match.group(1)
        if version not in out or date_key < out[version]:
            out[version] = date_key
    return dict(sorted(out.items(), key=lambda item: version_sort_key(item[0])))


def build_patch_version_ranges(version_dates):
    items = sorted(
        [(str(version), str(day)) for version, day in (version_dates or {}).items() if re.match(r"^\d{4}-\d{2}-\d{2}$", str(day))],
        key=lambda item: version_sort_key(item[0]),
    )
    ranges = {}
    for index, (version, start) in enumerate(items):
        ranges[version] = {
            "start": start,
            "end": items[index + 1][1] if index + 1 < len(items) else None,
        }
    return ranges


def day_in_patch_range(day, patch_range, max_day=None):
    if not day or not re.match(r"^\d{4}-\d{2}-\d{2}$", str(day)):
        return False
    start = patch_range.get("start") if patch_range else None
    end = patch_range.get("end") if patch_range else None
    if start and day < start:
        return False
    if end and day >= end:
        return False
    if max_day and day > max_day:
        return False
    return True


def parse_year_schedule_dates(export_dir: Path, max_day=None):
    path = export_dir / "year_schedules.debug.txt"
    if not path.exists():
        return {
            "leagueRounds": {},
            "leagueSlots": [],
            "tournament": [],
            "tournamentSlots": [],
            "tournamentMatchDates": {},
            "tournamentMatchIds": [],
            "tournamentScheduleSummary": {},
            "matchReports": {"bySignature": {}, "reports": 0, "sets": 0},
            "patchVersionDates": parse_patch_version_dates(export_dir),
            "patchVersionRanges": {},
        }
    text = path.read_text(encoding="utf-8", errors="ignore")
    event_re = re.compile(
        r"\((\d{4}-\d{2}-\d{2}),\s+"
        r"(LeagueMatch|LeaguePlayoff|TournamentGroupMatch|TournamentMatch)\s+\{\s+round:\s*(\d+)"
        r"(?:,\s*index:\s*(\d+))?\s*\}"
    )
    league_round_dates = defaultdict(list)
    league_slots = []
    tournament_dates = []
    tournament_slots = []
    league_season = 0
    previous_league_round = None
    schedule_order = 0
    for match in event_re.finditer(text):
        day, kind, raw_round, raw_index = match.groups()
        round_no = int(raw_round)
        index_no = int(raw_index) if raw_index is not None else None
        if kind in ("LeagueMatch", "LeaguePlayoff"):
            if previous_league_round is not None and round_no == 0 and previous_league_round > 0:
                if kind == "LeagueMatch":
                    league_season += 1
            if kind == "LeagueMatch":
                previous_league_round = round_no
            season_round = league_season * 100 + round_no + (80 if kind == "LeaguePlayoff" else 0)
            if day not in league_round_dates[season_round]:
                league_round_dates[season_round].append(day)
            league_slots.append(
                {
                    "day": day,
                    "kind": kind,
                    "round": season_round,
                    "rawRound": round_no,
                    "index": index_no,
                }
            )
        else:
            tournament_dates.append(day)
            tournament_slots.append(
                {
                    "day": day,
                    "kind": kind,
                    "round": round_no,
                    "rawRound": round_no,
                    "index": index_no,
                    "scheduleOrder": schedule_order,
                }
            )
        schedule_order += 1
    league_slots.sort(key=lambda row: (row.get("round", 0), row.get("index") if row.get("index") is not None else 99, row.get("day") or ""))
    for sequence, slot in enumerate(league_slots):
        slot["sequence"] = sequence
    tournament_slots.sort(key=lambda row: row.get("scheduleOrder", 0))
    for sequence, slot in enumerate(tournament_slots):
        slot["sequence"] = sequence
    tournament_schedule = parse_tournament_competition_match_dates(export_dir, tournament_slots, max_day=max_day)
    patch_version_dates = parse_patch_version_dates(export_dir)
    return {
        "leagueRounds": {
            str(round_no): sorted(days)
            for round_no, days in league_round_dates.items()
        },
        "leagueSlots": league_slots,
        "tournament": tournament_dates,
        "tournamentSlots": tournament_slots,
        "tournamentMatchDates": tournament_schedule.get("byId", {}),
        "tournamentMatchIds": sorted(
            [int(key) for key in tournament_schedule.get("byId", {}).keys() if str(key).isdigit()]
        ),
        "tournamentScheduleSummary": tournament_schedule.get("summary", {}),
        "matchReports": parse_match_report_replay_dates(export_dir),
        "patchVersionDates": patch_version_dates,
        "patchVersionRanges": build_patch_version_ranges(patch_version_dates),
    }


def parse_integer_list(raw):
    return [int(value) for value in re.findall(r"\d+", raw or "")]


def date_gap_days(left, right):
    try:
        return (datetime.fromisoformat(str(right)) - datetime.fromisoformat(str(left))).days
    except (TypeError, ValueError):
        return 0


def split_tournament_slot_windows(tournament_slots):
    ordered = sorted(tournament_slots or [], key=lambda row: (row.get("day") or "", row.get("sequence", 0)))
    windows = []
    current = []
    last_day = None
    for slot in ordered:
        day = slot.get("day")
        if current and last_day and day and date_gap_days(last_day, day) > 30:
            windows.append(current)
            current = []
        current.append(slot)
        if day:
            last_day = day
    if current:
        windows.append(current)

    out = []
    for index, slots in enumerate(windows):
        group_slots = [slot for slot in slots if slot.get("kind") == "TournamentGroupMatch"]
        bracket_slots = [slot for slot in slots if slot.get("kind") == "TournamentMatch"]
        days = [slot.get("day") for slot in slots if slot.get("day")]
        out.append(
            {
                "index": index,
                "start": min(days) if days else None,
                "end": max(days) if days else None,
                "groupSlots": group_slots,
                "bracketSlots": bracket_slots,
            }
        )
    return out


def select_tournament_slot_window(windows, max_day=None):
    if not windows:
        return None
    if max_day:
        eligible = [window for window in windows if not window.get("start") or window.get("start") <= max_day]
        if eligible:
            return eligible[-1]
    return windows[0]


def parse_tournament_competition_match_dates(export_dir: Path, tournament_slots, max_day=None):
    path = export_dir / "tournament_competitions.debug.txt"
    windows = split_tournament_slot_windows(tournament_slots)
    window = select_tournament_slot_window(windows, max_day=max_day)
    summary = {
        "competitions": 0,
        "windows": len(windows),
        "windowIndex": window.get("index") if window else None,
        "windowStart": window.get("start") if window else None,
        "windowEnd": window.get("end") if window else None,
        "groupSlots": len(window.get("groupSlots") or []) if window else 0,
        "bracketSlots": len(window.get("bracketSlots") or []) if window else 0,
        "mapped": 0,
        "groupMapped": 0,
        "bracketMapped": 0,
    }
    if not path.exists() or not window:
        return {"byId": {}, "summary": summary}

    text = path.read_text(encoding="utf-8", errors="ignore")
    competition_re = re.compile(
        r"TournamentCompetition\s+\{\s*id:\s*(\d+),\s*ty:\s*([A-Za-z0-9_]+),"
        r"\s*group_matches:\s*\[([^\]]*)\],\s*tournament_matches:\s*\[([^\]]*)\]",
        re.S,
    )
    group_slots = window.get("groupSlots") or []
    bracket_slots = window.get("bracketSlots") or []
    by_id = {}
    for match in competition_re.finditer(text):
        competition_id, competition_type, raw_group_ids, raw_bracket_ids = match.groups()
        summary["competitions"] += 1
        group_ids = parse_integer_list(raw_group_ids)
        bracket_ids = parse_integer_list(raw_bracket_ids)
        for slot_index, replay_id in enumerate(group_ids):
            if slot_index >= len(group_slots):
                break
            if str(replay_id) in by_id:
                continue
            slot = group_slots[slot_index]
            by_id[str(replay_id)] = {
                "day": slot.get("day"),
                "competitionId": int(competition_id),
                "competitionType": competition_type,
                "phase": "group",
                "slotIndex": slot_index,
                "scheduleRound": slot.get("round"),
                "scheduleIndex": slot.get("index"),
                "scheduleSequence": slot.get("sequence"),
                "scheduleWindow": window.get("index"),
            }
            summary["groupMapped"] += 1
        for slot_index, replay_id in enumerate(bracket_ids):
            if slot_index >= len(bracket_slots):
                break
            if str(replay_id) in by_id:
                continue
            slot = bracket_slots[slot_index]
            by_id[str(replay_id)] = {
                "day": slot.get("day"),
                "competitionId": int(competition_id),
                "competitionType": competition_type,
                "phase": "bracket",
                "slotIndex": slot_index,
                "scheduleRound": slot.get("round"),
                "scheduleIndex": slot.get("index"),
                "scheduleSequence": slot.get("sequence"),
                "scheduleWindow": window.get("index"),
            }
            summary["bracketMapped"] += 1
    summary["mapped"] = len(by_id)
    return {"byId": by_id, "summary": summary}


def assign_match_date(match, day, source, confidence, detail=None):
    if not day:
        return False
    match["date"] = f"{day}T00:00:00"
    match["dateKey"] = day
    match["dateLabel"] = day
    match["dateSource"] = source
    match["dateConfidence"] = confidence
    if detail:
        match["dateInference"] = detail
    return True


def parse_manifest_game_time(meta_manifest):
    raw = (meta_manifest or {}).get("game_time") or (meta_manifest or {}).get("gameTime")
    if not raw:
        return None
    text = str(raw).strip()
    if not text:
        return None
    try:
        return datetime.fromisoformat(text)
    except ValueError:
        return None


def manifest_game_day(meta_manifest):
    game_time = parse_manifest_game_time(meta_manifest)
    return game_time.date().isoformat() if game_time else None


def filter_future_match_analysis(matches, max_day):
    summary = {"maxGameDate": max_day, "removed": 0, "kept": len(matches or [])}
    if not max_day:
        return matches, summary
    kept = []
    removed = 0
    for match in matches or []:
        date_key = str(match.get("dateKey") or "")
        if re.match(r"^\d{4}-\d{2}-\d{2}$", date_key) and date_key > max_day:
            removed += 1
            continue
        kept.append(match)
    summary["removed"] = removed
    summary["kept"] = len(kept)
    return kept, summary


def int_value(value):
    try:
        return int(value)
    except (TypeError, ValueError):
        return None


def replay_signature(blue_team_id, red_team_id, game_tick, blue_kills, red_kills, blue_gold, red_gold, winner):
    values = [blue_team_id, red_team_id, game_tick, blue_kills, red_kills, blue_gold, red_gold]
    parsed = [int_value(value) for value in values]
    if any(value is None for value in parsed) or winner not in ("blue", "red"):
        return None
    return (*parsed, winner)


def match_analysis_signature(match):
    blue = match.get("blue") or {}
    red = match.get("red") or {}
    return replay_signature(
        match.get("blueTeamId"),
        match.get("redTeamId"),
        match.get("gameTick"),
        blue.get("killsTotal"),
        red.get("killsTotal"),
        blue.get("gold"),
        red.get("gold"),
        match.get("winner"),
    )


def parse_match_report_replay_dates(export_dir: Path):
    path = export_dir / "teams.debug.txt"
    if not path.exists():
        return {"bySignature": {}, "reports": 0, "sets": 0}
    text = path.read_text(encoding="utf-8", errors="ignore")
    by_signature = defaultdict(list)
    reports = 0
    sets = 0
    for block, _prefix in iter_struct_blocks_with_prefix(text, "News"):
        if "ty: MatchReport" not in block:
            continue
        report_day = date_key_from_datetime(parse_datetime_field(block, "date"))
        if report_day == "unknown":
            continue
        match_id = parse_first_int(block, "match_id")
        my_team_id = parse_first_int(block, "my_team_id")
        enemy_team_id = parse_first_int(block, "enemy_team_id")
        if my_team_id is None or enemy_team_id is None:
            continue
        reports += 1
        for set_index, set_block in enumerate(split_struct_blocks(extract_named_array(block, "set_data"), "MatchSetArticleData")):
            is_team1_blue = parse_bool(set_block, "is_team1_blue")
            is_team1_win = parse_bool(set_block, "is_team1_win")
            if is_team1_blue is None or is_team1_win is None:
                continue
            game_tick = parse_first_int(set_block, "game_tick")
            team1_kills = parse_first_int(set_block, "team1_total_kill")
            team2_kills = parse_first_int(set_block, "team2_total_kill")
            team1_gold = parse_first_int(set_block, "team1_total_gold")
            team2_gold = parse_first_int(set_block, "team2_total_gold")
            if is_team1_blue:
                blue_team_id, red_team_id = my_team_id, enemy_team_id
                blue_kills, red_kills = team1_kills, team2_kills
                blue_gold, red_gold = team1_gold, team2_gold
                winner = "blue" if is_team1_win else "red"
            else:
                blue_team_id, red_team_id = enemy_team_id, my_team_id
                blue_kills, red_kills = team2_kills, team1_kills
                blue_gold, red_gold = team2_gold, team1_gold
                winner = "red" if is_team1_win else "blue"
            signature = replay_signature(blue_team_id, red_team_id, game_tick, blue_kills, red_kills, blue_gold, red_gold, winner)
            if not signature:
                continue
            by_signature[signature].append(
                {
                    "day": report_day,
                    "matchId": match_id,
                    "setIndex": set_index,
                }
            )
            sets += 1
    return {"bySignature": dict(by_signature), "reports": reports, "sets": sets}


def parse_match_stat_versions(export_dir: Path):
    path = export_dir / "match_stats.debug.txt"
    if not path.exists():
        return {}
    text = path.read_text(encoding="utf-8", errors="ignore")
    versions = {}
    for block, _prefix in iter_struct_blocks_with_prefix(text, "MatchStat"):
        replay_id = parse_first_int(block, "id")
        version = parse_version(block)
        if replay_id is not None and version and version != "unknown":
            versions[replay_id] = version
    return versions


def apply_match_stat_versions(matches, versions):
    summary = {"loaded": len(versions or {}), "applied": 0, "changed": 0}
    if not versions:
        return summary
    for match in matches or []:
        replay_id = int_value(match.get("id"))
        if replay_id is None:
            continue
        version = versions.get(replay_id)
        if not version:
            continue
        old_version = match.get("version")
        match["version"] = version
        match["versionSource"] = "match_stats.debug.txt"
        summary["applied"] += 1
        if old_version != version:
            summary["changed"] += 1
    return summary


def valid_match_day(match):
    day = str((match or {}).get("dateKey") or "")
    return day if re.match(r"^\d{4}-\d{2}-\d{2}$", day) else None


def match_series_pair(match):
    blue_id = int_value((match or {}).get("blueTeamId"))
    red_id = int_value((match or {}).get("redTeamId"))
    if blue_id is None or red_id is None:
        return None
    return tuple(sorted((blue_id, red_id)))


def consecutive_series_groups(rows):
    groups = []
    current = None
    for match in sorted(rows or [], key=lambda row: NumberLike(row.get("id"))):
        pair = match_series_pair(match)
        if current and pair is not None and current["pair"] == pair:
            current["rows"].append(match)
            continue
        current = {"pair": pair, "rows": [match]}
        groups.append(current)
    return groups


def clustered_league_series_groups(rows, max_id_gap=120):
    by_pair = defaultdict(list)
    missing_pair_index = 0
    for match in rows or []:
        pair = match_series_pair(match)
        if pair is None:
            pair = ("unknown", missing_pair_index)
            missing_pair_index += 1
        by_pair[pair].append(match)

    groups = []
    for pair, pair_rows in by_pair.items():
        current = []
        for match in sorted(pair_rows, key=lambda row: NumberLike(row.get("id"))):
            if current and NumberLike(match.get("id")) - NumberLike(current[-1].get("id")) > max_id_gap:
                groups.append({"pair": pair, "rows": current})
                current = []
            current.append(match)
        if current:
            groups.append({"pair": pair, "rows": current})

    return sorted(
        groups,
        key=lambda group: min(NumberLike(row.get("id")) for row in group["rows"]),
    )


def infer_match_analysis_dates(matches, team_leagues, schedule_dates):
    if not matches:
        return {"inferred": 0, "source": "none"}
    league_rounds = schedule_dates.get("leagueRounds") or {}
    league_slots = schedule_dates.get("leagueSlots") or []
    tournament_dates = schedule_dates.get("tournament") or []
    tournament_match_dates = schedule_dates.get("tournamentMatchDates") or {}
    match_reports = schedule_dates.get("matchReports") or {}
    report_dates = match_reports.get("bySignature") or {}
    inferred = 0
    exact_report_matches = 0
    report_series_matches = 0
    tournament_scheduled_matches = 0
    series_estimated_matches = 0
    clustered_series_groups = 0
    series_groups = 0
    unmapped_other_series = 0
    league_groups = defaultdict(list)
    other_matches = []

    for match in sorted(matches, key=lambda row: NumberLike(row.get("id"))):
        signature = match_analysis_signature(match)
        report_entries = report_dates.get(signature) if signature else None
        if not report_entries:
            continue
        report_entry = report_entries[0]
        if assign_match_date(
            match,
            report_entry.get("day"),
            "team_news_match_report",
            "exact",
            {
                "matchId": report_entry.get("matchId"),
                "setIndex": report_entry.get("setIndex"),
            },
        ):
            inferred += 1
            exact_report_matches += 1

    for match in sorted(matches, key=lambda row: NumberLike(row.get("id"))):
        if valid_match_day(match):
            continue
        replay_id = int_value(match.get("id"))
        if replay_id is None:
            continue
        scheduled = tournament_match_dates.get(str(replay_id)) or tournament_match_dates.get(replay_id)
        if not scheduled:
            continue
        if assign_match_date(
            match,
            scheduled.get("day"),
            "tournament_competition_schedule",
            "scheduled",
            {
                "competitionId": scheduled.get("competitionId"),
                "competitionType": scheduled.get("competitionType"),
                "phase": scheduled.get("phase"),
                "slotIndex": scheduled.get("slotIndex"),
                "scheduleRound": scheduled.get("scheduleRound"),
                "scheduleIndex": scheduled.get("scheduleIndex"),
                "scheduleSequence": scheduled.get("scheduleSequence"),
                "scheduleWindow": scheduled.get("scheduleWindow"),
            },
        ):
            inferred += 1
            tournament_scheduled_matches += 1

    for match in sorted(matches, key=lambda row: NumberLike(row.get("id"))):
        blue = team_leagues.get(str(match.get("blueTeamId")))
        red = team_leagues.get(str(match.get("redTeamId")))
        if blue and red and blue.get("key") == red.get("key"):
            league_groups[blue["key"]].append(match)
        else:
            other_matches.append(match)

    for league_key, rows in league_groups.items():
        groups = clustered_league_series_groups(rows)
        clustered_series_groups += len(groups)
        series_groups += len(groups)
        for series_index, group in enumerate(groups):
            exact_days = [valid_match_day(row) for row in group["rows"] if row.get("dateSource") == "team_news_match_report"]
            exact_days = [day for day in exact_days if day]
            if exact_days:
                day = Counter(exact_days).most_common(1)[0][0]
                for row in group["rows"]:
                    if valid_match_day(row):
                        continue
                    if assign_match_date(
                        row,
                        day,
                        "team_news_match_report_series",
                        "exact",
                        {
                            "leagueKey": league_key,
                            "seriesIndex": series_index,
                        },
                    ):
                        inferred += 1
                        report_series_matches += 1
                continue
            if series_index >= len(league_slots):
                continue
            slot = league_slots[series_index]
            for row in group["rows"]:
                if valid_match_day(row):
                    continue
                if assign_match_date(
                    row,
                    slot.get("day"),
                    "year_schedule_cluster_order",
                    "estimated",
                    {
                        "leagueKey": league_key,
                        "seriesIndex": series_index,
                        "seriesPair": list(group["pair"]) if isinstance(group.get("pair"), tuple) else group.get("pair"),
                        "seriesReplayIds": [NumberLike(item.get("id")) for item in group["rows"]],
                        "scheduleRound": slot.get("round"),
                        "scheduleIndex": slot.get("index"),
                        "scheduleSequence": slot.get("sequence"),
                    },
                ):
                    inferred += 1
                    series_estimated_matches += 1

    for index, group in enumerate(consecutive_series_groups(other_matches)):
        series_groups += 1
        if any(valid_match_day(row) for row in group["rows"]):
            continue
        unmapped_other_series += 1

    return {
        "inferred": inferred,
        "source": "match_reports+year_schedules.debug.txt",
        "leagueRounds": len(league_rounds),
        "leagueSlots": len(league_slots),
        "tournamentDates": len(tournament_dates),
        "tournamentScheduledMatches": tournament_scheduled_matches,
        "tournamentMappedIds": len(tournament_match_dates),
        "unmappedOtherSeries": unmapped_other_series,
        "matchReportNews": match_reports.get("reports", 0),
        "matchReportSets": match_reports.get("sets", 0),
        "exactReportMatches": exact_report_matches,
        "reportSeriesMatches": report_series_matches,
        "seriesEstimatedMatches": series_estimated_matches,
        "seriesGroups": series_groups,
        "clusteredLeagueSeriesGroups": clustered_series_groups,
    }


def group_series_version(rows):
    versions = [
        str(row.get("version"))
        for row in rows or []
        if row.get("version") and str(row.get("version")) != "unknown"
    ]
    if not versions:
        return None
    counts = Counter(versions)
    best_count = max(counts.values())
    candidates = [version for version, count in counts.items() if count == best_count]
    return sorted(candidates, key=version_sort_key)[-1]


def patch_range_slots(league_slots, patch_range, max_day=None):
    slots = [
        slot
        for slot in league_slots or []
        if day_in_patch_range(str(slot.get("day") or ""), patch_range, max_day=max_day)
    ]
    return sorted(slots, key=lambda slot: (slot.get("day") or "", slot.get("sequence", 0)))


def normalize_league_series_patch_dates(matches, team_leagues, schedule_dates, max_day=None):
    patch_ranges = schedule_dates.get("patchVersionRanges") or {}
    league_slots = schedule_dates.get("leagueSlots") or []
    tournament_match_ids = {
        int_value(replay_id)
        for replay_id in (schedule_dates.get("tournamentMatchIds") or [])
    }
    tournament_match_ids.discard(None)
    if not matches or not patch_ranges or not league_slots:
        return {"checkedSeries": 0, "dateCorrected": 0, "versionNormalized": 0, "patchRanges": len(patch_ranges)}

    league_groups = defaultdict(list)
    for match in sorted(matches, key=lambda row: NumberLike(row.get("id"))):
        replay_id = int_value(match.get("id"))
        if replay_id in tournament_match_ids or match.get("dateSource") == "tournament_competition_schedule":
            continue
        blue = team_leagues.get(str(match.get("blueTeamId")))
        red = team_leagues.get(str(match.get("redTeamId")))
        if blue and red and blue.get("key") == red.get("key"):
            league_groups[blue["key"]].append(match)

    slots_by_version = {
        version: patch_range_slots(league_slots, patch_range, max_day=max_day)
        for version, patch_range in patch_ranges.items()
    }
    slot_cursor = defaultdict(int)
    checked_series = 0
    date_corrected = 0
    version_normalized = 0

    for league_key, rows in league_groups.items():
        for group in clustered_league_series_groups(rows):
            checked_series += 1
            version = group_series_version(group["rows"])
            if not version or version not in patch_ranges:
                continue
            patch_range = patch_ranges[version]
            exact_days = [
                valid_match_day(row)
                for row in group["rows"]
                if row.get("dateSource") == "team_news_match_report"
            ]
            exact_days = [day for day in exact_days if day]
            if exact_days and all(day_in_patch_range(day, patch_range, max_day=max_day) for day in exact_days):
                continue

            needs_date_correction = False
            for row in group["rows"]:
                replay_id = int_value(row.get("id"))
                if replay_id in tournament_match_ids or row.get("dateSource") == "tournament_competition_schedule":
                    continue
                if row.get("dateSource") == "team_news_match_report":
                    continue
                day = valid_match_day(row)
                if not day_in_patch_range(day, patch_range, max_day=max_day):
                    needs_date_correction = True
                    break
            if not needs_date_correction:
                continue

            slots = slots_by_version.get(version) or []
            if not slots:
                continue
            cursor_key = (league_key, version)
            slot_index = min(slot_cursor[cursor_key], len(slots) - 1)
            slot_cursor[cursor_key] += 1
            slot = slots[slot_index]
            slot_day = slot.get("day")
            if not slot_day:
                continue

            for row in group["rows"]:
                replay_id = int_value(row.get("id"))
                if replay_id in tournament_match_ids or row.get("dateSource") == "tournament_competition_schedule":
                    continue
                if row.get("dateSource") == "team_news_match_report":
                    continue
                old_version = str(row.get("version") or "")
                if old_version and old_version != version:
                    row["version"] = version
                    row["versionSource"] = "series_patch_range"
                    row["versionInference"] = {
                        "previous": old_version,
                        "leagueKey": league_key,
                        "seriesPair": list(group["pair"]) if isinstance(group.get("pair"), tuple) else group.get("pair"),
                    }
                    version_normalized += 1
                old_day = valid_match_day(row)
                if old_day == slot_day:
                    continue
                if assign_match_date(
                    row,
                    slot_day,
                    "year_schedule_patch_range",
                    "estimated",
                    {
                        "leagueKey": league_key,
                        "patchVersion": version,
                        "patchStart": patch_range.get("start"),
                        "patchEnd": patch_range.get("end"),
                        "previousDay": old_day,
                        "previousSource": row.get("dateSource"),
                        "seriesPair": list(group["pair"]) if isinstance(group.get("pair"), tuple) else group.get("pair"),
                        "seriesReplayIds": [NumberLike(item.get("id")) for item in group["rows"]],
                        "scheduleKind": slot.get("kind"),
                        "scheduleRound": slot.get("round"),
                        "scheduleIndex": slot.get("index"),
                        "scheduleSequence": slot.get("sequence"),
                    },
                ):
                    date_corrected += 1

    return {
        "checkedSeries": checked_series,
        "dateCorrected": date_corrected,
        "versionNormalized": version_normalized,
        "patchRanges": len(patch_ranges),
        "tournamentSkipped": len(tournament_match_ids),
    }


def NumberLike(value):
    try:
        return int(value)
    except (TypeError, ValueError):
        return 0


def extract_news_champion_stats(blob: bytes, champion_ids):
    text = readable_text(blob)
    champ = "|".join(sorted(map(re.escape, champion_ids), key=len, reverse=True))
    pattern = re.compile(
        rf"Position\t(?P<position>[a-z_]+)\tChampion\t(?P<champion>{champ})\t"
        rf"PickCount\t(?P<pick>\d+)\tWinRate\t(?P<rate>\d+)\t"
        rf"ChampionStats\t(?P=champion)\|(?P<games>\d+)\|(?P<wins>\d+)\|(?P<rate2>\d+)"
    )
    seen = set()
    rows = []
    for match in pattern.finditer(text):
        row = {
            "position": match.group("position"),
            "champion": match.group("champion"),
            "pickCount": int(match.group("games")),
            "wins": int(match.group("wins")),
            "winRate": int(match.group("rate2")),
            "source": "save_news_meta_report",
        }
        key = tuple(row.items())
        if key not in seen:
            seen.add(key)
            rows.append(row)
    return rows


def valid_token(raw: bytes):
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError:
        return None
    if not text:
        return None
    if all(32 <= ord(ch) < 127 for ch in text):
        return text
    return None


def scan_length_prefixed_tokens(blob: bytes, champion_ids):
    champion_ids = set(champion_ids)
    tokens = []
    i = 0
    end = len(blob) - 8
    while i <= end:
        length = struct.unpack_from("<Q", blob, i)[0]
        if 1 <= length <= 64 and i + 8 + length <= len(blob):
            token = valid_token(blob[i + 8 : i + 8 + length])
            if token and (token in champion_ids or DATE_VERSION_RE.match(token)):
                tokens.append((i, token))
                i += 8 + length
                continue
        i += 1
    return tokens


def extract_draft_like_groups(blob: bytes, champion_ids):
    tokens = scan_length_prefixed_tokens(blob, champion_ids)
    champion_ids = set(champion_ids)
    groups = []
    for idx, (offset, token) in enumerate(tokens):
        if not DATE_VERSION_RE.match(token):
            continue
        group = []
        last_end = offset + 8 + len(token)
        for next_offset, next_token in tokens[idx + 1 : idx + 20]:
            if DATE_VERSION_RE.match(next_token):
                break
            if next_token not in champion_ids:
                continue
            if next_offset - last_end > 96:
                break
            group.append(next_token)
            last_end = next_offset + 8 + len(next_token)
            if len(group) >= 10:
                break
        if len(group) >= 5:
            groups.append(group)

    mentions = Counter()
    pairs = defaultdict(Counter)
    for group in groups:
        unique = list(dict.fromkeys(group))
        for champ in unique:
            mentions[champ] += 1
        for champ in unique:
            for other in unique:
                if champ != other:
                    pairs[champ][other] += 1

    return {
        "groups": len(groups),
        "mentions": dict(mentions),
        "pairs": {
            champ: [
                {"champion": other, "count": count}
                for other, count in counter.most_common(8)
            ]
            for champ, counter in pairs.items()
        },
    }


def parse_champion_stats_block(text, champion_ids, total_match, source_version=None):
    parsed = {}

    def balanced_block(start):
        open_at = text.find("{", start)
        block, _ = read_balanced(text, open_at)
        return block

    def balanced_sub_block(body, start):
        open_at = body.find("{", start)
        block, _ = read_balanced(body, open_at)
        return block

    for champ in champion_ids:
        marker = f'"{champ}": ChampionSeasonStatistics'
        pos = text.find(marker)
        if pos < 0:
            continue
        body = balanced_block(pos)
        if not body:
            continue

        ban = parse_first_int(body, "bans")
        totals = Counter()
        by_position = {}
        for position_match in re.finditer(r"\b(Top|Jungle|Mid|Bottom|Support):\s+ChampionStatistics\s+\{", body):
            position = normalize_position(position_match.group(1))
            block = balanced_sub_block(body, position_match.start())
            if not block:
                continue
            row = {
                "wins": parse_first_int(block, "wins") or 0,
                "matches": parse_first_int(block, "matches") or 0,
                "dealing": parse_first_int(block, "dealing") or 0,
                "tanking": parse_first_int(block, "tanking") or 0,
                "healing": parse_first_int(block, "healing") or 0,
                "kills": parse_first_int(block, "kills") or 0,
                "deaths": parse_first_int(block, "deaths") or 0,
                "cs": parse_first_int(block, "cs") or 0,
                "gold": parse_first_int(block, "gold") or 0,
                "dealingLinePhase": parse_first_int(block, "dealing_line_phase") or 0,
                "tankingLinePhase": parse_first_int(block, "tanking_line_phase") or 0,
                "healingLinePhase": parse_first_int(block, "healing_line_phase") or 0,
                "goldLinePhase": parse_first_int(block, "gold_line_phase") or 0,
                "csLinePhase": parse_first_int(block, "cs_line_phase") or 0,
            }
            by_position[position] = row
            for key, value in row.items():
                totals[key] += value

        matches = totals["matches"]
        wins = totals["wins"]
        if matches or ban is not None:
            parsed[champ] = {
                "pickCount": matches,
                "banCount": ban,
                "wins": wins,
                "losses": max(0, matches - wins),
                "dealt": totals["dealing"],
                "taken": totals["tanking"],
                "healing": totals["healing"],
                "kills": totals["kills"],
                "deaths": totals["deaths"],
                "cs": totals["cs"],
                "gold": totals["gold"],
                "linePhase": {
                    "dealt": totals["dealingLinePhase"],
                    "taken": totals["tankingLinePhase"],
                    "healing": totals["healingLinePhase"],
                    "gold": totals["goldLinePhase"],
                    "cs": totals["csLinePhase"],
                },
                "byPosition": by_position,
                "totalMatch": total_match,
                "version": source_version,
                "source": "meta_exporter_debug",
            }
    return parsed


def merge_champion_stats_versions(by_version):
    merged = {}
    total_match = 0

    for version_stats in by_version.values():
        if not version_stats:
            continue
        total_match += max((row.get("totalMatch") or 0 for row in version_stats.values()), default=0)

        for champ, row in version_stats.items():
            target = merged.setdefault(
                champ,
                {
                    "pickCount": 0,
                    "banCount": 0,
                    "wins": 0,
                    "losses": 0,
                    "dealt": 0,
                    "taken": 0,
                    "healing": 0,
                    "kills": 0,
                    "deaths": 0,
                    "assists": 0,
                    "cs": 0,
                    "gold": 0,
                    "durationSec": 0,
                    "rating": 0,
                    "level": 0,
                    "linePhase": {"dealt": 0, "taken": 0, "healing": 0, "gold": 0, "cs": 0},
                    "byPosition": {},
                    "itemCounts": {},
                    "topItems": [],
                    "version": "all",
                    "source": row.get("source") or "meta_exporter_debug",
                },
            )

            for key in (
                "pickCount",
                "banCount",
                "wins",
                "losses",
                "dealt",
                "taken",
                "healing",
                "kills",
                "deaths",
                "assists",
                "cs",
                "gold",
                "durationSec",
                "rating",
                "level",
            ):
                target[key] += row.get(key) or 0

            for key, value in (row.get("linePhase") or {}).items():
                target["linePhase"][key] = target["linePhase"].get(key, 0) + (value or 0)

            for position, position_row in (row.get("byPosition") or {}).items():
                position_target = target["byPosition"].setdefault(position, {})
                for key, value in position_row.items():
                    if isinstance(value, (int, float)):
                        position_target[key] = position_target.get(key, 0) + value

            for item_id, count in (row.get("itemCounts") or {}).items():
                target["itemCounts"][item_id] = target["itemCounts"].get(item_id, 0) + (count or 0)

    for row in merged.values():
        row["totalMatch"] = total_match
        if row["itemCounts"]:
            row["topItems"] = [
                {"itemId": item_id, "count": count}
                for item_id, count in sorted(row["itemCounts"].items(), key=lambda item: (-item[1], item[0]))[:12]
            ]
        else:
            row.pop("itemCounts", None)
            row.pop("topItems", None)
    return merged


def parse_debug_champion_stats_versions(path: Path, champion_ids):
    if not path.exists():
        return {}, {}
    text = path.read_text(encoding="utf-8", errors="ignore")
    by_version = {}
    for match in re.finditer(r'"([^"]+)":\s+ChampionPatchStatistics\s+\{', text):
        version = match.group(1)
        open_at = text.find("{", match.start())
        body, _ = read_balanced(text, open_at)
        if not body:
            continue
        by_version[version] = parse_champion_stats_block(
            body,
            champion_ids,
            parse_first_int(body, "total_match") or 0,
            version,
        )
    if by_version:
        return merge_champion_stats_versions(by_version), by_version
    total_match = parse_first_int(text, "total_match") or 0
    return parse_champion_stats_block(text, champion_ids, total_match), {}


def parse_debug_champion_stats(path: Path, champion_ids):
    latest, _ = parse_debug_champion_stats_versions(path, champion_ids)
    return latest


def parse_first_int(text, field):
    match = re.search(rf"\b{re.escape(field)}:\s*(-?\d+)", text)
    return int(match.group(1)) if match else None


def printable_tokens(blob):
    text = "".join(chr(x) if 32 <= x < 127 else " " for x in blob)
    return re.findall(r"[A-Za-z0-9_./:#?%+-]+", text)


def parse_num(token):
    token = token.strip().rstrip("%")
    try:
        return float(token)
    except ValueError:
        return None


def parse_debug_scalar(raw):
    value = str(raw or "").strip().rstrip(",")
    if value.startswith('"') and value.endswith('"'):
        return value[1:-1]
    if re.fullmatch(r"-?\d+", value):
        return int(value)
    if re.fullmatch(r"-?\d+\.\d+", value):
        return float(value)
    return value


def find_matching_brace(text, open_index):
    depth = 0
    in_string = False
    escaped = False
    for index in range(open_index, len(text)):
        char = text[index]
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return index
    return None


def extract_debug_struct_block(text, pattern):
    match = re.search(pattern, text, flags=re.MULTILINE)
    if not match:
        return None
    open_index = text.find("{", match.start())
    if open_index < 0:
        return None
    close_index = find_matching_brace(text, open_index)
    if close_index is None:
        return None
    return text[open_index : close_index + 1]


def extract_champion_debug_block(text, champion_id):
    return extract_debug_struct_block(
        text,
        rf"^\s*{re.escape(champion_id)}:\s+\w+ChampionInfo\s*\{{",
    )


def extract_debug_field_block(block, field):
    return extract_debug_struct_block(
        block,
        rf"^\s*{re.escape(field)}:\s+\w+\s*\{{",
    )


def parse_debug_struct_values(block):
    if not block:
        return {}
    values = {}
    depth = 0
    for line in block.splitlines():
        match = re.match(r"^\s*([A-Za-z_][A-Za-z0-9_]*):\s*([^,\n]+),?", line)
        if depth == 1 and match:
            raw_value = match.group(2).strip()
            if "{" not in raw_value and "[" not in raw_value:
                values[match.group(1)] = parse_debug_scalar(raw_value)
        depth += line.count("{")
        depth -= line.count("}")
    return values


def normalize_champion_debug_actions(raw_actions):
    actions = dict(raw_actions)
    if "skill1" in raw_actions:
        actions["skill"] = raw_actions["skill1"]
        if "skill2" in raw_actions:
            actions["skill2"] = raw_actions["skill2"]
        elif "skill" in raw_actions:
            actions["skill2"] = raw_actions["skill"]
    return actions


def load_champion_debug_values(path, champion_ids):
    if not path.exists():
        return {}
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return {}

    out = {}
    for champion_id in champion_ids:
        block = extract_champion_debug_block(text, champion_id)
        if not block:
            continue
        raw_actions = {}
        for field in ["attack", "skill", "skill1", "skill2", "ult"]:
            action_block = extract_debug_field_block(block, field)
            if action_block:
                raw_actions[field] = parse_debug_struct_values(action_block)
        out[champion_id] = {
            "stats": parse_debug_struct_values(extract_debug_field_block(block, "stat")),
            "growth": parse_debug_struct_values(extract_debug_field_block(block, "growth")),
            "actions": normalize_champion_debug_actions(raw_actions),
            "rawActions": raw_actions,
        }
    return out


def stat_number(value):
    if isinstance(value, (int, float)):
        return value
    parsed = parse_num(str(value))
    return parsed if parsed is not None else None


def format_compact_number(value, max_digits=2):
    number = stat_number(value)
    if number is None:
        return str(value)
    if abs(number - round(number)) < 1e-9:
        return str(int(round(number)))
    return f"{number:.{max_digits}f}".rstrip("0").rstrip(".")


def format_cooltime_ticks(ticks):
    number = stat_number(ticks)
    if number is None:
        return None
    return format_compact_number(number / 60.0, 2)


def camel_to_snake(name):
    text = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", str(name))
    text = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", text)
    return text.strip("_").lower()


def format_time_ticks(ticks):
    number = stat_number(ticks)
    if number is None:
        return None
    return f"{number / 60.0:.2f}"


def placeholder_value_from_action(name, action):
    if not action:
        return None, None
    key = camel_to_snake(name)
    candidates = [key, f"{key}_value", f"{key}_amount", f"{key}_ratio", f"{key}_coef"]
    for candidate in candidates:
        if candidate in action:
            return candidate, action[candidate]

    if key == "time":
        time_keys = [
            item
            for item in action
            if item != "duration" and (item.endswith("_duration") or item.endswith("_time"))
        ]
        if len(time_keys) == 1:
            return time_keys[0], action[time_keys[0]]
    return None, None


def format_placeholder_value(name, action_key, value):
    number = stat_number(value)
    if number is None:
        return None
    key = action_key or ""
    if key.endswith("_duration") or key.endswith("_time") or camel_to_snake(name) in {"time", "tick"}:
        return format_time_ticks(number)
    return format_compact_number(number)


def apply_action_placeholders_to_description(description, current_action):
    if not description or not current_action:
        return description

    def replace(match):
        name = match.group(1)
        action_key, value = placeholder_value_from_action(name, current_action)
        if action_key is None:
            return match.group(0)
        formatted = format_placeholder_value(name, action_key, value)
        return formatted if formatted is not None else match.group(0)

    return re.sub(r"\{([A-Za-z][A-Za-z0-9_]*)\}", replace, description)


def replace_value_text(text, old_value, new_value, suffix=""):
    old_number = stat_number(old_value)
    new_number = stat_number(new_value)
    if old_number is None or new_number is None:
        return text
    old_text = format_compact_number(old_number) + suffix
    new_text = format_compact_number(new_number) + suffix
    if old_text == new_text or old_text not in text:
        return text
    return text.replace(old_text, new_text, 1)


def apply_action_values_to_description(description, base_action, current_action):
    if not description or not base_action or not current_action:
        return description
    text = description
    for key, old_value in base_action.items():
        if key not in current_action:
            continue
        new_value = current_action[key]
        if stat_number(old_value) is None or stat_number(new_value) is None:
            continue
        if abs(float(stat_number(old_value)) - float(stat_number(new_value))) < 1e-9:
            continue
        if key.endswith("_ratio") or key in {"attack_ratio", "magic_ratio", "heal_ratio", "shield_ratio", "slow"}:
            next_text = replace_value_text(text, old_value, new_value, "%")
            if next_text != text:
                text = next_text
                continue
        text = replace_value_text(text, old_value, new_value)
    return apply_action_placeholders_to_description(text, current_action)


def apply_current_champion_info(champions, current_info, base_info):
    if not current_info:
        return 0
    by_id = {champ.get("id"): champ for champ in champions}
    updated = 0
    stat_map = {
        "attack": "attack",
        "magic_power": "magicPower",
        "hp": "hp",
        "defence": "defence",
        "magic_resistance": "magicResistance",
        "move_speed": "moveSpeed",
    }
    skill_id_map = {"skill": "skill", "skill2": "skill2", "ult": "ult"}
    for champion_id, current in current_info.items():
        champ = by_id.get(champion_id)
        if not champ:
            continue

        stats = champ.setdefault("stats", {})
        for raw_key, out_key in stat_map.items():
            value = stat_number(current.get("stats", {}).get(raw_key))
            if value is not None:
                stats[out_key] = value
                if raw_key == "move_speed":
                    stats["moveSpeedDisplay"] = round(value * 0.06, 2)

        growth = champ.setdefault("growth", {})
        for raw_key, out_key in stat_map.items():
            value = stat_number(current.get("growth", {}).get(raw_key))
            if value is not None:
                if raw_key == "move_speed":
                    growth["moveSpeedDisplay"] = round(value * 0.06, 2)
                else:
                    growth[out_key] = value

        current_actions = current.get("actions", {})
        base_actions = (base_info.get(champion_id) or {}).get("actions", {}) if base_info else {}
        skill_by_id = {skill.get("id"): skill for skill in champ.get("skills", [])}
        for action_key, skill_id in skill_id_map.items():
            skill = skill_by_id.get(skill_id)
            action = current_actions.get(action_key, {})
            if not skill or not action:
                continue
            cooltime = format_cooltime_ticks(action.get("cooltime"))
            if cooltime is not None:
                skill["cooltime"] = cooltime
            skill["description"] = apply_action_values_to_description(
                skill.get("description", ""),
                base_actions.get(action_key, {}),
                action,
            )
            if skill.get("descriptionEn"):
                skill["descriptionEn"] = apply_action_values_to_description(
                    skill.get("descriptionEn", ""),
                    base_actions.get(action_key, {}),
                    action,
                )

        champ["currentInfoSource"] = "save_probe_champion_info"
        updated += 1
    return updated


def pct_change(old, new):
    if old is None or new is None or abs(old) < 1e-9:
        return 0.0
    return round((new / old - 1.0) * 100.0, 2)


def add_patch_delta(patches, champ, field, delta):
    if champ is None or abs(delta) < 0.01:
        return
    patches.setdefault(champ, {})
    patches[champ][field] = round(patches[champ].get(field, 0.0) + delta, 2)


def field_from_patch_asset(asset_key):
    key = asset_key.split("?", 1)[-1]
    if key == "stat.attack":
        return "attack", 1.0
    if key == "stat.magic_power":
        return "magicPower", 1.0
    if key == "stat.hp":
        return "hp", 1.0
    if key == "stat.defence":
        return "defence", 1.0
    if key == "stat.magic_resistance":
        return "magicResistance", 1.0
    if key == "stat.move_speed":
        return "moveSpeed", 1.0
    if key == "stat.attack_speed":
        return "cooldown", -1.0
    if key in {"patch_key.attack_coef", "patch_key.ap_coef", "patch_key.damage"}:
        return "damage", 1.0
    if key in {"patch_key.growth_attack", "patch_key.growth_magic_power"}:
        return "damage", 0.45
    if key == "patch_key.growth_hp":
        return "hp", 0.45
    if key == "patch_key.growth_defence":
        return "defence", 0.45
    if key == "patch_key.growth_magic_resistance":
        return "magicResistance", 0.45
    if key == "patch_key.cooltime":
        return "cooldown", 1.0
    if key in {"stat.range", "patch_key.hit_box", "patch_key.move_range"}:
        return "range", 1.0
    if key in {
        "patch_key.stun",
        "patch_key.airbone",
        "patch_key.bind",
        "patch_key.slow_time",
        "patch_key.slow_ratio",
        "patch_key.shield",
        "patch_key.heal",
        "patch_key.buff_time",
    }:
        return "utility", 1.0
    return None, 0.0


def extract_patch_blocks(payload):
    blocks = []
    stop_markers = [
        b"#asset/base/text/news?patch.title",
        b"#asset/base/text/news?solo_rank_report.title",
        b"#asset/base/text/news?article.",
    ]
    for match in re.finditer(rb"20\d\d\.\d+\.\d+", payload):
        start = max(0, match.start() - 80)
        end = min(len(payload), match.start() + 10000)
        for marker in stop_markers:
            marker_index = payload.find(marker, match.start() + 20, end)
            if marker_index != -1:
                end = min(end, marker_index)
        block = payload[start:end]
        if b"#asset/base/text/champion?" in block:
            blocks.append((match.group(0).decode("ascii"), match.start(), block))
    return blocks


def parse_patch_block(block, champion_ids, version=None, offset=None):
    tokens = printable_tokens(block)
    patches = {}
    changes = []
    current_champ = None
    i = 0
    champion_id_set = set(champion_ids)
    while i < len(tokens):
        token = tokens[i]
        if token in champion_id_set:
            current_champ = token
            i += 1
            continue
        if token.startswith("#asset/base/text/champion?"):
            field, weight = field_from_patch_asset(token)
            j = i + 1
            target_tokens = []
            while j < len(tokens) and parse_num(tokens[j]) is None:
                if tokens[j] in champion_id_set or tokens[j].startswith("#asset/"):
                    break
                target_tokens.append(tokens[j])
                j += 1
            if j + 1 < len(tokens):
                old = parse_num(tokens[j])
                new = parse_num(tokens[j + 1])
                if old is not None and new is not None and field:
                    target = next(
                        (
                            item
                            for item in target_tokens
                            if item in {"base_attack", "skill", "skill1", "skill2", "ult"}
                        ),
                        None,
                    )
                    delta = pct_change(old, new) * weight
                    add_patch_delta(patches, current_champ, field, delta)
                    changes.append(
                        {
                            "version": version,
                            "versionOffset": offset,
                            "champion": current_champ,
                            "asset": token.split("?", 1)[-1],
                            "target": target,
                            "field": field,
                            "old": old,
                            "new": new,
                            "delta": round(delta, 2),
                        }
                    )
                    i = j + 2
                    continue
        i += 1
    return patches, changes


def merge_patch_dicts(dicts):
    merged = {}
    for patch in dicts:
        for champ, fields in patch.items():
            for field, value in fields.items():
                add_patch_delta(merged, champ, field, value)
    return merged


def extract_current_patch_summary(payload, champion_ids, save_path):
    parsed = []
    parsed_by_version = {}
    all_changes = []
    changes_by_version = defaultdict(list)
    versions = []
    for version, offset, block in extract_patch_blocks(payload):
        patches, changes = parse_patch_block(block, champion_ids, version, offset)
        if changes:
            parsed.append(patches)
            parsed_by_version.setdefault(version, []).append(patches)
            all_changes.extend(changes)
            changes_by_version[version].extend(changes)
            versions.append({"version": version, "offset": offset, "changes": len(changes)})
    return {
        "meta": {
            "source": str(save_path) if save_path else None,
            "versions": versions,
            "changeCount": len(all_changes),
        },
        "patches": merge_patch_dicts(parsed),
        "patchesByVersion": {
            version: merge_patch_dicts(rows)
            for version, rows in parsed_by_version.items()
        },
        "changes": all_changes,
        "changesByVersion": dict(changes_by_version),
    }


def read_balanced(text, open_index, open_char="{", close_char="}"):
    if open_index < 0 or open_index >= len(text) or text[open_index] != open_char:
        return "", -1
    depth = 0
    for idx in range(open_index, len(text)):
        char = text[idx]
        if char == open_char:
            depth += 1
        elif char == close_char:
            depth -= 1
            if depth == 0:
                return text[open_index : idx + 1], idx
    return "", -1


def extract_named_array(body, name):
    marker = f"{name}: ["
    start = body.find(marker)
    if start < 0:
        return ""
    open_at = body.find("[", start)
    block, _ = read_balanced(body, open_at, "[", "]")
    return block[1:-1] if block else ""


def extract_named_struct(body, name, struct_name):
    marker = f"{name}: {struct_name} {{"
    start = body.find(marker)
    if start < 0:
        return ""
    open_at = body.find("{", start)
    block, _ = read_balanced(body, open_at)
    return block if block else ""


def parse_int_array(body, field):
    match = re.search(rf"\b{re.escape(field)}:\s*\[([^\]]*)\]", body)
    if not match:
        return []
    return [int(value) for value in re.findall(r"-?\d+", match.group(1))]


def parse_quoted_array(body, field):
    match = re.search(rf"\b{re.escape(field)}:\s*\[([^\]]*)\]", body)
    if not match:
        return []
    return re.findall(r'"([^"]+)"', match.group(1))


def split_struct_blocks(text, struct_name):
    blocks = []
    search = f"{struct_name} {{"
    offset = 0
    while True:
        start = text.find(search, offset)
        if start < 0:
            break
        open_at = text.find("{", start)
        block, end = read_balanced(text, open_at)
        if not block:
            break
        blocks.append(block)
        offset = end + 1
    return blocks


def parse_bool(body, field):
    match = re.search(rf"\b{re.escape(field)}:\s*(true|false)", body)
    return match.group(1) == "true" if match else None


def parse_quoted_field(body, field):
    match = re.search(rf"\b{re.escape(field)}:\s*\"([^\"]+)\"", body)
    return match.group(1) if match else None


def parse_datetime_field(body, field):
    match = re.search(rf"\b{re.escape(field)}:\s*([0-9]{{4}}-[0-9]{{2}}-[0-9]{{2}}T[0-9]{{2}}:[0-9]{{2}}:[0-9]{{2}})", body)
    return match.group(1) if match else None


def date_key_from_datetime(value):
    text = str(value or "")
    return text[:10] if re.match(r"^\d{4}-\d{2}-\d{2}", text) else "unknown"


def parse_version(body):
    return parse_quoted_field(body, "version") or "unknown"


def normalize_position(position):
    if not position:
        return None
    value = position.lower()
    return "bot" if value == "bottom" else value


def parse_position_enum(body):
    match = re.search(r"\bposition:\s*(Top|Jungle|Mid|Bottom|Support)", body)
    return normalize_position(match.group(1)) if match else None


def parse_position_from_stats(body):
    scores = {}
    for pos in POSITION_NAMES:
        value = parse_first_int(body, POSITION_FIELD_NAMES[pos])
        scores[pos] = value if value is not None else 0
    best, score = max(scores.items(), key=lambda item: item[1])
    return best if score > 0 else None


def blank_stat():
    return {
        "pickCount": 0,
        "banCount": 0,
        "wins": 0,
        "losses": 0,
        "dealt": 0,
        "taken": 0,
        "healing": 0,
        "kills": 0,
        "deaths": 0,
        "assists": 0,
        "cs": 0,
        "gold": 0,
        "durationSec": 0,
        "rating": 0,
        "level": 0,
        "itemCounts": Counter(),
        "byPosition": defaultdict(lambda: Counter()),
        "linePhase": Counter(),
        "source": "not_collected",
    }


def add_player_stat(stats, champion, won, player, source):
    row = stats[champion]
    row["pickCount"] += 1
    row["wins"] += 1 if won else 0
    row["losses"] += 0 if won else 1
    row["kills"] += player.get("kills", 0)
    row["deaths"] += player.get("deaths", 0)
    row["assists"] += player.get("assists", 0)
    row["cs"] += player.get("cs", 0)
    row["dealt"] += player.get("dealt", 0)
    row["taken"] += player.get("taken", 0)
    row["healing"] += player.get("healing", 0)
    row["rating"] += player.get("rating", 0)
    row["level"] += player.get("level", 0)
    row["gold"] += player.get("gold", 0)
    row["durationSec"] += player.get("durationSec", 0)
    line_gold = player.get("lineGold", 0)
    line_cs = player.get("lineCs", 0)
    if line_gold:
        row["linePhase"]["gold"] += line_gold
    if line_cs:
        row["linePhase"]["cs"] += line_cs
    for item in player.get("items", []):
        key = item_summary_key(item)
        if key:
            row["itemCounts"][key] += 1
    if player.get("position"):
        pos = row["byPosition"][player["position"]]
        pos["matches"] += 1
        pos["wins"] += 1 if won else 0
        pos["dealing"] += player.get("dealt", 0)
        pos["tanking"] += player.get("taken", 0)
        pos["healing"] += player.get("healing", 0)
        pos["kills"] += player.get("kills", 0)
        pos["deaths"] += player.get("deaths", 0)
        pos["assists"] += player.get("assists", 0)
        pos["cs"] += player.get("cs", 0)
        pos["rating"] += player.get("rating", 0)
        pos["level"] += player.get("level", 0)
        pos["gold"] += player.get("gold", 0)
        pos["durationSec"] += player.get("durationSec", 0)
        if line_gold:
            pos["goldLinePhase"] += line_gold
        if line_cs:
            pos["csLinePhase"] += line_cs
    row["source"] = source


def finalize_aggregated_stats(stats, total_match, source, item_catalog=None):
    finalized = {}
    for champion, row in stats.items():
        matches = row["pickCount"]
        wins = row["wins"]
        losses = row["losses"]
        out = {
            "pickCount": matches,
            "banCount": row.get("banCount", 0),
            "wins": wins,
            "losses": losses,
            "winRate": round(wins / matches * 100, 1) if matches else None,
            "pickRate": round(matches / total_match * 100, 1) if total_match else None,
            "banRate": round(row.get("banCount", 0) / total_match * 100, 1) if total_match else None,
            "banPickRate": round((matches + row.get("banCount", 0)) / total_match * 100, 1) if total_match else None,
            "dealt": row["dealt"],
            "taken": row["taken"],
            "healing": row["healing"],
            "kills": row["kills"],
            "deaths": row["deaths"],
            "assists": row["assists"],
            "cs": row["cs"],
            "gold": row["gold"],
            "durationSec": row["durationSec"],
            "rating": row["rating"],
            "level": row["level"],
            "itemCounts": dict(row["itemCounts"]) if row.get("itemCounts") else None,
            "topItems": item_top_list(row.get("itemCounts"), item_catalog),
            "linePhase": dict(row["linePhase"]) if row["linePhase"] else None,
            "byPosition": {pos: dict(values) for pos, values in row["byPosition"].items()},
            "totalMatch": total_match,
            "source": source,
            "confidence": "exported",
        }
        finalized[champion] = out
    return finalized


def parse_solo_rank_stats(path: Path, champion_ids, item_catalog=None):
    if not path.exists():
        return {}, {"groups": 0, "pairs": {}, "counters": {}}, {}, {}, empty_lane_synergy_payload(), {}
    text = path.read_text(encoding="utf-8", errors="ignore")
    champion_ids = set(champion_ids)
    stats = defaultdict(blank_stat)
    stats_by_version = defaultdict(lambda: defaultdict(blank_stat))
    relations = RelationAccumulator()
    relations_by_version = defaultdict(RelationAccumulator)
    lane_synergies = LanePairAccumulator()
    lane_synergies_by_version = defaultdict(LanePairAccumulator)
    total_matches = 0
    total_matches_by_version = Counter()

    offset = 0
    while True:
        start = text.find("SoloRankMatch {", offset)
        if start < 0:
            break
        open_at = text.find("{", start)
        block, end = read_balanced(text, open_at)
        if not block:
            break
        offset = end + 1
        if "played: true" not in block:
            continue
        blue_win = parse_bool(block, "blue_team_win")
        if blue_win is None:
            continue
        blue_players = parse_solo_team(extract_named_array(block, "blue_team"), champion_ids, item_catalog)
        red_players = parse_solo_team(extract_named_array(block, "red_team"), champion_ids, item_catalog)
        if not blue_players or not red_players:
            continue
        version = parse_version(block)
        total_matches += 1
        total_matches_by_version[version] += 1
        for player in blue_players:
            add_player_stat(stats, player["champion"], blue_win, player, "solo_rank_export")
            add_player_stat(stats_by_version[version], player["champion"], blue_win, player, "solo_rank_export")
        for player in red_players:
            add_player_stat(stats, player["champion"], not blue_win, player, "solo_rank_export")
            add_player_stat(stats_by_version[version], player["champion"], not blue_win, player, "solo_rank_export")
        relations.record([p["champion"] for p in blue_players], [p["champion"] for p in red_players], blue_win)
        relations_by_version[version].record([p["champion"] for p in blue_players], [p["champion"] for p in red_players], blue_win)
        lane_synergies.record(blue_players, blue_win)
        lane_synergies.record(red_players, not blue_win)
        lane_synergies_by_version[version].record(blue_players, blue_win)
        lane_synergies_by_version[version].record(red_players, not blue_win)

    version_stats = {
        version: finalize_aggregated_stats(rows, total_matches_by_version[version], "solo_rank_export", item_catalog)
        for version, rows in stats_by_version.items()
    }
    return (
        finalize_aggregated_stats(stats, total_matches, "solo_rank_export", item_catalog),
        relations.to_payload(),
        version_stats,
        {version: rel.to_payload() for version, rel in relations_by_version.items()},
        lane_synergies.to_payload(),
        {version: rel.to_payload() for version, rel in lane_synergies_by_version.items()},
    )


def parse_solo_team(team_text, champion_ids, item_catalog=None, save_lookup=None):
    players = []
    athlete_names = (save_lookup or {}).get("athletes", {})
    for slot_index, block in enumerate(split_struct_blocks(team_text, "SoloRankAthlete")):
        champion = parse_quoted_field(block, "champion")
        if champion not in champion_ids:
            continue
        athlete_id = parse_first_int(block, "athlete_id")
        stat_position = parse_position_from_stats(block)
        position = POSITION_NAMES[slot_index] if slot_index < len(POSITION_NAMES) else stat_position
        item_icons = parse_quoted_array(block, "items")
        item_details = with_item_order(describe_item_icons(item_icons, item_catalog))
        cs = parse_first_int(block, "cs") or 0
        players.append(
            {
                "champion": champion,
                "position": position,
                "statPosition": stat_position,
                "slot": slot_index,
                "athleteId": athlete_id,
                "name": athlete_names.get(str(athlete_id), f"선수 #{athlete_id}" if athlete_id is not None else "선수"),
                "kills": parse_first_int(block, "kill") or 0,
                "deaths": parse_first_int(block, "death") or 0,
                "assists": parse_first_int(block, "assist") or 0,
                "cs": cs,
                "lineCs": cs,
                "level": parse_first_int(block, "level") or 0,
                "dealt": parse_first_int(block, "dealing") or 0,
                "healing": parse_first_int(block, "healing") or 0,
                "taken": parse_first_int(block, "tanking") or 0,
                "rating": parse_first_int(block, "rating") or 0,
                "itemIcons": item_icons,
                "itemIds": [item["id"] for item in item_details if item.get("id") is not None],
                "itemNames": [item["name"] for item in item_details if item.get("name")],
                "items": item_details,
                "itemSource": "solo_rank_match_items",
            }
        )
    order = {role: index for index, role in enumerate(POSITION_NAMES)}
    players.sort(key=lambda row: order.get(row.get("position"), 99))
    return players


def compact_solo_team(players):
    return {
        "gold": 0,
        "killsTotal": sum(player.get("kills", 0) for player in players),
        "deathsTotal": sum(player.get("deaths", 0) for player in players),
        "epic": 0,
        "serpen": 0,
        "firstEpic": False,
        "firstSerpen": False,
        "lineGoldTotal": 0,
        "lineCsTotal": sum(player.get("cs", 0) for player in players),
        "dealtTotal": sum(player.get("dealt", 0) for player in players),
        "healingTotal": sum(player.get("healing", 0) for player in players),
        "tankingTotal": sum(player.get("taken", 0) for player in players),
    }


def parse_solo_match_analysis(path: Path, champion_ids, save_lookup, limit=None, item_catalog=None):
    if not path.exists():
        return []
    text = path.read_text(encoding="utf-8", errors="ignore")
    champion_ids = set(champion_ids)
    matches = []
    offset = 0
    while True:
        start = text.find("SoloRankMatch {", offset)
        if start < 0:
            break
        open_at = text.find("{", start)
        block, end = read_balanced(text, open_at)
        if not block:
            break
        offset = end + 1
        if "played: true" not in block:
            continue
        replay_id = parse_first_int(block, "id")
        blue_win = parse_bool(block, "blue_team_win")
        if replay_id is None or blue_win is None:
            continue
        blue_players = parse_solo_team(extract_named_array(block, "blue_team"), champion_ids, item_catalog, save_lookup)
        red_players = parse_solo_team(extract_named_array(block, "red_team"), champion_ids, item_catalog, save_lookup)
        if not blue_players or not red_players:
            continue
        start_time = parse_datetime_field(block, "date")
        result_time = parse_datetime_field(block, "result_time")
        date_value = result_time or start_time
        date_key = date_key_from_datetime(date_value)
        region_id = parse_first_int(block, "region_id")
        blue = compact_solo_team(blue_players)
        red = compact_solo_team(red_players)
        blue.update({"name": "Solo Rank Blue", "bans": [], "players": blue_players})
        red.update({"name": "Solo Rank Red", "bans": [], "players": red_players})
        matches.append(
            {
                "id": f"solo-{replay_id}",
                "source": "solo",
                "version": parse_version(block),
                "date": date_value,
                "dateKey": date_key,
                "dateLabel": date_key if date_key != "unknown" else "date not exported",
                "dateSource": "solo_rank_matches.debug.txt",
                "dateConfidence": "stored",
                "startTime": start_time,
                "resultTime": result_time,
                "regionId": region_id,
                "region": solo_region_meta(region_id),
                "gameTick": 0,
                "durationSec": 0,
                "winner": "blue" if blue_win else "red",
                "blue": blue,
                "red": red,
            }
        )
    matches.sort(key=lambda row: (row.get("resultTime") or row.get("date") or "", NumberLike(str(row.get("id", "")).split("-")[-1])), reverse=True)
    return matches if limit is None else matches[:limit]


def parse_match_replay_relations(path: Path, champion_ids, excluded_replay_ids=None):
    if not path.exists():
        return {"groups": 0, "pairs": {}, "counters": {}}, {}, empty_lane_synergy_payload(), {}
    text = path.read_text(encoding="utf-8", errors="ignore")
    champion_ids = set(champion_ids)
    relations = RelationAccumulator()
    relations_by_version = defaultdict(RelationAccumulator)
    lane_synergies = LanePairAccumulator()
    lane_synergies_by_version = defaultdict(LanePairAccumulator)

    offset = 0
    while True:
        start = text.find("MatchReplayData {", offset)
        if start < 0:
            break
        open_at = text.find("{", start)
        block, end = read_balanced(text, open_at)
        if not block:
            break
        offset = end + 1
        replay_id = parse_first_int(block, "id")
        blue_win = parse_bool(block, "blue_team_win")
        if blue_win is None:
            continue
        version = parse_version(block)
        blue = parse_replay_team(extract_named_array(block, "blue_team"), champion_ids)
        red = parse_replay_team(extract_named_array(block, "red_team"), champion_ids)
        if blue and red:
            relations.record([p["champion"] for p in blue], [p["champion"] for p in red], blue_win)
            relations_by_version[version].record([p["champion"] for p in blue], [p["champion"] for p in red], blue_win)
            lane_synergies.record(blue, blue_win)
            lane_synergies.record(red, not blue_win)
            lane_synergies_by_version[version].record(blue, blue_win)
            lane_synergies_by_version[version].record(red, not blue_win)
    return (
        relations.to_payload(),
        {version: rel.to_payload() for version, rel in relations_by_version.items()},
        lane_synergies.to_payload(),
        {version: rel.to_payload() for version, rel in lane_synergies_by_version.items()},
    )


def parse_replay_team(team_text, champion_ids):
    players = []
    for block in split_struct_blocks(team_text, "MatchReplayAthlete"):
        champion = parse_quoted_field(block, "champion")
        if champion in champion_ids:
            players.append({"champion": champion, "position": parse_position_enum(block)})
    return players


def array_at(values, index):
    if index is None or index < 0 or index >= len(values):
        return 0
    return values[index] or 0


def parse_match_performance(block):
    if not block:
        return {
            "gold": 0,
            "killsTotal": 0,
            "deathsTotal": 0,
            "epic": 0,
            "serpen": 0,
            "firstEpic": False,
            "firstSerpen": False,
            "kills": [],
            "deaths": [],
            "deals": [],
            "lineGold": [],
            "lineCs": [],
        }
    return {
        "gold": parse_first_int(block, "total_gold") or 0,
        "killsTotal": parse_first_int(block, "total_kills") or 0,
        "deathsTotal": parse_first_int(block, "total_deaths") or 0,
        "epic": parse_first_int(block, "epic_secured") or 0,
        "serpen": parse_first_int(block, "serpen_secured") or 0,
        "firstEpic": parse_bool(block, "first_epic") or False,
        "firstSerpen": parse_bool(block, "first_serpen") or False,
        "kills": parse_int_array(block, "kills"),
        "deaths": parse_int_array(block, "deaths"),
        "deals": parse_int_array(block, "deal"),
        "lineGold": parse_int_array(block, "gold_line_phase"),
        "lineCs": parse_int_array(block, "cs_line_phase"),
    }


def compact_performance(perf):
    return {
        "gold": perf["gold"],
        "killsTotal": perf["killsTotal"],
        "deathsTotal": perf["deathsTotal"],
        "epic": perf["epic"],
        "serpen": perf["serpen"],
        "firstEpic": perf["firstEpic"],
        "firstSerpen": perf["firstSerpen"],
        "lineGoldTotal": sum(perf["lineGold"]),
        "lineCsTotal": sum(perf["lineCs"]),
        "dealtTotal": sum(perf["deals"]),
    }


def parse_match_team_details(team_text, champion_ids, perf, save_lookup, item_catalog=None):
    players = []
    athlete_names = save_lookup.get("athletes", {})
    for block in split_struct_blocks(team_text, "MatchReplayAthlete"):
        champion = parse_quoted_field(block, "champion")
        if champion not in champion_ids:
            continue
        slot = parse_first_int(block, "id")
        athlete_id = parse_first_int(block, "athlete_id")
        perf_index = slot % 5 if slot is not None else len(players)
        item_ids = parse_int_array(block, "items")
        item_details = with_item_order(describe_item_ids(item_ids, item_catalog))
        line_gold = array_at(perf["lineGold"], perf_index)
        line_cs = array_at(perf["lineCs"], perf_index)
        players.append(
            {
                "champion": champion,
                "position": parse_position_enum(block),
                "slot": slot,
                "athleteId": athlete_id,
                "name": athlete_names.get(str(athlete_id), f"athlete #{athlete_id}" if athlete_id is not None else "athlete"),
                "kills": array_at(perf["kills"], perf_index),
                "deaths": array_at(perf["deaths"], perf_index),
                "dealt": array_at(perf["deals"], perf_index),
                "gold": line_gold,
                "cs": line_cs,
                "lineGold": line_gold,
                "lineCs": line_cs,
                "itemIds": item_ids,
                "itemIcons": [item["icon"] for item in item_details if item.get("icon")],
                "itemNames": [item["name"] for item in item_details if item.get("name")],
                "items": item_details,
                "itemSource": "saved_replay_items",
            }
        )
    order = {role: index for index, role in enumerate(POSITION_NAMES)}
    players.sort(key=lambda row: order.get(row.get("position"), 99))
    return players


def parse_match_analysis(path: Path, champion_ids, save_lookup, solo_replay_ids=None, limit=600, item_catalog=None):
    if not path.exists():
        return []
    text = path.read_text(encoding="utf-8", errors="ignore")
    champion_ids = set(champion_ids)
    team_names = save_lookup.get("teams", {})
    matches = []
    offset = 0
    while True:
        start = text.find("MatchReplayData {", offset)
        if start < 0:
            break
        open_at = text.find("{", start)
        block, end = read_balanced(text, open_at)
        if not block:
            break
        offset = end + 1

        replay_id = parse_first_int(block, "id")
        blue_win = parse_bool(block, "blue_team_win")
        if blue_win is None:
            continue
        blue_perf = parse_match_performance(extract_named_struct(block, "blue_performance", "MatchTeamPerformance"))
        red_perf = parse_match_performance(extract_named_struct(block, "red_performance", "MatchTeamPerformance"))
        blue_players = parse_match_team_details(extract_named_array(block, "blue_team"), champion_ids, blue_perf, save_lookup, item_catalog)
        red_players = parse_match_team_details(extract_named_array(block, "red_team"), champion_ids, red_perf, save_lookup, item_catalog)
        if not blue_players or not red_players:
            continue

        game_tick = parse_first_int(block, "game_tick") or 0
        blue_team_id = parse_first_int(block, "blue_team_id")
        red_team_id = parse_first_int(block, "red_team_id")
        blue = compact_performance(blue_perf)
        red = compact_performance(red_perf)
        blue.update(
            {
                "name": team_names.get(str(blue_team_id), f"blue team #{blue_team_id}" if blue_team_id is not None else "blue team"),
                "bans": parse_quoted_array(block, "blue_ban"),
                "players": blue_players,
            }
        )
        red.update(
            {
                "name": team_names.get(str(red_team_id), f"red team #{red_team_id}" if red_team_id is not None else "red team"),
                "bans": parse_quoted_array(block, "red_ban"),
                "players": red_players,
            }
        )
        matches.append(
            {
                "id": replay_id or len(matches),
                "source": "tournament",
                "version": parse_version(block),
                "date": None,
                "dateKey": "unknown",
                "dateLabel": "date not exported",
                "gameTick": game_tick,
                "durationSec": round(game_tick / 51),
                "blueTeamId": blue_team_id,
                "redTeamId": red_team_id,
                "winner": "blue" if blue_win else "red",
                "blue": blue,
                "red": red,
            }
        )
    matches.sort(key=lambda row: row["id"], reverse=True)
    return matches if limit is None else matches[:limit]


def has_collected_stats(stats):
    return any((row.get("pickCount") or 0) > 0 or (row.get("banCount") or 0) > 0 for row in stats.values())


def stats_total_matches(stats):
    return max((row.get("totalMatch") or 0 for row in (stats or {}).values()), default=0)


def stats_pick_count(stats):
    return sum((row.get("pickCount") or 0) for row in (stats or {}).values())


def should_prefer_replay_stats(current_stats, replay_stats):
    if not has_collected_stats(replay_stats):
        return False
    if not has_collected_stats(current_stats):
        return True

    current_matches = stats_total_matches(current_stats)
    replay_matches = stats_total_matches(replay_stats)
    if replay_matches > current_matches:
        return True

    current_picks = stats_pick_count(current_stats)
    replay_picks = stats_pick_count(replay_stats)
    return replay_picks > current_picks


def aggregate_match_analysis_stats(matches, champion_ids, item_catalog=None, sources=None):
    champion_ids = set(champion_ids)
    source_filter = set(sources or ["tournament"])
    stats = defaultdict(blank_stat)
    stats_by_version = defaultdict(lambda: defaultdict(blank_stat))
    total_matches = 0
    total_matches_by_version = Counter()

    for match in matches:
        if match.get("source") not in source_filter:
            continue
        version = match.get("version") or "unknown"
        total_matches += 1
        total_matches_by_version[version] += 1
        duration_sec = match.get("durationSec") or 0
        for side in ("blue", "red"):
            side_data = match.get(side) or {}
            won = match.get("winner") == side
            if match.get("source") == "tournament":
                for banned in side_data.get("bans") or []:
                    if banned not in champion_ids:
                        continue
                    stats[banned]["banCount"] += 1
                    stats[banned]["source"] = "replay_analysis_export"
                    stats_by_version[version][banned]["banCount"] += 1
                    stats_by_version[version][banned]["source"] = "replay_analysis_export"
            for player in side_data.get("players") or []:
                champion = player.get("champion")
                if champion not in champion_ids:
                    continue
                player_stat = dict(player)
                player_stat["durationSec"] = duration_sec
                add_player_stat(stats, champion, won, player_stat, "replay_analysis_export")
                add_player_stat(stats_by_version[version], champion, won, player_stat, "replay_analysis_export")

    version_stats = {
        version: finalize_aggregated_stats(rows, total_matches_by_version[version], "replay_analysis_export", item_catalog)
        for version, rows in stats_by_version.items()
    }
    return finalize_aggregated_stats(stats, total_matches, "replay_analysis_export", item_catalog), version_stats


def aggregate_match_analysis_stats_by_bucket(matches, champion_ids, item_catalog=None, sources=None):
    stats_by_bucket = {}
    stats_by_patch_bucket = defaultdict(dict)
    source_filter = set(sources or ["tournament"])
    for bucket in league_stat_bucket_keys():
        bucket_matches = [
            match
            for match in matches or []
            if match.get("source") in source_filter and bucket in match_bucket_keys(match)
        ]
        stats, stats_by_version = aggregate_match_analysis_stats(bucket_matches, champion_ids, item_catalog, source_filter)
        stats_by_bucket[bucket] = stats
        for version, rows in stats_by_version.items():
            stats_by_patch_bucket[version][bucket] = rows
    return stats_by_bucket, dict(stats_by_patch_bucket)


class RelationAccumulator:
    def __init__(self):
        self.groups = 0
        self.synergy = defaultdict(lambda: defaultdict(lambda: Counter(games=0, wins=0)))
        self.counter = defaultdict(lambda: defaultdict(lambda: Counter(games=0, wins=0)))

    def record(self, blue, red, blue_win):
        blue = list(dict.fromkeys(blue))
        red = list(dict.fromkeys(red))
        if not blue or not red:
            return
        self.groups += 1
        self._record_team(blue, blue_win)
        self._record_team(red, not blue_win)
        self._record_opponents(blue, red, blue_win)
        self._record_opponents(red, blue, not blue_win)

    def _record_team(self, team, won):
        for champ in team:
            for other in team:
                if champ == other:
                    continue
                row = self.synergy[champ][other]
                row["games"] += 1
                row["wins"] += 1 if won else 0

    def _record_opponents(self, team, enemies, won):
        for champ in team:
            for enemy in enemies:
                row = self.counter[champ][enemy]
                row["games"] += 1
                row["wins"] += 1 if won else 0

    def to_payload(self):
        return {
            "groups": self.groups,
            "pairs": relation_table(self.synergy, reverse=True),
            "counters": relation_table(self.counter, reverse=False),
        }


def relation_row_from_source(source, champ, other):
    stat = (source.get(champ) or {}).get(other)
    if not stat:
        return None
    games = stat["games"]
    wins = stat["wins"]
    if games <= 0:
        return None
    return {
        "champion": other,
        "games": games,
        "wins": wins,
        "winRate": round(wins / games * 100, 1),
    }


def preserve_counter_reciprocals(out, source):
    for champ, rows in list(out.items()):
        for row in list(rows):
            other = row.get("champion")
            if not other:
                continue
            reverse_row = relation_row_from_source(source, other, champ)
            if not reverse_row:
                continue
            target = out.setdefault(other, [])
            if any(existing.get("champion") == champ for existing in target):
                continue
            target.append(reverse_row)
    for rows in out.values():
        rows.sort(
            key=lambda row: (
                row["winRate"],
                -min(row["games"], 30),
                -row["games"],
            )
        )


def relation_table(source, reverse):
    out = {}
    for champ, counters in source.items():
        rows = []
        for other, stat in counters.items():
            games = stat["games"]
            wins = stat["wins"]
            if games <= 0:
                continue
            rows.append(
                {
                    "champion": other,
                    "games": games,
                    "wins": wins,
                    "winRate": round(wins / games * 100, 1),
                }
            )
        if any(row["games"] >= 5 for row in rows):
            rows = [row for row in rows if row["games"] >= 5]
        elif any(row["games"] >= 3 for row in rows):
            rows = [row for row in rows if row["games"] >= 3]
        if reverse:
            rows.sort(
                key=lambda row: (
                    row["winRate"],
                    min(row["games"], 30),
                    row["games"],
                ),
                reverse=True,
            )
            out[champ] = rows[:12]
            continue

        # Counter tables need both ends: hardest matchups and easiest matchups.
        # Keeping only the low-win-rate side makes the UI's "easy" column pick
        # from already-trimmed hard counters, which can show sub-50% rows.
        hard_rows = sorted(
            rows,
            key=lambda row: (
                row["winRate"],
                -min(row["games"], 30),
                -row["games"],
            ),
        )[:12]
        easy_rows = sorted(
            rows,
            key=lambda row: (
                -row["winRate"],
                -min(row["games"], 30),
                -row["games"],
            ),
        )[:12]
        merged_rows = []
        seen = set()
        for row in hard_rows + easy_rows:
            other = row.get("champion")
            if other in seen:
                continue
            seen.add(other)
            merged_rows.append(row)
        out[champ] = merged_rows
    if not reverse:
        preserve_counter_reciprocals(out, source)
    return out


LANE_COMBOS = [
    ("bot_support", "bot", "support"),
    ("top_jungle", "top", "jungle"),
    ("mid_jungle", "mid", "jungle"),
]


def empty_lane_synergy_payload():
    return {combo: [] for combo, _, _ in LANE_COMBOS}


class LanePairAccumulator:
    def __init__(self):
        self.rows = defaultdict(lambda: Counter(games=0, wins=0))

    def record(self, players, won):
        by_position = {}
        for player in players:
            position = player.get("position")
            champion = player.get("champion")
            if position and champion and position not in by_position:
                by_position[position] = champion
        for combo, left_role, right_role in LANE_COMBOS:
            left = by_position.get(left_role)
            right = by_position.get(right_role)
            if not left or not right or left == right:
                continue
            row = self.rows[(combo, left, right)]
            row["games"] += 1
            row["wins"] += 1 if won else 0

    def to_payload(self):
        out = empty_lane_synergy_payload()
        for (combo, left, right), stat in self.rows.items():
            games = stat["games"]
            wins = stat["wins"]
            if games <= 0:
                continue
            out[combo].append(
                {
                    "leftChampion": left,
                    "rightChampion": right,
                    "games": games,
                    "wins": wins,
                    "winRate": round(wins / games * 100, 1),
                }
            )
        for combo, rows in out.items():
            if any(row["games"] >= 5 for row in rows):
                rows[:] = [row for row in rows if row["games"] >= 5]
            elif any(row["games"] >= 3 for row in rows):
                rows[:] = [row for row in rows if row["games"] >= 3]
            rows.sort(key=lambda row: (row["winRate"], min(row["games"], 30), row["games"]), reverse=True)
            del rows[20:]
        return out


def aggregate_match_relations(matches, sources=None):
    source_filter = set(sources or ["tournament"])
    relations = RelationAccumulator()
    relations_by_version = defaultdict(RelationAccumulator)
    lane_synergies = LanePairAccumulator()
    lane_synergies_by_version = defaultdict(LanePairAccumulator)
    for match in matches or []:
        if match.get("source") not in source_filter:
            continue
        blue_win = match.get("winner") == "blue"
        version = match.get("version") or "unknown"
        blue_players = (match.get("blue") or {}).get("players") or []
        red_players = (match.get("red") or {}).get("players") or []
        blue = [p.get("champion") for p in blue_players if p.get("champion")]
        red = [p.get("champion") for p in red_players if p.get("champion")]
        if not blue or not red:
            continue
        relations.record(blue, red, blue_win)
        relations_by_version[version].record(blue, red, blue_win)
        lane_synergies.record(blue_players, blue_win)
        lane_synergies.record(red_players, not blue_win)
        lane_synergies_by_version[version].record(blue_players, blue_win)
        lane_synergies_by_version[version].record(red_players, not blue_win)
    return (
        relations.to_payload(),
        {version: rel.to_payload() for version, rel in relations_by_version.items()},
        lane_synergies.to_payload(),
        {version: rel.to_payload() for version, rel in lane_synergies_by_version.items()},
    )


def aggregate_match_relations_by_bucket(matches, sources=None):
    rel_by_bucket = {}
    rel_by_patch_bucket = defaultdict(dict)
    lane_by_bucket = {}
    lane_by_patch_bucket = defaultdict(dict)
    source_filter = set(sources or ["tournament"])
    for bucket in league_stat_bucket_keys():
        bucket_matches = [
            match
            for match in matches or []
            if match.get("source") in source_filter and bucket in match_bucket_keys(match)
        ]
        rel, rel_by_version, lane, lane_by_version = aggregate_match_relations(bucket_matches, source_filter)
        rel_by_bucket[bucket] = rel
        lane_by_bucket[bucket] = lane
        for version, rows in rel_by_version.items():
            rel_by_patch_bucket[version][bucket] = rows
        for version, rows in lane_by_version.items():
            lane_by_patch_bucket[version][bucket] = rows
    return rel_by_bucket, dict(rel_by_patch_bucket), lane_by_bucket, dict(lane_by_patch_bucket)


def load_replay_summary_count(export_usable=True):
    if not export_usable:
        return 0
    path = EXPORT_DIR / "match_replay_summary.tsv"
    if not path.exists():
        return 0
    count = 0
    with path.open("r", encoding="utf-8", errors="ignore") as f:
        for line in f:
            if line.startswith("new\t") or re.match(r"^\d+\t", line):
                count += 1
    return count


def parse_solo_rank_replay_ids(path: Path):
    if not path.exists():
        return set()
    text = path.read_text(encoding="utf-8", errors="ignore")
    replay_ids = set()
    for match in re.finditer(r"\b(\d+):\s+SoloRankMatch\s+\{\s+id:\s+(\d+),", text):
        replay_ids.add(int(match.group(2)))
    return replay_ids


def read_tsv_fields(path: Path):
    if not path.exists():
        return {}
    fields = {}
    with path.open("r", encoding="utf-8", errors="ignore") as f:
        for line in f:
            line = line.rstrip("\r\n")
            if not line or line == "field\tvalue":
                continue
            key, sep, value = line.partition("\t")
            if sep:
                fields[key] = value
    return fields


def meta_export_counts_look_sane(fields):
    limits = {
        "teams": 10_000,
        "athletes": 100_000,
        "champion_patch_statistics": 10_000,
        "solo_rank_matches": 100_000,
        "match_replays": 100_000,
        "league_competitions": 10_000,
        "tournament_competitions": 10_000,
        "year_schedules": 10_000,
        "match_stats": 100_000,
    }
    for key, limit in limits.items():
        raw = fields.get(key)
        if raw is None:
            continue
        try:
            if int(raw) > limit:
                return False
        except ValueError:
            continue
    return True


def meta_export_data_paths():
    return [
        EXPORT_DIR / "teams.debug.txt",
        EXPORT_DIR / "athletes.debug.txt",
        EXPORT_DIR / "champion_patch_statistics.debug.txt",
        EXPORT_DIR / "champion_patch_statistics.tsv",
        EXPORT_DIR / "solo_rank_matches.debug.txt",
        EXPORT_DIR / "match_replays.debug.txt",
        EXPORT_DIR / "league_competitions.debug.txt",
        EXPORT_DIR / "tournament_competitions.debug.txt",
        EXPORT_DIR / "year_schedules.debug.txt",
        EXPORT_DIR / "match_stats.debug.txt",
        EXPORT_DIR / "match_replay_summary.tsv",
        EXPORT_DIR / "match_replay_players.tsv",
    ]


# ★stale 검사 대상 = save_probe 가 매 실행마다 새로 쓰는 파일(.debug.txt)만.
#   구 Meta Exporter(인게임 모드) 시절의 .tsv 산출물(champion_patch_statistics.tsv,
#   match_replay_summary.tsv, match_replay_players.tsv)은 save_probe 가 갱신하지 않는다.
#   폴더에 한 번 남으면 mtime 이 영원히 옛것 → 매 실행 stale 오판정 → snapshot 전체 무시
#   → "세이브 분석 오류"가 영구 재발(2026-07-19 실측 제보 로그:
#      stale_export_files: teams.debug.txt, athletes.debug.txt,
#      champion_patch_statistics.debug.txt, champion_patch_statistics.tsv → matches=0).
def meta_export_stale_paths():
    return [path for path in meta_export_data_paths() if path.name.endswith(".debug.txt")]


# save_probe 는 debug 파일들을 먼저 쓰고 manifest.tsv 를 마지막에 쓴다. 세이브가 크거나 디스크가
# 느리면 그 간격이 수 초까지 벌어지므로, 기존 1초 임계는 "정상 실행"도 stale 로 오판했다
# (실측: 54MB 세이브에서 0.66초 = 임계 코앞). 진짜 잔재는 보통 수 시간~수 일 차이라
# 넉넉히 잡아도 탐지력은 유지된다.
STALE_TOLERANCE_SECONDS = 600


def inspect_meta_export():
    manifest_path = EXPORT_DIR / "manifest.tsv"
    manifest = read_tsv_fields(manifest_path)
    compatibility = read_tsv_fields(EXPORT_DIR / "compatibility_error.tsv")
    reason = None
    if compatibility:
        reason = compatibility.get("message") or compatibility.get("sdk") or "compatibility_error.tsv present"
    elif manifest.get("compatibility") == "incompatible_database_layout":
        reason = "incompatible_database_layout"
    elif manifest and not meta_export_counts_look_sane(manifest):
        reason = "impossible_manifest_counts"
    else:
        data_paths = [path for path in meta_export_data_paths() if path.exists()]
        if data_paths and not manifest_path.exists():
            reason = "export_data_without_manifest"
        elif data_paths and manifest_path.exists():
            manifest_time = manifest_path.stat().st_mtime
            stale = [
                path.name
                for path in meta_export_stale_paths()
                if path.exists() and path.stat().st_mtime + STALE_TOLERANCE_SECONDS < manifest_time
            ]
            if stale:
                reason = "stale_export_files: " + ", ".join(stale[:4])

    return {
        "usable": reason is None,
        "reason": reason,
        "manifest": manifest,
        "compatibility": compatibility,
    }


def latest_meta_export_timestamp():
    paths = [
        EXPORT_DIR / "manifest.tsv",
        EXPORT_DIR / "compatibility_error.tsv",
    ] + meta_export_data_paths()
    timestamps = [path.stat().st_mtime for path in paths if path.exists()]
    return max(timestamps) if timestamps else None


def merge_stats(champions, news_rows, draft_scan, exported_stats):
    news_best = {}
    for row in news_rows:
        current = news_best.get(row["champion"])
        if current is None or row["pickCount"] > current["pickCount"]:
            news_best[row["champion"]] = row

    stats = {}
    for champ in champions:
        cid = champ["id"]
        stat = {
            "pickCount": None,
            "wins": None,
            "losses": None,
            "winRate": None,
            "banCount": None,
            "banRate": None,
            "banPickRate": None,
            "dealt": None,
            "taken": None,
            "healing": None,
            "itemCounts": None,
            "topItems": [],
            "draftMentions": draft_scan["mentions"].get(cid, 0),
            "source": "not_collected",
            "confidence": "none",
        }

        if cid in news_best:
            row = news_best[cid]
            stat.update(
                {
                    "pickCount": row["pickCount"],
                    "wins": row["wins"],
                    "losses": max(0, row["pickCount"] - row["wins"]),
                    "winRate": row["winRate"],
                    "source": row["source"],
                    "confidence": "partial",
                }
            )

        if cid in exported_stats:
            row = exported_stats[cid]
            pick = row.get("pickCount")
            wins = row.get("wins")
            losses = row.get("losses")
            win_rate = None
            if wins is not None and losses is not None and wins + losses > 0:
                win_rate = round(wins / (wins + losses) * 100, 1)
            total_match = row.get("totalMatch")
            pick_rate = None
            ban_rate = None
            ban_pick_rate = None
            if total_match:
                if pick is not None:
                    pick_rate = round(pick / total_match * 100, 1)
                if row.get("banCount") is not None:
                    ban_rate = round(row.get("banCount") / total_match * 100, 1)
                if pick is not None and row.get("banCount") is not None:
                    ban_pick_rate = round((pick + row.get("banCount")) / total_match * 100, 1)
            stat.update(
                {
                    "pickCount": pick if pick is not None else stat["pickCount"],
                    "banCount": row.get("banCount"),
                    "pickRate": pick_rate,
                    "banRate": ban_rate,
                    "banPickRate": ban_pick_rate,
                    "wins": wins if wins is not None else stat["wins"],
                    "losses": losses if losses is not None else stat["losses"],
                    "winRate": win_rate if win_rate is not None else stat["winRate"],
                    "dealt": row.get("dealt"),
                    "taken": row.get("taken"),
                    "healing": row.get("healing"),
                    "kills": row.get("kills"),
                    "deaths": row.get("deaths"),
                    "cs": row.get("cs"),
                    "gold": row.get("gold"),
                    "linePhase": row.get("linePhase"),
                    "byPosition": row.get("byPosition"),
                    "totalMatch": total_match,
                    "source": row.get("source", "meta_exporter_debug"),
                    "confidence": "exported",
                }
            )

        stats[cid] = stat
    return stats


def calculated_tier(stat):
    meta_tier = tier_from_meta_score(stat)
    if meta_tier:
        return meta_tier
    sample = stat.get("pickCount") or 0
    win_rate = stat.get("winRate")
    if sample < 5 or win_rate is None:
        return "-"
    if win_rate >= 62:
        return "OP"
    if win_rate >= 57:
        return "1"
    if win_rate >= 53:
        return "2"
    if win_rate >= 49:
        return "3"
    return "4"


def tier_from_meta_score(stat):
    meta = (stat or {}).get("metaScore") or {}
    if not meta.get("eligible"):
        return None
    try:
        score = float(meta.get("score"))
    except (TypeError, ValueError):
        score = None
    if score is not None:
        grade = meta_score_grade(score)
    else:
        grade = str(meta.get("grade") or "").upper()
    if grade == "S":
        return "OP"
    if grade == "A":
        return "1"
    if grade == "B":
        return "2"
    if grade == "C":
        return "3"
    if grade == "D":
        return "4"
    return None


def refresh_tiers(champions, *stat_groups):
    for stats in stat_groups:
        if not stats:
            continue
        for champ in champions:
            row = stats.get(champ["id"])
            if row is not None:
                row["tier"] = calculated_tier(row)


SCORE_MODEL_VERSION = "role-aware-v6"
SCORE_MIN_PICKS = 5
SCORE_COUNTER_MIN_GAMES = 3
SCORE_PRIOR_PICKS = 24
SCORE_ROLE_PRIOR_PICKS = 16
SCORE_ROLE_MIN_PICKS = 5
SCORE_USABLE_ROLE_MIN_PICKS = 10
SCORE_USABLE_ROLE_STRONG_MIN_PICKS = 5
SCORE_RELIABILITY_PICKS = 40
SCORE_WILSON_Z = 1.28
SCORE_WIN_RISK_Z = 0.65
SCORE_POWER_WEIGHT = 0.70
SCORE_DRAFT_WEIGHT = 0.20
SCORE_VERSATILITY_WEIGHT = 0.10
SCORE_TOURNAMENT_DRAFT_WEIGHT = 0.75
SCORE_ALL_PICK_DRAFT_WEIGHT = 0.25


def clamp(value, low, high):
    return max(low, min(high, value))


def round_score(value):
    return round(value, 1)


def bayesian_win_rate(wins, games, prior_picks=None):
    prior_picks = SCORE_PRIOR_PICKS if prior_picks is None else prior_picks
    games = max(0, float(games or 0))
    wins = clamp(float(wins or 0), 0, games)
    if games <= 0:
        return 50.0
    prior_wins = prior_picks * 0.5
    return (wins + prior_wins) / (games + prior_picks) * 100


def bayesian_win_stddev(wins, games, prior_picks=None):
    prior_picks = SCORE_PRIOR_PICKS if prior_picks is None else prior_picks
    games = max(0, float(games or 0))
    wins = clamp(float(wins or 0), 0, games)
    prior_wins = prior_picks * 0.5
    alpha = wins + prior_wins
    beta = games - wins + prior_wins
    total = alpha + beta
    if total <= 0:
        return 0.0
    variance = (alpha * beta) / ((total * total) * (total + 1))
    return math.sqrt(max(0.0, variance)) * 100


def risk_adjusted_win_rate(wins, games, prior_picks=None):
    mean = bayesian_win_rate(wins, games, prior_picks)
    stddev = bayesian_win_stddev(wins, games, prior_picks)
    return clamp(mean - SCORE_WIN_RISK_Z * stddev, 0, 100), mean, stddev


def wilson_lower_win_rate(wins, games):
    games = max(0, float(games or 0))
    wins = clamp(float(wins or 0), 0, games)
    if games <= 0:
        return 50.0
    phat = wins / games
    z = SCORE_WILSON_Z
    denom = 1 + z * z / games
    center = phat + z * z / (2 * games)
    margin = z * math.sqrt((phat * (1 - phat) + z * z / (4 * games)) / games)
    return clamp((center - margin) / denom * 100, 0, 100)


def percentile_score(value, values):
    try:
        value = float(value)
    except (TypeError, ValueError):
        return 0.0
    clean = sorted(float(v) for v in values if v is not None and float(v) > 0)
    if not clean:
        return 0.0
    if clean[-1] == clean[0]:
        return 50.0 if value > 0 else 0.0
    below = sum(1 for v in clean if v <= value)
    return round((below - 1) / max(1, len(clean) - 1) * 100, 2)


def finite_number(value):
    try:
        value = float(value)
    except (TypeError, ValueError):
        return None
    return value if math.isfinite(value) else None


def percentile_rank(value, values):
    value = finite_number(value)
    clean = sorted(v for v in (finite_number(v) for v in values) if v is not None)
    if value is None or not clean:
        return 0.0
    if len(clean) == 1:
        return 100.0 if value >= clean[0] else 0.0
    if clean[-1] == clean[0]:
        return 50.0
    below_or_equal = sum(1 for v in clean if v <= value)
    return round((below_or_equal - 1) / (len(clean) - 1) * 100, 2)


def logit_rate(rate):
    rate = finite_number(rate)
    if rate is None:
        return None
    p = clamp(rate / 100, 0.001, 0.999)
    return math.log(p / (1 - p))


def z_scores_by_key(values_by_key):
    clean = [v for v in values_by_key.values() if finite_number(v) is not None]
    if not clean:
        return {}
    mean = sum(clean) / len(clean)
    variance = sum((v - mean) ** 2 for v in clean) / len(clean)
    stddev = math.sqrt(max(0.0, variance))
    if stddev <= 0:
        return {key: 0.0 for key in values_by_key}
    return {key: (value - mean) / stddev for key, value in values_by_key.items() if finite_number(value) is not None}


def score_metric_kind(champ, stat):
    tags = set(champ.get("tags") or [])
    if "Heal" in tags and (stat.get("healing") or 0) > 0:
        return "healing"
    if ({"Tank", "Frontline", "Shield"} & tags) and (stat.get("taken") or 0) > 0:
        return "taken"
    if stat.get("dealt") or 0:
        return "dealt"
    if stat.get("healing") or 0:
        return "healing"
    if stat.get("taken") or 0:
        return "taken"
    return "none"


def metric_per_game(stat, key):
    games = stat.get("pickCount") or 0
    if not games or key == "none":
        return 0
    return (stat.get(key) or 0) / games


def metric_rate(stat, key):
    if key == "none":
        return 0, "none"
    duration_sec = stat.get("durationSec") or 0
    if duration_sec > 0:
        return (stat.get(key) or 0) / (duration_sec / 60), "perMinute"
    return metric_per_game(stat, key), "perGame"


def counter_profile(relations, champion):
    hard = 0
    easy = 0
    for row in (relations.get("counters") or {}).get(champion, []):
        games = row.get("games") or 0
        win_rate = row.get("winRate")
        if games < SCORE_COUNTER_MIN_GAMES or win_rate is None:
            continue
        if win_rate <= 45:
            hard += 1
        elif win_rate >= 55:
            easy += 1
    return hard, easy


def meta_score_grade(score):
    if score is None:
        return "-"
    if score >= 85:
        return "S"
    if score >= 70:
        return "A"
    if score >= 55:
        return "B"
    if score >= 40:
        return "C"
    return "D"


def role_power_rows(row):
    out = []
    for position in POSITION_NAMES:
        role_row = (row.get("byPosition") or {}).get(position) or {}
        games = role_row.get("matches") or 0
        if games <= 0:
            continue
        wins = role_row.get("wins") or 0
        risk, bayes, stddev = risk_adjusted_win_rate(wins, games, SCORE_ROLE_PRIOR_PICKS)
        out.append(
            {
                "position": position,
                "sample": games,
                "wins": wins,
                "winRate": round_score(wins / games * 100) if games else None,
                "powerRaw": round_score(risk),
                "winRateBayes": round_score(bayes),
                "winRateStdDev": round_score(stddev),
            }
        )
    return out


def overall_power_row(row):
    sample = row.get("pickCount") or 0
    wins = row.get("wins") or 0
    risk, bayes, stddev = risk_adjusted_win_rate(wins, sample)
    return {
        "position": "all",
        "sample": sample,
        "wins": wins,
        "winRate": row.get("winRate"),
        "powerRaw": round_score(risk),
        "winRateBayes": round_score(bayes),
        "winRateStdDev": round_score(stddev),
    }


def tournament_presence_rate(row, scope):
    value = row.get("tournamentPresenceRate")
    if value is not None:
        return value
    if scope.startswith("tournament"):
        return row.get("banPickRate")
    if (row.get("banCount") or 0) > 0:
        return row.get("banRate")
    return None


def reliability_score(row):
    sample = row.get("pickCount") or 0
    bans = row.get("banCount") or 0
    ban_evidence_cap = sample * 4 if sample else 40
    effective = sample + min(bans, ban_evidence_cap) * 0.10
    if effective <= 0:
        return 0.0, 0.0
    return round_score(effective / (effective + SCORE_RELIABILITY_PICKS) * 100), round_score(effective)


def reliability_label(score):
    if score >= 80:
        return "High"
    if score >= 55:
        return "Medium"
    if score > 0:
        return "Low"
    return "None"


def attach_meta_scores(champions, stats, relationships, scope):
    eligible = {
        champ["id"]: stats.get(champ["id"], {})
        for champ in champions
        if (stats.get(champ["id"], {}).get("pickCount") or 0) >= SCORE_MIN_PICKS
        and stats.get(champ["id"], {}).get("winRate") is not None
    }
    role_rows_by_cid = {}
    role_values_by_position = defaultdict(list)
    best_power_raw_by_cid = {}
    best_role_by_cid = {}
    for cid, row in eligible.items():
        rows = role_power_rows(row)
        if not rows:
            rows = [overall_power_row(row)]
        role_rows_by_cid[cid] = rows
        for role_row in rows:
            if role_row["position"] != "all":
                role_values_by_position[role_row["position"]].append(role_row["powerRaw"])
        deployable = [role_row for role_row in rows if role_row["sample"] >= SCORE_ROLE_MIN_PICKS] or rows
        best = max(deployable, key=lambda role_row: (role_row["powerRaw"], role_row["sample"]))
        best_power_raw_by_cid[cid] = best["powerRaw"]
        best_role_by_cid[cid] = best["position"]

    power_values = list(best_power_raw_by_cid.values())
    presence_logits = {}
    pick_logits = {}
    for cid, row in stats.items():
        if (row.get("pickCount") or 0) <= 0 and (row.get("banCount") or 0) <= 0:
            continue
        presence = tournament_presence_rate(row, scope)
        if presence is not None:
            presence_logits[cid] = logit_rate(presence)
        if row.get("pickRate") is not None:
            pick_logits[cid] = logit_rate(row.get("pickRate"))
    presence_z = z_scores_by_key({key: value for key, value in presence_logits.items() if value is not None})
    pick_z = z_scores_by_key({key: value for key, value in pick_logits.items() if value is not None})
    draft_signal_by_cid = {}
    for cid in set(presence_z) | set(pick_z):
        if cid in presence_z:
            draft_signal_by_cid[cid] = (
                SCORE_TOURNAMENT_DRAFT_WEIGHT * presence_z.get(cid, 0.0)
                + SCORE_ALL_PICK_DRAFT_WEIGHT * pick_z.get(cid, 0.0)
            )
        else:
            draft_signal_by_cid[cid] = pick_z.get(cid, 0.0)
    draft_values = list(draft_signal_by_cid.values())

    scored = []
    for champ in champions:
        cid = champ["id"]
        row = stats.get(cid)
        if not row:
            continue
        sample = row.get("pickCount") or 0
        if cid not in eligible:
            row["metaScore"] = {
                "eligible": False,
                "score": None,
                "grade": "-",
                "sample": sample,
                "minSample": SCORE_MIN_PICKS,
                "reason": "sample_too_small",
                "scope": scope,
            }
            continue
        win_rate = row.get("winRate") or 0
        wins = row.get("wins") or 0
        risk_win_rate, adjusted_win_rate, win_stddev = risk_adjusted_win_rate(wins, sample)
        wilson_win_rate = wilson_lower_win_rate(wins, sample)
        role_rows = role_rows_by_cid.get(cid, [])
        for role_row in role_rows:
            role_values = role_values_by_position.get(role_row["position"], [])
            role_row["powerScore"] = percentile_rank(role_row["powerRaw"], role_values) if role_values else percentile_rank(role_row["powerRaw"], power_values)
            role_row["eligible"] = role_row["sample"] >= SCORE_ROLE_MIN_PICKS
        usable_roles = [
            role_row
            for role_row in role_rows
            if role_row["position"] != "all"
            and role_row["powerScore"] >= 40
            and (
                role_row["sample"] >= SCORE_USABLE_ROLE_MIN_PICKS
                or (
                    role_row["sample"] >= SCORE_USABLE_ROLE_STRONG_MIN_PICKS
                    and role_row["powerScore"] >= 70
                )
            )
        ]
        usable_sample = sum(role_row["sample"] for role_row in usable_roles)
        role_count_score = len(usable_roles) / len(POSITION_NAMES) * 100
        if usable_sample > 0 and len(usable_roles) > 1:
            entropy = 0.0
            for role_row in usable_roles:
                p = role_row["sample"] / usable_sample
                entropy -= p * math.log(p)
            entropy_score = entropy / math.log(len(POSITION_NAMES)) * 100
        else:
            entropy_score = 0.0
        versatility_score = round_score(clamp(role_count_score * 0.65 + entropy_score * 0.35, 0, 100))
        power_score = round_score(percentile_rank(best_power_raw_by_cid.get(cid), power_values))
        draft_pressure = round_score(percentile_rank(draft_signal_by_cid.get(cid), draft_values))
        reliability, effective_sample = reliability_score(row)
        hard_counters, easy_matchups = counter_profile(relationships or {}, cid)
        components = {
            "power": power_score,
            "draft": draft_pressure,
            "versatility": versatility_score,
            "reliability": reliability,
        }
        raw_score = clamp(
            power_score * SCORE_POWER_WEIGHT
            + draft_pressure * SCORE_DRAFT_WEIGHT
            + versatility_score * SCORE_VERSATILITY_WEIGHT,
            0,
            100,
        )
        score = round_score(raw_score)
        role_scores = {}
        for role_row in role_rows:
            if role_row["position"] == "all":
                continue
            role_reliability = round_score(role_row["sample"] / (role_row["sample"] + SCORE_RELIABILITY_PICKS) * 100) if role_row["sample"] else 0.0
            role_score = round_score(
                clamp(
                    role_row["powerScore"] * 0.85
                    + draft_pressure * 0.10
                    + role_reliability * 0.05,
                    0,
                    100,
                )
            )
            role_scores[role_row["position"]] = {
                "eligible": role_row["sample"] >= SCORE_ROLE_MIN_PICKS,
                "score": role_score if role_row["sample"] >= SCORE_ROLE_MIN_PICKS else None,
                "grade": meta_score_grade(role_score) if role_row["sample"] >= SCORE_ROLE_MIN_PICKS else "-",
                "formulaVersion": SCORE_MODEL_VERSION,
                "sample": role_row["sample"],
                "minSample": SCORE_ROLE_MIN_PICKS,
                "scope": scope,
                "roleScoped": True,
                "position": role_row["position"],
                "powerScore": round_score(role_row["powerScore"]),
                "draftPressure": draft_pressure,
                "versatility": 0,
                "reliability": role_reliability,
                "reliabilityLabel": reliability_label(role_reliability),
                "components": {
                    "power": round_score(role_row["powerScore"]),
                    "draft": draft_pressure,
                    "versatility": 0,
                    "reliability": role_reliability,
                },
                "bestRole": role_row["position"],
                "usableRoles": [role_row["position"]] if role_row in usable_roles else [],
                "winRateRaw": role_row["winRate"],
                "winRateBayes": role_row["winRateBayes"],
                "winRateStdDev": role_row["winRateStdDev"],
                "winRateRiskAdjusted": role_row["powerRaw"],
            }
        row["metaScore"] = {
            "eligible": True,
            "score": score,
            "grade": meta_score_grade(score),
            "formulaVersion": SCORE_MODEL_VERSION,
            "sample": sample,
            "minSample": SCORE_MIN_PICKS,
            "rawScore": round_score(raw_score),
            "winRateRaw": win_rate,
            "winRateBayes": round_score(adjusted_win_rate),
            "winRateStdDev": round_score(win_stddev),
            "winRateRiskAdjusted": round_score(risk_win_rate),
            "winRateWilson": round_score(wilson_win_rate),
            "winRateConfidence": round_score(risk_win_rate),
            "scope": scope,
            "components": components,
            "powerScore": power_score,
            "draftPressure": draft_pressure,
            "versatility": versatility_score,
            "reliability": reliability,
            "reliabilityLabel": reliability_label(reliability),
            "effectiveSample": effective_sample,
            "bestRole": best_role_by_cid.get(cid),
            "usableRoles": [role_row["position"] for role_row in usable_roles],
            "roleScores": role_scores,
            "rolePowerRaw": round_score(best_power_raw_by_cid.get(cid)),
            "tournamentPresenceRate": tournament_presence_rate(row, scope),
            "hardCounters": hard_counters,
            "easyMatchups": easy_matchups,
        }
        scored.append((score, cid))

    for rank, (_score, cid) in enumerate(sorted(scored, reverse=True), 1):
        stats[cid]["metaScore"]["rank"] = rank
    return stats


def attach_meta_score_group(champions, grouped_stats, grouped_relationships):
    for scope, stats in grouped_stats.items():
        attach_meta_scores(champions, stats, grouped_relationships.get(scope, {}), scope)


def empty_display_stat(draft_mentions=0):
    return {
        "pickCount": None,
        "wins": None,
        "losses": None,
        "winRate": None,
        "pickRate": None,
        "banCount": None,
        "banRate": None,
        "banPickRate": None,
        "dealt": None,
        "taken": None,
        "healing": None,
        "kills": None,
        "deaths": None,
        "assists": None,
        "cs": None,
        "gold": None,
        "durationSec": None,
        "itemCounts": None,
        "topItems": [],
        "linePhase": None,
        "byPosition": None,
        "draftMentions": draft_mentions,
        "metaScore": None,
        "source": "not_collected",
        "confidence": "none",
    }


def normalize_scope(champions, stats, draft_scan):
    normalized = {}
    for champ in champions:
        cid = champ["id"]
        row = empty_display_stat(draft_scan["mentions"].get(cid, 0))
        if cid in stats:
            row.update(stats[cid])
        row["tier"] = calculated_tier(row)
        normalized[cid] = row
    return normalized


def combine_scope_stats(champions, tournament, solo, draft_scan, item_catalog=None):
    combined = {}
    tournament_total = max((row.get("totalMatch") or 0 for row in tournament.values()), default=0)
    solo_total = max((row.get("totalMatch") or 0 for row in solo.values()), default=0)
    total = tournament_total + solo_total
    for champ in champions:
        cid = champ["id"]
        t = tournament.get(cid, {})
        s = solo.get(cid, {})
        tournament_picks = t.get("pickCount") or 0
        solo_picks = s.get("pickCount") or 0
        picks = tournament_picks + solo_picks
        wins = (t.get("wins") or 0) + (s.get("wins") or 0)
        losses = (t.get("losses") or 0) + (s.get("losses") or 0)
        bans = t.get("banCount") or 0
        line_phase = merge_counter_dicts(t.get("linePhase"), s.get("linePhase"))
        by_position = merge_position_dicts(t.get("byPosition"), s.get("byPosition"))
        item_counts = merge_counter_dicts(t.get("itemCounts"), s.get("itemCounts"))
        row = empty_display_stat(draft_scan["mentions"].get(cid, 0))
        if picks or bans:
            row.update(
                {
                    "pickCount": picks,
                    "banCount": bans,
                    "wins": wins,
                    "losses": losses,
                    "winRate": round(wins / picks * 100, 1) if picks else None,
                    "pickRate": round(picks / total * 100, 1) if total else None,
                    "banRate": round(bans / tournament_total * 100, 1) if tournament_total else None,
                    "banPickRate": round((picks + bans) / total * 100, 1) if total else None,
                    "tournamentPickCount": tournament_picks,
                    "soloPickCount": solo_picks,
                    "tournamentTotalMatch": tournament_total,
                    "soloTotalMatch": solo_total,
                    "tournamentPresenceRate": round((tournament_picks + bans) / tournament_total * 100, 1) if tournament_total else None,
                    "dealt": (t.get("dealt") or 0) + (s.get("dealt") or 0),
                    "taken": (t.get("taken") or 0) + (s.get("taken") or 0),
                    "healing": (t.get("healing") or 0) + (s.get("healing") or 0),
                    "kills": (t.get("kills") or 0) + (s.get("kills") or 0),
                    "deaths": (t.get("deaths") or 0) + (s.get("deaths") or 0),
                    "assists": (t.get("assists") or 0) + (s.get("assists") or 0),
                    "cs": (t.get("cs") or 0) + (s.get("cs") or 0),
                    "gold": (t.get("gold") or 0) + (s.get("gold") or 0),
                    "durationSec": (t.get("durationSec") or 0) + (s.get("durationSec") or 0),
                    "itemCounts": item_counts,
                    "topItems": item_top_list(item_counts, item_catalog),
                    "linePhase": line_phase,
                    "byPosition": by_position,
                    "totalMatch": total,
                    "source": "combined_export",
                    "confidence": "exported",
                }
            )
        row["tier"] = calculated_tier(row)
        combined[cid] = row
    return combined


def merge_counter_dicts(*items):
    total = Counter()
    for item in items:
        if item:
            total.update({key: value or 0 for key, value in item.items()})
    return dict(total) if total else None


def merge_position_dicts(*items):
    merged = defaultdict(Counter)
    for item in items:
        if not item:
            continue
        for pos, values in item.items():
            merged[pos].update({key: value or 0 for key, value in values.items()})
    return {pos: dict(values) for pos, values in merged.items()} if merged else None


def merge_relationship_payloads(*payloads):
    return {
        "groups": sum(payload.get("groups", 0) for payload in payloads if payload),
        "pairs": merge_relation_kind("pairs", payloads, reverse=True),
        "counters": merge_relation_kind("counters", payloads, reverse=False),
    }


def merge_lane_synergy_payloads(*payloads):
    merged = LanePairAccumulator()
    for payload in payloads:
        if not payload:
            continue
        for combo, rows in payload.items():
            for row in rows:
                key = (combo, row["leftChampion"], row["rightChampion"])
                merged.rows[key]["games"] += row.get("games", 0)
                merged.rows[key]["wins"] += row.get("wins", 0)
    return merged.to_payload()


def merge_relation_kind(kind, payloads, reverse):
    merged = defaultdict(lambda: defaultdict(lambda: Counter(games=0, wins=0)))
    for payload in payloads:
        if not payload:
            continue
        for champ, rows in payload.get(kind, {}).items():
            for row in rows:
                other = row["champion"]
                merged[champ][other]["games"] += row.get("games", row.get("count", 0))
                merged[champ][other]["wins"] += row.get("wins", 0)
    return relation_table(merged, reverse=reverse)


def parse_args():
    parser = argparse.ArgumentParser(description="Build TFM2 meta dashboard data.")
    parser.add_argument(
        "--save-path",
        default=None,
        help="Optional TFM2 save file, data folder, or TeamfightManager2 appdata folder.",
    )
    parser.add_argument(
        "--export-mod-files",
        action="store_true",
        help="Also write mod-facing helper files under the game's mods directory. Off by default.",
    )
    return parser.parse_args()


def main():
    args = parse_args()
    if not BANPICK_DATA.exists():
        raise SystemExit(
            "Required file is missing: "
            f"{BANPICK_DATA}\n"
            "Re-extract the dashboard package. It should include data\\banpick-data.js and assets\\."
        )
    base = load_js_json(BANPICK_DATA)
    champions = list(base["champions"])
    mod_champions, mod_roots = load_mod_champions({champ["id"] for champ in champions})
    if mod_champions:
        champions.extend(mod_champions)
    print(
        "Mod champions: "
        f"{len(mod_champions)} loaded "
        f"from {len(mod_roots)} local/Workshop mod roots"
    )
    champion_translation_count = apply_champion_translations(champions, load_champion_translations())
    if champion_translation_count:
        print(f"Champion translations: {champion_translation_count} champions loaded from champion.i18n")
    champion_ids = [champ["id"] for champ in champions]
    item_catalog = load_item_catalog()
    print(
        "Items: "
        f"{len(item_catalog.get('byId', {}))} loaded "
        f"from {item_catalog.get('source') or 'not found'}"
    )

    save_path, manual_search_roots = latest_save(args.save_path)
    print("Save search roots:")
    for save_dir in manual_search_roots:
        marker = "exists" if save_dir.exists() else "missing"
        print(f"  - {save_dir} [manual, {marker}]")
    for save_dir in SAVE_DIRS:
        marker = "exists" if save_dir.exists() else "missing"
        print(f"  - {save_dir} [{marker}]")
    print(f"Selected save: {save_path if save_path else 'not found'}")
    print(f"Diagnostic snapshot dir: {EXPORT_DIR if EXPORT_DIR.exists() else str(EXPORT_DIR) + ' [missing]'}")
    meta_export_status = inspect_meta_export()
    meta_manifest = meta_export_status.get("manifest") or {}
    meta_source_kind = meta_manifest.get("reason") or ("meta_exporter" if meta_export_status["usable"] else "unavailable")
    save_probe_active = meta_export_status["usable"] and meta_source_kind == "save_probe"
    game_day = manifest_game_day(meta_manifest)
    if meta_export_status["usable"]:
        print(f"Diagnostic snapshot status: usable ({meta_source_kind})")
    else:
        print(f"Diagnostic snapshot status: ignored ({meta_export_status['reason']})")
    if game_day:
        print(f"Save game date: {game_day}")
    blob = decompress_save(save_path) if save_path else b""
    exporter_lookup = extract_exporter_lookup(EXPORT_DIR) if meta_export_status["usable"] else {"teams": {}, "athletes": {}}
    snapshot_lookup_ready = meta_export_status["usable"] and bool(exporter_lookup["teams"] or exporter_lookup["athletes"])
    save_lookup_fallback = (
        {"teams": {}, "athletes": {}}
        if snapshot_lookup_ready
        else extract_save_lookup(blob) if blob else {"teams": {}, "athletes": {}}
    )
    save_lookup = {
        "teams": exporter_lookup["teams"] or save_lookup_fallback["teams"],
        # Replay athlete ids are safest when they come from the same exporter
        # snapshot as match_replays. The save fallback is order-based and can
        # attach the wrong current-roster name to old replay snapshots.
        "athletes": exporter_lookup["athletes"],
    }
    team_league_lookup = parse_team_league_lookup(EXPORT_DIR) if meta_export_status["usable"] else {}
    replay_schedule_dates = parse_year_schedule_dates(EXPORT_DIR, max_day=game_day) if meta_export_status["usable"] else {
        "leagueRounds": {},
        "leagueSlots": [],
        "tournament": [],
        "tournamentSlots": [],
        "tournamentMatchDates": {},
        "tournamentMatchIds": [],
        "tournamentScheduleSummary": {},
        "matchReports": {"bySignature": {}, "reports": 0, "sets": 0},
    }
    match_stat_versions = parse_match_stat_versions(EXPORT_DIR) if meta_export_status["usable"] else {}
    snapshot_lookup_label = "save_probe" if save_probe_active else "live_snapshot"
    team_lookup_source = snapshot_lookup_label if exporter_lookup["teams"] else "save_fallback"
    athlete_lookup_source = snapshot_lookup_label if exporter_lookup["athletes"] else "unavailable"
    print(
        "Lookup: "
        f"teams={len(save_lookup['teams'])} ({team_lookup_source}) "
        f"athletes={len(save_lookup['athletes'])} ({athlete_lookup_source})"
    )
    print(f"Leagues: {len(team_league_lookup)} teams mapped to region/division")
    print(
        "Replay dates: "
        f"{len(replay_schedule_dates.get('leagueRounds') or {})} league rounds, "
        f"{len(replay_schedule_dates.get('leagueSlots') or [])} league slots, "
        f"{len(replay_schedule_dates.get('tournament') or [])} tournament slots, "
        f"{len(replay_schedule_dates.get('tournamentMatchDates') or {})} scheduled tournament replay ids, "
        f"{len((replay_schedule_dates.get('matchReports') or {}).get('bySignature') or {})} report-set signatures"
    )
    if match_stat_versions:
        print(f"Replay versions: {len(match_stat_versions)} rows loaded from match_stats.debug.txt")
    if snapshot_lookup_ready:
        news_rows = []
        draft_scan = {"groups": 0, "mentions": {}, "pairs": {}}
        print("Fast save scan: skipped fallback news/draft/name scans because save data snapshot is usable")
    else:
        news_rows = extract_news_champion_stats(blob, champion_ids) if blob else []
        draft_scan = extract_draft_like_groups(blob, champion_ids) if blob else {"groups": 0, "mentions": {}, "pairs": {}}

    current_champion_info = {}
    current_champion_info_count = 0
    if meta_export_status["usable"]:
        current_champion_info = load_champion_debug_values(EXPORT_DIR / "champion_info_sheet.debug.txt", champion_ids)
        base_champion_info = load_champion_debug_values(EXPORT_DIR / "pre_patch_data.debug.txt", champion_ids)
        current_champion_info_count = apply_current_champion_info(champions, current_champion_info, base_champion_info)
        if current_champion_info_count:
            print(f"Champion current info: {current_champion_info_count} champions loaded from save_probe champion_info_sheet")

    current_patch = extract_current_patch_summary(blob, champion_ids, save_path) if blob else {"meta": {"source": None, "versions": [], "changeCount": 0}, "patches": {}, "changes": []}
    print(f"Current patch: versions={len(current_patch['meta']['versions'])} changes={current_patch['meta']['changeCount']}")
    if meta_export_status["usable"]:
        exported, exported_by_version = parse_debug_champion_stats_versions(
            EXPORT_DIR / "champion_patch_statistics.debug.txt",
            champion_ids,
        )
    else:
        exported, exported_by_version = {}, {}
    tournament_stats = merge_stats(champions, news_rows, draft_scan, exported)
    if meta_export_status["usable"]:
        (
            solo_stats_raw,
            solo_relationships,
            solo_stats_by_version_raw,
            solo_relationships_by_version,
            solo_lane_synergies,
            solo_lane_synergies_by_version,
        ) = parse_solo_rank_stats(EXPORT_DIR / "solo_rank_matches.debug.txt", champion_ids, item_catalog)
        solo_stats = normalize_scope(champions, solo_stats_raw, draft_scan)
        solo_replay_ids = parse_solo_rank_replay_ids(EXPORT_DIR / "solo_rank_matches.debug.txt")
        (
            tournament_relationships,
            tournament_relationships_by_version,
            tournament_lane_synergies,
            tournament_lane_synergies_by_version,
        ) = parse_match_replay_relations(EXPORT_DIR / "match_replays.debug.txt", champion_ids, solo_replay_ids)
        tournament_match_analysis = parse_match_analysis(
            EXPORT_DIR / "match_replays.debug.txt",
            champion_ids,
            save_lookup,
            solo_replay_ids,
            limit=None,
            item_catalog=item_catalog,
        )
        attach_match_league_meta(tournament_match_analysis, team_league_lookup)
        match_stat_version_summary = apply_match_stat_versions(tournament_match_analysis, match_stat_versions)
        replay_date_inference = infer_match_analysis_dates(tournament_match_analysis, team_league_lookup, replay_schedule_dates)
        patch_range_summary = normalize_league_series_patch_dates(tournament_match_analysis, team_league_lookup, replay_schedule_dates, game_day)
        replay_date_inference["matchStatVersions"] = match_stat_version_summary
        replay_date_inference["patchRangeCorrection"] = patch_range_summary
        tournament_match_analysis, future_replay_filter = filter_future_match_analysis(tournament_match_analysis, game_day)
        replay_date_inference["futureFilter"] = future_replay_filter
        if future_replay_filter.get("removed"):
            print(
                "Replay dates: "
                f"filtered {future_replay_filter['removed']} future tournament matches "
                f"after {future_replay_filter.get('maxGameDate')}"
            )
        (
            tournament_relationships,
            tournament_relationships_by_version,
            tournament_lane_synergies,
            tournament_lane_synergies_by_version,
        ) = aggregate_match_relations(tournament_match_analysis)
        solo_match_analysis = parse_solo_match_analysis(
            EXPORT_DIR / "solo_rank_matches.debug.txt",
            champion_ids,
            save_lookup,
            limit=600,
            item_catalog=item_catalog,
        )
        full_match_analysis = sorted(
            tournament_match_analysis + solo_match_analysis,
            key=lambda row: (row.get("resultTime") or row.get("date") or "", NumberLike(str(row.get("id", "")).split("-")[-1])),
            reverse=True,
        )
        match_analysis = full_match_analysis[:1200]
        match_analysis_keys = {match_analysis_row_key(row) for row in match_analysis}
        match_analysis_versions = {str(row.get("version") or "unknown") for row in match_analysis}
        match_analysis_chunk_payloads = defaultdict(list)
        for row in full_match_analysis:
            if match_analysis_row_key(row) in match_analysis_keys:
                continue
            version = str(row.get("version") or "unknown")
            if version in match_analysis_versions:
                continue
            if not re.match(r"^\d{4}-\d{2}-\d{2}$", str(row.get("dateKey") or "")):
                continue
            match_analysis_chunk_payloads[version].append(row)
        match_analysis_chunk_payloads = {
            version: sorted(
                rows,
                key=lambda row: (row.get("resultTime") or row.get("date") or "", NumberLike(str(row.get("id", "")).split("-")[-1])),
                reverse=True,
            )
            for version, rows in match_analysis_chunk_payloads.items()
        }
        replay_tournament_stats, replay_tournament_stats_by_version = aggregate_match_analysis_stats(
            full_match_analysis,
            champion_ids,
            item_catalog,
        )
        replay_tournament_stats_by_league_raw, replay_tournament_stats_by_patch_league_raw = aggregate_match_analysis_stats_by_bucket(
            full_match_analysis,
            champion_ids,
            item_catalog,
        )
        (
            tournament_relationships_by_league,
            tournament_relationships_by_patch_league,
            tournament_lane_synergies_by_league,
            tournament_lane_synergies_by_patch_league,
        ) = aggregate_match_relations_by_bucket(full_match_analysis)
        solo_stats_by_region_raw, solo_stats_by_patch_region_raw = aggregate_match_analysis_stats_by_bucket(
            full_match_analysis,
            champion_ids,
            item_catalog,
            sources={"solo"},
        )
        (
            solo_relationships_by_region,
            solo_relationships_by_patch_region,
            solo_lane_synergies_by_region,
            solo_lane_synergies_by_patch_region,
        ) = aggregate_match_relations_by_bucket(full_match_analysis, sources={"solo"})
        if not has_collected_stats(tournament_stats) and should_prefer_replay_stats(tournament_stats, replay_tournament_stats):
            print(
                "Tournament stats: using replay analysis stats because exported champion_patch_statistics "
                f"is unavailable ({stats_total_matches(replay_tournament_stats)} replay matches)"
            )
            tournament_stats = normalize_scope(champions, replay_tournament_stats, draft_scan)
            # Keep patch totals aligned with the same filtered replay set used for global tournament stats.
            exported_by_version = dict(replay_tournament_stats_by_version)
    else:
        empty_rel = {"groups": 0, "pairs": {}, "counters": {}}
        solo_stats_raw = {}
        solo_relationships = empty_rel
        solo_stats_by_version_raw = {}
        solo_relationships_by_version = {}
        solo_lane_synergies = empty_lane_synergy_payload()
        solo_lane_synergies_by_version = {}
        solo_stats = normalize_scope(champions, solo_stats_raw, draft_scan)
        solo_replay_ids = set()
        tournament_relationships = empty_rel
        tournament_relationships_by_version = {}
        tournament_lane_synergies = empty_lane_synergy_payload()
        tournament_lane_synergies_by_version = {}
        full_match_analysis = []
        match_analysis = []
        match_analysis_chunk_payloads = {}
        solo_match_analysis = []
        replay_date_inference = {"inferred": 0, "source": "none"}
        replay_tournament_stats_by_version = {}
        replay_tournament_stats_by_league_raw = {}
        replay_tournament_stats_by_patch_league_raw = {}
        solo_stats_by_region_raw = {}
        solo_stats_by_patch_region_raw = {}
        tournament_relationships_by_league = {}
        tournament_relationships_by_patch_league = {}
        tournament_lane_synergies_by_league = {}
        tournament_lane_synergies_by_patch_league = {}
        solo_relationships_by_region = {}
        solo_relationships_by_patch_region = {}
        solo_lane_synergies_by_region = {}
        solo_lane_synergies_by_patch_region = {}
    combined_stats = combine_scope_stats(champions, tournament_stats, solo_stats, draft_scan, item_catalog)

    tournament_stats_by_league = {
        bucket: normalize_scope(champions, rows, draft_scan)
        for bucket, rows in replay_tournament_stats_by_league_raw.items()
    }
    stats_by_patch_league = {
        version: {
            bucket: normalize_scope(champions, rows, draft_scan)
            for bucket, rows in bucket_rows.items()
        }
        for version, bucket_rows in replay_tournament_stats_by_patch_league_raw.items()
    }
    solo_stats_by_region = {
        bucket: normalize_scope(champions, rows, draft_scan)
        for bucket, rows in solo_stats_by_region_raw.items()
    }
    stats_by_region_scope = {}
    for bucket in league_stat_bucket_keys():
        tournament_bucket = tournament_stats_by_league.get(bucket) or normalize_scope(champions, {}, draft_scan)
        solo_bucket = solo_stats_by_region.get(bucket) or normalize_scope(champions, {}, draft_scan)
        stats_by_region_scope[bucket] = {
            "overall": combine_scope_stats(champions, tournament_bucket, solo_bucket, draft_scan, item_catalog),
            "tournament": tournament_bucket,
            "solo": solo_bucket,
        }

    for scope_stats in [tournament_stats, solo_stats, combined_stats, *tournament_stats_by_league.values()]:
        for champ in champions:
            scope_stats[champ["id"]]["tier"] = calculated_tier(scope_stats[champ["id"]])
    for bucket_rows in stats_by_patch_league.values():
        for scope_stats in bucket_rows.values():
            for champ in champions:
                scope_stats[champ["id"]]["tier"] = calculated_tier(scope_stats[champ["id"]])
    for bucket_rows in stats_by_region_scope.values():
        for scope_stats in bucket_rows.values():
            for champ in champions:
                scope_stats[champ["id"]]["tier"] = calculated_tier(scope_stats[champ["id"]])
    overall_relationships = merge_relationship_payloads(tournament_relationships, solo_relationships)
    overall_lane_synergies = merge_lane_synergy_payloads(tournament_lane_synergies, solo_lane_synergies)
    patch_versions = sorted(
        set(exported_by_version)
        | set(replay_tournament_stats_by_patch_league_raw)
        | set(solo_stats_by_patch_region_raw)
        | set(solo_stats_by_version_raw)
        | set(tournament_relationships_by_version)
        | set(tournament_relationships_by_patch_league)
        | set(solo_relationships_by_patch_region)
        | set(solo_relationships_by_version),
        key=version_sort_key,
    )
    stats_by_patch_region_scope = {}
    for version in patch_versions:
        version_rows = {}
        tournament_patch_buckets = stats_by_patch_league.get(version, {})
        solo_patch_raw = solo_stats_by_patch_region_raw.get(version, {})
        for bucket in league_stat_bucket_keys():
            tournament_bucket = tournament_patch_buckets.get(bucket) or normalize_scope(champions, {}, draft_scan)
            solo_bucket = normalize_scope(champions, solo_patch_raw.get(bucket, {}), draft_scan)
            version_rows[bucket] = {
                "overall": combine_scope_stats(champions, tournament_bucket, solo_bucket, draft_scan, item_catalog),
                "tournament": tournament_bucket,
                "solo": solo_bucket,
            }
        stats_by_patch_region_scope[version] = version_rows
    for version_rows in stats_by_patch_region_scope.values():
        for bucket_rows in version_rows.values():
            for scope_stats in bucket_rows.values():
                for champ in champions:
                    scope_stats[champ["id"]]["tier"] = calculated_tier(scope_stats[champ["id"]])
    stats_by_patch = {}
    relationships_by_patch = {}
    lane_synergies_by_patch = {}

    empty_rel = {"groups": 0, "pairs": {}, "counters": {}}
    for version in patch_versions:
        tournament_v = merge_stats(champions, [], draft_scan, exported_by_version.get(version, {}))
        solo_v = normalize_scope(champions, solo_stats_by_version_raw.get(version, {}), draft_scan)
        combined_v = combine_scope_stats(champions, tournament_v, solo_v, draft_scan, item_catalog)
        for scope_stats in [tournament_v, solo_v, combined_v]:
            for champ in champions:
                scope_stats[champ["id"]]["tier"] = calculated_tier(scope_stats[champ["id"]])
        tournament_rel_v = tournament_relationships_by_version.get(version, empty_rel)
        solo_rel_v = solo_relationships_by_version.get(version, empty_rel)
        overall_rel_v = merge_relationship_payloads(tournament_rel_v, solo_rel_v)
        tournament_lane_v = tournament_lane_synergies_by_version.get(version, empty_lane_synergy_payload())
        solo_lane_v = solo_lane_synergies_by_version.get(version, empty_lane_synergy_payload())
        stats_by_patch[version] = {
            "overall": combined_v,
            "tournament": tournament_v,
            "solo": solo_v,
        }
        relationships_by_patch[version] = {
            "overall": overall_rel_v,
            "tournament": tournament_rel_v,
            "solo": solo_rel_v,
        }
        lane_synergies_by_patch[version] = {
            "overall": merge_lane_synergy_payloads(tournament_lane_v, solo_lane_v),
            "tournament": tournament_lane_v,
            "solo": solo_lane_v,
        }

    relationships_by_region_scope = {}
    lane_synergies_by_region_scope = {}
    for bucket in league_stat_bucket_keys():
        tournament_rel = tournament_relationships_by_league.get(bucket, empty_rel)
        solo_rel = solo_relationships_by_region.get(bucket, empty_rel)
        tournament_lane = tournament_lane_synergies_by_league.get(bucket, empty_lane_synergy_payload())
        solo_lane = solo_lane_synergies_by_region.get(bucket, empty_lane_synergy_payload())
        relationships_by_region_scope[bucket] = {
            "overall": merge_relationship_payloads(tournament_rel, solo_rel),
            "tournament": tournament_rel,
            "solo": solo_rel,
        }
        lane_synergies_by_region_scope[bucket] = {
            "overall": merge_lane_synergy_payloads(tournament_lane, solo_lane),
            "tournament": tournament_lane,
            "solo": solo_lane,
        }

    relationships_by_patch_region_scope = {}
    lane_synergies_by_patch_region_scope = {}
    for version in patch_versions:
        rel_version_rows = {}
        lane_version_rows = {}
        tournament_rel_buckets = tournament_relationships_by_patch_league.get(version, {})
        solo_rel_buckets = solo_relationships_by_patch_region.get(version, {})
        tournament_lane_buckets = tournament_lane_synergies_by_patch_league.get(version, {})
        solo_lane_buckets = solo_lane_synergies_by_patch_region.get(version, {})
        for bucket in league_stat_bucket_keys():
            tournament_rel = tournament_rel_buckets.get(bucket, empty_rel)
            solo_rel = solo_rel_buckets.get(bucket, empty_rel)
            tournament_lane = tournament_lane_buckets.get(bucket, empty_lane_synergy_payload())
            solo_lane = solo_lane_buckets.get(bucket, empty_lane_synergy_payload())
            rel_version_rows[bucket] = {
                "overall": merge_relationship_payloads(tournament_rel, solo_rel),
                "tournament": tournament_rel,
                "solo": solo_rel,
            }
            lane_version_rows[bucket] = {
                "overall": merge_lane_synergy_payloads(tournament_lane, solo_lane),
                "tournament": tournament_lane,
                "solo": solo_lane,
            }
        relationships_by_patch_region_scope[version] = rel_version_rows
        lane_synergies_by_patch_region_scope[version] = lane_version_rows

    attach_meta_score_group(
        champions,
        {
            "overall": combined_stats,
            "tournament": tournament_stats,
            "solo": solo_stats,
        },
        {
            "overall": overall_relationships,
            "tournament": tournament_relationships,
            "solo": solo_relationships,
        },
    )
    for version, scope_stats in stats_by_patch.items():
        attach_meta_score_group(champions, scope_stats, relationships_by_patch.get(version, {}))
    for bucket, scope_stats in tournament_stats_by_league.items():
        attach_meta_scores(champions, scope_stats, tournament_relationships_by_league.get(bucket, {}), f"tournament:{bucket}")
    for version, bucket_rows in stats_by_patch_league.items():
        for bucket, scope_stats in bucket_rows.items():
            attach_meta_scores(
                champions,
                scope_stats,
                (tournament_relationships_by_patch_league.get(version, {}) or {}).get(bucket, {}),
                f"tournament:{bucket}",
            )
    for bucket, scope_rows in stats_by_region_scope.items():
        attach_meta_score_group(champions, scope_rows, relationships_by_region_scope.get(bucket, {}))
    for version, bucket_rows in stats_by_patch_region_scope.items():
        rel_bucket_rows = relationships_by_patch_region_scope.get(version, {})
        for bucket, scope_rows in bucket_rows.items():
            attach_meta_score_group(champions, scope_rows, rel_bucket_rows.get(bucket, {}))

    refresh_tiers(champions, combined_stats, tournament_stats, solo_stats, *tournament_stats_by_league.values())
    for scope_stats in stats_by_patch.values():
        refresh_tiers(champions, *scope_stats.values())
    for bucket_rows in stats_by_patch_league.values():
        refresh_tiers(champions, *bucket_rows.values())
    for scope_rows in stats_by_region_scope.values():
        refresh_tiers(champions, *scope_rows.values())
    for version_rows in stats_by_patch_region_scope.values():
        for scope_rows in version_rows.values():
            refresh_tiers(champions, *scope_rows.values())

    meta_export_ts = latest_meta_export_timestamp()
    save_ts = save_path.stat().st_mtime if save_path else None
    manifest_save_path = meta_manifest.get("save") or meta_manifest.get("save_path") or meta_manifest.get("save_file")
    manifest_matches_selected_save = False
    if manifest_save_path and save_path:
        manifest_path = Path(manifest_save_path)
        if manifest_path.is_absolute():
            manifest_matches_selected_save = str(manifest_path).lower() == str(save_path).lower()
        else:
            manifest_matches_selected_save = manifest_path.name.lower() == save_path.name.lower()
    export_save_delta = round(meta_export_ts - save_ts) if meta_export_ts and save_ts else None
    export_save_mismatched = (
        export_save_delta is not None
        and abs(export_save_delta) > 600
        and not (save_probe_active and manifest_matches_selected_save)
    )
    if export_save_mismatched:
        print(
            "WARNING: selected save and diagnostic snapshot files differ by "
            f"{abs(export_save_delta) // 60} minutes. Select the same save again "
            "or let auto refresh regenerate the save snapshot."
        )

    generated_at = datetime.now().isoformat(timespec="seconds")
    core_item_builds = build_core_item_builds(full_match_analysis, generated_at, save_path, patch_versions, item_catalog)
    ai_champion_policy = build_ai_champion_policy(
        champions,
        combined_stats,
        stats_by_patch,
        patch_versions,
        generated_at,
        save_path,
        export_mod_files=args.export_mod_files,
    )

    core_item_build_paths_preview = [CORE_ITEM_BUILDS_OUT]
    ai_champion_policy_paths_preview = [AI_CHAMPION_POLICY_OUT]
    if args.export_mod_files:
        core_item_build_paths_preview.extend([CORE_ITEM_BUILDS_MOD_OUT, CORE_ITEM_BUILDS_MOD_DATA_OUT])
        core_item_build_paths_preview.extend(workshop_mod_policy_paths("tfm2_meta_item_delegate", "core-item-builds.json"))
        core_item_build_paths_preview.extend(workshop_mod_policy_paths("tfm2_meta_item_delegate", Path("data") / "core-item-builds.json"))
        core_item_build_paths_preview.extend(configured_policy_target_paths("core_item_builds"))
        core_item_build_paths_preview.extend(configured_policy_target_paths("core_item_builds_data"))
        ai_champion_policy_paths_preview.extend([AI_CHAMPION_POLICY_MOD_OUT, CHAMPION_TIER_POLICY_MOD_OUT])
        ai_champion_policy_paths_preview.extend(workshop_mod_policy_paths("tfm2_ai_banpick_probe", "ai_champion_policy.tsv"))
        ai_champion_policy_paths_preview.extend(workshop_mod_policy_paths("tfm2_meta_champion_tiers", "champion_tier_policy.tsv"))
        ai_champion_policy_paths_preview.extend(configured_policy_target_paths("ai_champion_policy"))
        ai_champion_policy_paths_preview.extend(configured_policy_target_paths("champion_tier_policy"))

    detailed_patch_versions = recent_detail_patches(patch_versions, full_match_analysis, game_day)
    pruned_patch_count = max(0, len(patch_versions) - len(detailed_patch_versions))
    if pruned_patch_count:
        print(
            "Dashboard payload: "
            f"keeping detailed region/league data for {len(detailed_patch_versions)} recent patches "
            f"and compacting {pruned_patch_count} older patch-detail buckets"
        )

    stats_by_patch_league_payload = prune_patch_map(stats_by_patch_league, detailed_patch_versions)
    stats_by_patch_region_scope_payload = prune_patch_map(stats_by_patch_region_scope, detailed_patch_versions)
    relationships_by_patch_league_payload = prune_patch_map(tournament_relationships_by_patch_league, detailed_patch_versions)
    relationships_by_patch_region_scope_payload = prune_patch_map(relationships_by_patch_region_scope, detailed_patch_versions)
    lane_synergies_by_patch_league_payload = prune_patch_map(tournament_lane_synergies_by_patch_league, detailed_patch_versions)
    lane_synergies_by_patch_region_scope_payload = prune_patch_map(lane_synergies_by_patch_region_scope, detailed_patch_versions)
    detailed_patch_set = set(detailed_patch_versions)
    patch_detail_chunk_payloads = {}
    for version in patch_versions:
        if version in detailed_patch_set:
            continue
        patch_detail_chunk_payloads[version] = {
            "version": version,
            "generatedAt": generated_at,
            "statsByPatchLeague": stats_by_patch_league.get(version, {}),
            "statsByPatchRegionScope": stats_by_patch_region_scope.get(version, {}),
            "relationshipsByPatchLeague": tournament_relationships_by_patch_league.get(version, {}),
            "relationshipsByPatchRegionScope": relationships_by_patch_region_scope.get(version, {}),
            "laneSynergiesByPatchLeague": tournament_lane_synergies_by_patch_league.get(version, {}),
            "laneSynergiesByPatchRegionScope": lane_synergies_by_patch_region_scope.get(version, {}),
        }
    patch_detail_chunks = write_patch_detail_chunks(patch_detail_chunk_payloads)
    if patch_detail_chunks:
        print(f"Dashboard payload: wrote {len(patch_detail_chunks)} patch detail chunks to {META_CHUNKS_DIR}")
    match_analysis_chunks = write_match_analysis_chunks(match_analysis_chunk_payloads)
    if match_analysis_chunks:
        chunk_rows = sum(info.get("rows", 0) for info in match_analysis_chunks.values())
        print(f"Dashboard payload: wrote {len(match_analysis_chunks)} match analysis chunks ({chunk_rows} rows) to {META_CHUNKS_DIR}")

    visible_snapshot_active = bool(exported) and meta_export_status["usable"]
    visible_meta_exporter = visible_snapshot_active and not save_probe_active
    visible_team_lookup_source = team_lookup_source
    visible_athlete_lookup_source = athlete_lookup_source
    exact_replay_names = athlete_lookup_source in ("live_snapshot", "save_probe")
    if save_probe_active:
        match_analysis_source = "selected save data decoded through save_probe; match replay and solo rank records are generated from that save snapshot"
    elif meta_export_status["usable"]:
        match_analysis_source = "live diagnostic snapshot match replay and solo rank records"
    else:
        match_analysis_source = "disabled: selected save data could not be decoded, so stale replay diagnostic files were ignored"

    payload = {
        "generatedAt": generated_at,
        "save": {
            "path": str(save_path) if save_path else None,
            "lastModified": datetime.fromtimestamp(save_path.stat().st_mtime).isoformat(timespec="seconds") if save_path else None,
            "searchRoots": [str(path) for path in manual_search_roots + SAVE_DIRS],
        },
        "sources": {
            "championInfo": "save_probe champion_info_sheet" if current_champion_info_count else "bundled dashboard champion data",
            "championCurrentInfo": current_champion_info_count,
            "saveNewsStats": len(news_rows),
            "draftLikeGroups": draft_scan["groups"],
            "metaExporter": visible_meta_exporter,
            "saveProbe": save_probe_active,
            "metaExportSource": meta_source_kind,
            "metaExportUsable": meta_export_status["usable"],
            "metaExportReason": meta_export_status["reason"],
            "replaySummaries": load_replay_summary_count(meta_export_status["usable"]),
            "matchAnalysis": len(match_analysis),
            "soloMatchAnalysis": len(solo_match_analysis),
            "soloRankMatches": solo_relationships.get("groups", 0),
            "tournamentRelationshipMatches": tournament_relationships.get("groups", 0),
            "soloReplayIds": len(solo_replay_ids),
            "excludedSoloReplayIds": 0,
            "metaExportLastModified": datetime.fromtimestamp(meta_export_ts).isoformat(timespec="seconds") if meta_export_ts else None,
            "metaExportSaveDeltaSeconds": export_save_delta,
            "metaExportMismatched": export_save_mismatched,
            "teamLookupSource": visible_team_lookup_source,
            "athleteLookupSource": visible_athlete_lookup_source,
            "exactReplayAthleteNames": exact_replay_names,
            "matchAnalysisSource": match_analysis_source,
            "itemCatalogSource": item_catalog.get("source"),
            "itemCatalogItems": len(item_catalog.get("byId", {})),
            "coreItemBuilds": str(CORE_ITEM_BUILDS_OUT),
            "coreItemBuildsMod": str(CORE_ITEM_BUILDS_MOD_OUT) if args.export_mod_files else None,
            "coreItemBuildsModTargets": [str(path) for path in unique_paths(core_item_build_paths_preview[1:])] if args.export_mod_files else [],
            "coreItemBuildsTournamentMatches": core_item_builds["sources"]["tournamentMatches"],
            "aiChampionPolicy": str(AI_CHAMPION_POLICY_OUT),
            "aiChampionPolicyMod": str(AI_CHAMPION_POLICY_MOD_OUT) if args.export_mod_files else None,
            "championTierPolicyMod": str(CHAMPION_TIER_POLICY_MOD_OUT) if args.export_mod_files else None,
            "aiChampionPolicyModTargets": [str(path) for path in unique_paths(ai_champion_policy_paths_preview[1:])] if args.export_mod_files else [],
            "policyTargetConfig": [str(path) for path in active_dashboard_config_paths()],
            "modFileExport": bool(args.export_mod_files),
            "aiChampionPolicySource": ai_champion_policy["source"]["label"],
            "aiChampionPolicyScoredChampions": ai_champion_policy["source"]["scoredChampions"],
            "leagueMappedTeams": len(team_league_lookup),
            "replayDateInference": replay_date_inference,
            "dashboardDetailedPatchLimit": DASHBOARD_DETAILED_PATCH_LIMIT,
            "dashboardDetailedRecentDays": DASHBOARD_DETAILED_RECENT_DAYS,
            "dashboardDetailedPatches": detailed_patch_versions,
            "dashboardPrunedPatchDetailBuckets": pruned_patch_count,
            "dashboardPatchDetailChunks": len(patch_detail_chunks),
            "dashboardMatchAnalysisChunks": len(match_analysis_chunks),
            "dashboardMatchAnalysisChunkRows": sum(info.get("rows", 0) for info in match_analysis_chunks.values()),
        },
        "saveLookup": save_lookup,
        "itemCatalog": item_catalog,
        "leagueFilters": league_filter_payload(),
        "patches": patch_versions,
        "patchDetailChunks": patch_detail_chunks,
        "matchAnalysisChunks": match_analysis_chunks,
        "currentPatch": current_patch,
        "scoreModel": {
            "version": SCORE_MODEL_VERSION,
            "minPicks": SCORE_MIN_PICKS,
            "counterMinGames": SCORE_COUNTER_MIN_GAMES,
            "priorPicks": SCORE_PRIOR_PICKS,
            "rolePriorPicks": SCORE_ROLE_PRIOR_PICKS,
            "roleMinPicks": SCORE_ROLE_MIN_PICKS,
            "usableRoleMinPicks": SCORE_USABLE_ROLE_MIN_PICKS,
            "usableRoleStrongMinPicks": SCORE_USABLE_ROLE_STRONG_MIN_PICKS,
            "reliabilityPicks": SCORE_RELIABILITY_PICKS,
            "wilsonZ": SCORE_WILSON_Z,
            "winRiskZ": SCORE_WIN_RISK_Z,
            "weights": {
                "power": SCORE_POWER_WEIGHT,
                "draft": SCORE_DRAFT_WEIGHT,
                "versatility": SCORE_VERSATILITY_WEIGHT,
                "tournamentDraft": SCORE_TOURNAMENT_DRAFT_WEIGHT,
                "allPickDraft": SCORE_ALL_PICK_DRAFT_WEIGHT,
            },
            "gradeThresholds": {"S": 85, "A": 70, "B": 55, "C": 40},
            "notes": "Role-aware v6 scores champion x actual position first. Champion-level Power uses the best deployable role after empirical-Bayes risk adjustment; Draft Pressure prioritizes tournament pick+ban presence when bans exist and falls back to pick pressure for solo-only scopes; Versatility counts only roles with both sample and non-poor role power. Reliability is displayed separately and is not multiplied into the score. K/D is not used.",
        },
        "aiChampionPolicy": {
            "source": ai_champion_policy["source"],
            "paths": [str(path) for path in unique_paths(ai_champion_policy_paths_preview)],
            "formula": "native draft score += config clamp((overall - neutral) / divisor, min_bias, max_bias)",
        },
        "champions": champions,
        "skillIconAtlas": base["meta"].get("skillIconAtlas", {}),
        "stats": combined_stats,
        "statsByScope": {
            "overall": combined_stats,
            "tournament": tournament_stats,
            "solo": solo_stats,
        },
        "statsByPatch": stats_by_patch,
        "statsByLeague": tournament_stats_by_league,
        "statsByPatchLeague": stats_by_patch_league_payload,
        "statsByRegionScope": stats_by_region_scope,
        "statsByPatchRegionScope": stats_by_patch_region_scope_payload,
        "relationships": overall_relationships["pairs"],
        "relationshipsByScope": {
            "overall": overall_relationships,
            "tournament": tournament_relationships,
            "solo": solo_relationships,
        },
        "relationshipsByPatch": relationships_by_patch,
        "relationshipsByLeague": tournament_relationships_by_league,
        "relationshipsByPatchLeague": relationships_by_patch_league_payload,
        "relationshipsByRegionScope": relationships_by_region_scope,
        "relationshipsByPatchRegionScope": relationships_by_patch_region_scope_payload,
        "laneSynergiesByScope": {
            "overall": overall_lane_synergies,
            "tournament": tournament_lane_synergies,
            "solo": solo_lane_synergies,
        },
        "laneSynergiesByPatch": lane_synergies_by_patch,
        "laneSynergiesByLeague": tournament_lane_synergies_by_league,
        "laneSynergiesByPatchLeague": lane_synergies_by_patch_league_payload,
        "laneSynergiesByRegionScope": lane_synergies_by_region_scope,
        "laneSynergiesByPatchRegionScope": lane_synergies_by_patch_region_scope_payload,
        "matchAnalysis": match_analysis,
        "notes": [
            "챔피언 이름, 아이콘, 스킬, 기본 스탯은 게임 번들에서 직접 추출했습니다.",
            "대회 승률, 픽률, 밴률은 리플레이 분석 통계를 우선 사용하고, 없을 때 champion_patch_statistics를 사용합니다.",
            "솔랭 승률과 챔피언별 성과는 solo_rank_matches를 합산합니다.",
            "시너지와 상대 지표는 실제 리플레이/솔랭 경기에서 같은 팀 또는 상대 팀으로 만난 표본을 집계합니다.",
        ],
    }

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(
        "window.TFM2_META_DATA=" + json.dumps(payload, ensure_ascii=False, separators=(",", ":")) + ";\n",
        encoding="utf-8",
    )
    core_item_build_paths = write_core_item_builds(core_item_builds, export_mod_files=args.export_mod_files)
    ai_champion_policy_paths = write_ai_champion_policy(ai_champion_policy, export_mod_files=args.export_mod_files)
    print(f"Wrote {OUT}")
    for path in core_item_build_paths:
        print(f"Wrote {path}")
    for path in ai_champion_policy_paths:
        print(f"Wrote {path}")
    print(
        f"champions={len(champions)} news_stats={len(news_rows)} tournament_matches={tournament_relationships.get('groups', 0)} solo_matches={solo_relationships.get('groups', 0)} snapshot={bool(exported) and meta_export_status['usable']} source={meta_source_kind}"
    )


if __name__ == "__main__":
    main()
