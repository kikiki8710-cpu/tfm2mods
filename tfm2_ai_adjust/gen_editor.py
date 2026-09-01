import io, os, re
GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
DESCSRC = r"C:\tfm2mods\ai_adjust_editor\src\main.rs"
OUT_UI = r"C:\tfm2mods\tfm2_ai_adjust\ui_inject\aiadj_modal.ui"
OUT_RS = r"C:\tfm2mods\tfm2_ai_adjust\src\knobs.rs"
Q = chr(34)

# desc_static 파싱: "key" => "desc",
descs = {}
src = io.open(DESCSRC, encoding='utf-8').read()
for m in re.finditer(r'"([a-z_][a-z0-9_]*)"\s*=>\s*"((?:[^"\\]|\\.)*)"', src):
    descs[m.group(1)] = m.group(2)

SECTIONS = [
 ("§ 전체 성향 다이얼", ["t_engage","t_ttd","t_recall","t_gb"]),
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
 ("§ 견제 (Poke)", ["pf_edge_margin","pf_center_band","pf_diag_far","pf_diag_near","pf_band_width","pk_home_lo","pk_home_hi","pk_home_x1","pk_home_y1","pk_hp_main","pk_hp_retreat","pk_smallact_split","pk_threat_mult","pk_zone_hp","pk_engage_dist","pk_obj_hp"]),
 ("§ EpicPoke 수비값 (disc14, 실명 — 구 오라벨 DefenseNexus)", ["ep_nexus_hp","ep_home_lo","ep_home_hi","ep_home_x1","ep_home_y1","ep_hp_crit"]),   # ★[수정 07-16] dn_*→ep_* 통일(편집기가 쓰던 dn_ 키를 코드 serpen.rs는 ep_로 읽었음=미스매치). dead 5개(lane_margin/pred_dist/near_dist/hp_low/count_gate=read site 없음) 제거
 ("§ 신규 09-01: 에픽·세르펜 사냥 킬타깃/세부 (고급)", ["eh_fin_mode","eh_band_off","eh_commit_margin","eh_dist_clamp","eh_clamp2","eh_engage_dist","eh_dist_shift","eh_power_weight","eh_power_neutral","eh_power_sub","eh_time_slope","eh_window_cap","eh_score_floor","eh_score_gate","eh_helper_a","eh_helper_b","eh_hp_gate2","eh_grid_cost"]),
 ("§ 기타", ["d8_slot_thr","bt_home_lo","bt_home_hi","bt_home_x1","bt_home_y1","bt_hp_retreat"]),
]
knobs = []; seen = set(); rows = []
for hdr, keys in SECTIONS:
    rows.append(("H", hdr))
    for k in keys:
        if k in seen: continue
        seen.add(k); idx = len(knobs); knobs.append(k); rows.append(("K", idx, k))
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
    rs += '    "%s",\n' % rsesc(descs.get(k, ""))
rs += "];\n"
io.open(OUT_RS, "w", encoding="utf-8", newline="\n").write(rs)
print("knobs=%d descs=%d/%d content=%d" % (len(knobs), sum(1 for k in knobs if k in descs), len(knobs), content))
