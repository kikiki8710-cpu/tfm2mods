#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! tfm2_ai_adjust 설정 편집기 — config_editor.hta 의 무의존 네이티브 포트.
//! mshta / ActiveX 의존을 제거해 어떤 Windows 에서도 단일 exe 로 실행.
//! exe 가 위치한 폴더(=mod 폴더) 기준으로 tfm2_ai_adjust.cfg / config\*.cfg 를 읽고 씀.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use eframe::egui;

const ACTIVE_NAME: &str = "현재 (tfm2_ai_adjust.cfg)";
// 클래스별 오버라이드: "키_class_<en>" = 해당 클래스 전용값(미지정=전역 폴백). ChampionCategory 순서(melee0..assassin4).
const CLASS_EN: [&str; 5] = ["melee", "range", "magician", "util", "assassin"];
const CLASS_KR: [&str; 5] = ["전사", "원거리", "마법사", "전투보조", "암살자"];

// ============================ 탭 스키마 ============================
struct Tab { id: &'static str, title: &'static str, note: &'static str, keys: &'static [&'static str] }

// ★탭 순서 = subplan별 탭(위) + 공통 탭(아래). 각 subplan 탭 내부는 장르별 §단락, 폐기/미확인 키는 맨 아래 격리.
// 분류 정본 = ANA\subplan별-튜닝레버-분류.md
static TABS: &[Tab] = &[
  // ─────────────── subplan 탭 (disc 순) ───────────────
  Tab{ id:"lane", title:"• [3] 라인전 (dd7700)", keys:&[
      "aggr_lane",
      "§◆ 이동·후퇴 게이트","dd_frontier_mult","dd_ratio_thr","dd_n_thr","dd_ivar2_thr",
      "§◆ 거리 임계","dd_near_dist","dd_main_near_dist","dd_gatee_dist",
      "§◆ 커버·합류","dd_cover_count","dd_survivor_thr","dd_facet_thr",
      "§◆ 생존자 카운트","dd_f22e80_margin",
      "§⛔ 死레버 (0.5.2에서 게이트 삭제 — 값 바꿔도 무반영)","dd_early_p3_thr","dd_cover_p3_thr"], note:
    "<b>subplan 3 (라인전)</b> — 챔프가 라인에서 어디로 갈지(더 밀기 / 물러나기 / 수비 합류).<br>\
     물러나기: 적 전선진척 − 아군 전선진척 × dd_frontier_mult 가 작으면 후퇴<br>\
     수비 합류: 근처 적이 dd_cover_count명 이상 &amp; 라인 비율 &lt; dd_ratio_thr 면 합류 · 근접판정: 적 거리² &lt; dd_main_near_dist.<br>\
     (라인전 <b>전력회피</b>는 [공통] 전력탭 numbers_*_move · 시야는 [공통] 시야탭 dd_lane_margin)" },

  Tab{ id:"jungle", title:"• [4] 정글 (disc4 · ⚠경기중 미발화)", keys:&[
      "§⚠ 전부 폐기·미확인 — disc4(LineSafe)는 정규전 미발화(값 무반영)","d4_dmg_scale","d4_div_base","d4_coef_scale","d4_coef_min","d4_coef_clamp","d4_coord_dist","d4_ttd_scale","d4_ward_dist2","d4_engage_r2","d4_ref_dist2","d4_close_hp","d4_threat_min","d4_pathlen_thr","d4_wcast_thr"], note:
    "<b>[4] 정글 (disc4=LineSafe)</b> — ⚠<b>정규 경기중 발화 미확인.</b> 생성 코드는 '스테이지1'(구성테스트/튜토리얼 추정) 전용으로만 실존 → 정규 시즌엔 이 subplan이 안 떠서 아래 d4_* 는 전부 값 무반영(참고용).<br>\
     (챔프 교전판단은 [공통]교전 탭 eng_role 로 이동 · 정글 포탑딜로 알던 tower_dps 는 실은 아군포탑 force=[공통]전력탭.)" },

  Tab{ id:"recall", title:"• [5] 귀환 (Recall)", keys:&[
      "§◆ 체력 기반 복귀 (기본)","rc_u21_init","rc_ehp_t1","rc_ehp_t2","rc_ehp_t3","rc_ehp_v1","rc_ehp_v2","rc_norp_bonus","rc_ed_near","rc_ed_mid","rc_ed_far","rc_ed_near_pen","rc_ed_far_bonus","rc_ed_vfar_bonus","rc_ahp_t1","rc_ahp_t2","rc_u13_bonus","rc_ahp2_pen","rc_ad_near","rc_ad_mid","rc_ad_near_bonus","rc_ad_far_pen","rc_mult_bonus","rc_ally_hp_min",
      "§🆕 신규(재배선) — 복귀배율 RNG/정규화","rc_rng_center","rc_rng_spread_div","rc_rng_a_base","rc_score_div",
      "§★ 합류 이득 — 전략적 복귀 (rc_join_weight=0=끔)","rc_join_weight","rc_join_adv","rc_join_rescue","rc_join_dnear","rc_join_dmid","rc_join_obj_mult",
      "§✅ disc7 귀환 이동판단 [0.5.3 재가동] — d7_repl=1 기본ON. scan2 경로만 원본위임","d7_repl","d7_hp_normal","d7_hp_selfheal","d7_wp_dist2"], note:
    "<b>subplan 5 (귀환 점수)</b> — 집/안전지대로 돌아갈지 점수로 결정. (전체배율 [공통]주요성향 t_recall)<br>\
     시작값 rc_u21_init → ±[적 HP%] ±[리콜포인트~적 거리] ±[아군 HP%] ±[아군~적 거리] +[수적우세]<br>\
     점수 = 난수 × mult / 100 + t_recall. 점수 클수록 복귀.<br>\
     <b>합류 이득 (rc_join_*)</b>: rc_join_weight>0이면 체력 멀쩡해도 합류 이득 크면 복귀(합류 이동기). <b>rc_join_weight=0=끔(기본).</b><br>\
     <b>disc7 이동판단 (d7_*)</b>: 위 rc_*(점수)와 별개 층 — 귀환 subplan 실행 중 '버틸지 vs 뺄지'(movepri Recall 리졸버). <b>d7_repl=1</b>이면 대체 활성. d7_hp_normal(41)/d7_hp_selfheal(21)=HP% 후퇴 임계(자힐 유무별), 낮출수록 저체력에도 안 뺌." },

  Tab{ id:"disc8", title:"• [8] 갱커버 (disc8)", keys:&["d8_slot_thr"], note:
    "<b>subplan 8 (갱커버)</b> — 갱킹 커버(합류) 판단. 슬롯 우선순위 임계 하나로 분기." },

  Tab{ id:"object", title:"• [9·11] 견제 (Battle·Hide)", keys:&[
      "§◆ 위치술어 (pf)","pf_edge_margin","pf_center_band","pf_diag_far","pf_diag_near","pf_band_width",
      "§🆕 신규(재배선) — 견제 도달 게이트","poke_reach_bonus","poke_serpen_slot",
      "§⚠ 폐기 (값 무반영)","poke_phase_gate","poke_active_min","aggr_object","pk_home_lo","pk_home_hi","pk_home_x1","pk_home_y1","pk_hp_main","pk_hp_retreat","pk_smallact_split","pk_threat_mult","pk_zone_hp","pk_engage_dist","pk_obj_hp"], note:
    "<b>subplan 9/11 (Battle/Hide)</b> — 주요 오브젝트 견제·교전.<br>\
     pf_ 값들 = 맵을 구역(띠)으로 나눠 챔프 위치를 판정(견제 시 어디에 설지). poke_* = 재배선된 진입/도달 게이트.<br>\
     구 pk_* 키는 050에서 하드코딩(값 무반영) — 맨 아래 격리." },

  Tab{ id:"battle", title:"• [12·13] 모르가드 (교전판단·사냥)", keys:&[
      "§◆ [12]교전판단 HP 게이트","ec_oz_hp","ec_iz_hp","ec_self_hp_low","ec_valid_hp","ec_commit_hp","ec_count_hp",
      "§◆ [12]거리·카운트·시야","ec_engage_dist2","ec_count_radius","ec_vision_ticks",
      "§⛔ 死레버 (0.5.2에서 게이트 삭제 / 미발화 disc)","ec_gate_tick","ec_tgt_hp_low","d13_engage_hp_pct",
      "§⚠ 폐기 (bt 구키 — 값 무반영)","bt_home_lo","bt_home_hi","bt_home_x1","bt_home_y1","bt_hp_retreat"], note:
    "<b>subplan 12 (EpicCheck)</b> — 모르가드(에픽) 교전 판단. <b>07-23 편입 완료 = 실제 게임에 반영됨</b>(구 ec_* 키 유지).<br>\
     안전지대 안/밖 + 체력·목표 상태 → 교전(0xe)/대기·철수(7,0xc)/재배치(2)/HOLD(0xd) 결정.<br>\
     ★<b>ec_tgt_hp_low → ec_self_hp_low 로 대체</b>: 기존 재현이 <b>타겟</b> 체력을 보고 있었으나 원본은 <b>자기</b> 체력 기준(0.5.2 disasm 확정). 구 키는 무반영.<br>\
     ⛔<b>[13] 모르가드 사냥(EpicHunt)은 죽은 틀</b> — 게임이 이 subplan을 <b>생성하지 않음</b>(오더코드 도메인에 Hunt 코드 부재, 런타임 0발화 실측). d13_engage_hp_pct 는 값 무반영." },

  Tab{ id:"def", title:"• [14] 모르가드 견제 (EpicPoke)", keys:&[
      "§◆ 홈 회복존 게이트 (아군 베이스/분수 영역·부상시 홈대기)","ep_home_lo","ep_home_hi","ep_home_x1","ep_home_y1",
      "§◆ HP 게이트","ep_nexus_hp","ep_hp_crit",
      "§⚠ 폐기 (값 무반영 — disc16/17/4로 이동 / aggr_defense dead)","aggr_defense","ep_lane_margin","ep_pred_dist","ep_near_dist","ep_hp_low","ep_count_gate"], note:
    "<b>subplan 14 (EpicPoke)</b> — 에픽 오브젝트(용/바론류) 국면의 견제·포킹 실행(안전공격지점 잡거나 교전).<br>\
     출력=추격/유휴/교전/리포지션/대형이동. self가 <b>아군 베이스/분수 회복존</b>(부상 유닛이 힐 받는 존, geom+0x6d70·per-side) 안 & 부상(HP&lt;max)이면 <b>에픽 견제 나가지 말고 홈 대기(7)</b>. ※epic·넥서스 위치 아니라 아군 베이스 영역 기준(RE 확정). HP 게이트: ep_hp_crit · ep_nexus_hp(=self HP%). (진짜 넥서스는 [18·19]탭)<br>\
     구 ep_lane_margin/pred/near/hp_low/count_gate 는 disc16/17/4로 로직이 이동해 여기선 값 무반영." },

  Tab{ id:"disc17", title:"• [15·16·17] 세르펜", keys:&[
      "§◆ [17]견제 진척 게이트","disc17_prog_low","disc17_prog_crit","disc17_p3_gate","disc17_ref_hp",
      "§◆ [17]거리","disc17_near_dist","disc17_pred_dist",
      "§◆ [16]세르펜 사냥 — 홈대기 HP%","disc16_home_hp",
      "§⛔ 死레버 (0.5.3 감사: 코드에 read site 없음 — 값 바꿔도 무반영)","numbers_threat_sp16","numbers_threat_sp17",
      "§🆕 [15]세르펜 교전판단 (d15_repl=1 켜야·재현 미검증)","d15_repl","d15_engage_hp_pct"], note:
    "<b>17 (SerpenPoke·세르펜 견제)</b> — 진척/근접 게이트 공방(재배선 완료). <b>16 (SerpenHunt·사냥)</b>=disc16_home_hp(부상 홈대기 HP%, 기본100=원본). <b>15 (SerpenCheck·교전판단)</b>=d15_repl 켜야 반영(재현 미검증이라 기본 OFF·인게임 테스트 권장).<br>\
     진척(progress) 게이트와 근접/예측 거리²로 DEFEND/진격 결정." },

  Tab{ id:"disc19", title:"• [18·19] 넥서스 공수 (방어·공격)", keys:&[
      "oi_enable",
      "§◆ 방어 (disc19) — 실작동 byte-patch (d19i_enable=1 켜야 severity/retreat 반영)","d19i_enable","d19_retreat_hp","oi_dn_nexus_hp","oi_dn_hp_crit","oi_dn_hp_low","oi_dn_near_dist","oi_dn_pred_dist","oi_dn_lane_margin",
      "§◆ 공격 (disc18) — 실작동 byte-patch","oi_an_finish_hp","oi_an_cull_dist",
      "§⛔ 死레버 (0.5.2에서 무효 — 값 바꿔도 무반영)","oi_dn_count_gate","oi_an_count_gate",
      "§⚠ 은퇴 (disc19cmp 대조값만·게임 무영향)","d19_threat_mult","d19_range_atkme","d19_range_bld","d19_range_other","d19_range_idle"], note:
    "<b>[18·19] 넥서스 공수</b> — 넥서스 방어(disc19)/공격(disc18) 판단 튜닝. 대부분 <b>byte-patch</b>(게임 원본상수 직접 수정).<br>\
     ★<b>0.5.3 재핀 완료(2026-07-30)</b> — 전 사이트 실측 재핀(obj 14/14·d19 10/10). <b>oi_an_cull_dist</b> 는 0.5.3에서 게임이 후보를 <b>세 묶음으로 나눠 훑도록</b> 바뀌어 패치 자리가 1곳→3곳이 됐고, 비교 방향도 뒤집혔다(모드가 내부에서 보정하므로 <b>설정값 의미·체감은 동일</b>). / 구 <b>0.5.2 재핀(2026-07-23)</b> — 그 전까지 <b>0.5.1 주소가 남아 있어 oi_*·d19_* 값이 전부 무반영</b>이었습니다(로그 applied=0/13·0/15). 지금은 oi <b>12/12</b>·d19 <b>10/10</b>.<br>\
     <b>방어</b>: d19_retreat_hp(후퇴 HP%문턱, ↑=수비적) — <b>d19i_enable=1 켜야</b> byte-patch 반영(0=원본45 복원). + oi_dn_*(<b>oi_enable=1</b> 필요).<br>\
     <b>공격</b>: oi_an_finish_hp(적넥서스 HP% 마무리 게이트·↓=공격적)·oi_an_cull_dist(공격후보 거리).<br>\
     <b>oi_enable=1</b>이라야 oi_* 반영(=0 기본=원본 복원). ⚠은퇴칸(d19_threat_mult·d19_range_*)은 재현 대조용(dcap)이라 <b>게임 무영향</b>.<br>\
     ⚠<b>0.5.2 동작 변화</b>: 넥서스 방어 판단의 <b>phase 게이트가 전부 삭제</b>돼(구 phase≥30/≥39), 이제 <b>경기 시간대와 무관하게 상시</b> 위협·아군넥서스 판정을 합니다." },

  Tab{ id:"gb", title:"• [매크로] 운영전환·로밍 (GenericBuild)", keys:&[
      "§◆ 마스터 스위치 (1=켜야 아래 byte-patch 반영, 0=전량 원본)","gb_enable",
      "§🆕 로밍 거리·범위 (유닛, -1=원본유지)","gb_join_dist","gb_scout_radius","gb_close_radius","gb_line_range",
      "§🆕 진입 타이밍 게이트 (-1=원본유지)","gb_op_phase","gb_push_hp",
      "§⛔ 死레버 (0.5.2에서 무효 — 값 바꿔도 무반영)","gb_join_phase",
      "§⚠ 전역 사거리 (전 AI 공유·신중, -1=원본)","gb_reach_cap","gb_reach_margin",
      "§🔬 신규(소스배선만·미확인)","gb_cnt_skip","gb_da_thr","gb_cnt_move","gb_db_engage","gb_score_mult",
      "§⚠ 폐기 (구 거리밴드 — 값 무반영)","gb_rbx_div","gb_r15_div","gb_r14_num"], note:
    "<b>운영전환·로밍 (GenericBuild)</b> — 특정 subplan이 아니라 '어느 상태로 갈지'(합류/거점로밍/라인압박/운영진입)를 분기하는 매크로 판단.<br>\
     ✅<b>[0.5.3 재핀 2026-07-30]</b> 적용 사이트 <b>10곳 → 9곳</b>(<b>gb_scout_radius</b> 가 루프 앞뒤 2곳에서 <b>루프 안 1곳으로 병합</b>). 값이 줄어든 게 아니라 자리가 합쳐진 것이고, 비교 방향 반전도 모드가 보정하므로 <b>설정값 의미·체감은 동일</b>. 적용확인=gb_imm.txt(applied=N/<b>9</b>).<br>[07-16 경로A] <b>gb_enable=1</b> 켜야 아래 로밍 byte-patch가 걸림(0=게임 원본 그대로). 각 값 <b>-1=그 항목만 원본유지</b>.<br>\
     &nbsp;&nbsp;<b>gb_join_dist</b>(60000)=이 거리 이내면 '근접/합류 모드' 진입(지배 게이트). <b>gb_scout_radius</b>(120000)=거점/타겟 후보를 이 반경 안에서 수집=로밍 범위. <b>gb_close_radius</b>(≈387)·<b>gb_line_range</b>(≈500)=근접·라인 판정 반경.<br>\
     &nbsp;&nbsp;<b>gb_op_phase</b>(31)=경기진행 phase가 이 값 이상이면 운영 시작(낮추면 이른 운영). ~~gb_join_phase(12)~~=⛔0.5.2 死(게이트 삭제). <b>gb_push_hp</b>(30)=라인 대상 체력%가 이 값 미만이면 압박 오더.<br>\
     &nbsp;&nbsp;⚠<b>gb_reach_*</b>=GenericBuild 전용이 아닌 <b>전 AI 공유 사거리</b> 헬퍼 → 켜면 모든 판단의 사거리가 바뀜(신중). scout_radius는 헬퍼 콜엣지 미확정이라 gb_imm.txt applied 카운트로 적용확인 권장.<br>\
     gb_cnt_*/gb_da_*/gb_db_*/gb_score_* = 소스 배선됐으나 라이브 미발화(참고용). 구 거리밴드(gb_rbx/r15/r14)는 값 무반영." },

  // ─────────────── 공통 탭 ───────────────
  Tab{ id:"feel", title:"• [공통] 주요 성향 다이얼", keys:&[
      "t_engage","t_recall",
      "§⚠ 폐기 (값 무반영)","t_ttd","t_gb"], note:
    "AI 성향 메인 다이얼(각 subplan judge에 <b>전체 배율</b>로 곱함, 기본 100=원본). 빠르게 성향만 바꾸려면 여기만.<br>\
     <b>t_engage</b>=교전 전체배율(전 포지션·[공통]교전 탭 eng_role에 곱함) · <b>t_recall</b>=[5]귀환. (t_ttd·t_gb는 050 하드코딩=값 무반영 → 폐기.)" },

  Tab{ id:"engagerole", title:"• [공통] 교전 (engage 역할별)", keys:&[
      "§◆ 교전 후퇴 전환점 — 타깃 우선순위별 (전 포지션 공용)","eng_role4","eng_role3","eng_role2","eng_role_def"], note:
    "<b>교전(engage·facet#5)</b> — ★전 포지션 챔프 공용(정글 전용 아님). 교전 판단에서 <b>타깃(적) 우선순위</b>(obj+0x188=4/3/2/기타)별로 후퇴 전환점을 고름. 난수(0~100) &ge; eng_role[우선순위]이면 후퇴, 아니면 교전. 값↑=더 적극 교전. 전체배율은 [공통]주요성향 <b>t_engage</b>. engage_repl 필요." },

  Tab{ id:"engage", title:"• [공통] 전력·포탑 회피", keys:&[
      "§▲ 적 포탑 회피 — 사거리내 불리하면 후퇴","tower_threat","tower_range","tower_dps",
      "§◆ 교전판단 (한타)","numbers_threat","numbers_range","ally_tower_hp","ally_tower_dps","ally_tower_range","numbers_min_enemy",
      "§◇ 교전판단 (라인전) — 비우거나 -1이면 한타 수치 따름","numbers_threat_move","numbers_range_move","ally_tower_hp_move","ally_tower_dps_move","ally_tower_range_move","numbers_min_enemy_move"], note:
    "게임 원본에 <b>없던 추가 항</b>(초록=신규, 전부 <b>0이면 원본과 동일</b>). <b>여러 subplan의 교전판단에 공유</b>되는 공통 회피 시스템.<br>\
     <b>전력(force) 승산</b> = 근처 양팀 (ΣHP)×(Σ공격)(100=호각, &gt;100=우세). 한타값 / 라인전값(_move)로 이원화.<br>\
     <b>포탑회피</b>(tower_threat): 적 포탑 사거리 안 + tower_threat &ge; 승산이면 후퇴. <b>전력회피</b>(numbers_threat): numbers_threat &ge; 승산이면 후퇴(100=이길싸움만). <b>tower_dps</b>=아군 포탑 DPS를 아군 전력에 반영하는 힘 단위(라이너/전 disc 공용, ally_tower_dps>0일때)." },

  Tab{ id:"stat", title:"• [공통] 성향 반영", keys:&[
      "stat_influence",
      "§🆕 신규(재배선) — 성향 반영 세부","stat_neutral","stat_pos_div","stat_judg_ref","stat_noise_shift"], note:
    "챔프 성향스탯(공격성·에고·판단력)을 후퇴판단에 섞음(라이너). <b>stat_influence=0=off(원본).</b><br>\
     공격성↑=덜 후퇴, 에고↑=잘 안 빠짐, 판단력↓=양방향 오판(결정론적). 기준=공50·에50·판100. 세부는 stat_* 재배선." },

  Tab{ id:"severity", title:"• [공통] 위협 민감도 (severity)", keys:&[
      "sv_enable",
      "§◆ 위협비율 사다리 (tr = 위협×100÷현재체력)","sv_tr0","sv_tr1","sv_tr2","sv_tr3",
      "§◆ 체력 단계 경계","sv_hp1","sv_hp2","sv_hp3",
      "§◆ '사소' 위협 할인 (정본 함수 전용)","sv_discount_shift","sv_discount_cap"], note:
    "★[07-23 신설] 게임 AI 전체가 공유하는 <b>\"이 위협이 유의미한가\" 필터</b>를 직접 튜닝. 위협비율(tr)이 사다리 문턱을 못 넘으면 그 위협은 '사소'로 깎여서/무시되고, 넘으면 전액 반영됩니다.<br>     <b>sv_enable=1</b> 켜야 반영(0=게임 원본). 같은 사다리 <b>사본 4곳 29사이트를 같은 값으로 일괄 패치</b>: ①위협 평가 정본(전 판단 공유) ②넥서스 공방 계열 위협 총합 ③라인·정글 위협 필터 ④타겟 선택 스코어러.<br>     <b>tr 문턱↓ = 더 겁쟁이</b>(작은 위협도 심각하게 봄 → 일찍 후퇴/회피), <b>↑ = 더 대담</b>(웬만한 위협 무시). 체력이 낮을수록 낮은 문턱(sv_tr1~3)이 적용됨.<br>     ⚠[19]넥서스 방어의 자체 사다리는 별도 키(d19_sev_*) 소관 — 거기만 다른 기준을 주는 것도 가능.<br>     적용확인 = sev_imm.txt(applied=N/29)." },
  Tab{ id:"vision", title:"• [공통] 시야", keys:&[
      "dd_lane_margin","vis_window"], note:
    "적이 시야에서 사라져도 <b>일정 틱동안 '아는 적'으로 기억</b>해 판단에 반영. 값↑=더 오래 경계.<br>\
     <b>dd_lane_margin</b> = [3]라인 판단(dd7700) 전용 기억창(기본 120≈2초).<br>\
     <b>vis_window</b> = ✅[07-16 부활] <b>비-라인 전반</b> 기억창(기본 600≈10초). byte-patch(0x1caedd3) 단일 공유값이라 <b>여러 판단에 한꺼번에</b> 걸림.<br>\
     &nbsp;&nbsp;└ 영향 판단(13개 호출처): <b>넥서스 커밋([18·19])·오브젝트 평가·모르가드([12~14])·세르펜([15~17])·CONDGATE</b>. ↑=더 오래 '아는 적' 추적, ↓=빨리 잊음. 0=즉시 망각(주의).<br>\
     &nbsp;&nbsp;※ 각 subplan에 개별 시야창(120틱, 8곳)이 따로 있으나 현재 미개입(원하면 disc10/14/15별 노출 가능)." },

  Tab{ id:"engine", title:"• [공통] 엔진·대체스택 (고급)", keys:&[
      "fast_read","fast_guard",
      "§◆ 행(멈춤) 진단 워치독","hang_diag","hang_secs","hang_run_secs","hang_run_rate",
      "§◆ 활동창 프로파일러 (일정넘김 시간분포·⚠배포시 0)","adv_prof","adv_prof_min","adv_prof_seg",
      "§⛔ 死레버 (0.5.3에서 무효 — 패치 시그 소멸, 값 바꿔도 무반영)","sim_unchunk",
      "§◆ 대체 게이트 (전부 OFF=원본)","enabled","team","mp_repl","dd7_repl","poke_repl","recall_repl","engage_repl","cond_repl","d4_repl","e9jt","replay_reset"], note:
    "<b>고급/개발용</b> — AI judge 메모리 read 방식(fast_read)과 각 judge를 우리 코드로 대체할지 여부(대체 게이트).<br>\
     대체 게이트가 <b>전부 OFF면 게임 원본 로직</b> 사용. mp_repl(이동 마스터)이 꺼지면 dd7/poke도 동작 안 함. 문제 생기면 여기부터 롤백." },
];

fn is_toggle(k: &str) -> bool {
  matches!(k, "cond_repl"|"gbskip"|"mp_repl"|"dd7_repl"|"poke_repl"|"recall_repl"|"engage_repl"
    |"e9jt"|"d4_repl"|"d7_repl"|"d4ttd"|"perf_measure"|"read_bench"|"replay_reset"|"enabled")
}
fn is_added(k: &str) -> bool {
  matches!(k, "tower_threat"|"tower_range"|"tower_dps"|"numbers_min_enemy"|"numbers_min_enemy_move"|"stat_influence"|"ally_tower_range")
    || k.starts_with("numbers_threat") || k.starts_with("numbers_range") || k.starts_with("ally_tower_") || k.starts_with("rc_join") || k.starts_with("d19_")
}
fn is_removed(k: &str) -> bool { k == "numbers_margin" }
// 값 무반영(050 하드코딩/폐기). 라벨 "폐기"(빨강) + 비활성 입력.
fn is_dead(k: &str) -> bool {
  matches!(k,
    "pk_home_lo"|"pk_home_hi"|"pk_home_x1"|"pk_home_y1"|"pk_hp_main"|"pk_hp_retreat"|"pk_smallact_split"|"pk_threat_mult"|"pk_zone_hp"|"pk_engage_dist"|"pk_obj_hp"
    |"bt_home_lo"|"bt_home_hi"|"bt_home_x1"|"bt_home_y1"|"bt_hp_retreat"
    |"t_ttd"|"t_gb"|"aggr_object"|"aggr_defense"
    |"ep_lane_margin"|"ep_pred_dist"|"ep_near_dist"|"ep_hp_low"|"ep_count_gate"
    |"d4_dmg_scale"|"d4_div_base"|"d4_coef_scale"|"d4_coef_min"|"d4_coef_clamp"|"d4_coord_dist"|"d4_ttd_scale"
    |"gb_rbx_div"|"gb_r15_div"|"gb_r14_num")
}
// 소스 배선됐으나 미발화(라이브 검증불가). 라벨 "미확인"(회색) + 비활성 입력.
fn is_unfired(k: &str) -> bool {
  matches!(k,
    "d4_ward_dist2"|"d4_engage_r2"|"d4_ref_dist2"|"d4_close_hp"|"d4_threat_min"|"d4_pathlen_thr"|"d4_wcast_thr"
    |"gb_cnt_skip"|"gb_da_thr"|"gb_cnt_move"|"gb_db_engage"|"gb_score_mult")
}
// subplan별 키는 이름이 거의 같아 라벨에 한글 별칭을 덧붙임
fn disp_key(k: &str) -> &str {
  match k {
    _ => k,   // 라벨 = cfg 키 이름 그대로(영어)
  }
}
fn select_opts(k: &str) -> Option<&'static [(&'static str, &'static str)]> {
  match k {
    "fast_read" => Some(&[
      ("0","0 · VirtualQuery (원본/느림)"),
      ("1","1 · VEH spinlock"),
      ("2","2 · VEH lockless (안전·권장)"),
      ("3","3 · 직접read (VEH無·최速·⚠크래시위험·실험)"),
    ]),
    _ => None,
  }
}

fn desc_static(k: &str) -> Option<&'static str> {
  Some(match k {
    "t_engage" => "교전(engage·facet#5) 공격성 배율%. eng_role 4개 임계 전부에 ×t_engage/100. >100 적극/<100 소극. ※[재분석 07-16] '정글 전용' 아님 — 전 포지션 챔프의 교전판단에서 타깃 우선순위(obj+0x188=4/3/2/기타)별 후퇴임계 래더에 공용 적용. engage_repl 필요. 기본 100",
    "t_ttd" => "⚠DEAD(050 하드코딩·값 무반영) [4]정글 갱킹/처치 적극성 배율%. disc4 TTD 임계에 ×t_ttd/100. 크면 확실할 때만 갱킹, 작으면 무리해서도. 기본 100",
    "t_recall" => "복귀 성향 가산. recall 점수에 +t_recall. 0보다 크면 자주 복귀, 작으면 덜 복귀. 기본 0",
    "t_gb" => "⚠DEAD(050 하드코딩·값 무반영) 운영전환 거리 전체 배율%. 영역D 거리밴드에 ×t_gb/100. 크면 더 멀리까지 운영, 작으면 가까운 것만. 기본 100",
    "aggr_lane" => "✅LIVE. 라인전(dd7700) 공격성 배율% — 프론티어 후퇴 게이트 분모에 적용(l15×dd_frontier_mult×100/aggr_lane). >100=덜 물러남(공격적)/<100=잘 물러남. 기본 100",
    "aggr_object" => "⚠DEAD(파서 저장되나 읽는 곳 0=값 무반영). 오브젝트(disc9/11) 견제 공격성은 실제로는 poke_reach_bonus(↑=더 멀어도 견제)·poke_phase_gate로 조절. 기본 100",
    "aggr_defense" => "⚠DEAD(읽는 곳 0=값 무반영). 에픽견제(disc14) 공격성은 ep_hp_crit·ep_nexus_hp로, 넥서스 방어는 d19_retreat_hp·oi_dn_*로 조절. 기본 100",
    "rc_u21_init" => "mult 시작값(누적 기준선). 낮을수록 복귀 점수 전반↓. 여기에 아래 가·감산이 쌓임",
    "rc_ehp_t1" => "적 HP% 상한 — 가장 가까운 적 HP%가 이 값 미만일 때부터 복귀 가산 시작",
    "rc_ehp_t2" => "적 HP% 중간 임계 — 미만이면 더 큰 가산(rc_ehp_v1/v2 − 적HP%)",
    "rc_ehp_t3" => "적 HP% 하한 — 미만이면 rc_ehp_v1 기준, 이상이면 rc_ehp_v2 기준으로 가산",
    "rc_ehp_v1" => "적 HP% 최하위 구간 가산 기준값 (가산 = 이값 − 적HP%)",
    "rc_ehp_v2" => "적 HP% 중간 구간 가산 기준값",
    "rc_norp_bonus" => "리콜포인트(귀환 지점)가 없을 때 mult 가산",
    "rc_ed_near" => "리콜포인트→가장 가까운 적 거리 근접 임계(미만이면 적이 가까움→복귀 억제)",
    "rc_ed_mid" => "리콜포인트→적 중간 거리 임계(이 구간은 가감 없음)",
    "rc_ed_far" => "리콜포인트→적 원거리 임계(이상이면 초원거리 가산)",
    "rc_ed_near_pen" => "적이 리콜포인트에 근접할 때 감산(복귀 위험→억제)",
    "rc_ed_far_bonus" => "적이 원거리일 때 가산(안전→복귀 유도)",
    "rc_ed_vfar_bonus" => "적이 초원거리일 때 가산(더 큰 복귀 유도)",
    "rc_ahp_t1" => "아군 오브젝트 HP% 상한 — 이 미만이면 건강 보너스(rc_u13_bonus) 취소",
    "rc_ahp_t2" => "아군 오브젝트 HP% 하한 — 이 미만이면 추가 감산(rc_ahp2_pen)",
    "rc_u13_bonus" => "아군 오브젝트가 건강할 때 mult 가산",
    "rc_ahp2_pen" => "아군 오브젝트가 위험(HP%<rc_ahp_t2)일 때 감산",
    "rc_ad_near" => "아군 오브젝트→적 거리 근접 임계(미만이면 적이 아군 근처→복귀 유도)",
    "rc_ad_mid" => "아군→적 중간 거리 임계(이 구간 가감 없음)",
    "rc_ad_near_bonus" => "적이 아군 오브젝트에 근접할 때 가산(지키러 복귀)",
    "rc_ad_far_pen" => "적이 아군 오브젝트에서 멀 때 감산(급할 것 없음)",
    "rc_mult_bonus" => "아군 수가 적보다 많을 때(수적우세) mult 가산",
    "rc_ally_hp_min" => "아군 오브젝트를 '유효'로 셀 HP% 하한(이하면 무시)",
    "rc_join_weight" => "합류 이득 마스터 가중. 0=끔(기존 동작, 배포 기본). >0이면 체력이 멀쩡해도 합류 이득이 클수록 복귀 점수↑ → 복귀를 전략적 합류 이동기로. ※체력기반 점수에 더하는 게 아니라 max(체력기반 vs 합류기반 중 강한 쪽 채택). 권장 시작 20",
    "rc_join_adv" => "수적 우위(아군≥적)일 때 합류 보너스 계수. 클수록 승산 한타에 적극 합류",
    "rc_join_rescue" => "수적 열세(아군<적)일 때 구원 보너스 계수. 클수록 열세 아군 전투를 도우러 합류",
    "rc_join_dnear" => "합류 대상까지 근접 거리 임계(미만이면 거리가중 ×3). 작을수록 가까운 전투만 합류",
    "rc_join_dmid" => "합류 대상까지 중거리 임계(미만 ×2, 이상 ×1). 멀수록 합류 이득 감가",
    "rc_join_obj_mult" => "리콜포인트(거점/오브젝트)가 있을 때 합류 이득 배수. 오브젝트 전투 합류 우선",
    "eng_role4" => "핵심 타깃(우선순위 4) 후퇴 전환점. 난수 ≥ (이값×t_engage/100)이면 후퇴. 클수록 더 적극 교전. 기본 100",
    "eng_role3" => "우선순위 3 후퇴 전환점. 기본 70",
    "eng_role2" => "우선순위 2 후퇴 전환점. 기본 50",
    "eng_role_def" => "기타(낮은 우선순위) 후퇴 전환점. 기본 30 (가장 낮음=가장 잘 후퇴)",
    "d4_dmg_scale" => "⚠DEAD(050 하드코딩·값 무반영) 데미지를 처치 기여로 환산하는 배율. 클수록 TTD↓ → 갱킹 잘함. 기본 1000",
    "d4_div_base" => "⚠DEAD(050 하드코딩·값 무반영) 처치속도(coef) 계산의 분모 기준값. 키우면 처치를 더 빠르다고 계산. 기본 100",
    "d4_coef_scale" => "⚠DEAD(050 하드코딩·값 무반영) 처치속도(coef) 계산 배율. 기본 100",
    "d4_coef_min" => "⚠DEAD(050 하드코딩·값 무반영) 처치속도(coef) 최소 — 이보다 작으면 d4_coef_clamp로 고정. 기본 4",
    "d4_coef_clamp" => "⚠DEAD(050 하드코딩·값 무반영) coef가 너무 작을 때 대신 쓰는 값. 기본 3",
    "d4_coord_dist" => "⚠DEAD(050 하드코딩·값 무반영) 갱킹 사거리(거리²) — 이 안의 적만 판단. 기본 14400000001 = 120000²+1. 키우면 더 멀리도 갱킹",
    "d4_ttd_scale" => "⚠DEAD(050 하드코딩·값 무반영) TTD 식의 분자 배율(대상HP × 이값). 키우면 TTD↑ = 갱킹 덜함. 기본 1000",
    "gb_rbx_div" => "⚠DEAD(050 하드코딩·값 무반영) 근거리 구간 경계 = base / 이값. 키우면 근거리 범위 좁아짐. 기본 100",
    "gb_r15_div" => "⚠DEAD(050 하드코딩·값 무반영) 중거리 구간 경계 = base / 이값. 기본 50",
    "gb_r14_num" => "⚠DEAD(050 하드코딩·값 무반영) 원거리 구간 경계 = base × 이값 / 100. 키우면 원거리 관여 범위↑. 기본 3",
    "pf_edge_margin" => "맵 가장자리 띠 두께 — 이 안쪽이면 '가장자리'로 판정. 실좌표",
    "pf_center_band" => "중앙 대각선 띠 폭(맵 중앙선 근처인지 판정)",
    "pf_diag_far" => "대각선 띠의 원거리 컷",
    "pf_diag_near" => "대각선 띠의 근거리 컷",
    "pf_band_width" => "위치판정 한 모드(m1)에서 쓰는 띠 폭(세로−가로 절댓값 비교 기준)",
    "pk_home_lo" => "⚠DEAD(050 하드코딩·값 무반영) 홈영역 하위 X/Y 경계. 기본 64000",
    "pk_home_hi" => "⚠DEAD(050 하드코딩·값 무반영) 홈영역 상위 X/Y 경계. 기본 960000",
    "pk_home_x1" => "⚠DEAD(050 하드코딩·값 무반영) 홈 판정 X 안쪽 경계. 기본 892000",
    "pk_home_y1" => "⚠DEAD(050 하드코딩·값 무반영) 홈 판정 Y 안쪽 경계. 기본 896000",
    "pk_hp_main" => "⚠DEAD(050 하드코딩·값 무반영) 견제 진입 HP% — self HP%가 이보다 높고 안전하면 견제. 기본 50",
    "pk_hp_retreat" => "⚠DEAD(050 하드코딩·값 무반영) 귀환 HP% — 홈+완료에서 HP%가 이 미만이면 귀환. 기본 51 (에픽)",
    "pk_smallact_split" => "⚠DEAD(050 하드코딩·값 무반영) 소액션 코드 분기 임계. 기본 33",
    "pk_threat_mult" => "⚠DEAD(050 하드코딩·값 무반영) 위협 스케일 배수. 기본 5",
    "pk_zone_hp" => "⚠DEAD(050 하드코딩·값 무반영) zone(에픽 지역) 진입 HP% — 이 미만이면 진입 자제. 기본 21",
    "pk_engage_dist" => "⚠DEAD(050 하드코딩·값 무반영) 에픽 교전 거리²(이 안이면 교전 고려). 기본 22500000001",
    "pk_obj_hp" => "⚠DEAD(050 하드코딩·값 무반영) 오브젝트 HP% — 뱀(serpen)에서 이보다 높으면 관련 판단. 기본 20",
    "dd_frontier_mult" => "✅LIVE(07-23 부활) 전선 계산 배수 — (적 전선진척 − 아군 전선진척 × 이값)가 작으면 후퇴. 키우면 더 잘 물러남. 기본 30. ★07-23 이전엔 커버블록이 막혀 있어 값이 안 먹었음",
    "sv_enable" => "★[07-23 신설] 공유 위협 severity 사다리 byte-patch <b>마스터 스위치</b>. 0(기본)=게임 원본. 1이면 사본 4곳 29사이트에 sv_* 값이 일괄 적용. 적용확인=sev_imm.txt.",
    "sv_tr0" => "위협비율 기본 문턱(체력 무관, 원본 49). tr>이값이면 '심각'. ↓=겁쟁이(작은 위협도 심각) ↑=대담.",
    "sv_tr1" => "체력%<66 구간 문턱(원본 29). 다친 상태에선 더 낮은 문턱이 적용되는 구조.",
    "sv_tr2" => "체력%<41 구간 문턱(원본 17).",
    "sv_tr3" => "체력%<26 구간 문턱(원본 9). 빈사 상태 민감도.",
    "sv_hp1" => "1단계 체력 경계(원본 65 = 체력%>65면 기본 문턱만 적용).",
    "sv_hp2" => "2단계 체력 경계(원본 40).",
    "sv_hp3" => "3단계 체력 경계(원본 25).",
    "sv_discount_shift" => "위협 평가 정본 전용: '사소' 판정된 위협의 할인 지수(원본 2 = 1/4로 축소). 0=할인 없음(사소해도 전액 반영=전체적으로 겁쟁이), 커질수록 사소 위협을 더 무시.",
    "sv_discount_cap" => "위협 평가 정본 전용: 할인 후 위협 상한(원본 18). ↓=사소 위협의 영향 상한을 더 낮게.",
    "vis_window" => "✅LIVE[07-16 부활]. 비-라인 전반 적 시야기억창(틱). 적이 사라져도 이 틱동안 '아는 적'으로 추적. 단일 공유 byte-patch(0x1caedd3, 13개 호출처 공용)라 <b>여러 판단에 한꺼번에</b> 걸림 — 영향: <b>넥서스 커밋(18·19)·오브젝트 평가·모르가드(12~14)·세르펜(15~17)·CONDGATE</b>. ↑=더 오래 경계, ↓=빨리 잊음. 기본 600(≈10초, =원본). 0=즉시 망각(주의).",
    "dd_lane_margin" => "✅LIVE(07-23 부활) ★dd7700(라인 판단) 전용 — 사라진 적 기억 시간창(틱). 기본 120(≈2초). ↑=더 오래 경계, ↓=빨리 잊음",
    "dd_cover_count" => "✅LIVE(07-23 부활) 수비 합류 발동 적 수 — 근처 적이 이 수 이상이면 라인 버리고 합류. 기본 2. 0으로 두면 상시 합류 시도(합류 매우 잦아짐)",
    "dd_ratio_thr" => "✅LIVE(07-23 부활) 수비 합류 라인비율 기준 — 라인 비율이 이 미만이면 합류. 기본 51",
    "dd_facet_thr" => "운영 단계 전환 기준 — 운영 진척값이 이보다 크면 강하게 밀기. 기본 999",
    "dd_near_dist" => "△부분반영 — 근처 적 세는 거리². 기본 87890624. 교전 후반(engage) 단계 소관이라 그 경로를 탈 때만 반영됨",
    "dd_main_near_dist" => "내 주변 적 판정 거리²(이 안이면 가까운 적). 기본 87890625 ≈ 9375²",
    "dd_gatee_dist" => "특정 지점↔타깃 근접 거리². 기본 112890625",
    "dd_ivar2_thr" => "라인 진척 단계 기준값. 기본 2",
    "dd_n_thr" => "라인 슬롯 수 기준값. 기본 2",
    "dd_survivor_thr" => "생존자 수 기준(이하면 조정). 기본 3",
    "ep_lane_margin" => "⚠DEAD(050 하드코딩·값 무반영) 라인 밀기 허용 여유(라인전 dd_lane_margin과 같음). 기본 120",
    "ep_pred_dist" => "⚠DEAD(050 하드코딩·값 무반영) 에픽 위협 판정용 넓은 근접 거리²(240000²). 기본 57600000001",
    "ep_near_dist" => "⚠DEAD(050 하드코딩·값 무반영) 에픽 근접 거리²(적 웨이포인트가 이 안이면 위협). 기본 14400000001 = 120000²",
    "ep_home_lo" => "✅[재배선] EpicPoke: self가 아군 베이스/분수 회복존 안 + 부상(HP<max)이면 에픽 견제 나가지 말고 홈 대기(7). 영역 X/Y 하위 경계(side0=좌상단·side1=우하단 대각). ※RE확정: 부상 유닛 힐존(geom+0x6d70), epic·넥서스 아님. 기본 64000",
    "ep_home_hi" => "✅[재배선] 아군 베이스/분수 회복존 X/Y 상위 경계. 기본 960000",
    "ep_home_x1" => "✅[재배선] 회복존 X 안쪽 경계(side1=우하단). 기본 892000",
    "ep_home_y1" => "✅[재배선] 회복존 Y 안쪽 경계(side0=좌상단). 기본 896000",
    "ep_hp_crit" => "✅[재배선] EpicPoke: self HP%가 이 값 이하 & 에픽 오브젝트 근처 아군<적이면 견제 중단·후퇴(7). ↑=조금만 다쳐도 물러남. 기본 20",
    "ep_hp_low" => "⚠DEAD(050 하드코딩·값 무반영) 저 HP%(<31). 기본 31",
    "ep_count_gate" => "⚠DEAD(050 하드코딩·값 무반영) EpicPoke 행동 발동 카운트 기준. 기본 39",
    "ep_nexus_hp" => "✅[재배선] EpicPoke: (위협 대상 풀피 국면) self HP%가 이 값 미만이면 견제 중단·홀드(7). ※이름의 '넥서스'는 오판 잔재 — 실제 판정은 넥서스가 아니라 self 챔프 HP%. 기본 51",
    "d8_slot_thr" => "disc8 슬롯 우선순위 임계(슬롯값 <5 / ≥5 분기). 기본 5",
    "bt_home_lo" => "⚠DEAD(050 하드코딩·값 무반영) 홈 하위 경계(64000). 정상매치선 미발화. 기본 64000",
    "bt_home_hi" => "⚠DEAD(050 하드코딩·값 무반영) 홈 상위 경계(960000). 기본 960000",
    "bt_home_x1" => "⚠DEAD(050 하드코딩·값 무반영) 홈 판정 X 안쪽 경계. 기본 892000",
    "bt_home_y1" => "⚠DEAD(050 하드코딩·값 무반영) 홈 판정 Y 안쪽 경계. 기본 896000",
    "bt_hp_retreat" => "⚠DEAD(050 하드코딩·값 무반영) 귀환 HP%(<51). 기본 51",
    "fast_read" => "AI judge가 게임 메모리를 읽는 방식. 2=VEH lockless(가장 빠름, 권장). 문제 생기면 1→0 으로 낮춰 롤백",
    "fast_guard" => "✅[07-16 최적화] 메모리 범위검사(readable/writable)를 syscall(VirtualQuery) 대신 VEH 프로브로 가속. fast_read=2일 때만 유효. 1(기본)=가속, 0=원본(느림) 롤백. 시뮬 속도에 직결 — 문제(크래시) 생기면 0으로",
    "hang_diag" => "✅[07-16] 행(멈춤) 진단 워치독. 게임이 멈추면(일정 넘기다 연산 안 끝남 등) 자동으로 hang_diag.txt에 원인 덤프 — STALL(스레드 갇힘: 전 스레드 위치+스택) / RUNAWAY(시뮬이 계속 돎: 경기 교착 의심). 1(기본)=ON, 0=OFF. 오버헤드 무시가능",
    "hang_secs" => "[행진단] STALL 판정 대기 초. AI 판단이 이 시간 이상 완전 정지 + CPU 바쁨이면 덤프. 기본 8",
    "hang_run_secs" => "[행진단] RUNAWAY 판정 초. 고속 시뮬(judge 콜이 hang_run_rate/s 이상)이 이 시간 연속되면 '연산이 안 끝남' 덤프. 기본 30. 일정넘김이 원래 오래 걸리는 큰 리그면 60~120으로",
    "hang_run_rate" => "[행진단] RUNAWAY로 볼 초당 judge 콜수 문턱. 라이브 관전(수백/s)은 안 걸리고 백그라운드 고속시뮬만 걸리게 하는 값. 기본 5000",
    "adv_prof" => "✅[07-16] 활동창 프로파일러. 큰 연산(일정넘김 등)이 돌면 자동으로 구간을 감지해 adv_prof.txt에 기록 — 소요시간·CPU·judge 콜분포·<b>어느 게임코드(exe+RVA)에서 시간이 갔는지 TOP20</b>. 항상 켜두고 느린 일정넘김이 나오면 로그 확인. 1(기본)=ON. <b>⚠배포(zip)용 cfg에선 0으로</b>",
    "adv_prof_min" => "[프로파일러] 이 ms 미만의 짧은 활동창은 로그 생략(스팸 방지). 기본 3000(=3초 이상 연산만 기록)",
    "adv_prof_seg" => "[프로파일러] 끝나지 않는 긴 연산(일정넘김 안 끝남 등)의 중간 스냅샷 간격(ms). 이 간격마다 그때까지의 TOP 코드분포를 강제로 남김 → 무한연산도 어디서 시간 가는지 확인. 기본 15000(15초)",
    "sim_unchunk" => "⛔DEAD(0.5.3) — 패치 사이트 12B 시그가 0.5.3 exe 전역 0건(rayon 브리지 코드 변경). 원본바이트 재검증 후에만 패치하므로 ABORT=fail-safe(게임 무영향)이나 노브는 무반영. 실측=sim_unchunk.txt. (구 설명) [07-16 실험] 백그라운드 경기 시뮬 <b>병렬도 개선</b>. 일정넘김 때 게임이 여러 경기를 1개 rayon job에 묶어 순차처리(코어 ~60%만 사용) — 이걸 1경기=1job으로 쪼개 노는 코어를 채움. 게임 rayon 분할 게이트 1곳을 nop(정적·thread-safe·크래시 위험 없음, RE확정). <b>결과(경기 승패)는 불변</b>, 속도(가동률)만↑. 1=ON. ⚠효과는 배치 경기수·외곽 직렬병목에 캡됨 → 켜고 일정넘김 시간 A/B 직접 측정, 이상하면 0(원본복원). sim_unchunk.txt에 적용확인.",
    "cond_repl" => "facet#1 condgate(목표 커밋 판단)를 우리 코드로 대체",
    "gbskip" => "generic_build 영역D(운영전환)를 우리 코드로 대체(게임 원본 건너뜀)",
    "mp_repl" => "facet#4 movepriority 대체 — 이동 판단 마스터 스위치(아래 dd7/poke가 이걸 따름)",
    "dd7_repl" => "disc3 dd7700(라인전 이동) 대체 — mp_repl 켜져야 동작",
    "poke_repl" => "disc9/11 견제 대체(Battle/Hide) — mp_repl 켜져야 동작",
    "recall_repl" => "facet#5 recall(복귀 판단) 대체",
    "engage_repl" => "facet#5 교전 진입 대체",
    "e9jt" => "engage 점프테이블 경로 사용(정확도↑). 보통 1",
    "d4_repl" => "disc4(갱킹) 대체. 0=이 부분만 게임 원본 위임",
    "d7_repl" => "disc7(귀환 이동판단) 라이브 대체. 0(기본)=게임 원본. 1이면 아래 d7_* 반영. ⚠신모델 인게임 검증 진행중",
    "d7_hp_normal" => "🆕 disc7 일반 후퇴 HP%(자힐 없음). 이 % 이상이면 버팀(8), 미만이면 귀환(7). 기본 41(0x29)",
    "d7_hp_selfheal" => "🆕 disc7 자힐(HP리젠) 보유 시 후퇴 HP%(더 낮게 버팀). 기본 21(0x15)",
    "d7_wp_dist2" => "🆕 disc7 웨이포인트 근접 거리²(이 안이면 위협 TTD 판정). 기본 14400000000=120000²",
    "d4ttd" => "disc4 TTD 계산 경로 사용. 보통 1",
    "perf_measure" => "judge별 처리시간 측정→perf.txt. 측정 자체가 약간 느리게 함 → 평소 0",
    "read_bench" => "메모리 read 방식 벤치마크→readbench.txt. 평소 0",
    "replay_reset" => "다시보기 시작 시 내부 상태 리셋. 보통 1",
    "enabled" => "SDK 기반 Move override(개발용). 평소 0",
    "team" => "override 대상 팀(개발용)",
    "x" => "override 좌표 X(개발용)",
    "y" => "override 좌표 Y(개발용)",
    "coef_mult" => "데미지 계수 배율%(검증용). 기본 100",
    "engage_base" => "engage 베이스 임계 정적 패치(-1=원본)",
    "engage_thr_mult" => "engage 임계 배율%(구버전 레버). 기본 100",
    "tower_threat" => "★신규(게임에 없던 항). 포탑 조심 강도 0~100. 적 포탑 사거리 안에서 tower_threat≥전력승산이면 후퇴. 100=호각싸움도 수비, 0=원본. ※회피 본기능은 LIVE(라이너/is_under). (구설명의 '정글러 포탑딜 생존TTD 가산' 서브클레임은 dead 함수만 호출=무효). 기본 0",
    "tower_range" => "★신규. 포탑 위협 판정 반경 — 이 거리 안의 적 포탑만 셈. 작으면 가까이서만 반응, 크면 멀리서도. 기본 140000",
    "tower_dps" => "★신규. [재분석 07-16·정반대 정정] 실제=아군 포탑 DPS를 아군 전력(Σ공격)에 반영하는 힘 계수(ally_tower_contrib) — 라이너/전 disc 공용. 구설명의 '정글러 전용·라이너 미사용'은 오류(그 정글러-TTD 경로는 dead code). 포탑딜 단위(8000≈챔프1명분). 기본 8000",
    "numbers_threat" => "★신규(정식 DPS×HP). 일반교전 전력승산 회피 — (ΣHP)×(Σ공격) 비교 → numbers_threat≥승산이면 후퇴(포탑무관). 0=원본, 100=이길싸움만, 50=확실히 질때만. 기본 0",
    "numbers_range" => "★신규. 전력승산 계산할 때 근처 챔프/포탑 세는 반경(한타 때). 작으면 코앞만, 크면 넓게. 기본 150000",
    "numbers_range_move" => "★신규. 위 전력카운트 반경의 라인전 전용 값. -1=폴백(한타값 numbers_range 따름), N=라인전 반경. 라인전선 더 좁게/넓게 세고 싶을 때. 기본 -1",
    "numbers_min_enemy" => "★신규. 근처 적 챔프가 이 수 이상일 때만 전력후퇴 발동(머릿수 보조게이트, 한타). 1=현행, 2=적 2명+일때만. 기본 1",
    "numbers_min_enemy_move" => "★신규. 위 머릿수 게이트의 라인전 전용 값. -1=폴백(한타값 numbers_min_enemy 따름), N=라인전선 적 N명+일때만 후퇴. 기본 -1",
    "numbers_threat_move" => "★신규(라인전 멀뚱멀뚱 핵심). dd7700이 'Move(라인워크=딜교/미니언)'를 내려 할 때만 따로 적용하는 전력임계. -1=폴백(numbers_threat와 동일), 0=라인워크는 후퇴 안함(미니언/딜교 100% 보존). 게임이 교전/귀환(4/6/7) 의도일 땐 numbers_threat가 그대로 적용돼 한타 회피 유지. 기본 0(라인전 보존)",
    "ally_tower_hp" => "★신규. 아군 포탑의 '체력'을 아군 전력(ΣHP)에 반영하는 가중치 0~100(한타 때). 포탑을 탱커처럼 = 아군 유효HP↑ → 타워밑 승산↑. 포탑이 깎이면 같이 감소. 0=off, 100=포탑 풀HP. 기본 0",
    "ally_tower_hp_move" => "★신규. 위 포탑HP 가중치의 라인전 전용 값. -1=폴백(한타값 ally_tower_hp 따름), 0=라인전선 포탑HP 미반영, N=라인전 가중치. 기본 -1",
    "ally_tower_dps" => "★신규. 아군 포탑의 '공격력'을 아군 전력(Σ공격)에 반영하는 가중치 0~100(한타 때). 포탑을 딜러처럼 = 아군 공격↑ → 승산↑. 포탑딜 단위=tower_dps(8000). 0=off. 기본 0",
    "ally_tower_dps_move" => "★신규. 위 포탑DPS 가중치의 라인전 전용 값. -1=폴백(한타값 따름), 0=라인전선 미반영. 기본 -1",
    "ally_tower_range" => "★신규. 아군 포탑 인식범위(한타) — self 기준 이 반경 안의 아군 포탑만 전력에 반영. 작으면 코앞 포탑만, 크면 멀리 포탑도. 기본 150000",
    "ally_tower_range_move" => "★신규. 위 포탑 인식범위의 라인전 전용 값. -1=폴백(한타값 ally_tower_range 따름). 기본 -1",
    "numbers_threat_sp3"  => "★신규. disc3(실명 LineAttack=라인전) 전용 전력임계. 0=라인전선 후퇴 안 함(미니언/딜교 자유), -1=numbers_threat 폴백. 라인전에서 멀뚱멀뚱 안 들어가는 것 고치려면 여기를 0. 기본 -1",
    "numbers_threat_sp4" => "★신규. disc4(실명 LineSafe=정글) 전용 전력임계. -1=폴백(numbers_threat). 기본 -1",
    "numbers_threat_sp7" => "★신규. disc7(실명 Recall=복귀) 전용 전력임계. -1=폴백. 기본 -1",
    "numbers_threat_sp8" => "★신규. disc8(실명 Jungle) 전용 전력임계. -1=폴백. 기본 -1",
    "numbers_threat_sp9" => "★신규. disc9(실명 Battle=오브젝트 교전) 전용 전력임계. -1=폴백. 기본 -1",
    "numbers_threat_sp12" => "★신규. disc12(실명 EpicCheck=에픽 교전판단) 전용 전력임계. -1=폴백. 기본 -1",
    "numbers_threat_sp16" => "⛔DEAD(0.5.3 감사) — 코드에 read site 없음. 값 바꿔도 무반영. (구 설명) 세르펜 사냥(16) 전용 전력회피 임계. 세르펜 사냥 중 전력승산이 이값 미만이면 후퇴. ↑=아슬한 싸움도 뺌(겁많음)/0=세르펜 사냥 땐 후퇴안함. -1(기본)=공통 numbers_threat 따름. ※유리할 땐(팀과 함께) 안 걸림. '혼자 세르펜 가서 죽는' 억제용은 60~70 권장.",
    "numbers_threat_sp17" => "세르펜 견제(17) 전용 전력회피 임계. 세르펜 견제 중 불리하면 후퇴. ↑=겁많음/0=견제 땐 후퇴안함. -1(기본)=공통 따름. ※유리할 땐 안 걸림.",
    "numbers_threat_sp11" => "★신규. disc11(실명 Hide) 전용 전력임계. -1=폴백. 기본 -1",
    "numbers_threat_sp13" => "★신규. disc13(EpicHunt) 전용 전력임계. -1=폴백. 기본 -1",
    "numbers_threat_sp14" => "★신규. disc14(EpicPoke) 전용 전력임계. -1=폴백. 기본 -1",
    "stat_influence" =>"★신규. 성향스탯 반영 강도 0~100(0=off). 챔프 공격성·에고·판단력으로 라이너 후퇴판단 보정 — 공격성↑=덜 후퇴, 에고↑=잘 안 빠짐, 판단력↓=양방향 오판(결정론적). 기준=공50·에50·판100. 기본 0",
    "d19i_enable" => "넥서스 방어(disc19) byte-patch 마스터 스위치. 1=severity 사다리·d19_retreat_hp·phase 상수를 cfg값으로 패치. 0(기본)=게임 원본 복원(retreat=45 등). ★d19_retreat_hp를 실제 반영하려면 이게 1이어야 함(0이면 만져도 원본).",
    "d19_threat_mult" => "⚠[은퇴] 은퇴한 my_disc19 재현부(dcap-gated)에서만 read. d19thr=1 켜도 실제 게임 판단이 아니라 disc19cmp.txt 대조값만 바뀜(게임 행동 무영향). 실제 넥서스 방어 튜닝은 d19_retreat_hp(byte-patch·항상반영)·oi_dn_*(oi_enable=1). (넥서스 방어 위협점수 배수%.) 기본 100",
    "d19_retreat_hp" => "✅게이트무관 항상반영. 넥서스 후퇴 HP%문턱. 위협이 애매할 때 넥서스HP%가 이 값 이하면 후퇴. ↑=높은 HP에도 후퇴(수비적), ↓=저HP까지 안 후퇴(공격적). 원본 45",
    "d19_range_atkme" => "⚠[은퇴] 은퇴 my_disc19 재현부(disc19cmp 대조값)만 read=게임 행동 무영향. '넥서스(나)를 직접 공격중' 적 위협 가중. 원본 100",
    "d19_range_bld" => "⚠[은퇴] disc19cmp 대조값만=게임 무영향. '내 다른 건물 공격중' 적 위협 가중. 원본 60",
    "d19_range_other" => "⚠[은퇴] disc19cmp 대조값만=게임 무영향. '딴 대상 공격중' 적 위협 가중. 원본 40",
    "d19_range_idle" => "⚠[은퇴] disc19cmp 대조값만=게임 무영향. '비교전(놀고있는)' 적 위협 가중. 원본 80",
    "poke_phase_gate" => "⛔DEAD(0.5.2) — 대응 게이트가 원본에서 삭제됨. 값 바꿔도 무반영. 편집기 잔존은 cfg 호환용",
    "poke_active_min" => "⛔DEAD(0.5.2) — 대응 게이트가 원본에서 삭제됨. 값 바꿔도 무반영",
    "poke_reach_bonus" => "✅[재배선] 근접 도달거리 보너스(좌표단위). ↑=더 멀어도 contested 판정. 기본 120000",
    "poke_serpen_slot" => "✅[재배선] serpen 웨이포인트 점유 임계(코드 5). 기본 5",
    "dd_early_p3_thr" => "⛔DEAD(0.5.2, 07-23 확정) — 원본에 p3 비교 게이트가 아예 없음(0.5.0_3엔 있었음). 이 게이트가 조기분기를 항상 차단해 code 4·6이 전량 소실되던 버그의 원인이었고, 07-23에 삭제됨. 값 무반영",
    "dd_cover_p3_thr" => "⛔DEAD(0.5.2, 07-23 확정) — 원본에 대응 게이트 없음. 이 값이 커버블록 전체를 막고 있어서 dd_frontier_mult·dd_lane_margin·dd_cover_count·dd_ratio_thr 가 전부 안 먹던 원인. 07-23 삭제 후 그 4개가 실제로 작동. 값 무반영",
    "dd_f22e80_margin" => "✅[재배선] 생존자카운트 웨이포인트-타깃 근접 허용폭(√거리). 기본 150000",
    "rc_rng_center" => "✅[재배선] 복귀배율 RNG 밴드 중심%(평균). ※recall_repl=1 완전대체시 반영. 기본 100",
    "rc_rng_spread_div" => "✅[재배선] 복귀배율 RNG 분산폭 제수(작을수록 변동↑). 기본 20",
    "rc_rng_a_base" => "✅[재배선] 공격성 기준선(spread base). 기본 1000",
    "rc_score_div" => "✅[재배선] recall score 정규화 제수. 기본 100",
    "stat_neutral" => "✅[재배선] 공격성/에고 중립 피벗(50=중립). ↓=더 많이 공격적분류(덜 후퇴). 기본 50",
    "stat_pos_div" => "✅[재배선] 공격방향 감쇠 제수(2=절반반영, 1=풀강도). 기본 2",
    "stat_judg_ref" => "✅[재배선] 판단력 노이즈 기준(이상이면 노이즈0=완벽판단). 기본 100",
    "stat_noise_shift" => "✅[재배선] 판단력 노이즈 시간코히런스 시프트(tick>>N). ↑=느리게 갱신(뭉툭). 기본 5",
    "disc17_prog_low" => "✅[재배선] 세르펜(SerpenPoke) 진척 저 게이트(미만이면 DEFEND). 기본 31",
    "disc17_prog_crit" => "✅[재배선] 세르펜 진척 위급 게이트. 기본 21",
    "disc17_p3_gate" => "✅[재배선] 위협 병합경로 게이트(param3 초과, 코드 0x26). 기본 38",
    "disc17_ref_hp" => "✅[재배선] ref 대상 풀피%(이하면 조기 진격). 기본 50",
    "disc17_near_dist" => "✅[재배선] 근접 판정 거리²(120000²). 기본 14400000001",
    "disc17_pred_dist" => "✅[재배선] 예측 위협 거리²(240000²). 기본 57600000001",
    "d13_engage_hp_pct" => "⛔DEAD — [13 EpicHunt]은 게임이 생성하지 않는 죽은 subplan(오더코드 도메인에 Hunt 코드 부재 + 런타임 0발화 실측). 값 무반영",
    "disc16_home_hp" => "✅LIVE[07-16 신규]. [16 세르펜 사냥] self가 아군 지대 안에서 체력%가 이 값 미만이면 홈 대기(7). 기본 100=원본(=조금이라도 다치면 대기). ↓낮추면 더 크게 다쳐야 대기(공격적으로 계속 사냥).",
    "d15_repl" => "[15 세르펜 교전판단] 라이브 대체 스위치. 0(기본)=게임 원본. 1이면 아래 d15_engage_hp_pct 반영. ⚠재현 미검증(표본 부족)이라 켜면 인게임 테스트 권장(이상하면 0으로 롤백).",
    "d15_engage_hp_pct" => "[15 세르펜 교전판단] self 체력%가 이 값 미만이면 (목표 풀피 시) 교전 개시(7). ↑=더 공격적. ★d15_repl=1 켜야 반영(기본 OFF). 기본 51",
    "ec_oz_hp" => "✅[재배선] EpicCheck 존밖 HP% 상한(초과시 교전). 기본 50",
    "ec_iz_hp" => "✅[재배선] 존안 HP% 하한(미만시 귀환). 기본 51",
    "ec_self_hp_low" => "✅LIVE(07-23 신규) [12] 자기 체력%가 이 값 이하이고 아군이 수적 열세면 철수(7). 기본 20. ↑=더 쉽게 물러남. ★구 ec_tgt_hp_low(타겟 체력 기준)를 대체 — 원본은 자기 체력을 본다",
    "ec_tgt_hp_low" => "⛔DEAD(07-23 대체됨) → ec_self_hp_low 사용. 기존 재현이 타겟 체력을 봤으나 원본은 자기 체력 기준(0.5.2 disasm 확정). 이 키는 이제 읽는 곳 없음",
    "ec_engage_dist2" => "✅[재배선] 교전 거리²(150000²). 기본 22500000001",
    "ec_valid_hp" => "✅[재배선] 유효타겟 최소 HP%. 기본 40",
    "ec_gate_tick" => "⛔DEAD(0.5.2, 07-23 확정) — engage_gate·reposition_fight 양쪽에서 tick 인자가 원본 본문에 아예 쓰이지 않음(프롤로그에서 즉시 파괴). 값 무반영",
    "ec_commit_hp" => "✅[재배선] commit 아군 HP% 하한. 기본 40",
    "ec_count_hp" => "✅[재배선] 아군/적 카운트 HP% 하한. 기본 40",
    "ec_count_radius" => "✅[재배선] 아군/적 카운트 반경. 기본 180000",
    "ec_vision_ticks" => "✅[재배선] 시야포그 유지 틱. 기본 120",
    "oi_enable" => "objective 원본상수 대체 마스터 스위치. 0(기본)=무개입(원본 복원). 1=oi_* 값으로 넥서스 방어/공격 원본상수 byte-patch. ✅[07-16] 0.5.1 재핀 완료·인게임 applied=13/13 검증(정상 동작). 기본 0",
    "oi_dn_count_gate" => "⛔DEAD(0.5.2) — 대응 상수가 원본에서 삭제됨(그 자리가 <code>cmp rdx,[rbp-0x50]</code> 레지스터 비교로 대체=상수 소멸). 값 바꿔도 무반영. 구 설명: 수비 발동 최소 아군 인원(원본 38)",
    "oi_dn_nexus_hp" => "[oi] 아군 넥서스 HP%≤이면 적극 수비 전환. ↑=일찍 수비. 원본 50",
    "oi_dn_hp_crit" => "[oi] 챔프 위급 HP% 임계. 원본 21",
    "oi_dn_hp_low" => "[oi] 챔프 저 HP% 임계. 원본 31",
    "oi_dn_near_dist" => "[oi] 넥서스 근접 판정 거리(코드가 제곱). ↑=더 멀리서 근접판정. 원본 120000",
    "oi_dn_pred_dist" => "[oi] 넥서스 위협 예측 거리(술어). 원본 240000",
    "oi_dn_lane_margin" => "[oi] 레인 진척 허용 마진. ↑=더 밀려도 수비 유지. 원본 120",
    "oi_an_finish_hp" => "[oi·확증07-16] 적 넥서스 <b>마무리오더</b> 발행 HP% 게이트(disc18 0x1c7df47). 적넥서스 HP%가 <b>이값 이상</b>이면 아군 합류수 무관 즉시 마무리오더, 미만이면(13분 이후) 아군 2명↑ 도달가능할 때만 발행. ↓낮추면 더 낮은 넥서스 HP%에서도 단독 압박=<b>공격적</b>, ↑높이면 스쿼드 대기=신중. 원본 56",
    "oi_an_cull_dist" => "[oi·확증07-16] 넥서스 공격후보 <b>거리 컬링</b>(disc18 0x1c7d5f9). 아군↔적넥서스 거리²(>>14 스케일)가 이값 초과면 그 아군은 넥서스 공격후보에서 제외. ↑높이면 더 먼 아군도 넥서스 공세 동원(탐색범위↑), ↓낮추면 근접 아군만. 원본 390624(≈넥서스 2.5셀 반경)",
    "oi_an_count_gate" => "⛔DEAD(0.5.2 · 애초에 <b>오식별</b>이었음 — 재핀 금지) — 이 사이트는 튜닝 레버가 아니라 <b>컴파일러가 뿌린 배열 bounds-check 관용구</b>였다(0.5.2에 같은 패턴 37곳: <code>cmp [X+0x5b0],N</code> → <code>lea 정적더미</code> → <code>cmovae 실원소</code> → <code>cmp [reg+0x30],-1</code>, imm 3/5가 항상 짝). 값을 바꾸면 <b>없는 원소를 실포인터로 읽어 OOB=크래시</b>라 사이트 자체를 제거했다. ※구 설명(07-16 '강화방어 프로파일 임계')도 폐기.",
    "gb_enable" => "✅[07-16 경로A] GenericBuild 로밍 byte-patch <b>마스터 스위치</b>. 0(기본)=게임 원본 그대로. 1이면 아래 gb_* 로밍/게이트 값이 실제 게임 판단상수에 덮어써짐(각 값 -1이면 그 항목만 원본유지). 인게임 gb_imm.txt에 applied=N/10 기록. ★0.5.2 재핀 완료(07-23) — 그 전까진 주소가 어긋나 applied=0/12=전량 무반영이었음. gb_join_phase는 0.5.2에서 死.",
    "gb_join_dist" => "✅[07-16] 합류/근접 <b>전환거리</b>(유닛, 원본 60000). 대상까지 이 거리 이내면 '근접/합류 모드'로 전환하는 <b>지배 게이트</b>(라인range 세팅도 여기서). ↑키우면 더 먼 거리에서도 합류/근접 모드=적극 합류, ↓줄이면 바짝 붙어야 합류. -1=원본. (거리²로 인코딩됨, 유닛으로 입력)",
    "gb_scout_radius" => "✅[07-16] 거점/타겟 <b>후보수집 반경</b>(유닛, 원본 120000)=로밍 탐색범위. 후보 유닛/거점을 이 반경 안에서만 채택. ↑키우면 더 먼 거점까지 로밍 대상=로밍 범위 확장, ↓줄이면 근처만. -1=원본. ⚠수집 헬퍼(0x1e29xxx)의 GenericBuild 콜엣지 미확정 → gb_imm.txt applied 카운트로 sig매칭 확인 권장.",
    "gb_close_radius" => "✅[07-16] <b>근접 판정 반경</b>(유닛, 원본≈387). 근접 여부를 재는 반경 파라미터. ↑키우면 근접 판정 범위↑. -1=원본. (imm32 부호확장 제약: √값 46340 미만)",
    "gb_line_range" => "✅[07-16] <b>라인 판정 반경</b>(유닛, 원본≈500). 근접모드 진입 시 세팅되는 라인 거리 임계. -1=원본. (imm32 제약: √값 46340 미만)",
    "gb_op_phase" => "✅[07-16] <b>운영 진입 phase 임계</b>(원본 31, =경기진행 카운터>30). phase가 이 값 이상이면 운영 로직 진입. ↓낮추면 더 이른 시점부터 운영 시작, ↑높이면 늦게. -1=원본. (0~127 범위)",
    "gb_join_phase" => "⛔DEAD(0.5.2) — 게임이 합류 거리분기 직후의 phase 게이트를 삭제(리팩터링·전 인코딩 스캔 0건). 값 바꿔도 무반영. 합류 타이밍 자체는 이제 phase 무관·거리(gb_join_dist)만으로 갈림. 구 설명(원본 12·합류 허용 임계)은 0.5.1 이하 전용.",
    "gb_push_hp" => "✅[07-16] <b>라인 압박 HP% 임계</b>(원본 30). 라인 대상 체력%가 이 값 미만이면 압박 오더(order state=3). ↑높이면 더 높은 체력에도 압박=공격적, ↓낮추면 빈사일 때만. -1=원본. (0~127 범위)",
    "gb_reach_cap" => "⚠[07-16] <b>전역 사거리 상한</b>(유닛, 원본≈140052). ★GenericBuild 전용 아님 — 전 AI가 공유하는 reach 헬퍼(FUN_141e30c00, 10콜사이트)라 켜면 <b>모든 판단의 사거리 상한</b>이 바뀜(신중). ↑키우면 전반적으로 더 먼 대상 도달가능 판정. -1=원본(무개입 권장).",
    "gb_reach_margin" => "⚠[07-16] <b>전역 사거리 여유</b>(유닛, 원본 25000). reach 2차판정의 이동-외삽 여유폭. gb_reach_cap과 마찬가지로 <b>전 AI 공유</b>(신중). ↑키우면 사거리 판정에 여유↑. -1=원본(무개입 권장).",
    "gb_cnt_skip" => "⚠미확인(소스배선·미발화) GenericBuild 스킵 카운트 임계",
    "gb_da_thr" => "⚠미확인(소스배선·미발화) GenericBuild 거리A 임계",
    "gb_cnt_move" => "⚠미확인(소스배선·미발화) GenericBuild 이동 카운트 임계",
    "gb_db_engage" => "⚠미확인(소스배선·미발화) GenericBuild 거리B 교전 임계",
    "gb_score_mult" => "⚠미확인(소스배선·미발화) GenericBuild 점수 배율%",
    "d4_ward_dist2" => "⚠미확인(소스배선·미발화) [4]정글 워드/근접 거리²(코드>>8). 기본 0x6BA9301",
    "d4_engage_r2" => "⚠미확인(소스배선·미발화) [4]정글 교전 반경 거리²(코드>>8). 기본 0x53D1AC1",
    "d4_ref_dist2" => "⚠미확인(소스배선·미발화) [4]정글 참조/예측 거리². 기본 0x9502F9000",
    "d4_close_hp" => "⚠미확인(소스배선·미발화) [4]정글 근접 HP% 임계(이상이면 code4). 기본 51",
    "d4_threat_min" => "⚠미확인(소스배선·미발화) [4]정글 위협 최소 카운트. 기본 2",
    "d4_pathlen_thr" => "⚠미확인(소스배선·미발화) [4]정글 경로길이 임계. 기본 3",
    "d4_wcast_thr" => "⚠미확인(소스배선·미발화) [4]정글 캐스팅 임계(초과시 code7 else code6). 기본 2",
    _ => return None,
  })
}

