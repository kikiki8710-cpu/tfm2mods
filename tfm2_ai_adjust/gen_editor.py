import io, os, re, sys
GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
DESCSRC = r"C:\tfm2mods\ai_adjust_editor\src\main.rs"
MODSRC  = r"C:\tfm2mods\tfm2_ai_adjust\src"
OUT_UI = r"C:\tfm2mods\tfm2_ai_adjust\ui_inject\aiadj_modal.ui"
OUT_RS = r"C:\tfm2mods\tfm2_ai_adjust\src\knobs.rs"
Q = chr(34)

# ── 설명 파싱 ──────────────────────────────────────────────────────────────
# ⚠main.rs 에는 `"key" => "값"` 표가 **둘** 있다: ①기본값표 ②설명표(fn desc_static).
#   전체를 훑으면 기본값("640" 같은 숫자)이 설명으로 들어간다 → desc_static 본문만 읽는다.
src = io.open(DESCSRC, encoding='utf-8').read()
_m = re.search(r'fn desc_static\s*\([^)]*\)[^{]*\{', src)
if not _m:
    sys.exit("desc_static 을 못 찾음 — main.rs 구조가 바뀌었는지 확인")
_i, _d = _m.end(), 1
while _d:
    _d += (src[_i] == '{') - (src[_i] == '}')
    _i += 1
DESCBODY = src[_m.end():_i - 1]
descs = {m.group(1): m.group(2)
         for m in re.finditer(r'"([a-z_][a-z0-9_]*)"\s*=>\s*"((?:[^"\\]|\\.)*)"', DESCBODY)}

# ── 모드 소스에서 "실배선/별칭" 파악 ────────────────────────────────────────
# 노브가 나오는 자리는 3종인데 뒤 2종은 읽기가 아니다.
#   ①실배선 tune("k") / 파서 `"k" => { ATOMIC.store(..) }` / 별칭 `"k" => "new"`
#   ②skip_groups.rs 의 키 나열   ③g(&["k",..]) 게이트 나열
# ②③에만 있으면 편집기에서 값을 넣어도 아무것도 안 변한다(= 죽은 컨트롤).
_files = {}
for _r, _, _fs in os.walk(MODSRC):
    for _f in _fs:
        if _f.endswith('.rs') and _f != 'knobs.rs' and '.bak' not in _f:
            _files[_f] = io.open(os.path.join(_r, _f), encoding='utf-8', errors='replace').read()
ALIAS = {}
for _t in _files.values():
    for m in re.finditer(r'((?:"[a-z0-9_]+"\s*\|\s*)*"[a-z0-9_]+")\s*=>\s*"([a-z0-9_]+)"\s*,', _t):
        for s in re.findall(r'"([a-z0-9_]+)"', m.group(1)):
            if s != m.group(2):
                ALIAS[s] = m.group(2)

def wired(k):
    for fn, txt in _files.items():
        if fn == 'skip_groups.rs':
            continue
        lines = txt.splitlines()
        for m in re.finditer('"%s"' % k, txt):
            line = lines[txt.count('\n', 0, m.start())]
            if re.search(r'\bg\(&\[', line):
                continue
            return True
    return False

DEADMARK = re.compile('작동하지 않|값 무반영|死레버')

def desc_of(k):
    """설명은 별칭 새이름 쪽에 있는 경우가 있다(oi_*→nx_*, ep_*→sn_*)."""
    return descs.get(k) or descs.get(ALIAS.get(k, ''), '')