// ============================ cfg 파서/직렬화 ============================
#[derive(Clone)]
enum Entry { Blank, Comment(String), Kv { key: String, val: String, desc: String } }

#[derive(Clone, Default)]
struct Model { entries: Vec<Entry>, map: HashMap<String, usize> }

fn to_dec(v: &str) -> String {
  let s = v.trim();
  let (neg, body) = if let Some(r) = s.strip_prefix('-') { (true, r) }
    else if let Some(r) = s.strip_prefix('+') { (false, r) } else { (false, s) };
  let hex = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X"));
  if let Some(h) = hex {
    if !h.is_empty() && h.chars().all(|c| c.is_ascii_hexdigit()) {
      if let Ok(n) = u128::from_str_radix(h, 16) {
        return if neg { format!("-{}", n) } else { n.to_string() };
      }
    }
  }
  s.to_string()
}

fn parse_kv(ln: &str) -> Option<(String, String, String)> {
  let eq = ln.find('=')?;
  let key = ln[..eq].trim();
  if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') { return None; }
  let rest = &ln[eq + 1..];
  let (val, desc) = match rest.find('#') {
    Some(h) => {
      let v = rest[..h].trim().to_string();
      let d = rest[h + 1..].trim().to_string();
      (v, d)
    }
    None => (rest.trim().to_string(), String::new()),
  };
  Some((key.to_string(), val, desc))
}

fn parse_cfg(text: &str) -> Model {
  let mut entries = Vec::new();
  let mut map = HashMap::new();
  for raw in text.split('\n') {
    let ln = raw.strip_suffix('\r').unwrap_or(raw);
    let t = ln.trim();
    if t.is_empty() { entries.push(Entry::Blank); continue; }
    if t.starts_with('#') {
      let r = t.trim_start_matches('#').trim_start().to_string();
      entries.push(Entry::Comment(r));
      continue;
    }
    if let Some((key, val, desc)) = parse_kv(ln) {
      map.insert(key.clone(), entries.len());
      entries.push(Entry::Kv { key, val: to_dec(&val), desc });
    } else {
      entries.push(Entry::Comment(t.to_string()));
    }
  }
  Model { entries, map }
}

// HTML note → 평문 (egui 는 HTML 미지원)
fn html_to_text(s: &str) -> String {
  let s = s.replace("<br>", "\n").replace("<br/>", "\n").replace("<br />", "\n");
  let mut out = String::new();
  let mut intag = false;
  for c in s.chars() {
    match c {
      '<' => intag = true,
      '>' => intag = false,
      _ if !intag => out.push(c),
      _ => {}
    }
  }
  out.replace("&ge;", "≥").replace("&le;", "≤").replace("&gt;", ">")
     .replace("&lt;", "<").replace("&nbsp;", " ").replace("&amp;", "&")
}