SECTIONS = [
 # ★[09-05] t_ttd·t_gb 제거 — 설명표가 스스로 "⛔작동하지 않습니다(050 하드코딩·값 무반영)"라 밝힌 死레버.
 ("§ 전체 성향 다이얼", ["t_engage","t_recall"]),
 ("§ 신규: 적 포탑 회피", ["tower_threat","tower_range","tower_dps"]),
 ("§ 신규: 교전판단 (한타)", ["numbers_threat","numbers_range","ally_tower_hp","ally_tower_dps","ally_tower_range","numbers_min_enemy"]),
 ("§ 신규: 교전판단 (라인전, -1=한타값)", ["numbers_threat_move","numbers_range_move","ally_tower_hp_move","ally_tower_dps_move","ally_tower_range_move","numbers_min_enemy_move"]),
 # ★[비활성화 07-16] subplan별 전력회피 섹션 편집기서 제거(값 -1=공통 따름이 곧 비활성. 코드 폴백은 유지). 필요시 이 줄 복원.
 # ("§ subplan별 전력회피", [...sp4~17...]),
 ("§ 넥서스 방어 성향 (disc19 위협/후퇴 — 순수화 완료)", ["d19_threat_mult","d19_retreat_hp","d19_range_atkme","d19_range_bld","d19_range_other","d19_range_idle"]),
 ("§ 넥서스 값튜닝: 수비(disc19 계열) — oi_enable=1 필요", ["oi_enable","oi_dn_count_gate","oi_dn_nexus_hp","oi_dn_hp_crit","oi_dn_hp_low","oi_dn_near_dist","oi_dn_pred_dist","oi_dn_lane_margin"]),
 ("§ 넥서스 값튜닝: 공격(disc18 계열)", ["oi_an_count_gate","oi_an_finish_hp","oi_an_cull_dist"]),
 ("§ 신규: 성향 반영", ["stat_influence"]),
 ("§ 시야 (사라진 적 기억)", ["vis_window","dd_lane_margin"]),
 ("§ 교전 (Engage)", ["eng_role4","eng_role3","eng_role2","eng_role_def"]),
 ("§ 신규 09-01: 개시 게이트 (갱크·교전·결사전)", ["gk2_gank_radius","gk2_gank_hp","eng_camp_radius","db_retreat_margin"]),
 ("§ 갱킹/처치 (TTD)", ["d4_dmg_scale","d4_div_base","d4_coef_scale","d4_coef_min","d4_coef_clamp","d4_coord_dist","d4_ttd_scale"]),
 ("§ 복귀 (Recall) - 체력기반", ["rc_u21_init","rc_ehp_t1","rc_ehp_t2","rc_ehp_t3","rc_ehp_v1","rc_ehp_v2","rc_norp_bonus","rc_ed_near","rc_ed_mid","rc_ed_far","rc_ed_near_pen","rc_ed_far_bonus","rc_ed_vfar_bonus","rc_ahp_t1","rc_ahp_t2","rc_u13_bonus","rc_ahp2_pen","rc_ad_near","rc_ad_mid","rc_ad_near_bonus","rc_ad_far_pen","rc_mult_bonus","rc_ally_hp_min"]),
 ("§ 복귀 - 합류 이득 (rc_join_weight=0=끔)", ["rc_join_weight","rc_join_adv","rc_join_rescue","rc_join_dnear","rc_join_dmid","rc_join_obj_mult"]),
 ("§ 운영전환 (GenericBuild)", ["gb_rbx_div","gb_r15_div","gb_r14_num"]),
 ("§ 라인전 (dd7700)", ["dd_frontier_mult","dd_cover_count","dd_ratio_thr","dd_facet_thr","dd_near_dist","dd_main_near_dist","dd_gatee_dist","dd_ivar2_thr","dd_n_thr","dd_survivor_thr"]),
 # ★[09-05] pk_* 11개 제거 — 07-15 리팩터링 때 tune() 배선이 끊긴 뒤 재연결 안 됨(구 모놀리식 소스엔 있었다).
 #   소스에 남은 자리는 skip_groups.rs 나열과 g(&[..]) 게이트뿐 = 값은 무반영인데 재구현만 켜지는 손해.
 #   되살릴 앵커 = `_backup_pre_refactor_20260715_1829\tfm2_ai_adjust.rs`. 살아있는 poke 키로 교체.
 #   ⚠poke_phase_gate·poke_active_min 은 소스가 死레버로 표기(0.5.2에 대응 게이트 부재)라 노출하지 않는다.
 ("§ 견제 (Poke)", ["pf_edge_margin","pf_center_band","pf_diag_far","pf_diag_near","pf_band_width","poke_reach_bonus","poke_serpen_slot"]),
 ("§ EpicPoke 수비값 (disc14, 실명 — 구 오라벨 DefenseNexus)", ["sn_self_hp","ep_home_lo","ep_home_hi","ep_home_x1","ep_home_y1","ep_hp_crit"]),   # ★[09-05] ~~ep_nexus_hp~~ → sn_self_hp: 08-03 개명 때 코드만 고치고 편집기를 안 고쳐 5주간 읽는 곳 0인 유령 컨트롤이었다(게다가 재는 값은 넥서스가 아니라 자기 HP%라 옛 이름이 오해를 준다). 나머지 ep_* 는 별칭(→sn_*)이 살아 있어 그대로 둔다.  # ★[수정 07-16] dn_*→ep_* 통일(편집기가 쓰던 dn_ 키를 코드 serpen.rs는 ep_로 읽었음=미스매치). dead 5개(lane_margin/pred_dist/near_dist/hp_low/count_gate=read site 없음) 제거
 ("§ 신규 09-01: 에픽·세르펜 사냥 킬타깃/세부 (고급)", ["eh_fin_mode","eh_band_off","eh_commit_margin","eh_dist_clamp","eh_clamp2","eh_engage_dist","eh_dist_shift","eh_power_weight","eh_power_neutral","eh_power_sub","eh_time_slope","eh_window_cap","eh_score_floor","eh_score_gate","eh_helper_a","eh_helper_b","eh_hp_gate2","eh_grid_cost"]),
 # ★[09-05] 이 섹션은 6개가 전부 무효였다 — d8_slot_thr 은 설명표가 "⛔판단 8은 원본이 무조건 7 고정이라
 #   임계를 읽는 코드에 도달하지 않음"이라 밝혔고, bt_home_*·bt_hp_retreat 는 pk_* 와 같은 07-15 배선 끊김.
 #   살아있는 bt_*(tune 배선 확인)로 교체.
 ("§ 전투 추격·후퇴 (battle)", ["bt_hp_flee","bt_hp_gate","bt_chase_stop","bt_chase_keep","bt_vision_mem"]),
 # ★[09-05 신설] 0.5.8 재핀으로 208사이트를 되살린 경로탐색 노브. 바이트패치(apply_path_imm)라
 #   skip_untuned 게이트와 무관하게 항상 먹는다. 전 키 -1=원본.
 ("§ 경로탐색 — 어디로 걸어갈지 (0.5.8 복구)", ["path_orth_cost","path_diag_cost","path_greedy","path_danger_cost","path_threat_floor","path_threat_cap","path_threat_scale","path_threat_default","path_wave_risk_ret"]),
]
knobs = []; seen = set(); rows = []
for hdr, keys in SECTIONS:
    rows.append(("H", hdr))
    for k in keys:
        if k in seen: continue
        seen.add(k); idx = len(knobs); knobs.append(k); rows.append(("K", idx, k))