// ============================ 파일 I/O (UTF-8 BOM 없음) ============================
fn read_utf8(path: &Path) -> Option<String> {
  let bytes = std::fs::read(path).ok()?;
  // BOM 제거
  let bytes = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) { &bytes[3..] } else { &bytes[..] };
  Some(String::from_utf8_lossy(bytes).into_owned())
}
fn write_utf8_nobom(path: &Path, text: &str) -> std::io::Result<()> {
  std::fs::write(path, text.as_bytes())
}

// ============================ 앱 상태 ============================
struct App {
  folder: PathBuf,
  active_path: PathBuf,
  cfg_dir: PathBuf,
  default_path: PathBuf,
  default_text: String,
  defaults: HashMap<String, String>,
  model: Model,
  active_tab: usize,
  active_class: i8,   // -1 = 기본(전역) / 0..4 = 클래스(melee/range/magician/util/assassin)
  config_list: Vec<String>,
  selected_config: String,
  show_save_as: bool,
  save_as_name: String,
  toast: String,
  toast_err: bool,
  toast_until: f64,
}

impl App {
  fn new() -> Self {
    let folder = std::env::current_exe().ok()
      .and_then(|p| p.parent().map(|d| d.to_path_buf()))
      .unwrap_or_else(|| PathBuf::from("."));
    let active_path = folder.join("tfm2_ai_adjust.cfg");
    let cfg_dir = folder.join("config");
    let default_path = cfg_dir.join("default.txt");
    let _ = std::fs::create_dir_all(&cfg_dir);

    let mut default_text = read_utf8(&default_path)
      .or_else(|| read_utf8(&active_path))
      .unwrap_or_default();
    if default_text.is_empty() {
      // 둘 다 못 찾으면 모델만 빈 채로
      default_text = String::new();
    }
    let mut defaults = HashMap::new();
    for e in parse_cfg(&default_text).entries {
      if let Entry::Kv { key, val, .. } = e { defaults.insert(key, val); }
    }

    let active_text = read_utf8(&active_path).unwrap_or_else(|| default_text.clone());

    let mut app = App {
      folder, active_path, cfg_dir, default_path,
      default_text, defaults,
      model: Model::default(),
      active_tab: 0,
      active_class: -1,
      config_list: Vec::new(),
      selected_config: ACTIVE_NAME.to_string(),
      show_save_as: false,
      save_as_name: String::new(),
      toast: String::new(), toast_err: false, toast_until: 0.0,
    };
    app.refresh_list();
    app.load_into(&active_text);
    app
  }

  fn get_val(&self, k: &str) -> String {
    if let Some(&i) = self.model.map.get(k) {
      if let Entry::Kv { val, .. } = &self.model.entries[i] { return val.clone(); }
    }
    String::new()
  }
  fn set_val(&mut self, k: &str, v: &str) {
    if let Some(&i) = self.model.map.get(k) {
      if let Entry::Kv { val, .. } = &mut self.model.entries[i] { *val = v.to_string(); }
    } else {
      self.model.map.insert(k.to_string(), self.model.entries.len());
      self.model.entries.push(Entry::Kv { key: k.to_string(), val: v.to_string(), desc: String::new() });
    }
  }
  // 키 제거(클래스 오버라이드 '기본 따름' 복귀 시) — 엔트리 삭제 후 맵 재구축(인덱스 시프트 대응).
  fn remove_key(&mut self, k: &str) {
    if let Some(&i) = self.model.map.get(k) {
      self.model.entries.remove(i);
      self.model.map.clear();
      for (idx, e) in self.model.entries.iter().enumerate() {
        if let Entry::Kv { key, .. } = e { self.model.map.insert(key.clone(), idx); }
      }
    }
  }
  // 해당 클래스의 오버라이드 개수(탭 배지용)
  fn class_override_count(&self, pos: usize) -> usize {
    let suf = format!("_class_{}", CLASS_EN[pos]);
    self.model.map.keys().filter(|k| k.ends_with(&suf)).count()
  }
  fn desc_of(&self, k: &str) -> String {
    if let Some(d) = desc_static(k) { return d.to_string(); }
    if let Some(&i) = self.model.map.get(k) {
      if let Entry::Kv { desc, .. } = &self.model.entries[i] {
        if !desc.is_empty() { return desc.clone(); }
      }
    }
    "(설명 없음)".to_string()
  }