# ── ★재발 방지 검사 ────────────────────────────────────────────────────────
# 2026-09-05 실측: 노출 142개 중 20개가 "만져도 아무 일 없는" 컨트롤이었다.
#   17개 = 07-15 리팩터링·08-03 개명 때 코드만 고치고 이 목록을 안 고쳐서(pk_*·bt_home_*·ep_nexus_hp),
#    3개 = 설명표가 스스로 死레버라 밝혔는데도 노출(t_ttd·t_gb·d8_slot_thr).
# 둘 다 "손으로 관리하는 두 번째 목록"이라 생긴 드리프트라, 생성할 때마다 기계로 잡는다.
_dead   = [k for k in knobs if not wired(k)]
_marked = [k for k in knobs if DEADMARK.search(desc_of(k) or '')]
_nodesc = [k for k in knobs if not desc_of(k)]
if _dead or _marked:
    print("★죽은 컨트롤 — 노출하면 유저가 값을 넣어도 무반영이다")
    for k in _dead:
        print("   [배선없음] %s   (소스에 tune()/파서/별칭 없음. skip_groups·g() 나열은 읽기가 아니다)" % k)
    for k in _marked:
        print("   [死레버표기] %s   %s" % (k, (desc_of(k) or '')[:60]))
    sys.exit("SECTIONS 에서 위 %d개를 빼거나 소스에 배선한 뒤 다시 실행" % len(set(_dead) | set(_marked)))
if _nodesc:
    print("⚠도움말 공백 %d개(모달 하단 설명이 빈칸으로 뜬다): %s" % (len(_nodesc), ', '.join(_nodesc)))
ROW_H, HDR_H, SP = 38, 32, 4
content = SP + sum((HDR_H if r[0] == "H" else ROW_H) + SP for r in rows)

def esc(s):
    # .ui 문자열 안전화: 백슬래시 계열 desc 정리(desc는 파싱시 원문 이스케이프 포함 가능) + 큰따옴표 제거
    return s.replace(chr(92) + chr(92), "").replace(chr(92), "").replace(Q, "'")