  fn load_into(&mut self, text: &str) {
    let base_src = if !self.default_text.is_empty() { self.default_text.clone() } else { text.to_string() };
    let mut base = parse_cfg(&base_src);
    let loaded = parse_cfg(text);
    for (k, &li) in &loaded.map {
      if let Entry::Kv { val, .. } = &loaded.entries[li] {
        if let Some(&bi) = base.map.get(k) {
          if let Entry::Kv { val: bv, .. } = &mut base.entries[bi] { *bv = val.clone(); }
        } else {
          base.map.insert(k.clone(), base.entries.len());
          base.entries.push(Entry::Kv { key: k.clone(), val: val.clone(), desc: String::new() });
        }
      }
    }
    self.model = base;
  }

  fn serialize(&self) -> String {
    let mut out: Vec<String> = Vec::new();
    for e in &self.model.entries {
      match e {
        Entry::Kv { key, val, .. } => { if is_removed(key) { continue; } out.push(format!("{} = {}", key, val)); }
        Entry::Blank => out.push(String::new()),
        Entry::Comment(r) => out.push(format!("# {}", r)),
      }
    }
    out.join("\r\n") + "\r\n"
  }

  fn changed_count(&self) -> usize {
    let mut n = 0;
    for e in &self.model.entries {
      if let Entry::Kv { key, val, .. } = e {
        if let Some(d) = self.defaults.get(key) { if d != val { n += 1; } }
      }
    }
    n
  }

  fn refresh_list(&mut self) {
    let mut names = vec![ACTIVE_NAME.to_string(), "default.txt".to_string()];
    if let Ok(rd) = std::fs::read_dir(&self.cfg_dir) {
      for ent in rd.flatten() {
        let nm = ent.file_name().to_string_lossy().into_owned();
        if nm.to_lowercase().ends_with(".cfg") && !names.contains(&nm) { names.push(nm); }
      }
    }
    self.config_list = names;
  }

  fn resolve_path(&self, name: &str) -> PathBuf {
    if name == ACTIVE_NAME { return self.active_path.clone(); }
    if name == "default.txt" { return self.default_path.clone(); }
    let safe: String = name.chars().filter(|c| !matches!(c, '\\'|'/'|':'|'*'|'?'|'"'|'<'|'>'|'|')).collect();
    let lower = safe.to_lowercase();
    let fname = if lower.ends_with(".cfg") || lower.ends_with(".txt") { safe } else { format!("{}.cfg", safe) };
    self.cfg_dir.join(fname)
  }

  fn set_toast(&mut self, ctx: &egui::Context, msg: impl Into<String>, err: bool) {
    self.toast = msg.into();
    self.toast_err = err;
    self.toast_until = ctx.input(|i| i.time) + 2.6;
  }

  fn do_load(&mut self, ctx: &egui::Context) {
    let name = self.selected_config.clone();
    let path = self.resolve_path(&name);
    match read_utf8(&path) {
      Some(txt) => { self.load_into(&txt); self.set_toast(ctx, format!("불러옴: {}", name), false); }
      None => self.set_toast(ctx, "불러오기 실패: 파일 없음", true),
    }
  }
  fn do_reset(&mut self, ctx: &egui::Context) {
    match read_utf8(&self.default_path) {
      Some(txt) => { self.load_into(&txt); self.set_toast(ctx, "기본값으로 초기화", false); }
      None => self.set_toast(ctx, "default.txt 를 찾지 못함", true),
    }
  }
  fn do_apply(&mut self, ctx: &egui::Context) {
    let _ = std::fs::copy(&self.active_path, self.cfg_dir.join("_prev.bak"));
    let data = self.serialize();
    match write_utf8_nobom(&self.active_path, &data) {
      Ok(_) => self.set_toast(ctx, "tfm2_ai_adjust.cfg 에 적용됨 → 게임 메뉴 진입 시 반영", false),
      Err(e) => self.set_toast(ctx, format!("적용 실패: {}", e), true),
    }
  }
  fn do_save_as(&mut self, ctx: &egui::Context) {
    let raw = self.save_as_name.trim().to_string();
    let cleaned: String = raw.chars().filter(|c| !matches!(c, '\\'|'/'|':'|'*'|'?'|'"'|'<'|'>'|'|')).collect();
    let cleaned = cleaned.trim().to_string();
    if cleaned.is_empty() { return; }
    let path = self.resolve_path(&cleaned);
    let data = self.serialize();
    match write_utf8_nobom(&path, &data) {
      Ok(_) => {
        let fname = path.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or(cleaned);
        self.refresh_list();
        self.selected_config = fname.clone();
        self.set_toast(ctx, format!("저장됨: config\\{}", fname), false);
        self.show_save_as = false;
        self.save_as_name.clear();
      }
      Err(e) => self.set_toast(ctx, format!("저장 실패: {}", e), true),
    }
  }
}