body = []; hc = 0
for r in rows:
    if r[0] == "H":
        body.append('    #hdr_%d:label { @"asset/base/style/main#label"; width: 820px; height: %dpx; align_y: Center; size: 16; color: #37d5b3ff; text: "%s"; }' % (hc, HDR_H, esc(r[1]))); hc += 1
    else:
        i, k = r[1], r[2]
        body.append(
'''    #kbrow_%d:empty {
      width: 820px; height: %dpx;
      #kl_%d:label { @"asset/base/style/main#label"; x: 10px; width: 400px; height: %dpx; align_y: Center; size: 15; text: "%s"; }
      #aiadj_f%d:text_edit { @"asset/base/style/main#text_edit"; x: 430px; y: 2px; width: 322px; height: 34px; size: 15; padding: { left: 10px; top: 4px; right: 10px; bottom: 4px; } }
      #aiadj_r%d:color_icon_button { x: 758px; y: 2px; width: 56px; height: 34px; color: #c8ccd8ff; hover: { color: #ffffffff; } btn: { color: #2a2d3aff; hover: { color: #3a3f52ff; } rounding: Uniform { rounding: 6; } } text: { text: "기본"; rect: { x: 0; y: 8; w: 56; h: 18; } align_x: Center; align_y: Center; size: 13; font: "asset/base/font/set/regular"; } }
    }''' % (i, ROW_H, i, ROW_H, k, i, i))
rows_ui = "\n".join(body)

ui = '''aiadj_modal:empty {
  width: 100%%; height: 100%%; visible: false;
  #aiadj_dim:color_icon_button { width: 100%%; height: 100%%; btn: { color: #000000cc; } }
  #aiadj_panel:color {
    anchor_x: 0.5; pivot_x: 0.5; anchor_y: 0.5; pivot_y: 0.5;
    width: 900px; height: 660px; color: #161721ff; rounding: Uniform { rounding: 12; }
    #aiadj_title:label { @"asset/base/style/main#label"; x: 32px; y: 22px; width: 620px; height: 32px; align_y: Center; size: 22; text: "AI"; }
    #aiadj_close:color_icon_button {
      anchor_x: 1.0; pivot_x: 1.0; x: -22px; y: 20px; width: 44px; height: 40px;
      color: #c8ccd8ff; hover: { color: #ffffffff; }
      btn: { color: #3a3f52ff; hover: { color: #4a4f62ff; } rounding: Uniform { rounding: 8; } }
      text: { text: "X"; rect: { x: 0; y: 10; w: 44; h: 20; } align_x: Center; align_y: Center; size: 18; font: "asset/base/font/set/regular"; }
    }
    #aiadj_scroll:scroll_view {
      x: 30px; y: 74px; width: 840px; height: 486px; speed: 120; bar_width: 5;
      bar: { source: "asset/base/sprite/white"; color: #37d5b3ff; }
      #aiadj_list:empty { width: 820px; height: %dpx; child_type: TopToBottom { spacing: %dpx; }
%s
      }
    }
    #aiadj_hint:label { @"asset/base/style/main#label"; x: 30px; y: 584px; width: 690px; height: 66px; size: 14; line_height: 19; color: #b8bcc8ff; text: "빈칸=전역 따름 · 항목에 마우스 올리면 여기에 설명"; }
    #aiadj_save:color_icon_button {
      anchor_x: 1.0; pivot_x: 1.0; anchor_y: 1.0; pivot_y: 1.0; x: -30px; y: -18px; width: 150px; height: 44px; color: #ffffffff;
      btn: { color: #1f6f4aff; hover: { color: #2a8f5aff; } rounding: Uniform { rounding: 8; } }
      text: { text: "SAVE"; rect: { x: 0; y: 12; w: 150; h: 20; } align_x: Center; align_y: Center; size: 17; font: "asset/base/font/set/regular"; }
    }
  }
}
''' % (content, SP, rows_ui)
io.open(OUT_UI, "w", encoding="utf-8", newline="\n").write(ui)

def rsesc(s):  # Rust 문자열 리터럴 이스케이프
    return s.replace(chr(92), chr(92)+chr(92)).replace(chr(34), chr(92)+chr(34))
rs = "// 자동생성. aiadj_f{i} <-> KNOBS[i] <-> DESCS[i].\npub const KNOBS: [&str; %d] = [\n" % len(knobs)
for i in range(0, len(knobs), 6):
    rs += "    " + ", ".join('"%s"' % k for k in knobs[i:i+6]) + ",\n"
rs += "];\n// 항목 설명(호버 도움말). DESCS[i] = KNOBS[i] 설명.\npub const DESCS: [&str; %d] = [\n" % len(knobs)
for k in knobs:
    # ★[09-05] 별칭 추종 — 구이름으로 노출된 16개(oi_*→nx_*, ep_*→sn_*)가 도움말 공백이었다.
    rs += '    "%s",\n' % rsesc(desc_of(k) or "")
rs += "];\n"
io.open(OUT_RS, "w", encoding="utf-8", newline="\n").write(rs)
print("knobs=%d descs=%d/%d content=%d" % (len(knobs), sum(1 for k in knobs if desc_of(k)), len(knobs), content))