// ============================ 렌더 ============================
const BLUE: egui::Color32 = egui::Color32::from_rgb(0x5b, 0x9d, 0xff);
const GREEN: egui::Color32 = egui::Color32::from_rgb(0x48, 0xc7, 0x8e);

impl eframe::App for App {
  fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    ctx.set_visuals(egui::Visuals::dark());   // OS 라이트테마 추종 무시, 항상 다크
    // ── 상단 헤더 ──
    egui::TopBottomPanel::top("hdr").show_inside(root, |ui| {
      ui.add_space(4.0);
      ui.horizontal(|ui| {
        ui.heading("tfm2_ai_adjust 설정 편집기");
        ui.add_space(10.0);
        ui.label(egui::RichText::new(format!("변경 {}", self.changed_count())).color(BLUE));
      });
      ui.add_space(2.0);
      ui.horizontal_wrapped(|ui| {
        ui.label("설정 파일:");
        let list = self.config_list.clone();
        egui::ComboBox::from_id_salt("cfgsel")
          .selected_text(self.selected_config.clone())
          .width(260.0)
          .show_ui(ui, |ui| {
            for n in &list { ui.selectable_value(&mut self.selected_config, n.clone(), n.as_str()); }
          });
        if ui.button("불러오기").clicked() { self.do_load(ctx); }
        if ui.button("＋ config에 저장…").clicked() { self.show_save_as = true; self.save_as_name.clear(); }
        if ui.button("↺ 초기화").on_hover_text("모든 값을 config\\default.txt 로 되돌림").clicked() { self.do_reset(ctx); }
        let apply = egui::Button::new(egui::RichText::new("게임에 적용").color(egui::Color32::WHITE)).fill(egui::Color32::from_rgb(0x3a,0x6f,0xd8));
        if ui.add(apply).on_hover_text("tfm2_ai_adjust.cfg 덮어쓰기 (게임 재진입 시 반영)").clicked() { self.do_apply(ctx); }
      });
      ui.add_space(4.0);
      // ── 클래스 선택기: 기본(전역) / 클래스별 오버라이드 ──
      ui.horizontal_wrapped(|ui| {
        ui.label("클래스:");
        if ui.selectable_label(self.active_class == -1, "기본(전역)")
          .on_hover_text("모든 클래스 공통 기본값").clicked() { self.active_class = -1; }
        for p in 0..5usize {
          let cnt = self.class_override_count(p);
          let lbl = if cnt > 0 { format!("{} ({})", CLASS_KR[p], cnt) } else { CLASS_KR[p].to_string() };
          let mut txt = egui::RichText::new(lbl);
          if cnt > 0 { txt = txt.color(GREEN); }
          if ui.selectable_label(self.active_class == p as i8, txt)
            .on_hover_text("이 클래스만 다른 값을 줄 항목을 '기본 따름' 끄고 지정").clicked() { self.active_class = p as i8; }
        }
      });
      ui.add_space(4.0);
    });

    // ── 좌측 탭 ──
    egui::SidePanel::left("nav").resizable(false).exact_width(220.0).show_inside(root, |ui| {
      ui.add_space(6.0);
      ui.label(egui::RichText::new("탭").weak());
      ui.add_space(2.0);
      for (i, t) in TABS.iter().enumerate() {
        if ui.selectable_label(self.active_tab == i, html_to_text(t.title)).clicked() {
          self.active_tab = i;
        }
      }
    });

    // ── 본문 ──
    egui::CentralPanel::default().show_inside(root, |ui| {
      let tab: &'static Tab = &TABS[self.active_tab];
      egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgb(0x1c, 0x1f, 0x27))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(0x33,0x39,0x47)))
        .show(ui, |ui| {
          ui.label(egui::RichText::new(html_to_text(tab.note)).color(egui::Color32::from_rgb(0x9a,0xa3,0xb2)));
        });
      if self.active_class >= 0 {
        ui.add_space(4.0);
        ui.label(egui::RichText::new(format!(
          "▶ '{}' 클래스 편집 중 — 항목의 '기본 따름'을 끄면 그 항목만 이 클래스 전용 값. (켜짐=전역값 상속, 저장 안 됨)",
          CLASS_KR[self.active_class as usize])).color(GREEN));
      }
      ui.add_space(8.0);

      egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        egui::Grid::new("fields").num_columns(3).striped(true).spacing([14.0, 8.0]).show(ui, |ui| {
          for &k in tab.keys {
            if let Some(h) = k.strip_prefix('§') {
              // ★위 간격용 빈 행 (Grid 셀 내 add_space는 같은 행이라 안 벌어짐 → 별도 행으로)
              ui.add_space(18.0);
              ui.end_row();
              ui.label(egui::RichText::new(h).strong().color(egui::Color32::from_rgb(0xcd,0xd3,0xdf)));
              ui.label("");
              ui.label("");
              ui.end_row();
              continue;
            }
            let cur = self.get_val(k);
            let def = self.defaults.get(k).cloned();
            let changed = def.as_ref().map_or(false, |d| d != &cur);

            // 1열: 키 + 기본값 + 신규배지 (고정폭 — "기본 …" 큰 숫자도 안 잘리게)
            ui.vertical(|ui| {
              ui.set_min_width(190.0);
              ui.set_max_width(190.0);
              ui.horizontal(|ui| {
                let mut t = egui::RichText::new(disp_key(k)).strong().monospace();
                if changed { t = t.color(BLUE); }
                ui.label(t);
                // 폐기/미확인 뱃지가 신규 뱃지를 대체(우선). 값 무반영/미발화 격리 표시.
                if is_dead(k) {
                  ui.label(egui::RichText::new("폐기").small().strong().color(egui::Color32::from_rgb(0xe0,0x6c,0x6c)));
                } else if is_unfired(k) {
                  ui.label(egui::RichText::new("미확인").small().weak());
                } else if is_added(k) {
                  ui.label(egui::RichText::new("신규").small().strong().color(GREEN));
                }
              });
              ui.label(egui::RichText::new(format!("기본 {}", def.clone().unwrap_or_else(|| "—".into()))).small().weak());
            });

            // 2열: 컨트롤 (add_sized 로 폭 강제 — 그리드가 눌러도 큰 숫자 다 보이게)
            if self.active_class >= 0 {
              // ── 클래스 오버라이드 모드: '기본 따름' 토글 + 전용값 입력 ──
              let pos = self.active_class as usize;
              let pk = format!("{}_class_{}", k, CLASS_EN[pos]);
              let was = self.model.map.contains_key(&pk);
              let gval = cur.clone();                 // 전역 현재값(상속 대상)
              let mut inherit = !was;
              let mut new_val: Option<String> = None;
              ui.horizontal(|ui| {
                ui.checkbox(&mut inherit, "기본 따름");
                if inherit {
                  let mut gv = gval.clone();
                  ui.add_enabled(false, egui::TextEdit::singleline(&mut gv)
                    .desired_width(150.0).font(egui::TextStyle::Monospace));
                } else {
                  let mut v = if was { self.get_val(&pk) }
                              else if gval.is_empty() { "0".to_string() } else { gval.clone() };
                  let resp = ui.add_sized([150.0, 24.0],
                    egui::TextEdit::singleline(&mut v).font(egui::TextStyle::Monospace));
                  if resp.changed() || !was { new_val = Some(v.trim().to_string()); }
                }
              });
              if inherit { if was { self.remove_key(&pk); } }
              else if let Some(v) = new_val { self.set_val(&pk, &v); }
            } else if let Some(opts) = select_opts(k) {
              let mut sel = cur.clone();
              let shown = opts.iter().find(|(v, _)| *v == sel).map(|(_, l)| *l).unwrap_or(&sel).to_string();
              egui::ComboBox::from_id_salt(k).selected_text(shown).width(270.0).show_ui(ui, |ui| {
                for (v, l) in opts { ui.selectable_value(&mut sel, v.to_string(), *l); }
              });
              if sel != cur { self.set_val(k, &sel); }
            } else if is_toggle(k) {
              let mut on = cur == "1" || cur == "true";
              let lbl = if on { "켜짐" } else { "꺼짐" };
              if ui.checkbox(&mut on, lbl).changed() {
                self.set_val(k, if on { "1" } else { "0" });
              }
            } else if is_dead(k) || is_unfired(k) {
              // 값 무반영(폐기)/미발화(미확인) — 편집 불가(회색), set_val 호출 안 함.
              let mut v = cur.clone();
              ui.add_enabled(false, egui::TextEdit::singleline(&mut v)
                .desired_width(270.0).font(egui::TextStyle::Monospace));
            } else {
              let mut v = cur.clone();
              let resp = ui.add_sized([270.0, 24.0], egui::TextEdit::singleline(&mut v).font(egui::TextStyle::Monospace));
              if resp.changed() { self.set_val(k, v.trim()); }
            }

            // 3열: 설명 (줄바꿈 — 긴 한글이 폭을 무한히 늘려 다른 칸을 누르지 않게)
            ui.scope(|ui| {
              ui.set_max_width(460.0);
              ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
              ui.add(egui::Label::new(egui::RichText::new(self.desc_of(k)).color(egui::Color32::from_rgb(0x9a,0xa3,0xb2))));
            });
            ui.end_row();
          }
        });
      });
    });

    // ── config에 저장 모달 ──
    if self.show_save_as {
      let mut open = true;
      egui::Window::new("config 폴더에 저장")
        .collapsible(false).resizable(false).open(&mut open)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
          ui.label("저장할 이름 (예: 공격적):");
          let resp = ui.add(egui::TextEdit::singleline(&mut self.save_as_name).desired_width(280.0));
          let enter = resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
          ui.add_space(6.0);
          ui.horizontal(|ui| {
            if ui.button("저장").clicked() || enter { self.do_save_as(ctx); }
            if ui.button("취소").clicked() { self.show_save_as = false; }
          });
          ui.add_space(2.0);
          ui.label(egui::RichText::new("→ config\\<이름>.cfg 로 저장됩니다.").small().weak());
        });
      if !open { self.show_save_as = false; }
    }

    // ── 토스트 ──
    let now = ctx.input(|i| i.time);
    if now < self.toast_until {
      let (bg, fg) = if self.toast_err {
        (egui::Color32::from_rgb(0xf0,0xa0,0x20), egui::Color32::from_rgb(0x3a,0x26,0x00))
      } else {
        (GREEN, egui::Color32::from_rgb(0x06,0x23,0x1a))
      };
      egui::Area::new(egui::Id::new("toast")).anchor(egui::Align2::RIGHT_BOTTOM, [-18.0, -18.0]).show(ctx, |ui| {
        egui::Frame::NONE.fill(bg).inner_margin(egui::Margin::symmetric(16, 10)).corner_radius(8).show(ui, |ui| {
          ui.label(egui::RichText::new(&self.toast).color(fg).strong());
        });
      });
      ctx.request_repaint();
    }
  }
}

// ============================ 한국어 폰트 ============================
fn install_korean_font(ctx: &egui::Context) {
  let candidates = [
    "C:\\Windows\\Fonts\\malgun.ttf",
    "C:\\Windows\\Fonts\\malgunsl.ttf",
    "C:\\Windows\\Fonts\\NanumGothic.ttf",
    "C:\\Windows\\Fonts\\batang.ttc",
  ];
  for p in candidates {
    if let Ok(bytes) = std::fs::read(p) {
      let mut fonts = egui::FontDefinitions::default();
      fonts.font_data.insert("kr".to_owned(), std::sync::Arc::new(egui::FontData::from_owned(bytes)));
      fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "kr".to_owned());
      fonts.families.entry(egui::FontFamily::Monospace).or_default().push("kr".to_owned());
      ctx.set_fonts(fonts);
      return;
    }
  }
}

fn main() -> eframe::Result<()> {
  let native_options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_inner_size([1480.0, 900.0])
      .with_min_inner_size([900.0, 560.0])
      .with_title("tfm2_ai_adjust 설정 편집기"),
    ..Default::default()
  };
  eframe::run_native(
    "tfm2_ai_adjust 설정 편집기",
    native_options,
    Box::new(|cc| {
      install_korean_font(&cc.egui_ctx);
      cc.egui_ctx.set_visuals(egui::Visuals::dark());
      Ok(Box::new(App::new()))
    }),
  )
}
