#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! tfm2_ai_adjust 설정 편집기 — config_editor.hta 의 무의존 네이티브 포트.
//! mshta / ActiveX 의존을 제거해 어떤 Windows 에서도 단일 exe 로 실행.
//! exe 가 위치한 폴더(=mod 폴더) 기준으로 tfm2_ai_adjust.cfg / config\*.cfg 를 읽고 씀.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use eframe::egui;

const ACTIVE_NAME: &str = "현재 (tfm2_ai_adjust.cfg)";
#[path = "class_capable.rs"] mod class_capable;
use class_capable::CLASS_CAPABLE;
/// 이 노브에 클래스별 값을 줄 수 있는가. 바이트패치 전용 노브는 원리상 불가능하다
/// (exe 기계어 상수를 고치는 방식이라 선수별로 다를 수 없다) — 칸을 아예 내주지 않는다.
fn class_capable(k: &str) -> bool { CLASS_CAPABLE.contains(&k) }
// 클래스별 오버라이드: "키_class_<en>" = 해당 클래스 전용값(미지정=전역 폴백). ChampionCategory 순서(melee0..assassin4).
const CLASS_EN: [&str; 5] = ["melee", "range", "magician", "util", "assassin"];
const CLASS_KR: [&str; 5] = ["전사", "원거리", "마법사", "전투보조", "암살자"];

// ============================ 탭 스키마 ============================
// ★[08-05] 표 3열 고정폭 — 탭마다 입력칸·설명칸 위치가 달라지던 문제의 해결책.
//   컨트롤 종류(입력/콤보/체크박스/비활성)가 달라도 열 x좌표는 항상 같아야 한다.
const NOTE_MAX_LINES: f32 = 10.0;   // 탭 상단 안내문 최대 표시 줄 수(넘으면 스크롤)
const COL1_W: f32 = 200.0;   // 키 이름 + 원본값
const COL2_W: f32 = 280.0;   // 값 컨트롤
const COL3_W: f32 = 470.0;   // 설명

struct Tab { id: &'static str, title: &'static str, note: &'static str, keys: &'static [&'static str] }

// ★탭 순서 = subplan별 탭(위) + 공통 탭(아래). 각 subplan 탭 내부는 장르별 §단락, 폐기/미확인 키는 맨 아래 격리.
// 분류 정본 = ANA\subplan별-튜닝레버-분류.md
static TABS: &[Tab] = &[
 // ─────────────── subplan 탭 (disc 순) ───────────────
 Tab{ id:"lane", title:"• [판단 0·1·3] 라인전 (passive_line ", keys:&[
 "§◆ 이동·후퇴 게이트","dd_frontier_mult","dd_ratio_thr","dd_ivar2_thr",
 "§◆ 거리 임계","dd_near_dist","dd_main_near_dist","dd_gatee_dist",
 "§◆ 커버·합류","dd_cover_count","dd_survivor_thr","dd_facet_thr","dd_cover_role_min",
 "§◆ 적을 몇 명이라고 보는가","dd_f22e80_margin",
 "§◆ 라인 배정 · 봇 듀오","d4_hp_safe","d4_partner_dist","d4_ally_radius_a","d4_ally_radius_b","d4_early_leave","d4_from_mid","d4_from_mid_mode","d4_ally_cnt","d4_minion_cnt","d4_gather_radius",
 "§◆ 라인 총력전","lt_revive_join","lt_ally_join","lt_around_radius","lt_phase_mask",
 "§◆ 라인 대기","lw_wait_dist","lw_back","lw_radius",
 "§◆ 라인 안전","ls_radius",
 ], note:
 "이 판단(게임 원본 `passive_line`)은 <b>경기 중 가장 자주 돌아가는 경로</b>로, 라인전 체감의 본체입니다. 흐름 = ①플래그 즉시분기 → ②<b>봇 라인 커버 블록</b>(내가 봇 듀오이고·파트너가 아군 진영 안·라인 회랑 안 적이 dd_cover_count명 이상 → 내 HP%가 dd_ratio_thr 미만이면 후퇴/합류) → ③메인(근처 적 탐색·기억창) → ④라인 상태·거리 게이트 3종 → ⑤종단(귀환/대기).<br>\
 ⚠<b>[정정] dd_ratio_thr 의 게임 원본값은 51입니다</b>(구 문서의 '원본 31'은 유저 튜닝값을 원본으로 잘못 적은 것).<br>\
 ⚠<b>시야 기억창은 여기서 dd_lane_margin이 담당</b> — [공통]시야 탭의 <b>vw_lane은 dd7_repl=1인 동안 무효</b>입니다(대체된 함수 내부라 게임 원본이 실행되지 않음). 라인전 기억창은 <b>dd_lane_margin</b>으로 조절하세요.<br>\
 <b>subplan 3 (라인전)</b> — 챔프가 라인에서 어디로 갈지(더 밀기 / 물러나기 / 수비 합류).<br>\
 물러나기: 적 전선진척 − 아군 전선진척 × dd_frontier_mult 가 작으면 후퇴<br>\
 수비 합류: 근처 적이 dd_cover_count명 이상 &amp; 라인 비율 &lt; dd_ratio_thr 면 합류 · 근접판정: 적 거리² &lt; dd_main_near_dist.<br>\
 (라인전 <b>전력회피</b>는 [공통] 전력탭 numbers_*_move · 시야는 [공통] 시야탭 dd_lane_margin)" },


 Tab{ id:"recall", title:"• [판단 8] 귀환 (ActiveRecall)", keys:&[
 "§◆ 체력 기반 복귀","rc_u21_init","rc_ehp_t1","rc_ehp_t2","rc_ehp_t3","rc_ehp_v1","rc_ehp_v2","rc_norp_bonus","rc_ed_near","rc_ed_mid","rc_ed_far","rc_ed_near_pen","rc_ed_far_bonus","rc_ed_vfar_bonus","rc_ahp_t1","rc_ahp_t2","rc_u13_bonus","rc_ahp2_pen","rc_ad_near","rc_ad_mid","rc_ad_near_bonus","rc_ad_far_pen","rc_mult_bonus","rc_ally_hp_min",
 "§◆ 복귀 성향 다이얼","t_recall",
 "§◆ 복귀배율 RNG/정규화","rc_rng_center","rc_rng_spread_div","rc_rng_a_base","rc_score_div",
 "§◆ 판단 7 귀환 이동판단 [0.5.3 재가동]","d7_repl","d7_hp_normal","d7_hp_selfheal","d7_wp_dist2",], note:
 "<b>subplan 5 (귀환 점수)</b> — 집/안전지대로 돌아갈지 점수로 결정. (전체배율 [공통]주요성향 t_recall)<br>\
 시작값 rc_u21_init → ±[적 HP%] ±[리콜포인트~적 거리] ±[아군 HP%] ±[아군~적 거리] +[수적우세]<br>\
 점수 = 난수 × mult / 100 + t_recall. 점수 클수록 복귀.<br>\
 <b>합류 이득 (rc_join_*)</b>: rc_join_weight>0이면 체력 멀쩡해도 합류 이득 크면 복귀(합류 이동기). <b>rc_join_weight=0=끔(기본).</b><br>\
 <b>disc7 이동판단 (d7_*)</b>: 위 rc_*(점수)와 별개 층 — 귀환 subplan 실행 중 '버틸지 vs 뺄지'(movepri Recall 리졸버). <b>d7_repl=1</b>이면 대체 활성. d7_hp_normal(41)/d7_hp_selfheal(21)=HP% 후퇴 임계(자힐 유무별), 낮출수록 저체력에도 안 뺌." },


 Tab{ id:"object", title:"• [판단 9·11] 교전·갱커버 (Battle·LineGankCover)", keys:&[
 "§◆ 위치술어","pf_edge_margin","pf_center_band","pf_diag_far","pf_diag_near","pf_band_width",
 "§◆ 견제 도달 게이트","poke_reach_bonus","poke_serpen_slot",
 ], note:
 "⚠<b>번호가 겹치는 이유</b>: 게임 안에 번호 체계가 <b>두 개</b> 있습니다 — <b>판단</b>(무엇을 할지)과 <b>실행</b>(어떻게 움직일지)이 각각 따로 번호를 매기고, 번호가 같아도 서로 다른 것입니다. 그래서 <b>판단 11(갱커버)</b>과 <b>실행 11(숨기)</b>이 둘 다 있습니다.<br><b>subplan 9/11 (Battle/Hide)</b> — 주요 오브젝트 견제·교전.<br>\
 pf_ 값들 = 맵을 구역(띠)으로 나눠 챔프 위치를 판정(견제 시 어디에 설지). poke_* = 재배선된 진입/도달 게이트.<br>\
 구 pk_* 키는 050에서 하드코딩(값 무반영) — 맨 아래 격리." },

 // 전술 게이트 주의: 이 탭 전체(및 [14]·[15·16·17])는 **미니언웨이브 전술이 '웨이브 우선'일 때만** 오브젝티브 시스템이 살아 있다.
 Tab{ id:"battle", title:"• [판단 12] 에픽 사냥·견제 (EpicHuntAndPoke)", keys:&[
 "§◆ 교전판단 HP 게이트","ec_oz_hp","ec_iz_hp","ec_self_hp_low","ec_valid_hp","ec_commit_hp","ec_count_hp",
 "§◆ 거리·카운트·시야","ec_engage_dist2","ec_count_radius","ec_vision_ticks",
 
 "§◆ 사냥할 때 붙는 거리","eh_trace_arrive","eh_band_low","eh_band_high","eh_around_radius","eh_recall_radius",
 "§◆ 사냥을 걸지 말지","eh_abort_hp","eh_abort_dist","eh_commit_hp","eh_commit_r_low","eh_commit_r_high","eh_flee_clear_hp",
 "§◆ 전술이 행동을 가르는 단 두 곳","eh_reach_margin","eh_score_norm",
 "§◆ 킬타깃·세부 (신규 09-01, 고급)","eh_fin_mode","eh_band_off","eh_commit_margin","eh_dist_clamp","eh_clamp2","eh_engage_dist","eh_dist_shift","eh_power_weight","eh_power_neutral","eh_power_sub","eh_time_slope","eh_window_cap","eh_score_floor","eh_score_gate","eh_helper_a","eh_helper_b","eh_hp_gate2","eh_grid_cost",], note:
 "<b>팀전술이 판단 발화를 가릅니다</b> — 리그 경기 1판(44,309틱·양팀 전수 로그)에서 <b>같은 경기인데 팀마다 뜨는 판단이 달랐습니다</b>: 세르펜 사냥은 <b>오브젝트 마무리='처치 우선'</b>인 팀에서만 433회(전투 우선 팀 0회), 세르펜 견제는 반대로 '전투 우선' 팀에서만 471회. ⟹ <b>값이 안 먹으면 그 판단이 우리 팀 전술에서 안 뜨는 것일 수 있습니다.</b><br>\
 &nbsp;&nbsp;⚠<b>정정</b>: 구 안내 '미니언 웨이브 설정에 따라 모르가드·세르펜 계열이 통째로 침묵'은 <b>반박됐습니다</b>(웨이브 우선 팀·합류 우선 팀 <b>양쪽 다</b> 모르가드 판단이 활발히 발화). 미니언 웨이브만으로 이 탭이 죽지는 않습니다.<br>\
 추가로 <b>ec_valid_hp·ec_commit_hp·ec_count_hp·ec_count_radius·ec_vision_ticks</b> 5키는 <b>오브젝트 빌드업 = '스플릿'(라인별)</b>이고 그 라인이 내 라인일 때만 작동(모이기/유연에선 무반영).<br>\
 <b>subplan 12 (EpicCheck)</b> — 모르가드(에픽) 교전 판단. 실제 게임에 반영됩니다.<br>\
 안전지대 안/밖 + 체력·목표 상태 → 교전(0xe)/대기·철수(7,0xc)/재배치(2)/HOLD(0xd) 결정.<br>\
 <b>ec_tgt_hp_low → ec_self_hp_low 로 대체</b>: 기존 재현이 <b>타겟</b> 체력을 보고 있었으나 원본은 <b>자기</b> 체력 기준(0.5.2 disasm 확정). 구 키는 무반영.<br>\
 ⛔<b>[13] 모르가드 사냥(EpicHunt)은 실제로 발동하지 않습니다.</b> 코드는 있지만 경기 중 한 번도 뜨지 않으므로 <b>d13_engage_hp_pct 는 값을 넣어도 반영되지 않습니다</b>." },

 Tab{ id:"def", title:"• [판단 14] 세르펜 사냥·견제 (SerpenHuntAndPoke)", keys:&[
 "§◆ 홈 회복존 게이트","sn_home_lo","sn_home_hi","sn_home_x1","sn_home_y1",
 "§◆ HP 게이트","sn_self_hp","sn_hp_crit","disc16_home_hp",], note:
 "<b></b> 이 판단(모르가드 견제)은 리그 경기 1판에서 <b>양 팀 모두 활발히 발화</b>(총 2,992회)했습니다 — 구 안내 '미니언 웨이브에 따라 통째로 침묵'은 <b>반박됨</b>. 단 발화 <b>횟수</b>는 팀별로 달랐으니(785 vs 2,207) 전술이 빈도에는 영향을 줍니다.<br>\
 <b>subplan 14 (EpicPoke)</b> — 에픽 오브젝트(용/바론류) 국면의 견제·포킹 실행(안전공격지점 잡거나 교전).<br>\
 출력=추격/유휴/교전/리포지션/대형이동. self가 <b>아군 베이스/분수 회복존</b>(부상 유닛이 힐 받는 존, geom+0x6d70·per-side) 안 & 부상(HP&lt;max)이면 <b>에픽 견제 나가지 말고 홈 대기(7)</b>. ※epic·넥서스 위치 아니라 아군 베이스 영역 기준(RE 확정). HP 게이트: sn_hp_crit · ep_nexus_hp(=self HP%). (진짜 넥서스는 [18·19]탭)<br>\
 구 ep_lane_margin/pred/near/hp_low/count_gate 는 판단 16/17/4로 로직이 이동해 여기선 값 무반영입니다(비활성 표시용으로만 남겨둔 이름).<br>\n ⚠<b>이 다섯 개와 아래 에픽 견제 값들(ep_home_* · sn_hp_crit · sn_self_hp)은 접두사만 같고 서로 무관합니다</b> — 후자는 08-05에 sn_ 에서 옮겨온 <b>실제로 동작하는</b> 값들입니다." },

 Tab{ id:"disc17", title:"• [판단 17] 넥서스 방어 (DefenseNexus)", keys:&[
 "§◆ 견제 진척 게이트","nxd_prog_low","nxd_prog_crit","nxd_p3_gate","nxd_ref_hp",
 "§◆ 거리","nxd_near_dist","nxd_pred_dist",
 ], note:
 "<b></b> 리그 경기 1판 전수 로그에서 <b>[16]세르펜 사냥은 '오브젝트 마무리 = 처치 우선'인 팀에서만 433회</b>(전투 우선 팀 0회), <b>[17]세르펜 견제는 '전투 우선' 팀에서만 471회</b>(처치 우선 팀 0회)로 완전히 갈렸습니다. ⟹ 판단 16_home_hp가 안 먹으면 <b>오브젝트 마무리를 '처치 우선'</b>으로, disc17_* 계열은 '전투 우선'에서 확인하세요(⬜1경기 관측이라 인과 확정은 아님).<br>\
 <b>17 (SerpenPoke·세르펜 견제)</b> — 진척/근접 게이트 공방(재배선 완료). <b>16 (SerpenHunt·사냥)</b>=판단 16_home_hp(부상 홈대기 HP%, 기본100=원본). 16·17 대체는 [엔진] 탭의 <b>nx_repl</b>로 켜고 끕니다(기본 켜짐). ⛔<b>15 (SerpenCheck)는 죽은 판단</b>입니다 — 게임이 이 판단을 아예 만들지 않으므로 <b>d15_repl을 켜도 실행되지 않습니다</b>.<br>\
 진척(progress) 게이트와 근접/예측 거리²로 DEFEND/진격 결정.<br>\
 세르펜 전용 전력회피(<b>numbers_threat_sp15/16/17</b>)는 정상 동작합니다.<br>\
 &nbsp;&nbsp;이 키들은 <b>[공통] 전력·포탑 회피</b> 탭의 'subplan별 개별 임계'로 옮겼습니다(전 subplan을 한 화면에서 비교)." },

 Tab{ id:"disc19", title:"• [실행 18·19] 넥서스 공수", keys:&[
 "nx_enable",
 "§◆ 방어","d19i_enable","d19_retreat_hp","nx_dn_nexus_hp","nx_dn_hp_crit","nx_dn_hp_low","nx_dn_near_dist","nx_dn_pred_dist","nx_dn_vision_mem",
 "§◆ 방어 측 교전 컷 · 배회 반경","nx_cull_dist19","nx_around_def",
 "§◆ 공격 측 배회 반경","nx_around_atk",
 "§◆ 공격","nx_an_finish_hp","nx_an_cull_dist","§◆ 넥서스로 밀어붙일지 (0.5.4 신설)","an_tower_gate","an_attack_sub","an_home_wait","an_fallback","an_fallback_wave","an_fallback_style","§◆ 넥서스가 위험할 때 — 위험도 사다리","d19_ally_hp","nx_dn_count_gate","nx_an_count_gate","d19_sev_hp_1","d19_sev_hp_2","d19_sev_hp_3","d19_sev_ratio_0","d19_sev_ratio_1","d19_sev_ratio_2","d19_sev_ratio_3",
 "§★ 비상 수비 — 어느 상황에서 얼마나 적극적으로 (0.5.4 게임 신규 판단)","nxe_twin0","nxe_twin1","nxe_t2_1","nxe_t2_2","nxe_t2_3","nxe_t1_1","nxe_t1_2","nxe_t1_3",
 "§☆ 비상 수비 — 부작용 분리(평소엣 건드리지 마세요)","nxe_supp_off","nxe_battle_off",], note:
 "<b>[18·19] 넥서스 공수</b> — 넥서스 방어(disc19)/공격(실행 18) 판단 튜닝. 대부분 <b>byte-patch</b>(게임 원본상수 직접 수정).<br>\
 이 탭의 값들은 현재 버전에서 정상 적용됩니다. 적용 여부는 <b>obj_imm.txt</b>에서 확인할 수 있습니다.<br>\
 <b>방어</b>: d19_retreat_hp(후퇴 HP%문턱, ↑=수비적) — <b>d19i_enable=1 켜야</b> byte-patch 반영(0=원본45 복원). + oi_dn_*(<b>nx_enable=1</b> 필요).<br>\
 <b>공격</b>: nx_an_finish_hp(적넥서스 HP% 마무리 게이트·↓=공격적)·nx_an_cull_dist(공격후보 거리).<br>\
 <b>nx_enable=1</b>이라야 oi_* 반영(=0 기본=원본 복원). ⚠은퇴칸(d19_threat_mult·d19_range_*)은 재현 대조용(dcap)이라 <b>게임 무영향</b>.<br>\
 ⚠<b>0.5.2 동작 변화</b>: 넥서스 방어 판단의 <b>phase 게이트가 전부 삭제</b>돼(구 phase≥30/≥39), 이제 <b>경기 시간대와 무관하게 상시</b> 위협·아군넥서스 판정을 합니다." },

 Tab{ id:"gb", title:"• [매크로] 운영전환·로밍 (GenericBuild)", keys:&[
 "§◆ 마스터 스위치","gb_enable",
 "§◆ 로밍 거리·범위","gb_join_dist","gb_scout_radius","gb_close_radius","gb_line_range",
 "§◆ 진입 타이밍 게이트","gb_op_phase","gb_push_hp",
 "§◆ 전역 사거리","gb_reach_cap","gb_reach_margin",
 "§◆ 라인개입 갱 셋업","gk_wait","gk_hp_base_gank","gk_window_margin",
 "§◆ 개시 게이트 (신규 09-01: 갱크·교전·결사전)","gk2_gank_radius","gk2_gank_hp","eng_camp_radius","db_retreat_margin",
 ], note:
 "<b>운영전환·로밍 (GenericBuild)</b> — 특정 subplan이 아니라 '어느 상태로 갈지'(합류/거점로밍/라인압박/운영진입)를 분기하는 매크로 판단.<br>\
 적용 여부는 <b>gb_imm.txt</b>에서 확인할 수 있습니다.<br><b>gb_enable=1</b> 켜야 아래 로밍 byte-patch가 걸림(0=게임 원본 그대로). 각 값 <b>-1=그 항목만 원본유지</b>.<br>\
 &nbsp;&nbsp;<b>gb_join_dist</b>(60000)=이 거리 이내면 '근접/합류 모드' 진입(지배 게이트). <b>gb_scout_radius</b>(120000)=거점/타겟 후보를 이 반경 안에서 수집=로밍 범위. <b>gb_close_radius</b>(≈387)·<b>gb_line_range</b>(≈500)=근접·라인 판정 반경.<br>\
 &nbsp;&nbsp;<b>gb_op_phase</b>(31)=경기진행 phase가 이 값 이상이면 운영 시작(낮추면 이른 운영). ~~gb_join_phase(12)~~=⛔0.5.2 死(게이트 삭제). <b>gb_push_hp</b>(30)=라인 대상 체력%가 이 값 미만이면 압박 오더.<br>\
 &nbsp;&nbsp;⚠<b>gb_reach_*</b>=GenericBuild 전용이 아닌 <b>전 AI 공유 사거리</b> 헬퍼 → 켜면 모든 판단의 사거리가 바뀜(신중). scout_radius는 헬퍼 콜엣지 미확정이라 gb_imm.txt applied 카운트로 적용확인 권장.<br>\
 gb_cnt_*/gb_da_*/gb_db_*/gb_score_* = 소스 배선됐으나 라이브 미발화(참고용). 구 거리밴드(gb_rbx/r15/r14)는 값 무반영." },

 // ─────────────── 공통 탭 ───────────────
 Tab{ id:"exec", title:"• [실행] 오판·대기·오더유지 (실행층)", keys:&[
 "§◆ 판단력 오판 게이트","ex_judge_floor","ex_judge_slope","ex_judge_cap",
 
 "§◆ 행동을 얼마나 자주·오래 붙잡나","ex_order_hold","ex_think_min","ex_think_max","ex_fail_min_ticks",
 "§◆ 스킬 해금 레벨","ex_skill2_level","ex_ult_level",
 "§◆ 기본공격 접근·대상 선택","ex_attack_margin","ex_attack_margin_sp","ex_attack_seek",
 "§◆ 이동 도착·추격 판정","mv_bush_arrive","mv_hide_near","mv_trace_dist",], note:
 "<b>게임이 '실제 움직임'을 만드는 층</b> — 판단이 정해진 뒤, 그 판단이 내놓은 여러 후보 행동 중 하나를 고르고 실행하는 단계입니다. 모드가 대체하지 않는 층이라 값이 그대로 먹습니다.<br>\
 <b>판단력 오판 게이트 = 게임의 '판단력' 스탯이 실제로 작동하는 방식</b>(리버스로 규명):<br>\
 &nbsp;&nbsp;<code>문턱 = min(판단력, cap) × slope ÷ 10 + floor</code> → <b>주사위(0~999)가 문턱보다 크면 최선 후보 대신 무작위 후보를 고릅니다.</b><br>\
 &nbsp;&nbsp;원본 기준 <b>판단력 100 = 오판 0%</b> / <b>판단력 0 = 오판 85%</b>. ⟹ <b>ex_judge_floor를 올리면 전 챔프의 오판이 줄어듭니다</b>(예 400 → 최대 오판 60%). slope를 올리면 판단력 스탯의 영향력이 커집니다.<br>\
 <b>대기 위치</b>: 라인을 밀고 나간 아군이 기준점에서 <b>ex_wait_dist</b> 이상 멀면, 그 아군 경로의 <b>끝에서 ex_wait_back 만큼 뒤</b>에서 대기합니다. back↑ = 더 소극적으로 물러나 대기.<br>\
 <b>오더 유지</b>: 고른 행동을 최소 몇 틱 유지할지. ↑ = 재선정 억제 = <b>우왕좌왕 완화 후보</b>.<br>\
 전부 <b>-1(원본)</b>이 기본. 적용확인 = <b>exec_imm.txt</b>(applied=6/6)." },

 Tab{ id:"movein", title:"• [실행] 실제 이동 만들기 (모든 이동이 통과)", keys:&[
 "§◆ 목적지에 다 왔다고 보는 거리","mv2_arrive_snap",
 "§◆ 남을 피해 돌아가는 정도","mv2_avoid_coef","mv2_avoid_margin","mv2_avoid_bias",
 "§◆ 우물에서 강제로 밀어내기","mv2_well_radius","mv2_well_dist",
 "§◆ 자리 잡기 이동 모드","mv2_pos_mode_thr","§◆ 어디로 걸어갈지 — 경로·거리 (0.5.4 신설)","path_orth_cost","path_diag_cost","path_greedy","path_threat_floor","path_threat_cap","path_threat_scale","path_threat_default","path_danger_cost","path_wave_risk_ret",], note:
 "고른 행동이 <b>이동</b>이면, 목적지 좌표만 정해진 채 <b>이 한 곳을 통과해 실제 이동 입력이 만들어집니다</b>. 그래서 여기 값들은 도망·추적·접근·자리잡기에 <b>전부 동시에</b> 영향을 줍니다.<br> <b>도착 판정</b>: 목적지까지 남은 거리가 <b>mv2_arrive_snap</b> 안이면 회피 계산을 생략하고 곧장 목적지로 갑니다. 올리면 막판에 파고들고, 내리면 끝까지 남을 피해 돌아갑니다.<br> <b>회피</b>: 주변 유닛과 겹치지 않게 경로를 틀 때 쓰는 값입니다. 올리면 넓게 우회해 뭉침이 줄지만 이동이 길어집니다.<br> <b>우물 탈출</b>은 판단보다 <b>먼저</b> 걸립니다 — 기지 안에 있으면 고른 행동을 무시하고 밖으로 밀어냅니다. 두 곳에 같은 로직이 있어 모드가 한꺼번에 맞춥니다.<br> 전부 <b>-1(원본)</b>이 기본. 적용확인 = <b>move2_imm.txt</b>." },

 Tab{ id:"hide", title:"• [실행 11] 숨기 (hide)", keys:&[
 "§◆ 숨을 자리 고르기","hd_cand_select","hd_skip_landmark","hd_bush_near","hd_path_radius","hd_around_radius","hd_ph0_ttl",
 "§◆ 숨을지 판단할 때 보는 범위","hd_detect_max","hd_fight_cut","hd_vision_mem","hd_trace_leash",
], note:
 "⚠<b>번호가 겹치는 이유</b>: 게임 안에 번호 체계가 <b>두 개</b> 있습니다 — <b>판단</b>(무엇을 할지)과 <b>실행</b>(어떻게 움직일지)이 각각 따로 번호를 매기고, 번호가 같아도 서로 다른 것입니다. 그래서 <b>판단 11(갱커버)</b>과 <b>실행 11(숨기)</b>이 둘 다 있습니다.<br><b>숨기</b>는 그동안 설정값이 <b>하나도 없던 판단</b>입니다. 수풀로 갈지, 어디로 물러날지를 전부 코드에 박힌 거리로 정하고 있었습니다.<br>\
 <b>hd_cand_select</b>가 이 판단에서 압도적으로 많이 쓰이는 값입니다(30곳) — 숨을 자리·물러날 자리 후보를 고르는 기준이라, 여기만 바꿔도 은신 동선이 크게 달라집니다.<br>\
 거리는 전부 <b>그냥 거리</b>로 넣으세요 — 제곱 변환은 모드가 합니다.<br>\
 ※라인 배정은 <b>[0·1·3] 라인전</b> 탭, 넥서스 공수는 <b>[18·19]</b> 탭으로 옮겼습니다.<br>\
 적용확인 = <b>hd_imm.txt</b>." },

 Tab{ id:"cast", title:"• [실행 9] 평타·스킬 사거리·조건 (교전 내부 후보)", keys:&[
 "§◆ 언제부터 '닿는다'고 보고 공격을 시작할지","cs_lead_attack","cs_lead_skill","cs_lead_skill2","cs_lead_ult","cs_lead_steal",
 "§◆ 궁 사용 조건","cs_ult_range","cs_ult_range_global","cs_ult_mode_mask",
 "§◆ 무엇을 노릴지","cs_steal_hp","cs_unit_hits","cs_minion_vision",
 "§◆ 아군 지원 스킬","cs_ally_hp","cs_ally_radius",
 "§◆ 추격 판정","cs_cc_mask",
 "§◆ 데스매치","dm_lookahead","dm_ult_lookahead","dm_near_ally","dm_near_enemy",
 "§◆ 데스매치","dm_execute_hp","dm_lasthit","dm_skill_hp",
 "§◆ 데스매치","dm_ult_rally","dm_ult_rally2","dm_ult_range","dm_ult_mask_rally","dm_ult_mask_focus","dm_ult_mask_safe",
 "§◆ 데스매치","dm_skill2_level","dm_ult_level",
 "§◆ 아군 지원스킬 낭비 방지","c3_ally_hp","c3_enemy_near_a","c3_enemy_near_b","c3_minion_near","c3_hurt_scale","c3_minion_margin",
 "§◆ 해금 레벨","ex_ult_level_x","ex_skill2_level_x",
 "§◆ 특수 효과 스킬의 고정 점수","bv_ally_flat","bv_ally_cap","bv_out_of_fight","bv_b_in","bv_b_out","bv_d_in","bv_d_out","bv_c_cap","bv_c_none",
 "§◆ 전투 실익의 상한과 집중포화","bv_cap_main","bv_cap_half","bv_focus_max","bv_focus_radius",
 "§◆ 때려도 되는지 판정","sf_margin","sf_radius","sf_mem",], note:
 "<b>교전 중 '평타·스킬·스킬2·궁을 누구에게 쓸지' 후보를 고르는 단계</b>입니다. 여기서 후보로 올라간 것만 나중에 점수 경쟁에 참가합니다.<br>\
 <b>언제부터 닿는다고 보나</b>: 사거리 판정이 <code>실제거리 ≤ 사거리 + (선행예측틱 × 접근속도)</code> 라서, 이 틱을 올리면 <b>아직 사거리 밖이어도 '곧 닿는다'고 보고 먼저 달려듭니다</b>. 내리면 사거리 안에 확실히 들어와야 움직입니다. 궁만 원본이 60이고 나머지는 30입니다.<br>\
 <b>궁 사용 조건</b>: 궁 후보는 <b>팀이 지정한 지점 근처</b>에 있어야만 올라갑니다(원본 6,000 — 사실상 '바로 그 자리'). <b>cs_ult_range를 올리면 궁을 훨씬 자유롭게</b> 씁니다. 맵 전역을 노리는 궁은 별도로 90,000이 적용됩니다.<br>\
 &nbsp;&nbsp;<b>cs_ult_mode_mask</b>는 이 근접 요구를 <b>어떤 팀 작전에서 적용할지</b>입니다(원본 0x6f — 넥서스 공격 작전만 면제). <b>0으로 두면 모든 상황에서 근접 요구가 사라져 궁을 남발</b>합니다.<br>\
 <b>무엇을 노릴지</b>: 중립 몬스터는 체력이 <b>cs_steal_hp</b>% 이하일 때만 막타를 노립니다(원본 20). <b>cs_unit_hits</b>는 '몇 대 안에 죽일 수 있는 유닛까지 때릴지'(원본 2 = 3방컷) — 올리면 단단한 유닛도 공격합니다.<br>\
 <b>추격 판정</b>: <b>cs_cc_mask</b>에 없는 상태이상에 걸린 적은 '못 움직인다'고 보고 <b>상대 속도 대신 내 전속력</b>으로 거리를 계산합니다. 0으로 두면 모든 군중제어를 이동불가로 취급해 <b>더 공격적으로</b> 달려듭니다.<br>\
 전부 <b>-1(원본)</b>이 기본. 적용확인 = <b>cast_imm.txt</b>." },

 Tab{ id:"judge", title:"• [실행] 성향 흔들림·교전·추격 (경매층)", keys:&[
 "§◆ 주변 머릿수에 따른 배율","sc_adv_lo","sc_adv_m1","sc_adv_0","sc_adv_p1","sc_adv_hi",
 "§◆ 주변 머릿수에 따른 배율","mv0_adv_lo","mv0_adv_m1","mv0_adv_0","mv0_adv_p1","mv0_adv_hi",
 "§◆ 몇 명을 '이 싸움'으로 셀지","sc_ally_radius","sc_enemy_radius",
 "§◆ 점수 보정","sc_near_bonus","sc_obj_bonus","sc_keep_thr",
 "§◆ 행동의 실익을 어떻게 계산하나","sc_turret_radius","sc_engage_radius","sc_cell_dist","sc_dive_margin","sc_score_vision",
 "§◆ 위험하다고 보는 기준","sc_risk_dmg","sc_risk_hp1","sc_risk_dmg1","sc_risk_hp2","sc_risk_dmg2","sc_risk_hp3","sc_risk_dmg3",
 "§◆ 보너스 상한","sc_focus_cap","sc_kill_cap","sc_kill_pct","sc_null_score",
 
 "§◆ 행동 성향 흔들림","au_noise_off","au_noise_amp","au_score_center",
 "§◆ 교전 판단","bt_hp_flee","bt_hp_gate","bt_chase_stop","bt_chase_keep","bt_vision_mem",
 "§◆ 라인 수비","ld_chase_stop","ld_ally_near","ld_intervene","ld_vision_mem","ld_est_base",
 "§◆ 라인 수비","ld_around_range","ld_around_delay","ld_mode_mask","ld_move_pct","ld_threat_state","ld_rand_min",
 "§◆ 도망 점수의 가중치와 보너스","mv0_risk_shift","mv0_engage_shift","mv0_base_penalty","mv0_near_bonus","mv0_near_gate",
 "§◆ 포탑이 점수에 끼치는 영향","mv_tower_margin","mv_tower_cap","mv2_gain_shift","mv_engage_thr","vis_mem_global",
 "§◆ 라인 수비 후보 점수","ldsc_lost_target","ldsc_skill_factor","ldsc_vision_mem","ldsc_early_mask",
 "§◆ 팀 모드 자동취소","tm_cancel_mask",
 "§◆ 적 위치 추정","eg_spread_base","eg_disk_radius","eg_radius_cap",
 "§◆ 시전 후보 2차 검열","cf_risk_near","cf_risk_far","cf_dmg_pct","cf_reach_pad","cf_reach_pad_ult","cf_filter_off","cf_flee_kill_off",
 "§◆ 후보 점수 하한","ld_score_floor",
 "§◆ 고르고 나서 다시 고르기","re_cast_promote","re_trace_pad","re_gate_subplan",
 "§◆ 전역 궁 요청","gu_level","gu_enemy_mem","gu_suppress_r","§◆ 경매 중 강제 귀환 (0.5.4 신설)","auc_flee_version_gate","auc_flee_score","auc_flee_hp_field","auc_flee_nexus_mask","auc_flee_goal_far","auc_flee_goal_near_a","auc_flee_goal_near_b","auc_flee_end_delay","auc_flee_with_skill","auc_flee_action_tag","auc_flee_pathfinder","auc_flee_undying_gate",], note:
 "<b>행동 성향 흔들림 = 판단력 스탯의 두 번째 작동 방식</b>(리버스로 규명. 위 [실행] 탭의 '오판 게이트'와는 <b>다른 장치</b>입니다).<br>\
 게임은 행동 후보들을 <b>11개 부류</b>(이동·교전·귀환·스킬 4종 등)로 나눠 각 부류에 <b>가중치</b>를 곱해 점수를 매깁니다. 그 가중치는 고정값이 아니라 <b>판단이 바뀔 때마다 부류별로 주사위를 굴려 새로 정해집니다</b>.<br>\
 &nbsp;&nbsp;<code>흔들림폭 = (au_noise_amp − 9 × 판단력) ÷ 2</code> → 각 부류 가중치 = <code>[중심−흔들림폭, 중심+흔들림폭]</code> 중 무작위.<br>\
 &nbsp;&nbsp;원본 기준 <b>판단력 100 = 흔들림 0</b>(모든 부류 정확히 100%) / <b>판단력 0 = ±45%</b>(어떤 부류는 55%, 어떤 부류는 145%로 제멋대로) — 이게 <b>'판단력 낮은 선수가 성향이 들쭉날쭉한' 이유</b>입니다.<br>\
 &nbsp;&nbsp;<b>au_noise_off=1</b> → 주사위 자체를 없앰 = 판단력과 무관하게 <b>모든 선수가 일관된 성향</b>. 우왕좌왕·부쉬 왕복 완화 1순위 후보.<br>\
 &nbsp;&nbsp;⚠<b>au_noise_amp는 900 미만으로 내리지 마세요</b>(계산이 음수로 넘어가 점수 체계가 깨집니다 — 모드가 900으로 자동 보정합니다). 흔들림을 줄이려면 amp가 아니라 <b>au_noise_off</b>를 쓰세요.<br>\
 &nbsp;&nbsp;<b>au_score_center</b>를 올리면 <b>모든 행동 점수가 일괄 증폭</b>됩니다(부류 간 상대비는 유지).<br>\
 <b>교전 판단(bt_*)</b>: 실제 싸움을 담당하는 층입니다. <b>bt_hp_flee</b>(원본 21%)는 '이 체력 밑이면 후퇴/추격 판단', <b>bt_chase_stop</b>(원본 15000)은 '적에게 이만큼 다가가면 멈춤' — <b>올리면 소극적</b>, <b>bt_chase_keep</b>(원본 80000)은 '이 거리까지는 계속 쫓음' — <b>올리면 끈질기게</b> 쫓습니다.<br>\
 <b>라인 수비 2차(ld_*)</b>: <b>ld_ally_near</b>(원본 160000)는 '아군이 이 거리 안이면 붙어있다고 판단' — 올리면 아군을 더 자주 믿고 공격적이 됩니다. <b>ld_est_base</b>(원본 10)는 AI가 거리·피해를 어림잡을 때의 <b>오차 하한</b>으로, <b>올릴수록 AI의 추정이 정확</b>해집니다.<br>\
 <b>tm_cancel_mask</b>(원본 0xb00 = 갱·다이브·컴백픽): 팀 작전이 <b>자동 취소되는 대상</b> 목록입니다.<br>\
 <b>적 위치 추정(ep_*)</b>: 안 보이는 적이 <b>지금 어디쯤 있을지</b>를 AI가 추정하는 방식입니다. 마지막으로 본 시각·위치·이동속도로 <b>원판</b>을 그리고, 그 원판이 관심 지점에 닿으면 “거기 있을 수 있다”고 셉니다. <b>ep_spread_base</b>(3000)와 <b>ep_disk_radius</b>(40000)를 올리면 원판이 커져 <b>적을 더 넓게 의심</b>하고, <b>ep_radius_cap</b>(300000)을 넘으면 그 적은 아예 후보에서 빠집니다. 지금까지는 반경(dd_f22e80_margin) 하나만 열려 있었습니다.<br>\n <b>시전 후보 2차 검열(cf_*)</b>: <b>라인 수비 판단에서만</b> 도는 추가 관문으로, 평타·스킬·스킬2·궁 후보만 심사합니다(이동 계열은 통과). <b>cf_risk_near</b>(9)가 <b>가장 자주 걸리는 컷</b>이고, <b>cf_dmg_pct</b>(35)는 “곧 받을 피해가 내 체력의 이만큼이면 시전 포기”입니다.<br>\n &nbsp;&nbsp;★<b>cf_flee_kill_off=1</b>이 체감 변화가 가장 큽니다 — 원본은 <b>후보 목록에 후퇴 계열이 하나라도 있으면 시전 후보를 통째로 버립니다</b>. 1로 두면 그 몰살이 사라집니다.<br>\n &nbsp;&nbsp;<b>cf_filter_off=1</b>은 이 검열을 전면 무효화합니다(전부 통과).<br>\n <b>cs_score_floor</b>(30): 후보를 버리는 점수 하한의 절댓값입니다. 원본은 <b>−30점까지는 살려둡니다</b>. 내리면 나쁜 후보가 더 빨리 잘려 나갑니다.<br>\n <b>재경매(re_*)</b>: 최고점을 고른 뒤 <b>한 번 더 고르는</b> 단계입니다. <b>re_cast_promote</b>를 3으로 올리면 지금은 빠져 있는 <b>궁도 갈아타기 대상</b>이 됩니다. <b>re_trace_pad</b>(25000)는 쫓아가다 재경매로 전환하는 여유 거리입니다.<br>\n <b>전역 궁(gu_*)</b>: 아군이 채팅으로 궁을 요청했을 때 <b>경매를 통째로 건너뛰고</b> 궁을 쏘는 경로입니다. <b>gu_suppress_r</b>(150000) 안에 보이는 적이 하나라도 있으면 발동하지 않아 <b>교전 중에는 거의 안 나갑니다</b> — 이 값을 줄이면 더 자주 나갑니다.<br>\n 전부 <b>-1(원본)</b>이 기본. 적용확인 = <b>auction_imm.txt</b> / 신규 그룹은 <b>new_imm.txt</b>." },

 Tab{ id:"posrisk", title:"• [공통] 자리가 위험한지 계산 (position_eval)", keys:&[
 "§◆ 자리가 얼마나 위험한가","pe_collect_radius","pe_champ_threat","pe_minion_add","pe_filter_radius","pe_near_cut","pe_field_radius","pe_count_radius",
 "§◆ 자리 평가","pe_reach_bonus","pe_skillshot_width","pe_bodyblock_width","pe_outer_band","pe_tower_margin",
 "§◆ 자리 평가","pe_source_cap","pe_predict_cap","pe_tower_far","pe_kind_scale","pe_wall_risk","pe_well_risk","pe_ally_gain_cut","pe_state_gate","pe_mode_mask","pe_kind_mask",
 "§◆ 자리 판단의 흔들림","pe_noise_exempt","pe_noise_amp","pe_noise_amp_mode2",
 "§◆ 자리 위험 수치를 만드는 값","th_collect_radius","th_skill_margin","th_atk_margin","th_band_margin","th_cap",
 "§◆ 라인 수비 후보 점수","ae_bonus_kill","ae_bonus_near","ae_bonus_soon","ae_bonus_struct","ae_gain_shift","ae_risk_shift","ae_tower_shift","ae_threat_limit","ae_none_mask",
 ], note:
 "이동할 자리를 고를 때 <b>그 자리가 얼마나 위험한가</b>를 계산하는 곳입니다. 도망·추격·접근·자리잡기가 <b>전부 이 계산을 공유</b>합니다.<br>\
 단위는 <b>내 현재 체력의 몇 %</b>입니다 — 나눗셈과 상한을 <b>쓰는 쪽이 아니라 만드는 쪽</b>에서 걸기 때문에, 체력이 낮을수록 같은 적이 급격히 무서워집니다.<br>\
 <b>th_*</b> = 위험 수치를 <b>만드는</b> 값(사거리 띄·상한·반경) · <b>pe_*</b> = 그걸 <b>쓰는</b> 값 · <b>ae_*</b> = 라인 수비에서 이동 후보를 채점하는 값.<br>\
 ⚠실제 피해 숫자 자체는 경기 시작 때 만들어둔 표에서 오기 때문에, 여기 값을 바꿔도 <b>어느 범위까지 무서워하느냐만 바뀌고 피해량 자체는 그대로</b>입니다.<br>\
 적용확인 = <b>pe_imm.txt · th_imm.txt · ae_imm.txt</b>." },
 Tab{ id:"regrouped", title:"• [공통] 교전 진입 · 합류 · 포탑 · 능력치", keys:&[
 "§◆ 교전에 들어갈지 정하는 확률","engage_thr_mult","eng_role2","eng_role3","eng_role4","eng_role_def","t_engage","engage_base","§◆ 아군 전투에 합류할지","rc_join_weight","rc_join_adv","rc_join_rescue","rc_join_obj_mult","rc_join_dnear","rc_join_dmid","§◆ 포탑을 전력·위협으로 어떻게 셀지","ally_tower_dps","ally_tower_hp","ally_tower_range","tower_dps","tower_range","tower_threat","ally_tower_dps_move","ally_tower_hp_move","ally_tower_range_move","§◆ 머릿수를 보고 물러날지","numbers_range","numbers_threat","numbers_min_enemy_move","numbers_margin","numbers_min_enemy","numbers_range_move","numbers_threat_move","§◆ 선수 능력치를 판단에 어떻게 반영할지","stat_influence","stat_judg_ref","stat_neutral","stat_noise_shift","stat_pos_div","§◆ 그 밖","aggr_lane","dd_n_thr","jungle_retreat_threat",], note:
 "<b>배선은 돼 있는데 편집기에서 사라져 있던 노브</b>들입니다. 살아 있는 것만 골라 다시 꺼냈습니다.<br>\
 함께 숨어 있던 나머지는 <b>일부러 그대로 두었습니다</b> — 대부분 게임 쪽 코드가 사라져 값을 바꿔도 아무 일도 일어나지 않는 것들이고(설명에 ⛔로 표시돼 있습니다), 나머지는 개발·검증용이거나 무슨 값인지 아직 확인되지 않은 것들입니다.<br>\
 <b>교전 확률</b>과 <b>합류</b>는 성향을 크게 바꾸는 레버입니다. <b>포탑</b>·<b>머릿수</b>는 언제 물러날지를 정합니다.<br>\
 ⚠<b>전부 -1(원본)이 기본</b>이라 그냥 두면 게임과 같습니다. 한 번에 하나씩 실험하세요.", },
 Tab{ id:"planpick", title:"• [상위] 판단 선택 게이트 (플랜 결정기)", keys:&[
 "§◆ 어떤 판단을 만들지 고르는 최상위 단계","§◆ 후퇴 트리거","rt_a_offset","rt_a_slope","rt_a_base","rt_b_slope","rt_b_base","rt_c_slope","rt_c_base","rt_deadline_min",
 "§◆ 정글을 계속 돌 체력 기준","jg_hp_fight","jg_hp_nofight",
 "§◆ 어떤 판단을 만들지 고르는 최상위 단계","pl_obj_role","pl_ganker_gate","pl_serpen_phase_mask","pl_epic_phase_min",], note:
 "<b>다른 탭보다 한 단계 위</b> — 다른 탭이 '그 판단이 떴을 때 어떻게 행동하나'를 만진다면, 이 탭은 <b>애초에 그 판단을 만들지 말지</b>를 정하는 게임 최상위 단계(plan 결정기)를 직접 고칩니다.<br>\
 <b>왜 여기만 byte-patch가 먹나</b>: 모드는 하위 단계를 대체하는데 이 함수는 대체 대상이 아니라서 게임 원본이 그대로 실행됩니다(반대로 [3]라인전 같은 대체 대상 함수 내부는 패치해도 무효 — vw_lane 사례).<br>\
 ⚠<b>전부 -1(원본)이 기본</b>입니다. 값을 주면 <b>게임이 원래 만들지 않던 판단이 생길 수 있습니다</b> — 모드 재현이 처리 못 하는 판단이면 게임 원본으로 넘어가므로 크래시는 아니지만 AI 성향이 크게 바뀝니다. <b>한 번에 하나씩</b> 실험하세요.<br>\
 적용확인 = 모드 폴더 <b>plan_imm.txt</b>(applied=7/7)." },
 Tab{ id:"severity", title:"• [공통] 위협 민감도 (severity)", keys:&[
 "sv_enable",
 "§◆ 위협비율 사다리","sv_tr0","sv_tr1","sv_tr2","sv_tr3",
 "§◆ 체력 단계 경계","sv_hp1","sv_hp2","sv_hp3",
 "§◆ '사소' 위협 할인","sv_discount_shift","sv_discount_cap",
 "§◆ 소극 경로 별도 4임계","sv_pa_hp_hi","sv_pa_tr_hi","sv_pa_hp_lo","sv_pa_tr_lo",], note:
 "게임 AI 전체가 공유하는 <b>\"이 위협이 유의미한가\" 필터</b>를 직접 튜닝. 위협비율(tr)이 사다리 문턱을 못 넘으면 그 위협은 '사소'로 깎여서/무시되고, 넘으면 전액 반영됩니다.<br> <b>sv_enable=1</b> 켜야 반영(0=게임 원본). 같은 사다리 <b>사본 4곳 29사이트를 같은 값으로 일괄 패치</b>: ①위협 평가 정본(전 판단 공유) ②넥서스 공방 계열 위협 총합 ③라인·정글 위협 필터 ④타겟 선택 스코어러.<br> <b>tr 문턱↓ = 더 겁쟁이</b>(작은 위협도 심각하게 봄 → 일찍 후퇴/회피), <b>↑ = 더 대담</b>(웬만한 위협 무시). 체력이 낮을수록 낮은 문턱(sv_tr1~3)이 적용됨.<br> <b>소극 경로(sv_pa_*)</b>: 라인·정글 위협 필터(사본 [C])에는 공용 사다리(branch B)와 별개로 <b>훨씬 엄격한 '소극 경로'(branch A)</b>가 있음 — 원본: hp%≤25 & tr>34 통과 / hp%>15 차단 / hp%≤15 & tr≥20 통과. 이동/부쉬 대기 중 판단이 이 경로를 탈 수 있어 <b>정글 왕복(부쉬 오락가락) 튜닝의 직접 레버</b>. sv_enable=1 필요.<br>\
 ⚠[19]넥서스 방어의 자체 사다리는 별도 키(d19_sev_ratio_0~3·d19_sev_hp_1~3) 소관 — 거기만 다른 기준을 주는 것도 가능(편집기 미노출·cfg에 직접 추가하면 d19i_enable=1일 때 반영).<br> 적용확인 = sev_imm.txt(applied=N/<b>33</b> — 사다리 26 + 할인 3 + 소극 4. 구 29는 )." },
 Tab{ id:"vision", title:"• [공통] 시야", keys:&[
 "dd_lane_margin","vis_window",
 "§◆ 판단별 개별 단기 시야창","vw_jungle","vw_check","vw_nexus","vw_threat","vw_score",
 ], note:
 "적이 시야에서 사라져도 <b>일정 틱동안 '아는 적'으로 기억</b>해 판단에 반영. 값↑=더 오래 경계.<br>\
 <b>dd_lane_margin</b> = [3]라인 판단(dd7700) 전용 기억창(기본 120≈2초).<br>\
 <b>vis_window</b> = <b>비-라인 전반</b> 기억창(기본 600≈10초). 단일 공유 byte-patch(0.5.3 재핀완 applied=1/1)라 <b>여러 판단에 한꺼번에</b> 걸림.<br>\
 &nbsp;&nbsp;└ 영향 판단(13개 호출처): <b>넥서스 커밋([18·19])·오브젝트 평가·모르가드([12~14])·세르펜([15~17])·CONDGATE</b>. ↑=더 오래 '아는 적' 추적, ↓=빨리 잊음. 0=즉시 망각(주의).<br>\
 <b></b> 이 탭 전 키 = <b>팀전술 무관</b>(상위 경로까지 확인).<br>\
 <b>개별 단기 시야창(vw_*) 노출</b> — 구 『120틱 8곳 미개입』 → 0.5.3 전수 스캔으로 <b>25사이트 확정·6그룹 노출</b>(클론 분열로 8이 아니라 25. 구 8 카운트는 다른 상수였을 가능성). 전부 <b>-1(기본)=원본 120 유지</b>, imm8이라 <b>상한 127틱</b>.<br>\
 &nbsp;&nbsp;↑=사라진 적을 오래 경계(신중·왕복 억제 방향), ↓=금방 잊음(대담·재전진 빨라짐). <b>정글 부쉬 왕복 튜닝이면 vw_check·vw_threat부터.</b><br>\
 &nbsp;&nbsp;<b>완전 대체하는 함수 안의 사이트는 무효입니다</b> — <b>vw_lane</b>은 라인전 함수 내부라 `dd7_repl=1`인 동안 **아무 효과 없음**(라인전 기억창 = [3]탭 `dd_lane_margin`). <b>vw_check</b>도 라인전 커버 경로와 공유하므로 `dd_lane_margin`과 **같은 값**으로 맞추는 게 안전합니다(재현측 짝 = `ec_vision_ticks`도 동일).<br>\
 &nbsp;&nbsp;⚠vw_check는 라인·정글·모르가드(12/14) <b>공유 1사이트</b>라 subplan 단독 조절 불가 / disc17 창은 [18·19]탭 nx_dn_vision_mem 소관(별도) / disc7·9·10·11·13·15엔 개별 창 없음. 적용확인 = visshort_imm.txt(applied=N/25)." },

 Tab{ id:"misc", title:"• [기타] 게임 원본의 결함 고치기", keys:&[
 "§◆ 적의 두 번째 스킬 피해를 무시하는 문제","fix_skill2_dmg",
 "§◆ 체력 비례 스킬의 피해를 무시하는 문제","fix_hp_ratio",], note:
 "게임 원본에 있는 <b>계산 결함</b>을 선택적으로 고칩니다. <b>전부 기본 꺼짐</b>이라, 켜지 않으면 원본과 완전히 같습니다.<br>\
 <br>\
 <b>■ 적의 두 번째 스킬 피해를 무시하는 문제</b><br>\
 AI가 &quot;저 자리로 가면 얼마나 아플까&quot;를 계산할 때, 적의 <b>기본공격 · 첫 번째 스킬 · 궁극기</b>는 제대로 세면서 \
 <b>두 번째 스킬만 첫 번째 스킬 값으로 대신</b> 넣습니다. 코드에서 값을 옮기는 줄이 잘못 쓰여 있어서 생긴 문제이고, \
 <b>아군을 볼 때는 정상</b>입니다.<br>\
 그래서 AI는 <b>두 번째 스킬이 강한 적을 실제보다 덜 무서워하고</b>, 반대로 두 번째 스킬이 약한 적은 과하게 피합니다. \
 체감으로는 &quot;딜러 옆에 겁 없이 서 있다&quot; 쪽으로 나타납니다.<br>\
 <b>켜면</b> 두 번째 스킬 피해를 제대로 세도록 바꿉니다 — 전체적으로 <b>조금 더 몸을 사리는</b> 판단이 됩니다.<br>\
 ⚠원본 게임과 다르게 동작하므로, 원본 그대로를 원하면 꺼 두세요. 적용확인 = <b>fix_imm.txt</b>.<br>\
 <br>\
 <b>■ 체력 비례 스킬의 피해를 무시하는 문제</b><br>\
 스킬 피해는 <b>고정값 + 공격력 비례 + 체력 비례</b>로 정해지는데, AI가 &quot;이 스킬 얼마나 아플까&quot;를 계산할 때 \
 <b>체력 비례 부분만 통째로 빼먹습니다</b>. 실제 전투에서는 정상적으로 들어가므로, <b>AI가 보는 값과 실제 피해가 다릅니다</b>.<br>\
 그래서 체력 비례 스킬을 가진 선수는 AI가 <b>자기 스킬을 실제보다 약하다고 여겨</b> 덜 적극적으로 쓰고, \
 반대로 그런 적을 만나도 <b>덜 위험하다고 여겨</b> 겁 없이 접근합니다.<br>\
 <b>켜면</b> 체력 비례 부분을 예측에 포함시킵니다.<br>\
 ⚠<b>실제 피해량은 전혀 바뀌지 않습니다</b> — 바뀌는 것은 AI의 판단(스킬을 쓸지, 누굴 노릴지, 피할지)뿐입니다.<br>\
 ⚠<b>영향받는 선수는 많지 않습니다</b> — 기본 챔피언 중에는 <b>뱀파이어</b>(자기 체력 비례) 정도이고, \
 나머지는 워크샵에서 받은 챔피언 일부입니다. 대부분의 경기에서는 아무 차이도 안 납니다.<br>\
 ⚠<b>대상 체력 비례</b>(적의 체력에 비례하는 스킬, 예: 채찍)는 <b>고칠 수 없습니다</b> — 예측 계산에 상대 정보 자체가 넘어오지 않습니다.<br>\
 적용확인 = <b>fix_hp_ratio.txt</b>." },

 Tab{ id:"engine", title:"• [공통] 엔진·대체스택 (고급)", keys:&[
 "fast_read","fast_guard",
 "§◆ 행(멈춤) 진단 워치독","hang_diag","hang_secs","hang_run_secs","hang_run_rate",
 "§◆ 활동창 프로파일러","adv_prof","adv_prof_min","adv_prof_seg",
 "§◆ 대체 게이트","mp_repl","dd7_repl","poke_repl","recall_repl","engage_repl","cond_repl","e9jt","replay_reset",
 "§◆ 대체 게이트","nx_repl","d12_repl","d14_repl",
 "§◆ 속도","skip_untuned",
 "§◆ 계측","perf_measure","read_bench","probe",
 "§◆ 판단별 후퇴발동 누적 측정","sp_seen","sp_seen_tag","self_team_only",], note:
 "<b>고급/개발용</b> — AI judge 메모리 read 방식(fast_read)과 각 judge를 우리 코드로 대체할지 여부(대체 게이트).<br>\
 대체 게이트가 <b>전부 OFF면 게임 원본 로직</b> 사용. mp_repl(이동 마스터)이 꺼지면 dd7/poke도 동작 안 함. 문제 생기면 여기부터 롤백.<br>\
 ⚠<b>관전 즉시보기 결과가 실행마다 바뀐다면 여기부터 의심</b> — 대체 함수의 난수 소비가 원본과 어긋나면 <b>배경 시뮬(확정) ↔ 관전 재시뮬</b>의 결정성이 깨집니다. 결정성 확정 조합 = <b>mp·dd7·recall·engage·cond ON</b>. <b>d12_repl·d14_repl·poke_repl</b>은 현재 기본 켜짐이며 결과에 영향을 주지 않는 것으로 확인됐습니다. 증상이 나타나면 <b>poke → d12 → d14</b> 순으로 0으로 되돌려 보세요.<br>\
 <b>skip_untuned=1</b>(권장) = 손대지 않은 judge는 게임 원본을 그대로 써서 <b>결과는 같고 속도만 빨라집니다</b>(일정 넘김 가속). 단 선수별/클래스별 오버라이드를 쓰면 자동 해제됩니다.<br>\
 <b>perf_measure=1</b> = judge별 소요시간을 <b>perf.txt</b> 로 기록(어느 대체가 무거운지 식별). 측정 끝나면 0으로 되돌릴 것.<br>\
 <b>sp_seen=1</b> = 어느 판단에서 '전력 때문에 후퇴'가 몇 번 걸렸는지 <b>sp_seen.txt</b> 에 누적 기록(<b>log과 무관하게 이것만 켜면 됨</b>·게임 재시작해도 누적 유지).<br>\
 &nbsp;&nbsp;<b>sp_seen_tag</b> 에 전술 이름을 적어두면, 그 값을 바꾸는 순간 직전 구간이 <b>sp_seen_hist.txt</b> 에 한 줄로 확정됩니다 ⟹ <b>전술별 비교표가 자동으로 쌓임</b>.<br>\
 &nbsp;&nbsp;초기화 = 모드 폴더의 <b>sp_seen_acc.txt</b> 삭제." },
];

fn is_toggle(k: &str) -> bool {
 matches!(k, "cond_repl"|"gbskip"|"mp_repl"|"dd7_repl"|"poke_repl"|"recall_repl"|"engage_repl"
 |"e9jt"|"d4_repl"|"d7_repl"|"d4ttd"|"perf_measure"|"read_bench"|"replay_reset"|"enabled"
 |"nx_repl"|"d12_repl"|"d14_repl"|"d15_repl"|"skip_untuned"|"sp_seen"
 |"fix_skill2_dmg"|"fix_hp_ratio"|"probe"|"hd_skip_landmark"|"lt_revive_join"
 |"nxe_supp_off"|"nxe_battle_off")   // 08-08 넥서스 비상 부작용 분리 스위치 // 07-31 노출분 (sp_seen_tag는 자유 문자열이라 제외)
}
// ⛔is_added / "신규" 뱃지는 제거됨(유저 지시 2026-08-03) — 추가분을 구분 표시하지 않는다.
// 나열돼 있던 키(tower_*·numbers_*·ally_tower_*·rc_join*·stat_influence)는 08-03 원본 순수화로 전부 폐기됐다.
fn is_removed(k: &str) -> bool { k == "numbers_margin" }
// 값 무반영(050 하드코딩/폐기). 라벨 "폐기"(빨강) + 비활성 입력.
fn is_dead(k: &str) -> bool {
 matches!(k,
 "pk_home_lo"|"pk_home_hi"|"pk_home_x1"|"pk_home_y1"|"pk_hp_main"|"pk_hp_retreat"|"pk_smallact_split"|"pk_threat_mult"|"pk_zone_hp"|"pk_engage_dist"|"pk_obj_hp"
 |"bt_home_lo"|"bt_home_hi"|"bt_home_x1"|"bt_home_y1"|"bt_hp_retreat"
 |"t_ttd"|"t_gb"|"aggr_object"|"aggr_defense"
 |"ep_lane_margin"|"ep_pred_dist"|"ep_near_dist"|"ep_hp_low"|"ep_count_gate"
 |"d4_dmg_scale"|"d4_div_base"|"d4_coef_scale"|"d4_coef_min"|"d4_coef_clamp"|"d4_coord_dist"|"d4_ttd_scale"
 |"gb_rbx_div"|"gb_r15_div"|"gb_r14_num"
 )
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
 _ => k, // 라벨 = cfg 키 이름 그대로(영어)
 }
}
/// 기본값이 `-1`(=원본 그대로)인 키의 **게임 원본값**.
/// `-1`만 보이면 무엇을 덮어쓰는 건지 알 수 없어서, 원본을 같이 보여준다.
fn orig_val(k: &str) -> Option<&'static str> {
 Some(match k {
        "nxe_twin0" => "100",
        "nxe_twin1" => "0",
        "nxe_t2_1" => "0",
        "nxe_t2_2" => "0",
        "nxe_t2_3" => "0",
        "nxe_t1_1" => "0",
        "nxe_t1_2" => "0",
        "nxe_t1_3" => "0",
        "nxe_supp_off" => "0",
        "nxe_battle_off" => "0",

 "ex_skill2_level" => "3", "ex_ult_level" => "5",
 "ex_attack_margin" => "15000", "ex_attack_margin_sp" => "2000", "ex_attack_seek" => "100",
 // 성향 흔들림
 "au_noise_off" => "주사위 굴림", "au_noise_amp" => "900", "au_score_center" => "1000",
 // [08-05 신설]
 "eg_spread_base" => "3000", "eg_disk_radius" => "40000", "eg_radius_cap" => "300000",
 "cf_risk_near" => "9", "cf_risk_far" => "25", "cf_dmg_pct" => "35",
 "cf_reach_pad" => "15000", "cf_reach_pad_ult" => "150000",
 "cf_filter_off" => "0", "cf_flee_kill_off" => "0",
 "ld_score_floor" => "30",
 "re_cast_promote" => "2", "re_trace_pad" => "25000", "re_gate_subplan" => "1",
 "gu_level" => "5", "gu_enemy_mem" => "120", "gu_suppress_r" => "150000",
 // 교전 판단
 "bt_hp_flee" => "21", // ── [08-05 신설] 적 위치 추정 ──
 "eg_spread_base" => "3000",
 "eg_disk_radius" => "40000",
 "eg_radius_cap" => "300000",
 // ── [08-05 신설] 시전 후보 2차 검열 (라인 수비 전용) ──
 "cf_risk_near" => "9",
 "cf_risk_far" => "25",
 "cf_dmg_pct" => "35",
 "cf_reach_pad" => "15000",
 "cf_reach_pad_ult" => "150000",
 "cf_filter_off" => "0",
 "cf_flee_kill_off" => "0",
 // ── [08-05 신설] 1차 점수컷 ──
 "ld_score_floor" => "30",
 // ── [08-05 신설] 재경매 ──
 "re_cast_promote" => "2",
 "re_trace_pad" => "25000",
 "re_gate_subplan" => "1",
 // ── [08-05 신설] 전역 궁 ──
 "gu_level" => "5",
 "gu_enemy_mem" => "120",
 "gu_suppress_r" => "150000",
 "bt_hp_gate" => "41", "bt_chase_stop" => "15000",
 "bt_chase_keep" => "80000", "bt_vision_mem" => "120",
 // 라인 수비 2차
 "ld_chase_stop" => "15000", "ld_ally_near" => "160000", "ld_intervene" => "50000",
 "ld_vision_mem" => "120", "ld_est_base" => "10",
 // 팀 작전
 "tm_cancel_mask" => "2816",
 // 시전 후보
 "cs_lead_attack" | "cs_lead_skill" | "cs_lead_skill2" | "cs_lead_steal" => "30",
 "cs_lead_ult" => "60", "cs_ult_range" => "6000", "cs_ult_range_global" => "90000",
 "cs_ult_mode_mask" => "111", "cs_steal_hp" => "20", "cs_unit_hits" => "2",
 "cs_minion_vision" => "120", "cs_ally_hp" => "79", "cs_ally_radius" => "120000",
 "cs_cc_mask" => "952",
 // 행동 점수
 "sc_turret_radius" => "150000", "sc_engage_radius" => "122474",
 "sc_cell_dist" => "35000", "sc_dive_margin" => "15000", "sc_score_vision" => "120",
 "sc_risk_dmg" => "49", "sc_risk_hp1" => "65", "sc_risk_dmg1" => "29",
 "sc_risk_hp2" => "40", "sc_risk_dmg2" => "17", "sc_risk_hp3" => "25", "sc_risk_dmg3" => "10",
 "sc_focus_cap" => "80", "sc_kill_cap" => "80", "sc_kill_pct" => "60", "sc_null_score" => "-10",
 "sc_adv_lo" => "30", "sc_adv_m1" => "60", "sc_adv_0" => "80",
 "sc_adv_p1" => "150", "sc_adv_hi" => "200",
 "sc_ally_radius" => "150000", "sc_enemy_radius" => "100000",
 "mv0_adv_lo" => "40", "mv0_adv_m1" => "75", "mv0_adv_0" => "100",
 "mv0_adv_p1" => "200", "mv0_adv_hi" => "300",
 "mv0_risk_shift" => "2", "mv0_engage_shift" => "9", "mv0_base_penalty" => "-2",
 "mv0_near_bonus" => "10", "mv0_near_gate" => "950",
 "mv_tower_margin" => "30000", "mv_tower_cap" => "100", "mv2_gain_shift" => "7",
 "mv_engage_thr" => "9999", "vis_mem_global" => "120",
 "ld_around_range" => "80000", "ld_around_delay" => "5", "ld_mode_mask" => "417",
 "ld_move_pct" => "100", "ld_threat_state" => "13", "ld_rand_min" => "2",
 "dm_near_ally" => "150000", "dm_near_enemy" => "150000",
 "dm_lookahead" => "30", "dm_ult_lookahead" => "60",
 "dm_execute_hp" => "20", "dm_lasthit" => "2", "dm_skill_hp" => "79",
 "dm_ult_rally" => "6000", "dm_ult_rally2" => "90000", "dm_ult_range" => "150000",
 "dm_ult_mask_rally" => "111", "dm_ult_mask_focus" => "78", "dm_ult_mask_safe" => "33",
 "dm_skill2_level" => "3", "dm_ult_level" => "5",
 "sf_margin" => "15000", "sf_radius" => "120000", "sf_mem" => "120",
 "pe_collect_radius" => "200000", "pe_filter_radius" => "150000", "pe_near_cut" => "70000",
 "pe_minion_add" => "64000", "pe_champ_threat" => "100000", "pe_field_radius" => "250000",
 "pe_count_radius" => "120000", "pe_reach_bonus" => "80000", "pe_outer_band" => "32000",
 "pe_skillshot_width" => "20000", "pe_bodyblock_width" => "28000", "pe_tower_margin" => "18000",
 "pe_source_cap" => "150", "pe_predict_cap" => "140", "pe_tower_far" => "656",
 "pe_noise_amp_mode2" => "1000", "pe_noise_amp" => "2000", "pe_noise_exempt" => "100000",
 "pe_kind_scale" => "120", "pe_mode_mask" => "417", "pe_kind_mask" => "771",
 "pe_wall_risk" => "9999", "pe_well_risk" => "9999", "pe_ally_gain_cut" => "1200",
 "pe_state_gate" => "180",
 "ldsc_vision_mem" => "120", "ldsc_skill_factor" => "100",
 "ldsc_early_mask" => "128611", "ldsc_lost_target" => "-99999",
 "sc_near_bonus" => "10", "sc_obj_bonus" => "10", "sc_keep_thr" => "-30",
 // 라인 대기·안전 / 이동
 "lw_wait_dist" => "180000", "lw_back" => "180000", "lw_radius" => "80000",
 "ls_radius" => "80000",
 "mv_bush_arrive" => "16000", "mv_hide_near" => "12000", "mv_trace_dist" => "120000",
 // 판단 생성 게이트
 "pl_obj_role" => "1", "pl_ganker_gate" => "11",
 "pl_serpen_phase_mask" => "417", "pl_epic_phase_min" => "249",
 // ★[08-06] 체크박스 기본값 전수 대조 정정(v54\chk_toggles.py).
//   실제 기본값은 arm 이 쓰는 static 초기값 / tune 두 번째 인자에서만 읽는다 — 키 이름으로 추측하지 않는다.
 "gbskip" => "꺼짐",
 "d4ttd" => "꺼짐",
 "d15_repl" => "꺼짐",
 "d4_repl" => "켜짐",
 "fix_skill2_dmg" => "꺼짐",
 "fix_hp_ratio" => "꺼짐",
 "probe" => "꺼짐",
 "hd_skip_landmark" => "꺼짐",
 "lt_revive_join" => "꺼짐",
 // ★[08-06] 마지막 3개 — 설명에 근사치/사이트별로만 적혀 있던 것.
 "gb_close_radius" => "387",
 "gb_line_range" => "500",
 "gk_wait" => "10 / 12 / 15",
 // ★[08-06] 4차 보강 — tune 줄 주석의 `원본 N`, 토글은 static 초기값.
 "adv_prof" => "꺼짐",
 "bv_focus_radius" => "60000",
 "c3_ally_hp" => "79",
 "c3_enemy_near_a" => "120000",
 "c3_enemy_near_b" => "120000",
 "c3_minion_margin" => "64000",
 "c3_minion_near" => "120000",
 "cf_filter_off" => "150000",
 "cond_repl" => "꺼짐",
 "d12_repl" => "켜짐",
 "d14_repl" => "켜짐",
 "d4_ally_cnt" => "3",
 "d4_ally_radius_a" => "150000",
 "d4_ally_radius_b" => "150000",
 "d4_early_leave" => "170000",
 "d4_from_mid" => "1000",
 "d4_from_mid_mode" => "2001",
 "d4_gather_radius" => "150000",
 "d4_hp_safe" => "51",
 "d4_minion_cnt" => "2",
 "d4_partner_dist" => "200000",
 "d7_repl" => "꺼짐",
 "dd7_repl" => "꺼짐",
 "e9jt" => "꺼짐",
 "eh_abort_dist" => "220000",
 "eh_abort_hp" => "44",
 "eh_around_radius" => "80000",
 "eh_band_high" => "45000",
 "eh_band_low" => "12000",
 "eh_commit_hp" => "50",
 "eh_commit_r_high" => "40000",
 "eh_commit_r_low" => "70000",
 "eh_flee_clear_hp" => "29",
 "eh_reach_margin" => "25000",
 "eh_recall_radius" => "60000",
 "eh_score_norm" => "320000",
 "eh_trace_arrive" => "15000",
 "engage_base" => "-1",
 "engage_repl" => "꺼짐",
 "ex_skill2_level_x" => "3",
 "ex_ult_level_x" => "5",
 "fast_guard" => "1",
 "fast_read" => "꺼짐",
 "gk_window_margin" => "5",
 "hang_diag" => "꺼짐",
 "hd_around_radius" => "80000",
 "hd_bush_near" => "100000",
 "hd_cand_select" => "150000",
 "hd_detect_max" => "250000",
 "hd_fight_cut" => "150000",
 "hd_path_radius" => "60000",
 "hd_ph0_ttl" => "5",
 "hd_trace_leash" => "15000",
 "hd_vision_mem" => "120",
 "jungle_retreat_threat" => "100",
 "lt_ally_join" => "50000",
 "lt_around_radius" => "80000",
 "lt_phase_mask" => "417",
 "mp_repl" => "꺼짐",
 "mv2_well_radius" => "260000",
 "nx_around_atk" => "80000",
 "nx_around_def" => "80000",
 "nx_cull_dist19" => "80000",
 "nx_repl" => "켜짐",
 "perf_measure" => "꺼짐",
 "poke_repl" => "꺼짐",
 "read_bench" => "꺼짐",
 "recall_repl" => "꺼짐",
 "replay_reset" => "꺼짐",
 "skip_untuned" => "꺼짐",
 "sp_seen" => "꺼짐",
 "sp_seen_tag" => "꺼짐",
 "th_collect_radius" => "200000",
 // ★[08-06] 3차 보강 — tune("키", 원본값) 의 2번째 인자에서 직접 추출.
 "bv_c_none" => "-9999",
 "d19_ally_hp" => "50",
 "d19i_enable" => "0",
 "disc16_home_hp" => "100",
 "gb_enable" => "0",
 "hd_skip_landmark" => "0",
 "lt_revive_join" => "0",
 "nx_an_count_gate" => "5",
 "nx_dn_count_gate" => "38",
 "nx_enable" => "0",
 "pf_band_width" => "64000",
 "pf_center_band" => "704000",
 "pf_diag_far" => "95999 / 96000",
 "pf_diag_near" => "63999",
 "pf_edge_margin" => "192000",
 "rc_ad_far_pen" => "25",
 "rc_ad_mid" => "120001",
 "rc_ad_near" => "80000",
 "rc_ad_near_bonus" => "15",
 "rc_ahp2_pen" => "30",
 "rc_ahp_t1" => "70",
 "rc_ahp_t2" => "50",
 "rc_ally_hp_min" => "40",
 "rc_ed_far" => "200000",
 "rc_ed_far_bonus" => "20",
 "rc_ed_mid" => "160000",
 "rc_ed_near" => "130000",
 "rc_ed_near_pen" => "60",
 "rc_ed_vfar_bonus" => "40",
 "rc_ehp_t1" => "80",
 "rc_ehp_t2" => "60",
 "rc_ehp_t3" => "40",
 "rc_ehp_v1" => "90",
 "rc_ehp_v2" => "80",
 "rc_join_adv" => "10",
 "rc_join_dmid" => "160000",
 "rc_join_dnear" => "80000",
 "rc_join_obj_mult" => "2",
 "rc_join_rescue" => "6",
 "rc_join_weight" => "0",
 "rc_mult_bonus" => "20",
 "rc_norp_bonus" => "35",
 "rc_u13_bonus" => "10",
 "rc_u21_init" => "-40",
 "rt_a_slope" => "-800",
 "sv_enable" => "0",
 // ★[08-06] 2차 보강 — 토글은 켜짐/꺼짐, 사이트마다 다른 값은 병기.
 "adv_prof" => "꺼짐",
 "d12_repl" => "켜짐",
 "d14_repl" => "켜짐",
 "d19i_enable" => "꺼짐",
 "d7_repl" => "켜짐",
 "fast_guard" => "켜짐",
 "gb_enable" => "꺼짐",
 "hang_diag" => "꺼짐",
 "nx_enable" => "꺼짐",
 "nx_repl" => "켜짐",
 "sv_enable" => "꺼짐",
 // ★[08-06] 원본값 자동 보강 — 설명문의 `원본 N` 또는 코드 b1/b4 실측에서 뽑았다.
 "adv_prof_min" => "3000",
 "adv_prof_seg" => "15000",
 "ae_bonus_kill" => "140",
 "ae_bonus_near" => "70",
 "ae_bonus_soon" => "25",
 "ae_bonus_struct" => "80",
 "ae_gain_shift" => "7",
 "ae_none_mask" => "129123",
 "ae_risk_shift" => "6",
 "ae_threat_limit" => "9999",
 "ae_tower_shift" => "6",
 "aggr_lane" => "100",
 "ally_tower_dps" => "0",
 "ally_tower_dps_move" => "-1",
 "ally_tower_hp" => "0",
 "ally_tower_hp_move" => "-1",
 "ally_tower_range" => "150000",
 "ally_tower_range_move" => "-1",
 "bv_ally_cap" => "90",
 "bv_ally_flat" => "10",
 "bv_b_in" => "25",
 "bv_b_out" => "8",
 "bv_c_cap" => "60",
 "bv_cap_half" => "80",
 "bv_cap_main" => "160",
 "bv_d_in" => "90",
 "bv_d_out" => "30",
 "bv_focus_max" => "3",
 "bv_out_of_fight" => "5",
 "c3_hurt_scale" => "100",
 "cs_lead_attack" => "30",
 "cs_lead_skill" => "30",
 "cs_lead_skill2" => "30",
 "d19_retreat_hp" => "45",
 "d19_sev_hp_1" => "66",
 "d19_sev_hp_2" => "41",
 "d19_sev_hp_3" => "26",
 "d19_sev_ratio_0" => "49",
 "d19_sev_ratio_1" => "29",
 "d19_sev_ratio_2" => "17",
 "d19_sev_ratio_3" => "9",
 "d7_hp_normal" => "41",
 "d7_hp_selfheal" => "21",
 "d7_wp_dist2" => "14400000000",
 "dd_cover_count" => "2",
 "dd_cover_role_min" => "3",
 "dd_f22e80_margin" => "150000",
 "dd_facet_thr" => "999",
 "dd_frontier_mult" => "30",
 "dd_gatee_dist" => "112890625",
 "dd_ivar2_thr" => "2",
 "dd_lane_margin" => "120",
 "dd_main_near_dist" => "87890625",
 "dd_n_thr" => "2",
 "dd_near_dist" => "87890624",
 "dd_ratio_thr" => "31",
 "dd_survivor_thr" => "3",
 "ec_commit_hp" => "40",
 "ec_count_hp" => "40",
 "ec_count_radius" => "180000",
 "ec_engage_dist2" => "150000",
 "ec_iz_hp" => "51",
 "ec_oz_hp" => "50",
 "ec_self_hp_low" => "20",
 "ec_valid_hp" => "40",
 "ec_vision_ticks" => "120",
 "eng_role2" => "50",
 "eng_role3" => "70",
 "eng_role4" => "100",
 "eng_role_def" => "30",
 "engage_thr_mult" => "100",
 "ex_fail_min_ticks" => "119",
 "ex_judge_cap" => "100",
 "ex_judge_floor" => "150",
 "ex_judge_slope" => "85",
 "ex_order_hold" => "10",
 "ex_think_max" => "800",
 "ex_think_min" => "400",
 
 
 "gb_join_dist" => "60000",
 "gb_op_phase" => "31",
 "gb_push_hp" => "30",
 "gb_reach_cap" => "140000",
 "gb_reach_margin" => "25000",
 "gb_scout_radius" => "120000",
 "gk_hp_base_gank" => "70",
 "hang_run_rate" => "5000",
 "hang_run_secs" => "30",
 "hang_secs" => "8",
 "jg_hp_fight" => "21",
 "jg_hp_nofight" => "41",
 "mv2_arrive_snap" => "2000",
 "mv2_avoid_bias" => "1500",
 "mv2_avoid_coef" => "400",
 "mv2_avoid_margin" => "6000",
 "mv2_pos_mode_thr" => "10",
 "mv2_well_dist" => "260000",
 "numbers_margin" => "0",
 "numbers_min_enemy" => "1",
 "numbers_min_enemy_move" => "-1",
 "numbers_range" => "150000",
 "numbers_range_move" => "-1",
 "numbers_threat" => "0",
 "numbers_threat_move" => "0",
 "nx_an_cull_dist" => "390624",
 "nx_an_finish_hp" => "56",
 "nx_dn_hp_crit" => "21",
 "nx_dn_hp_low" => "31",
 "nx_dn_near_dist" => "120000",
 "nx_dn_nexus_hp" => "50",
 "nx_dn_pred_dist" => "240000",
 "nx_dn_vision_mem" => "120",
 "nxd_near_dist" => "120000",
 "nxd_p3_gate" => "38",
 "nxd_pred_dist" => "240000",
 "nxd_prog_crit" => "21",
 "nxd_prog_low" => "31",
 "nxd_ref_hp" => "50",
 "poke_reach_bonus" => "120000",
 "poke_serpen_slot" => "5",
 "rc_rng_a_base" => "1000",
 "rc_rng_center" => "100",
 "rc_rng_spread_div" => "20",
 "rc_score_div" => "100",
 "rt_a_base" => "80000",
 "rt_a_offset" => "80",
 "rt_b_base" => "45",
 "rt_b_slope" => "450",
 "rt_c_base" => "15",
 "rt_c_slope" => "350",
 "rt_deadline_min" => "60",
 "self_team_only" => "1",
 "sn_home_hi" => "960000",
 "sn_home_lo" => "64000",
 "sn_home_x1" => "892000",
 "sn_home_y1" => "896000",
 "sn_hp_crit" => "20",
 "sn_self_hp" => "51",
 "stat_influence" => "0",
 "stat_judg_ref" => "100",
 "stat_neutral" => "50",
 "stat_noise_shift" => "5",
 "stat_pos_div" => "2",
 "sv_discount_cap" => "18",
 "sv_discount_shift" => "2",
 "sv_hp1" => "65",
 "sv_hp2" => "40",
 "sv_hp3" => "25",
 "sv_pa_hp_hi" => "25",
 "sv_pa_hp_lo" => "15",
 "sv_pa_tr_hi" => "34",
 "sv_pa_tr_lo" => "20",
 "sv_tr0" => "49",
 "sv_tr1" => "29",
 "sv_tr2" => "17",
 "sv_tr3" => "9",
 "t_engage" => "100",
 "t_recall" => "0",
 "th_atk_margin" => "50000",
 "th_band_margin" => "32000",
 "th_cap" => "150",
 "th_skill_margin" => "18000",
 "tower_dps" => "8000",
 "tower_range" => "140000",
 "tower_threat" => "0",
 "vis_window" => "600",
 "vw_check" => "120",
 "vw_nexus" => "120",
 "vw_threat" => "120",
 // ★[0.5.4 신설] 경로/거리 시스템
 "path_orth_cost" => "640", "path_diag_cost" => "896",
 "path_danger_cost" => "1281", "path_greedy" => "7",
 "path_threat_floor" => "2", "path_threat_cap" => "60",
 "path_threat_scale" => "30", "path_threat_default" => "2",
 "path_wave_risk_ret" => "3",
 // ★[0.5.4 신설] 경매 중 강제 귀환
 "auc_flee_version_gate" => "1", "auc_flee_undying_gate" => "0",
 "auc_flee_hp_field" => "1624", "auc_flee_nexus_mask" => "256",
 "auc_flee_goal_far" => "928000", "auc_flee_goal_near_a" => "32000",
 "auc_flee_goal_near_b" => "32000", "auc_flee_end_delay" => "5",
 "auc_flee_pathfinder" => "2", "auc_flee_with_skill" => "1",
 "auc_flee_score" => "99999", "auc_flee_action_tag" => "3",
 // ★[0.5.4 신설] 판단14 넥서스 공격
 "an_home_wait" => "7", "an_tower_gate" => "0", "an_fallback" => "2",
 "an_attack_sub" => "18", "an_fallback_wave" => "2", "an_fallback_style" => "0",
 // 개별 시야창
 "vw_jungle" | "vw_check" | "vw_nexus" | "vw_threat" | "vw_score" => "120",
 _ => return None,
 })
}

/// 값 칸 아래에 붙는 한 줄 — 기본값과, 기본이 `-1`이면 원본값까지.

/// 설명문은 HTML 조각(`<b>`·`<br>`·`\` 줄이음)을 섞어 쓴다. 순서도에 그대로 그리면 태그가 보이므로
/// 평문으로 바꾼다. 굵게 표시는 못 살리지만 순서도에선 한 줄 요약이면 충분하다.
fn desc_plain(k: &str) -> Option<String> {
 let raw = desc_static(k)?;
 let mut out = String::with_capacity(raw.len());
 let mut in_tag = false;
 for c in raw.chars() {
 match c {
 '<' => in_tag = true,
 '>' => { in_tag = false; }
 _ if in_tag => {}
 '\n' => out.push(' '),
 _ => out.push(c),
 }
 }
 // 태그 자리에서 생긴 이중 공백 정리
 let mut t = String::with_capacity(out.len());
 let mut sp = false;
 for c in out.chars() {
 if c == ' ' { if !sp { t.push(c); } sp = true; } else { sp = false; t.push(c); }
 }
 let t = t.trim().to_string();
 if t.is_empty() { None } else { Some(t) }
}

/// ★[08-06] 텍스트박스에 보여줄 값 — `-1`(원본 유지)이나 빈칸이면 **실제 기본값**을 대신 보여준다.
///   ⚠기본값이 숫자가 아니면(`주사위 굴림`·`약 387`·`10 / 12 / 15`) cfg 에 넣을 수 없으므로 원래 값을 그대로 둔다.
fn shown_val(k: &str, cur: &str) -> String {
    if !(cur.is_empty() || cur == "-1") { return cur.to_string(); }
    match orig_val(k) {
        Some(o) if !o.is_empty() && o.trim_start_matches('-').chars().all(|c| c.is_ascii_digit()) => o.to_string(),
        _ => cur.to_string(),
    }
}

fn base_line(k: &str, def: &Option<String>) -> String {
 let d = def.clone().unwrap_or_else(|| "—".into());
 // ★[08-06] `기본 -1`·`기본 —` 은 사용자에게 아무 정보가 없다 — 기본값 맵으로 폴백한다.
 //   baseline(default.txt)에 없는 키는 def=None → "—" 가 되는데, 재노출한 키들이 전부 여기 걸렸다.
 match (d.as_str(), orig_val(k)) {
 ("-1", Some(o)) | ("—", Some(o)) | ("", Some(o)) => format!("기본 {}", o),
 _ => format!("기본 {}", d),
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
    "nxe_twin0" => "쌍둥이 타워가 **다 부서졌을 때** 얼마나 적극적으로 넥서스를 지킬지. 게임 0.5.4 가 원래 발동하던 상황입니다. 100 = 게임 원본과 같은 세기 / 200 = 더 적극적 / 50 = 덜 적극적 / 0 = 이 상황을 비상으로 보지 않음(원본 동작을 끕니다). 비상이 되면 멀리 있어도 수비 후보가 살아남아 달려오고, 죽을 것 같으면 빠지는 동작이 취소됩니다. 원본 100",
    "nxe_twin1" => "쌍둥이 타워가 **하나만 남아도** 비상으로 볼지. 값은 적극도(100=원본 세기). 0=끔. 쌍둥이는 팀당 2기입니다. 원본 0(끔)",
    "nxe_t2_1" => "**2차 타워가 1개 이상** 부서지면 비상으로 볼지(쌍둥이가 멀쩡해도). 적극도, 0=끔. 원본 0(끔)",
    "nxe_t2_2" => "**2차 타워가 2개 이상** 부서지면. 적극도, 0=끔. 1개 조건과 함께 걸리면 **높은 쪽**이 쓰입니다. 원본 0(끔)",
    "nxe_t2_3" => "**2차 타워가 3개(전부)** 부서지면. 적극도, 0=끔. 원본 0(끔)",
    "nxe_t1_1" => "**1차 타워가 1개 이상** 부서지면 비상으로 볼지. 가장 이른 시점입니다. 적극도, 0=끔. 원본 0(끔)",
    "nxe_t1_2" => "**1차 타워가 2개 이상** 부서지면. 적극도, 0=끔. 원본 0(끔)",
    "nxe_t1_3" => "**1차 타워가 3개(전부)** 부서지면. 적극도, 0=끔. 원본 0(끔)",
    "nxe_supp_off" => "⚠부작용 분리용. 비상일 때 게임이 '다른 행동'의 점수를 크게 깎아 눌러버리는 조항이 하나 있습니다. 비상을 자주 켜면 그 행동이 자주 죽습니다. 켜면 그 조항을 없앱니다. 평소엔 끔. 원본 0",
    "nxe_battle_off" => "⚠부작용 분리용. 교전 판단도 이 비상 신호를 입력으로 받습니다(효과 방향은 아직 규명 전). 켜면 교전 판단이 이 신호를 무시합니다. 평소엔 끔. 원본 0",
    "numbers_margin" => "단순 인원차 후퇴 임계(원본 0=끔). 1 이상으로 두면 (적 수 − 아군 수)가 이 값 이상일 때 후퇴합니다. 게임 원본에 없는 모드 추가 판정이라 0이 아니면 원본과 달라집니다. 0=끔",
    "aggr_object" => "⛔작동하지 않습니다(파서 저장되나 읽는 곳 0=값 무반영). 오브젝트(판단 9·11) 견제 공격성은 실제로는 poke_reach_bonus(↑=더 멀어도 견제)·poke_phase_gate로 조절. 원본 100",
    "aggr_defense" => "⛔작동하지 않습니다(읽는 곳 0=값 무반영). 에픽견제(판단 14) 공격성은 sn_hp_crit·nxd_ref_hp로, 넥서스 방어는 d19_retreat_hp·nx_dn_*로 조절. 원본 100",
    "t_gb" => "⛔작동하지 않습니다(050 하드코딩·값 무반영). 운영전환 거리 전체 배율%. 영역D 거리밴드에 ×t_gb/100. 크면 더 멀리까지 운영, 작으면 가까운 것만. 원본 100",
    "t_ttd" => "⛔작동하지 않습니다(050 하드코딩·값 무반영). 정글 갱킹/처치 적극성 배율%. 판단 4 TTD 임계에 ×t_ttd/100. 크면 확실할 때만 갱킹, 작으면 무리해서도. 원본 100",
    "d4_repl" => "판단 4(갱킹) 대체. 0=이 부분만 게임 원본 위임",
    "d15_repl" => "⛔작동하지 않습니다. [15 SerpenCheck]는 게임이 생성하지 않는 죽은 실행 단위(생성 사이트 부재·영구 미발화)이라 1로 켜도 재현이 실행될 일이 없음. 값 무의미·재시도 금지.",
    "gbskip" => "generic_build 영역D(운영전환)를 우리 코드로 대체(게임 원본 건너뜀)",
    "d4ttd" => "판단 4 TTD 계산 경로 사용. 보통 1",
    "cf_flee_kill_off" => "후퇴 의사가 있으면 시전 후보를 몰살하는 규칙을 끕니다. 원본은 후보 목록에 후퇴 계열이 하나라도 있으면 평타·스킬 후보를 통째로 버립니다. ⚠시전 지점이 2초 이내인 경우에만 풀립니다 — 먼 목적지에서는 '후퇴 의사 + 목적지 위험 25 이상'일 때 여전히 버립니다. 1=끔 / -1=원본(몰살 함)",
    "cf_risk_far" => "먼 경로(2초 초과)의 목적지 위험 임계(원본 25). ⚠후퇴 의사가 있을 때만 이 임계가 쓰입니다 — 후퇴 의사가 없으면 먼 목적지는 위험도로 걸러지지 않습니다. -1=원본",
    "gu_suppress_r" => "전역 궁 억제 반경(원본 150,000). 이 안에 보이는 적이 하나라도 있으면 전역 궁이 발동하지 않습니다 — 그래서 원본은 교전 중에 거의 안 나갑니다. 줄이면 교전 중에도 아군 요청 궁이 나갑니다. -1=원본",
    "re_cast_promote" => "재경매에서 갈아탈 수 있는 행동 개수(원본 2 = 평타·스킬·스킬2). 3으로 올리면 궁도 갈아타기 대상이 됩니다. ⚠4 이상은 '정지' 오더까지 승격 대상이 되므로 권장하지 않습니다. -1=원본",
    "re_gate_subplan" => "추격을 골랐을 때 '너무 멀면 그냥 계속 쫓기'로 조기 확정하는 판단 범위(원본 1 = 라인전 계열만). 올리면 다른 판단에서도 멀 때 재경매를 건너뛰고 계속 쫓습니다. 실제 체감 차이는 작습니다(멀면 어차피 공격 후보가 안 남습니다). -1=원본",
    "eg_spread_base" => "안 보이는 적의 위치 불확실성 기본항(원본 3000). AI는 마지막으로 본 시각·위치·이동속도로 \"지금 이쯤 있겠다\"는 원판을 그립니다. 이 값이 그 원판이 시간당 커지는 속도의 기본치입니다. ↑=적을 더 넓게 의심(조심스러워짐). -1=원본",
    "eg_disk_radius" => "추정 원판의 기본 반경(원본 40000, 2곳 동시). 시간이 얼마 안 지났어도 최소 이만큼은 불확실하다고 봅니다. -1=원본",
    "eg_radius_cap" => "추정 반경 상한(원본 300000). 너무 오래 못 본 적은 원판이 이 크기를 넘고, 그러면 그 적을 아예 판단에서 제외합니다. -1=원본",
    "cf_risk_near" => "시전하러 갈 자리의 위험 임계 — 가까울 때(원본 9). 시전 지점까지 2초 이내면 이 값을 넘는 위험한 자리로는 가지 않습니다. ★이 검열에서 가장 자주 걸리는 컷입니다. 라인 수비 전용. -1=원본",
    "cf_dmg_pct" => "시전 포기 피해 임계(%, 원본 35). 곧 받을 피해가 내 체력의 이만큼을 넘으면 시전 후보를 버립니다. 라인 수비 전용. -1=원본",
    "cf_reach_pad" => "닿는다고 보는 사거리 여유 — 평타·스킬·스킬2(원본 15000). ↑=더 먼 거리에서도 \"닿는다\"고 판단합니다. 라인 수비 전용. -1=원본",
    "cf_reach_pad_ult" => "같은 여유 — 궁 전용(원본 150000). 궁만 유독 관대합니다. 라인 수비 전용. -1=원본",
    "cf_filter_off" => "이 검열을 통째로 끕니다. 켜면 라인 수비의 모든 시전 후보가 심사 없이 통과합니다. 1=끔 / -1=원본(검열 함)",
    "ld_score_floor" => "후보를 버리는 점수 하한(절댓값, 원본 30 = −30점까지 살려둠). 라인 수비 1차 검열에서 이보다 나쁜 후보는 버립니다. ⚠판단력이 낮으면 이 검열 자체를 건너뜁니다(판단력 0이면 84.9% 건너뜀 — ex_judge_* 참조). -1=원본",
    "re_trace_pad" => "추격 재경매 전환 여유거리(원본 25000). 쫓는 대상이 사거리+이 값보다 멀면 그냥 쫓고, 그 안이면 같은 대상을 겨냥한 공격 후보로 다시 겨룹니다. -1=원본",
    "gu_level" => "전역 궁 요청이 먹히는 최소 레벨(원본 5). 아군이 채팅으로 궁을 요청해도 이 레벨 미만이면 무시됩니다. -1=원본",
    "gu_enemy_mem" => "전역 궁 억제 판정의 적 기억 시간(틱, 원본 120, 0~127만 유효). ↓=금방 잊어 궁이 더 자주 나갑니다. -1=원본",
    "d4_close_hp" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다).",
    "d4_threat_min" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다).",
    "d4_ref_dist2" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다).",
    "d4_engage_r2" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다).",
    "d4_ward_dist2" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다).",
    "d4_wcast_thr" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다).",
    "d4_pathlen_thr" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다).",
    "d4_dmg_scale" => "⛔폐기된 값입니다(재현 코드에 호출자가 없어 어떤 상황에서도 실행되지 않습니다).",
    "d4_div_base" => "⛔폐기된 값입니다(재현 코드에 호출자가 없어 어떤 상황에서도 실행되지 않습니다).",
    "d4_coef_scale" => "⛔폐기된 값입니다(재현 코드에 호출자가 없어 어떤 상황에서도 실행되지 않습니다).",
    "d4_coef_min" => "⛔폐기된 값입니다(재현 코드에 호출자가 없어 어떤 상황에서도 실행되지 않습니다).",
    "d4_coef_clamp" => "⛔폐기된 값입니다(재현 코드에 호출자가 없어 어떤 상황에서도 실행되지 않습니다).",
    "d4_coord_dist" => "⛔폐기된 값입니다(재현 코드에 호출자가 없어 어떤 상황에서도 실행되지 않습니다).",
    "d4_ttd_scale" => "⛔폐기된 값입니다(재현 코드에 호출자가 없어 어떤 상황에서도 실행되지 않습니다).",
    "gb_cnt_move" => "⛔폐기된 값입니다(운영 영역D 훅이 0.5.3에서 아직 배선되지 않아 재현 코드가 돌지 않습니다).",
    "gb_cnt_skip" => "⛔폐기된 값입니다(운영 영역D 훅이 0.5.3에서 아직 배선되지 않아 재현 코드가 돌지 않습니다).",
    "gb_da_thr" => "⛔폐기된 값입니다(운영 영역D 훅이 0.5.3에서 아직 배선되지 않아 재현 코드가 돌지 않습니다).",
    "gb_db_engage" => "⛔폐기된 값입니다(운영 영역D 훅이 0.5.3에서 아직 배선되지 않아 재현 코드가 돌지 않습니다).",
    "gb_r14_num" => "⛔폐기된 값입니다(운영 영역D 훅이 0.5.3에서 아직 배선되지 않아 재현 코드가 돌지 않습니다).",
    "gb_r15_div" => "⛔폐기된 값입니다(운영 영역D 훅이 0.5.3에서 아직 배선되지 않아 재현 코드가 돌지 않습니다).",
    "gb_rbx_div" => "⛔폐기된 값입니다(운영 영역D 훅이 0.5.3에서 아직 배선되지 않아 재현 코드가 돌지 않습니다).",
    "gb_score_mult" => "⛔폐기된 값입니다(운영 영역D 훅이 0.5.3에서 아직 배선되지 않아 재현 코드가 돌지 않습니다).",
    "t_engage" => "교전(engage·facet#5) 공격성 배율%. eng_role 4개 임계 전부에 ×t_engage/100. >100 적극/<100 소극. ※[재분석 ] '정글 전용' 아님 — 전 포지션 챔프의 교전판단에서 타깃 우선순위(obj+0x188=4/3/2/기타)별 후퇴임계 래더에 공용 적용. engage_repl 필요. 원본 100",
    "aggr_lane" => "라인전(dd7700) 공격성 배율% — 프론티어 후퇴 게이트 분모에 적용(l15×dd_frontier_mult×100/aggr_lane). >100=덜 물러남(공격적)/<100=잘 물러남. 원본 100",
    "rc_join_weight" => "합류 이득 마스터 가중. 0=끔(기존 동작, 배포 기본). >0이면 체력이 멀쩡해도 합류 이득이 클수록 복귀 점수↑ → 복귀를 전략적 합류 이동기로. ※체력기반 점수에 더하는 게 아니라 max(체력기반 vs 합류기반 중 강한 쪽 채택). 권장 시작 20",
    "rc_join_adv" => "수적 우위(아군≥적)일 때 합류 보너스 계수. 클수록 승산 한타에 적극 합류",
    "rc_join_rescue" => "수적 열세(아군<적)일 때 구원 보너스 계수. 클수록 열세 아군 전투를 도우러 합류",
    "rc_join_dnear" => "합류 대상까지 근접 거리 임계(미만이면 거리가중 ×3). 작을수록 가까운 전투만 합류",
    "rc_join_dmid" => "합류 대상까지 중거리 임계(미만 ×2, 이상 ×1). 멀수록 합류 이득 감가",
    "rc_join_obj_mult" => "리콜포인트(거점/오브젝트)가 있을 때 합류 이득 배수. 오브젝트 전투 합류 우선",
    "dd_n_thr" => "라인 슬롯 수 기준값. 원본 2",
    "sn_home_lo" => "세르펜 사냥·견제: 아군 기지의 회복존 안에 있고 다친 상태(체력<최대)면 나가지 않고 대기합니다. 그 회복존 영역의 X/Y 하위 경계(원본 64000). 회복존은 넥서스도 에픽도 아닌 부상 유닛 회복 구역입니다. -1=원본",
    "sn_home_hi" => "세르펜 사냥·견제: 위 회복존 영역의 X/Y 상위 경계(원본 960000). -1=원본",
    "sn_home_x1" => "세르펜 사냥·견제: 회복존 X 안쪽 경계(원본 892000). -1=원본",
    "sn_home_y1" => "세르펜 사냥·견제: 회복존 Y 안쪽 경계(원본 896000). -1=원본",
    "sn_hp_crit" => "세르펜 사냥·견제: 내 체력%가 이 값 이하이고 세르펜 근처 아군이 적보다 적으면 견제를 접고 물러납니다(원본 20). ↑=조금만 다쳐도 물러남. -1=원본",
    "d8_slot_thr" => "⛔작동하지 않습니다. 판단 8은 원본이 무조건 7(대기) 고정이라 이 임계를 읽는 코드에 도달하지 않음. 값 무반영. (구 설명: 슬롯값 <5/≥5 분기. 원본 5)",
    "sim_unchunk" => "⛔작동하지 않습니다(0.5.3). 패치 사이트 12B 시그가 0.5.3 exe 전역 0건(rayon 브리지 코드 변경). 원본바이트 재검증 후에만 패치하므로 ABORT=fail-safe(게임 무영향)이나 노브는 무반영. 실측=sim_unchunk.txt. (구 설명)백그라운드 경기 시뮬 병렬도 개선. 일정넘김 때 게임이 여러 경기를 1개 rayon job에 묶어 순차처리(코어 ~60%만 사용) — 이걸 1경기=1job으로 쪼개 노는 코어를 채움. 게임 rayon 분할 게이트 1곳을 nop(정적·thread-safe·크래시 위험 없음, RE확정). 결과(경기 승패)는 불변, 속도(가동률)만↑. 1=ON. ⚠효과는 배치 경기수·외곽 직렬병목에 캡됨 → 켜고 일정넘김 시간 A/B 직접 측정, 이상하면 0(원본복원). sim_unchunk.txt에 적용확인.",
    "team" => "override 대상 팀(개발용)",
    "x" => "override 좌표 X(개발용)",
    "y" => "override 좌표 Y(개발용)",
    "coef_mult" => "데미지 계수 배율%(검증용). 원본 100",
    "engage_base" => "engage 베이스 임계 정적 패치(-1=원본)",
    "engage_thr_mult" => "engage 임계 배율%(구버전 레버). 원본 100",
    "tower_threat" => "신규(게임에 없던 항). 포탑 조심 강도 0~100. 적 포탑 사거리 안에서 tower_threat≥전력승산이면 후퇴. 100=호각싸움도 수비, 0=원본. ※회피 본기능은 LIVE(라이너/is_under). (구설명의 '정글러 포탑딜 생존TTD 가산' 서브클레임은 dead 함수만 호출=무효). 원본 0",
    "tower_range" => "신규. 포탑 위협 판정 반경 — 이 거리 안의 적 포탑만 셈. 작으면 가까이서만 반응, 크면 멀리서도. 원본 140000",
    "tower_dps" => "포탑 하나를 전력으로 환산하는 값(원본 8000 ≈ 챔피언 한 명분). 아군 포탑을 우리 전력에 넣을 때와 적 포탑 밑 위험을 잴 때 함께 쓰입니다. 두 용도의 스위치(ally_tower_dps·tower_threat)가 모두 기본 0이라, 그대로 두면 이 값은 아무 효과가 없습니다. -1=원본",
    "numbers_threat" => "신규(정식 DPS×HP). 일반교전 전력승산 회피 — (ΣHP)×(Σ공격) 비교 → numbers_threat≥승산이면 후퇴(포탑무관). 0=원본, 100=이길싸움만, 50=확실히 질때만. 원본 0",
    "numbers_range" => "신규. 전력승산 계산할 때 근처 챔프/포탑 세는 반경(한타 때). 작으면 코앞만, 크면 넓게. 원본 150000",
    "numbers_range_move" => "신규. 위 전력카운트 반경의 라인전 전용 값. -1=폴백(한타값 numbers_range 따름), N=라인전 반경. 라인전선 더 좁게/넓게 세고 싶을 때. 원본 -1",
    "numbers_min_enemy" => "신규. 근처 적 챔프가 이 수 이상일 때만 전력후퇴 발동(머릿수 보조게이트, 한타). 1=현행, 2=적 2명+일때만. 원본 1",
    "numbers_min_enemy_move" => "신규. 위 머릿수 게이트의 라인전 전용 값. -1=폴백(한타값 numbers_min_enemy 따름), N=라인전선 적 N명+일때만 후퇴. 원본 -1",
    "numbers_threat_move" => "신규(라인전 멀뚱멀뚱 핵심). dd7700이 'Move(라인워크=딜교/미니언)'를 내려 할 때만 따로 적용하는 전력임계. -1=폴백(numbers_threat와 동일), 0=라인워크는 후퇴 안함(미니언/딜교 100% 보존). 게임이 교전/귀환(4/6/7) 의도일 땐 numbers_threat가 그대로 적용돼 한타 회피 유지. 원본 0(라인전 보존)",
    "ally_tower_hp" => "신규. 아군 포탑의 '체력'을 아군 전력(ΣHP)에 반영하는 가중치 0~100(한타 때). 포탑을 탱커처럼 = 아군 유효HP↑ → 타워밑 승산↑. 포탑이 깎이면 같이 감소. 0=off, 100=포탑 풀HP. 원본 0",
    "ally_tower_hp_move" => "신규. 위 포탑HP 가중치의 라인전 전용 값. -1=폴백(한타값 ally_tower_hp 따름), 0=라인전선 포탑HP 미반영, N=라인전 가중치. 원본 -1",
    "ally_tower_dps" => "신규. 아군 포탑의 '공격력'을 아군 전력(Σ공격)에 반영하는 가중치 0~100(한타 때). 포탑을 딜러처럼 = 아군 공격↑ → 승산↑. 포탑딜 단위=tower_dps(8000). 0=off. 원본 0",
    "ally_tower_dps_move" => "신규. 위 포탑DPS 가중치의 라인전 전용 값. -1=폴백(한타값 따름), 0=라인전선 미반영. 원본 -1",
    "ally_tower_range" => "신규. 아군 포탑 인식범위(한타) — self 기준 이 반경 안의 아군 포탑만 전력에 반영. 작으면 코앞 포탑만, 크면 멀리 포탑도. 원본 150000",
    "ally_tower_range_move" => "신규. 위 포탑 인식범위의 라인전 전용 값. -1=폴백(한타값 ally_tower_range 따름). 원본 -1",
    "numbers_threat_sp3" => "라인 공격 판단의 전력 열세 임계(%). 실측에서 이 판단은 한 경기 동안 한 번도 발동하지 않았습니다 — 라인전에서 멀뚱거리는 문제를 고치려면 여기가 아니라 numbers_threat_sp0을 보세요. -1=원본",
    "numbers_threat_sp0" => "라인전 판단의 전력 열세 임계(%). 실측 한 경기에서 38,060회 발동으로 전 판단 중 압도적 1위 — 라인전 성향을 바꾸려면 여기가 주 레버입니다. 낮출수록 열세여도 버팁니다. -1=원본",
    "numbers_threat_sp1" => "판단 1 전용 전력임계. sp0과 같은 라인전(dd7700) 계열이지만 발동은 훨씬 적다(실측 540회). sp0과 동일 사유로 그간 무반영이었다. -1=폴백. 원본 -1",
    "numbers_threat_sp2" => "신규. 판단 2(실명 LineDefense) 전용 전력임계. ⚠정규 경기에서 이 실행 단위은 미발화로 관측돼 값이 무반영일 수 있다. -1=폴백. 원본 -1",
    "numbers_threat_sp5" => "신규. 판단 5(귀환) 전용 전력임계. -1=폴백. 원본 -1",
    "numbers_threat_sp6" => "신규. 판단 6(교전) 전용 전력임계. -1=폴백. 원본 -1",
    "numbers_threat_sp10" => "신규. 판단 10(오브젝트 배틀) 전용 전력임계. 실경기 발동 확인됨(sp_seen.txt). -1=폴백. 원본 -1",
    "numbers_threat_sp15" => "신규. 판단 15(SerpenCheck=세르펜 교전판단) 전용 전력임계. ⚠플랜 15는 게임이 생성하지 않는 죽은 틀 → 실측 발동 0회 = 사실상 무의미. -1=폴백. 원본 -1",
    "numbers_threat_sp4" => "신규. 판단 4(실명 LineSafe=정글) 전용 전력임계. -1=폴백(numbers_threat). 원본 -1",
    "numbers_threat_sp7" => "신규. 판단 7(실명 Recall=복귀) 전용 전력임계. -1=폴백. 원본 -1",
    "numbers_threat_sp8" => "신규. 판단 8(실명 Jungle) 전용 전력임계. -1=폴백. 원본 -1",
    "numbers_threat_sp9" => "신규. 판단 9(실명 Battle=오브젝트 교전) 전용 전력임계. -1=폴백. 원본 -1",
    "numbers_threat_sp12" => "신규. 판단 12(실명 EpicCheck=에픽 교전판단) 전용 전력임계. -1=폴백. 원본 -1",
    "numbers_threat_sp16" => "[정정 ] LIVE — 감사의 '死레버(read site 없음)' 판정은 오판이었다(접두 파싱 키라 리터럴 스캔에 안 잡혔을 뿐). 인게임 실측으로 발동 524회 확인. 세르펜 사냥(16) 전용 전력회피 임계. 세르펜 사냥 중 전력승산이 이값 미만이면 후퇴. ↑=아슬한 싸움도 뺌(겁많음)/0=세르펜 사냥 땐 후퇴안함. -1(기본)=공통 numbers_threat 따름. ※유리할 땐(팀과 함께) 안 걸림. '혼자 세르펜 가서 죽는' 억제용은 60~70 권장.",
    "numbers_threat_sp17" => "발동 853회. 세르펜 견제(17) 전용 전력회피 임계. 세르펜 견제 중 불리하면 후퇴. ↑=겁많음/0=견제 땐 후퇴안함. -1(기본)=공통 따름. ※유리할 땐 안 걸림.",
    "numbers_threat_sp11" => "신규. 판단 11(실명 Hide) 전용 전력임계. -1=폴백. 원본 -1",
    "numbers_threat_sp13" => "신규. 판단 13(EpicHunt) 전용 전력임계. ⚠플랜 13은 게임이 생성하지 않는 죽은 틀 → 실측 발동 0회 = 사실상 무의미. -1=폴백. 원본 -1",
    "numbers_threat_sp14" => "신규. 판단 14(EpicPoke) 전용 전력임계. -1=폴백. 원본 -1",
    "stat_influence" => "신규. 성향스탯 반영 강도 0~100(0=off). 챔프 공격성·에고·판단력으로 라이너 후퇴판단 보정 — 공격성↑=덜 후퇴, 에고↑=잘 안 빠짐, 판단력↓=양방향 오판(결정론적). 기준=공50·에50·판100. 원본 0",
    "d19_threat_mult" => "⚠[은퇴] 은퇴한 my_실행 19 재현부(dcap-gated)에서만 read. d19thr=1 켜도 실제 게임 판단이 아니라 실행 19cmp.txt 대조값만 바뀜(게임 행동 무영향). 실제 넥서스 방어 튜닝은 d19_retreat_hp(byte-patch·항상반영)·nx_dn_*(nx_enable=1). (넥서스 방어 위협점수 배수%.) 원본 100",
    "d19_range_atkme" => "⚠[은퇴] 은퇴 my_실행 19 재현부(실행 19cmp 대조값)만 read=게임 행동 무영향. '넥서스(나)를 직접 공격중' 적 위협 가중. 원본 100",
    "d19_range_bld" => "⚠[은퇴] 실행 19cmp 대조값만=게임 무영향. '내 다른 건물 공격중' 적 위협 가중. 원본 60",
    "d19_range_other" => "⚠[은퇴] 실행 19cmp 대조값만=게임 무영향. '딴 대상 공격중' 적 위협 가중. 원본 40",
    "d19_range_idle" => "⚠[은퇴] 실행 19cmp 대조값만=게임 무영향. '비교전(놀고있는)' 적 위협 가중. 원본 80",
    "poke_phase_gate" => "⛔작동하지 않습니다(0.5.2). 대응 게이트가 원본에서 삭제됨. 값 바꿔도 무반영. 편집기 잔존은 cfg 호환용",
    "poke_active_min" => "⛔작동하지 않습니다(0.5.2). 대응 게이트가 원본에서 삭제됨. 값 바꿔도 무반영",
    "dd_early_p3_thr" => "⛔작동하지 않습니다(0.5.2,). 원본에 p3 비교 게이트가 아예 없음(0.5.0_3엔 있었음). 이 게이트가 조기분기를 항상 차단해 code 4·6이 전량 소실되던 버그의 원인이었고, 에 삭제됨. 값 무반영",
    "dd_cover_p3_thr" => "⛔작동하지 않습니다(0.5.2,). 원본에 대응 게이트 없음. 이 값이 커버블록 전체를 막고 있어서 dd_frontier_mult·dd_lane_margin·dd_cover_count·dd_ratio_thr 가 전부 안 먹던 원인. 삭제 후 그 4개가 실제로 작동. 값 무반영",
    "stat_neutral" => "공격성/에고 중립 피벗(50=중립). ↓=더 많이 공격적분류(덜 후퇴). 원본 50",
    "stat_pos_div" => "공격방향 감쇠 제수(2=절반반영, 1=풀강도). 원본 2",
    "stat_judg_ref" => "판단력 노이즈 기준(이상이면 노이즈0=완벽판단). 원본 100",
    "stat_noise_shift" => "판단력 노이즈 시간코히런스 시프트(tick>>N). ↑=느리게 갱신(뭉툭). 원본 5",
    "d13_engage_hp_pct" => "⛔작동하지 않습니다 — [13 EpicHunt]은 게임이 생성하지 않는 죽은 실행 단위(오더코드 도메인에 Hunt 코드 부재 + 런타임 0발화 실측). 값 무반영",
    "판단 16_home_hp" => "✅LIVE[07-16 신규]. [16 세르펜 사냥] self가 아군 지대 안에서 체력%가 이 값 미만이면 홈 대기(7). 원본 100=원본(=조금이라도 다치면 대기). ↓낮추면 더 크게 다쳐야 대기(공격적으로 계속 사냥).",
    "d15_engage_hp_pct" => "⛔작동하지 않습니다. 플랜 15 자체가 영구 미발화(죽은 틀)라 값 무반영. (구 설명: self 체력%<이값이면 교전 개시. 원본 51)",
    "ec_gate_tick" => "⛔작동하지 않습니다(0.5.2,). engage_gate·reposition_fight 양쪽에서 tick 인자가 원본 본문에 아예 쓰이지 않음(프롤로그에서 즉시 파괴). 값 무반영",
    "gb_join_phase" => "⛔작동하지 않습니다(0.5.2). 게임이 합류 거리분기 직후의 phase 게이트를 삭제(리팩터링·전 인코딩 스캔 0건). 값 바꿔도 무반영. 합류 타이밍 자체는 이제 phase 무관·거리(gb_join_dist)만으로 갈림. 구 설명(원본 12·합류 허용 임계)은 0.5.1 이하 전용.",
 "fix_skill2_dmg" => "체크하면 적의 두 번째 스킬 피해를 위험 계산에 제대로 반영합니다. 체크를 풀면 게임 원본 그대로(스킬1 값을 두 번 쓰는 상태)로 돌아갑니다",
 "fix_hp_ratio" => "체크하면 체력 비례 스킬의 피해를 AI 예측에 포함시킵니다. 실제 피해량은 안 바뀌고 AI 판단만 바뀝니다. 영향받는 선수는 뱀파이어 등 소수입니다",
 "eng_role2" => "교전 진입 확률 — 우선순위 2 대상(%, 원본 50). 0~100 주사위가 이 값 미만이면 교전에 들어갑니다. ⚠키 이름의 role은 잘못 붙은 것으로 실제로는 후보 우선순위입니다. -1=원본",
 "eng_role3" => "교전 진입 확률 — 우선순위 3 대상(%, 원본 70). -1=원본",
 "eng_role4" => "교전 진입 확률 — 우선순위 4 대상(%, 원본 100 = 항상 진입). -1=원본",
 "eng_role_def" => "교전 진입 확률 — 그 외 우선순위(%, 원본 30). 네 값 모두 t_engage(%)로 한 번 더 곱해집니다. -1=원본",
 "probe" => "계측 모드(1=켬). 게임 함수 11곳에 관찰용 훅을 설치해 분포를 셉니다. 게임 동작은 바뀌지 않습니다(카운터만 증가). 평소에는 0으로 두세요 — 배포 시 반드시 0. -1/0=끔",
 "lt_revive_join" => "체크하면 「아군에게 붙기」 판단을 되살립니다. 원본은 가장 가까운 아군을 찾을 때 자기 자신을 빼지 않아 항상 자기가 뽑히고, 그래서 이 판단이 한 번도 실행되지 않습니다. 체크하면 자기를 뺀 최근접 아군으로 다시 고릅니다. ⚠개발사가 테스트하지 않은 판단이라 결과가 어떻게 달라질지 미지수입니다",
 "lt_ally_join" => "⛔게임 원본이 도달할 수 없는 자리입니다(가장 가까운 아군을 찾을 때 자기 자신을 빼지 않아 조건이 영원히 거짓). lt_revive_join으로 되살릴 수 있습니다. 라인 총력전에서 아군에게 붙기 시작하는 거리. ⚠위의 「아군에게 붙기 되살리기」를 켜야 효과가 있습니다 — 끈 상태에서는 판단 자체가 실행되지 않아 값이 무의미합니다",
 "lt_around_radius" => "라인 총력전 중 배회하는 반경. 참고로 게임 내 3곳 중 1곳은 위의 죽은 판단 안에 있어 실제로는 2곳에만 적용됩니다",
 "lt_phase_mask" => "이 판단이 도는 경기 구간. 손대지 않는 것을 권장",
 "nx_cull_dist19" => "넥서스를 지킬 때 교전 후보로 볼 최대 거리. 공격 쪽에는 이미 있던 값인데 방어 쪽만 빠져 있었습니다",
 "nx_around_atk" => "적 넥서스를 칠 때 배회하는 반경",
 "nx_around_def" => "내 넥서스를 지킬 때 배회하는 반경",
 "hd_bush_near" => "매복은 2단계입니다 — 먼저 진영 안 거점으로 가고, 거기 이만큼 가까워지면 그때 수풀로 향합니다. 이 값은 그 전환 거리입니다. 올리면 거점 우회를 건너뛰어 수풀로 빨리 갑니다. ⚠내리면 반대로 우회가 길어집니다",
 "hd_path_radius" => "1단계(거점으로 갈 때) 목표점이 무작위로 흔들리는 반경",
 "hd_around_radius" => "1단계에서 거점 주변을 도는 반경",
 "hd_ph0_ttl" => "1단계 이동 명령을 몇 틱 붙잡는지. 올리면 목표를 더 오래 유지합니다",
 "hd_skip_landmark" => "체크하면 1단계(진영 안 거점 우회)를 통째로 건너뛰고 처음부터 수풀로 갑니다. 정글러가 매복 시간의 절반을 우회에 쓰는 것을 없앱니다",
 "hd_detect_max" => "숨을지 판단할 때 고려하는 적의 최대 거리. 내리면 멀리 있는 적은 무시합니다",
 "hd_fight_cut" => "교전 상황으로 볼 거리",
 "hd_trace_leash" => "숨은 상태에서 적을 쫓을 때 붙는 거리",
 "hd_cand_select" => "숨을 자리·물러날 자리 후보를 고를 때 쓰는 거리. 이 판단에서 가장 많이 쓰이는 값입니다",
 "hd_vision_mem" => "적을 본 기억이 남는 시간(틱). 올리면 오래된 목격도 위협으로 칩니다",
 "d4_ally_radius_a" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 아군으로 인정하는 거리",
 "d4_ally_radius_b" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 아군으로 인정하는 거리(다른 경로)",
 "d4_early_leave" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 이 거리를 넘으면 라인을 일찍 뜹니다",
 "d4_partner_dist" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 봇 듀오에서 파트너를 인지하는 거리",
 "d4_hp_safe" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 체력이 이 % 미만이면 안전하게 물러나거나 귀환합니다",
 "d4_from_mid" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 미드에서 이만큼 떨어져 있어야 귀환을 허용합니다",
 "d4_from_mid_mode" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 미드에서 이 거리 이하면 다른 처리로 넘어갑니다",
 "d4_ally_cnt" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 주변 아군 수 기준",
 "d4_minion_cnt" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 주변 미니언 수 기준",
 "d4_gather_radius" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 주변 상황을 모을 때 훑는 반경",
 "c3_enemy_near_a" => "지원 스킬을 아껴 두는 조건 — 대상 아군 주변에 적이 있는지 볼 반경. 내리면 스킬을 더 함부로 씁니다",
 "c3_enemy_near_b" => "위와 같은 반경(다른 경로)",
 "c3_minion_near" => "미니언이 가깝다고 볼 거리",
 "c3_ally_hp" => "이 체력% 미만인 아군에게만 지원 스킬을 씁니다. 내리면 진짜 위급할 때만 씁니다",
 "c3_minion_margin" => "미니언을 찾을 때 사거리에 더해 주는 여유. 올리면 먼 미니언도 노립니다",
 "c3_hurt_scale" => "'다친 아군' 판정 배율(원본 100). ⚠지원 스킬뿐 아니라 '사거리 안에 다친 아군이 있나'를 묻는 모든 판단(교전 시전 후보 포함, 호출 지점 11곳)에 동시에 걸립니다. -1=원본",
 "ex_ult_level_x" => "궁 해금 레벨이 코드에 박혀 있는 나머지 자리. ex_ult_level과 같은 값을 넣으세요",
 "ex_skill2_level_x" => "스킬2 해금 레벨의 나머지 자리. ex_skill2_level과 같은 값을 넣으세요",
 "eh_flee_clear_hp" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 도망 상태가 풀리는 체력%. 올리면 더 오래 도망칩니다",
 "eh_reach_margin" => "전술 ‘마무리’가 처치 우선일 때만. 교전 우선이면 이 필터 자체가 사라져 후보가 전원 통과합니다. 전술이 실제로 행동을 가르는 단 두 곳 중 하나입니다. 전술이 '킬 우선'일 때만 쓰는 교전 도달 여유. 이 값이 전술이 실제로 바꾸는 두 곳 중 하나입니다",
 "eh_recall_radius" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 귀환 국면에서 모이는 반경",
 "eh_around_radius" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 사냥 중 배회하는 반경",
 "eh_trace_arrive" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 추적할 때 이 거리까지만 붙고 멈춥니다",
 "eh_band_low" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 몬스터에게 접근할 때 유지하는 최소 거리",
 "eh_band_high" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 몬스터에게 접근할 때 유지하는 최대 거리",
 "eh_commit_hp" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 몬스터 체력이 이 % 미만이면 넓게 모입니다",
 "eh_commit_r_low" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 몬스터 체력이 낮을 때 모이는 반경",
 "eh_commit_r_high" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 몬스터 체력이 높을 때 모이는 반경",
 "eh_abort_hp" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 몬스터 체력이 이 %를 넘으면 사냥을 접습니다. 올리면 더 단단한 몬스터에도 덤빕니다",
 "eh_abort_dist" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 이 거리보다 멀면 사냥을 접습니다",
 "eh_score_norm" => "전술 무관. 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. 거리를 점수로 바꿀 때의 상한",
 "mv2_arrive_snap" => "목적지에 이만큼 가까워지면 남을 피해 돌아가지 않고 곧장 목적지로 파고듭니다. 올리면 막판에 밀고 들어가 뭉치기 쉽고, 내리면 끝까지 우회합니다 (원본 2000)",
 "mv2_avoid_coef" => "앞을 막은 유닛을 피해 얼마나 크게 방향을 트는지. 올리면 넓게 돌아갑니다 (원본 400)",
 "mv2_avoid_margin" => "피해서 돌아갈 때 상대와 띄우는 여유 거리. 올리면 더 멀찍이 돕니다 (원본 6000)",
 "mv2_avoid_bias" => "이 거리보다 가까이 붙은 상대에게만 회피가 걸립니다 (원본 1500)",
 "mv2_well_radius" => "기지(우물) 중심에서 이 거리 안이면 강제로 밖으로 밀어냅니다",
 "mv2_well_dist" => "우물에서 밀어낼 때 목표로 잡는 바깥 거리 (원본 260000)",
 "mv2_pos_mode_thr" => "자리잡기 이동에서 목적지 종류를 가르는 기준. 원본대로 두는 것을 권장 (원본 10)",
 "bv_cap_main" => "스킬 한 방으로 인정하는 이득의 상한. 올리면 광역기·한타 스킬을 더 아꼈다 크게 씁니다 (원본 160)",
 "bv_cap_half" => "보조 항목의 이득 상한(주 상한의 절반 위치) (원본 80)",
 "bv_focus_max" => "한 스킬로 몇 명까지 이득을 세는지. 올리면 다수를 맞추는 스킬을 더 높게 칩니다 (원본 3)",
 "bv_focus_radius" => "그 인원을 세는 반경",
 "bv_ally_flat" => "아군에게 거는 이로운 효과의 기본 점수 (원본 10)",
 "bv_ally_cap" => "아군 대상 효과 점수의 상한 (원본 90)",
 "bv_out_of_fight" => "교전 중이 아닐 때 주는 점수(평소엔 거의 안 쓰게 만드는 값) (원본 5)",
 "bv_b_in" => "교전 중일 때의 점수 (원본 25)",
 "bv_b_out" => "교전 밖일 때의 점수 (원본 8)",
 "bv_d_in" => "교전 중일 때의 점수(다른 효과 계열) (원본 90)",
 "bv_d_out" => "교전 밖일 때의 점수(다른 효과 계열) (원본 30)",
 "bv_c_cap" => "해당 계열 효과 점수의 상한 (원본 60)",
 "bv_c_none" => "대상이 없을 때 매기는 점수. 원본이 음수(-100)라 사실상 '쓰지 않음'을 뜻합니다. 되돌리려면 -9999",
 "ae_none_mask" => "이 함수가 점수를 내지 않을 행동 태그 비트마스크(원본 129123). 켜진 태그는 상위 판단이 자체 점수를 씁니다(비트 번호 = 태그−3, 태그 0~2는 7번 비트). ⚠구 표기 '128611'은 오류였습니다. -1=원본",
 "ae_risk_shift" => "위험 항목을 얼마나 줄여서 반영할지. 올릴수록 위험을 덜 무서워합니다 (원본 6)",
 "ae_tower_shift" => "포탑 위협을 얼마나 줄여서 반영할지. 올릴수록 포탑 앞으로 잘 나갑니다 (원본 6)",
 "ae_gain_shift" => "이득 항목을 얼마나 줄여서 반영할지. 올릴수록 이득에 둔감해집니다 (원본 7)",
 "ae_bonus_soon" => "곧 처치할 수 있는 대상이 있을 때의 가산 (원본 25)",
 "ae_bonus_kill" => "확실히 처치 가능한 대상이 있을 때의 가산. 이 탭에서 가장 큰 값이라 막타 집착을 좌우합니다 (원본 140)",
 "ae_bonus_near" => "가까운 대상이 있을 때의 가산 (원본 70)",
 "ae_bonus_struct" => "구조물(포탑 등)이 얽힌 자리의 가산 (원본 80)",
 "ae_threat_limit" => "위협 값이 이 이상이면 더 세지 않고 잘라냅니다 (원본 9999)",
 "th_collect_radius" => "위협을 셀 때 훑는 반경. 이 안의 적만 위험으로 칩니다. 줄이면 시야 밖 위협에 둔감해져 과감해집니다",
 "th_skill_margin" => "적 스킬 사거리에 더해 잡는 여유. 올리면 스킬 사거리 밖에서도 위험하다고 봅니다 (원본 18000)",
 "th_atk_margin" => "적 평타 사거리에 더해 잡는 여유 (원본 50000)",
 "th_band_margin" => "위험 구간을 나눌 때 쓰는 폭. 올리면 위험 범위가 넓게 잡힙니다 (원본 32000)",
 "th_cap" => "한 적에게서 나오는 위협의 상한. 내리면 강한 적 한 명을 덜 무서워합니다 (원본 150)",
 "rt_a_slope" => "후퇴 판정 첫 항목의 기울기. 원본이 음수(-800)입니다",
 "rt_a_base" => "후퇴 판정 첫 항목의 기준값 (원본 80000)",
 "rt_a_offset" => "후퇴 판정 첫 항목의 보정값 (원본 80)",
 "rt_b_slope" => "후퇴 판정 둘째 항목의 기울기 (원본 450)",
 "rt_b_base" => "후퇴 판정 둘째 항목의 기준값 (원본 45)",
 "rt_c_slope" => "후퇴 판정 셋째 항목의 기울기 (원본 350)",
 "rt_c_base" => "후퇴 판정 셋째 항목의 기준값 (원본 15)",
 "rt_deadline_min" => "'곧 죽는다'고 볼 최소 시간. 올리면 더 일찍 물러납니다 (원본 60)",
 "jg_hp_fight" => "자힐(체력 재생·힐/실드·궁)을 가진 상태에서 정글을 계속 돌 최소 체력(%, 원본 21). 미만이면 귀환. ⚠현재 설정에서는 이 값이 아니라 d7_hp_selfheal이 실제로 쓰입니다. 구 설명의 '교전 중'은 오라벨이었습니다. -1=원본",
 "jg_hp_nofight" => "자힐 수단이 없을 때 정글을 계속 돌 최소 체력(%, 원본 41). ⚠현재 설정에서는 d7_hp_normal이 실제 레버입니다. -1=원본",
 "t_recall" => "복귀 성향 가산. recall 점수에 +t_recall. 0보다 크면 자주 복귀, 작으면 덜 복귀. 원본 0",
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
 "pf_edge_margin" => "맵 가장자리 띠 두께 — 이 안쪽이면 '가장자리'로 판정. 실좌표",
 "pf_center_band" => "중앙 대각선 띠 폭(맵 중앙선 근처인지 판정)",
 "pf_diag_far" => "대각선 띠의 원거리 컷",
 "pf_diag_near" => "대각선 띠의 근거리 컷",
 "pf_band_width" => "위치판정 한 모드(m1)에서 쓰는 띠 폭(세로−가로 절댓값 비교 기준)",
 "dd_frontier_mult" => "합류(커버) 판정을 봉인하는 시점 계수(원본 30 ≈ 종료 30초 전). 경기 페이즈가 0·5·7·8이고 '종료 이만큼 전'을 지나면 봇 듀오 커버 판정을 아예 하지 않고 라인 유지로 갑니다. ↑=커버가 더 일찍 봉인돼 라인에 오래 남습니다 / ↓=끝까지 합류 판정. ⚠구 설명('전선 계산 배수', '올리면 더 잘 물러남')은 뜻도 방향도 반대였습니다. -1=원본",
 "sv_enable" => "이 탭 전 키 = 팀전술 무관(상위 호출 경로까지 확인) — 어떤 전술에서도 동일하게 작동합니다. 이하: 공유 위협 severity 사다리 byte-patch 마스터 스위치. 0(기본)=게임 원본. 1이면 사본 4곳 29사이트에 sv_* 값이 일괄 적용. 적용확인=sev_imm.txt.",
 "sv_tr0" => "위협비율 기본 문턱(체력 무관, 원본 49). tr>이값이면 '심각'. ↓=겁쟁이(작은 위협도 심각) ↑=대담.",
 "sv_tr1" => "체력%<66 구간 문턱(원본 29). 다친 상태에선 더 낮은 문턱이 적용되는 구조.",
 "sv_tr2" => "체력%<41 구간 문턱(원본 17).",
 "sv_tr3" => "체력%<26 구간 문턱(원본 9). 빈사 상태 민감도.",
 "sv_hp1" => "1단계 체력 경계(원본 65 = 체력%>65면 기본 문턱만 적용).",
 "sv_hp2" => "2단계 체력 경계(원본 40).",
 "sv_hp3" => "3단계 체력 경계(원본 25).",
 "sv_discount_shift" => "위협 평가 정본 전용: '사소' 판정된 위협의 할인 지수(원본 2 = 1/4로 축소). 0=할인 없음(사소해도 전액 반영=전체적으로 겁쟁이), 커질수록 사소 위협을 더 무시.",
 "sv_discount_cap" => "위협 평가 정본 전용: 할인 후 위협 상한(원본 18). ↓=사소 위협의 영향 상한을 더 낮게.",
 "sv_pa_hp_hi" => "소극 경로 A1: hp% 상단 게이트(원본 25). 내 체력%가 이 이하일 때만 1단 문턱(sv_pa_tr_hi) 검사에 들어감. ↑=1단 검사 hp 구간 확대(위협 인정 기회↑=완화).",
 "sv_pa_tr_hi" => "소극 경로 A2: tr 문턱 1단(원본 34, tr=위협×100÷현재체력). 초과=위협 인정(통과). ↓=소극 경로에서도 작은 위협을 인정(겁쟁이) ↑=대담.",
 "sv_pa_hp_lo" => "소극 경로 A3: hp% 하단 게이트(원본 15). 1단 불통과 시 체력%가 이 초과면 위협 무시(차단). ↑=차단 구간 축소(완화).",
 "sv_pa_tr_lo" => "소극 경로 A4: tr 문턱 2단·빈사 구간(원본 20). ⚠의미 = tr이 이 값 이상이면 통과(초과 아님·jb 인코딩 그대로 노출). ↓=빈사 시 작은 위협도 인정.",
 "vis_window" => "적 위치 추정에 쓰는 시야 기억창(틱, 원본 600 ≈ 10초). 패치 자리는 한 곳이지만 '적이 지금 어디 있을 수 있나'를 계산하는 함수에 들어가고, 그 함수를 9개 판단이 호출합니다. ⚠구 설명의 '넥서스 공수·조건게이트에 영향'은 실제 호출 관계에 없었습니다. 0=즉시 망각(주의). -1=원본",
 "dd_lane_margin" => "dd7700(라인 판단) 재현측 기억창(틱). 원본 120(≈2초). ↑=더 오래 경계, ↓=빨리 잊음. 게임측 쌍둥이 = vw_lane — 대체 ON 상태로 바꾸려면 둘을 같은 값으로",
 "vw_jungle" => "⛔일반 경기에서는 작동하지 않습니다(단일 라인 모드 전용 판단이라 정규전에서 한 번도 발화하지 않습니다). 판단 4 컨테이너 개별 시야창 5사이트(틱). -1=원본. ⚠판단 4는 정규전 미발화 관측이라 체감 없을 수 있음(참고용)",
 "vw_check" => "'마지막으로 본 적'이 유효한 시간(틱, 원본 120). ⚠25개 판단 함수가 공유하는 준-전역 값입니다(호출 지점 143곳). 한 판단만 바꾸려는 목적으로는 쓰지 마세요. -1=원본",
 "vw_nexus" => "'마지막으로 본 적'이 유효한 시간(틱, 원본 120). ⚠넥서스 전용이 아닙니다 — 라인 수비·총력전·대기·안전·교전·정글·넥서스 공수 등 13~16개 판단이 공유하는 헬퍼 두 개입니다. 올리면 AI 전반이 사라진 적을 오래 경계합니다. -1=원본",
 "vw_threat" => "위협평가 정본[A]·위협 컨텍스트 빌더[B] 헬퍼 2사이트(틱). -1=원본 120. 위협 합산에 '최근 목격' 적을 얼마나 오래 포함하나 — ↑=이동 중 위협을 오래 기억(왕복 주기 늘리기 후보)",
 "vw_score" => "타겟 후보 스코어링[E] 10사이트(틱). -1=원본 120. 타겟 선택에서 '최근 목격' 적의 유효 시간",
 "dd_cover_count" => "발화는 전술과 무관 · 다만 “어느 라인에 배정되는지”는 전술 영향. 값 자체는 항상 작동합니다. 수비 합류 발동 적 수 — 근처 적이 이 수 이상이면 라인 버리고 합류. 원본 2. 0으로 두면 상시 합류 시도(합류 매우 잦아짐)",
 "dd_ratio_thr" => "발화는 전술과 무관 · 다만 “어느 라인에 배정되는지”는 전술 영향. 값 자체는 항상 작동합니다. 커버 블록 종단의 내 체력% 임계 — 근처 적이 dd_cover_count명 이상일 때, 내 체력%가 이 값 미만이면 후퇴/대기(7) 판정으로, 이상이면 합류(4). 게임 원본값 = 51(구 '원본 31' 표기는 유저 튜닝값 오기). ↑=조금만 다쳐도 물러남 / ↓=낮은 체력에도 버팀",
 "dd_cover_role_min" => "발화는 전술과 무관 · 다만 “어느 라인에 배정되는지”는 전술 영향. 값 자체는 항상 작동합니다. 커버 블록 진입 역할 슬롯 하한(원본 3 = 봇 듀오 전용). 이 값 이상인 역할만 '봇 라인 2:1 커버' 판정을 수행합니다. 2로 낮추면 미드까지 커버 대상이 되고, 5로 올리면 커버가 사실상 봉인됩니다. ⚠원본은 3이라 기본값 유지 시 비트동일. dd7_repl=1 필요(재현측 배선)",
 "dd_facet_thr" => "체력이 낮을 때 어디로 빠질지 가르는 운영 진척 기준(원본 999). 진척이 이보다 크면 라인을 버리고 정글로, 아니면 총력전 합류로 갑니다. ⚠구 설명('크면 강하게 밀기')은 방향이 반대였습니다. -1=원본",
 "dd_near_dist" => "아군 근접 카운트 거리(원본 87890624 = 실거리 150,000). 아군 5칸 중 나 또는 타깃에서 이 거리 안인 수를 세고, 그 수가 '주변에 있을 수 있는 적 수' 이상이면 라인 공격으로 마무리합니다. ⚠구 설명('근처 적을 센다', '≈9375')은 대상도 단위도 틀렸습니다 — 값은 거리²÷256 인코딩입니다. -1=원본",
 "dd_main_near_dist" => "주 경로에서 '가까운 적'으로 볼 거리(원본 87890625 = 실거리 150,000, 거리²÷256 인코딩). ⚠구 설명의 '≈9375²'는 인코딩 오해였습니다. -1=원본",
 "dd_gatee_dist" => "라인 거점↔타깃 거리 하한(원본 112890625 = 실거리 170,000, 거리²÷256 인코딩). 이보다 가까우면 그냥 라인 공격으로 끝냅니다. -1=원본",
 "dd_ivar2_thr" => "발화는 전술과 무관 · 다만 “어느 라인에 배정되는지”는 전술 영향. 값 자체는 항상 작동합니다. 라인 진척 단계 기준값. 원본 2",
 "dd_survivor_thr" => "타깃 주변에 '있을 수 있는 적' 수 임계(원본 3). 이 수 이하면 무리하지 않고 라인 공격으로 내립니다. 이 적 수는 dd_f22e80_margin 반경으로 셉니다. ⚠구 설명의 '생존자 수'는 오해였습니다 — 생존자가 아니라 위치 추정으로 센 적 수입니다. -1=원본",
 "sn_self_hp" => "세르펜 사냥·견제: 위협 대상이 온전할 때, 내 체력%가 이 값 미만이면 견제를 멈추고 대기합니다(원본 51). 재는 것은 넥서스가 아니라 내 챔피언 체력입니다. -1=원본",
 "fast_read" => "AI judge가 게임 메모리를 읽는 방식. 2=VEH lockless(가장 빠름, 권장). 문제 생기면 1→0 으로 낮춰 롤백",
 "fast_guard" => "메모리 범위검사(readable/writable)를 syscall(VirtualQuery) 대신 VEH 프로브로 가속. fast_read=2일 때만 유효. 1(기본)=가속, 0=원본(느림) 롤백. 시뮬 속도에 직결 — 문제(크래시) 생기면 0으로",
 "hang_diag" => "게임이 멈추면 자동으로 hang_diag.txt에 원인을 기록하는 진단 도구 — STALL(스레드 갇힘: 전 스레드 위치+스택) / RUNAWAY(시뮬이 계속 돎: 경기 교착 의심). 0(기본)=OFF(opt-in) — 켜면 judge 콜마다 마킹 비용. 멈춤을 재현해 조사할 때만 1로 두세요",
 "hang_secs" => "[행진단] STALL 판정 대기 초. AI 판단이 이 시간 이상 완전 정지 + CPU 바쁨이면 덤프. 원본 8",
 "hang_run_secs" => "[행진단] RUNAWAY 판정 초. 고속 시뮬(judge 콜이 hang_run_rate/s 이상)이 이 시간 연속되면 '연산이 안 끝남' 덤프. 원본 30. 일정넘김이 원래 오래 걸리는 큰 리그면 60~120으로",
 "hang_run_rate" => "[행진단] RUNAWAY로 볼 초당 judge 콜수 문턱. 라이브 관전(수백/s)은 안 걸리고 백그라운드 고속시뮬만 걸리게 하는 값. 원본 5000",
 "adv_prof" => "활동창 프로파일러. 큰 연산(일정넘김 등)이 돌면 자동으로 구간을 감지해 adv_prof.txt에 기록 — 소요시간·CPU·judge 콜분포·어느 게임코드(exe+RVA)에서 시간이 갔는지 TOP20. 0(기본)=OFF — 켜면 활동 중 100ms마다 전 활성스레드 Suspend/Resume = rayon 워커 주기 정지(게임 느려짐). 성능 조사할 때만 1, 끝나면 0. ⚠배포(zip)용 cfg는 0 필수",
 "adv_prof_min" => "[프로파일러] 이 ms 미만의 짧은 활동창은 로그 생략(스팸 방지). 원본 3000(=3초 이상 연산만 기록)",
 "adv_prof_seg" => "[프로파일러] 끝나지 않는 긴 연산(일정넘김 안 끝남 등)의 중간 스냅샷 간격(ms). 이 간격마다 그때까지의 TOP 코드분포를 강제로 남김 → 무한연산도 어디서 시간 가는지 확인. 원본 15000(15초)",
 "cond_repl" => "facet#1 condgate(목표 커밋 판단)를 우리 코드로 대체",
 "mp_repl" => "facet#4 movepriority 대체 — 이동 판단 마스터 스위치(dd7/poke/d12/d14/d1617 등 movepri 계열 개별 스위치가 이걸 따름 — 이게 꺼지면 전부 원본)",
 "dd7_repl" => "판단 3 dd7700(라인전 이동) 대체 — mp_repl 켜져야 동작",
 "poke_repl" => "견제 판단(교전·은신)을 우리 재현으로 대체합니다. mp_repl이 켜져 있어야 동작합니다. 켜면 은신(부쉬·웨이포인트 자세) 관련 값들이 실제로 쓰입니다. 1=대체 / 0=게임 원본",
 "recall_repl" => "facet#5 recall(복귀 판단) 대체",
 "engage_repl" => "facet#5 교전 진입 대체",
 "e9jt" => "교전 판단에서 점프테이블 경로를 씁니다(정확도↑). ⚠**기본 꺼짐 — 켜지 마세요.** 켜면 모드가 게임 함수를 직접 호출하는데(vtable 함수포인터), 0.5.4에서 이 경로가 **멈춤·크래시를 일으킵니다**(08-06 확인). 끈 상태에서는 부정확한 쪽으로 안전하게 우회합니다",
 "d7_repl" => "귀환 이동 판단을 우리 재현으로 대체합니다. 켜면 아래 d7_* 값들이 실제로 반영됩니다. 1(기본)=대체 / 0=게임 원본",
 "d7_hp_normal" => "판단 7 일반 후퇴 HP%(자힐 없음). 이 % 이상이면 버팀(8), 미만이면 귀환(7). 원본 41(0x29)",
 "d7_hp_selfheal" => "판단 7 자힐(HP리젠) 보유 시 후퇴 HP%(더 낮게 버팀). 원본 21(0x15)",
 "d7_wp_dist2" => "판단 7 웨이포인트 근접 거리²(이 안이면 위협 TTD 판정). 원본 14400000000=120000²",
 "perf_measure" => "judge별 처리시간 측정→perf.txt. 측정 자체가 약간 느리게 함 → 평소 0",
 "read_bench" => "메모리 read 방식 벤치마크→readbench.txt. 평소 0",
 "replay_reset" => "다시보기 시작 시 내부 상태 리셋. 보통 1",
 "d19i_enable" => "넥서스 방어(실행 19) byte-patch 마스터 스위치. 1=severity 사다리·d19_retreat_hp·phase 상수를 cfg값으로 패치. 0(기본)=게임 원본 복원(retreat=45 등). d19_retreat_hp를 실제 반영하려면 이게 1이어야 함(0이면 만져도 원본).",
 "d19_retreat_hp" => "실작동 byte-patch — d19i_enable=1 켜야 게임에 반영(0이면 원본 45 유지. 구 설명 '게이트무관 항상반영'은 오류 — 그건 은퇴한 재현부 static 얘기였음). 넥서스 후퇴 HP%문턱. 위협이 애매할 때 넥서스HP%가 이 값 이하면 후퇴. ↑=높은 HP에도 후퇴(수비적), ↓=저HP까지 안 후퇴(공격적). 원본 45",
 "poke_reach_bonus" => "주 경로는 전술 무관 · 두 갈래만 전술 전용. 포탑=다이브와 수비=교전일 때만 타는 경로가 따로 있습니다. ✅근접 도달거리 보너스(좌표단위). ↑=더 멀어도 contested 판정. 원본 120000",
 "poke_serpen_slot" => "주 경로는 전술 무관 · 두 갈래만 전술 전용. 포탑=다이브와 수비=교전일 때만 타는 경로가 따로 있습니다. ✅serpen 웨이포인트 점유 임계(코드 5). 원본 5",
 "dd_f22e80_margin" => "발화는 전술과 무관 · 다만 “어느 라인에 배정되는지”는 전술 영향. 값 자체는 항상 작동합니다. 적이 여기 있을 수 있다고 보는 반경(원본 150000). AI는 안 보이는 적의 위치를 원판으로 추정하는데, 그 원판이 이 반경 안에 닿으면 “적이 근처에 있다”고 셉니다. ↑=적을 더 많이 있다고 보고 소극적. ⚠이 값은 「적 위치 추정」 그룹(ep_*)과 같은 계산의 일부입니다 — 함께 보세요. 원본 150000",
 "rc_rng_center" => "복귀배율 RNG 밴드 중심%(평균). ※recall_repl=1 완전대체시 반영. 원본 100",
 "rc_rng_spread_div" => "복귀배율 RNG 분산폭 제수(작을수록 변동↑). 원본 20",
 "rc_rng_a_base" => "공격성 기준선(spread base). 원본 1000",
 "rc_score_div" => "recall score 정규화 제수. 원본 100",
 "nxd_prog_low" => "넥서스 방어: 내 챔피언 체력%가 이 값 미만이면 방어를 포기하고 귀환합니다(원본 31). ↑=조금만 다쳐도 귀환. -1=원본",
 "nxd_prog_crit" => "넥서스 방어: 위협이 있을 때 내 챔피언 체력%가 이 값 미만이고 넥서스 구역 밖이면 귀환합니다(원본 21). -1=원본",
 "nxd_p3_gate" => "넥서스 방어의 위협 병합 게이트(원본 38). 게임 원본에는 대응 조건이 없고 우리 재현 코드에만 있는 값이라, 건드리면 게임과 결과가 갈릴 수 있습니다. -1=원본",
 "nxd_ref_hp" => "넥서스 방어: 내 넥서스 체력%가 이 값 이하이고 적 접근이 감지되면 즉시 방어로 전환합니다(원본 50). 이 그룹에서 실제로 넥서스 체력을 재는 유일한 값입니다. -1=원본",
 "nxd_near_dist" => "넥서스 방어: 넥서스 근접 판정 거리(원본 120000, 거리 제곱으로 저장). 적이 이 안에 들어오면 넥서스가 위협받는다고 봅니다. -1=원본",
 "nxd_pred_dist" => "넥서스 방어: 위협 예측 거리(원본 240000, 거리 제곱으로 저장). 근접 판정보다 넓은 바깥 띠로 곧 올 위협을 미리 봅니다. -1=원본",
 "nx_repl" => "⚠이름은 넥서스(nx)처럼 보이지만 넥서스와 무관합니다 — 예전 이름 잔재이고 실제로는 [16·17 세르펜 사냥·견제] 대체 스위치입니다. 1(기본)=우리 재현 사용 / 0=그 둘만 게임 원본 그대로(passthrough). 리플레이 A/B 실험용 — 같은 다시보기를 이 값만 바꿔 두 번 돌리면 개입 효과를 직접 확인할 수 있다. 결정성에는 영향 없음.",
 "d12_repl" => "모르가드 교전 판단을 우리 재현으로 대체합니다. 1(기본)=대체 / 0=게임 원본. 관전 중 이상이 보이면 poke_repl → 이 값 → d14_repl 순으로 0으로 되돌려 보세요",
 "d14_repl" => "모르가드 견제 판단을 우리 재현으로 대체합니다. 1(기본)=대체 / 0=게임 원본",
 "sp_seen" => "누적 측정 스위치. 1=어느 실행 단위(판단 종류)에서 '전력 때문에 후퇴'가 몇 번 발동했는지 sp_seen.txt 에 기록. log과 독립이라 이것만 켜도 되고(여러 판 연속 측정 시 권장), 게임을 재시작해도 sp_seen_acc.txt 에서 누적을 이어받는다. 부하는 사실상 0(원자 카운터 + 5초마다 작은 파일 1개). 측정 안 할 땐 0.",
 "sp_seen_tag" => "구간 라벨(자유 문자열 — 예: A_세르펜무조건, B_세르펜포기). 이 값을 바꾸는 순간 직전 구간의 발동수가 sp_seen_hist.txt 에 한 줄로 확정되고 카운터가 리베이스된다 ⟹ 전술을 바꿔가며 돌리면 전술별 비교표가 쌓인다. ⚠게임 실행 중에 바꿔도 ~0.5초 안에 반영된다(경기 도중에 바꾸면 그 경기가 두 구간에 걸쳐 쪼개지니, 경기 사이에 바꿀 것). ⛔auto는 무효 — 조합 테스트 경기는 DB(match_replays)에 저장되지 않아 전술을 읽어올 수 없다(리그 경기 값만 잡힘).",
 "skip_untuned" => "속도. 1(권장)=기본값에서 손대지 않은 judge는 게임 원본을 그대로 실행 → 결과는 완전히 같고 일정 넘김(배경 경기)이 빨라진다. 0=전 judge를 우리 코드로. ※선수별(players/*.cfg)·클래스별 오버라이드를 쓰면 자동으로 해제된다.",
 "ec_oz_hp" => "에픽(모르가드) 사냥·견제: 지정 구역 밖에 있을 때 내 체력%가 이 값을 넘으면 교전을 이어갑니다(원본 50). 이하면 대기. -1=원본",
 "ec_iz_hp" => "에픽 사냥·견제: 구역 안에 있고 대상이 온전할 때 내 체력%가 이 값 미만이면 교전을 포기하고 대기합니다(원본 51). -1=원본",
 "ec_self_hp_low" => "에픽 사냥·견제: 내 체력%가 이 값 이하이고 아군이 수적으로 열세면 철수합니다(원본 20). -1=원본",
 "ec_engage_dist2" => "에픽 사냥·견제: 교전으로 볼 거리(원본 150000, 거리 제곱으로 저장). -1=원본",
 "ec_valid_hp" => "에픽 사냥·견제: 유효 타깃으로 볼 최소 체력%(원본 40). 세르펜 견제 판단에도 같이 적용됩니다. -1=원본",
 "ec_commit_hp" => "에픽 사냥·견제: 교전을 굳힐 아군 체력% 하한(원본 40). 세르펜 견제 경로에도 적용됩니다. -1=원본",
 "ec_count_hp" => "에픽 사냥·견제: 아군·적 머릿수를 셀 때 인정할 최소 체력%(원본 40). 세르펜 견제 경로에도 적용됩니다. -1=원본",
 "ec_count_radius" => "에픽 사냥·견제: 아군·적 머릿수를 세는 반경(원본 180000). 세르펜 견제 경로에도 적용됩니다. -1=원본",
 "ec_vision_ticks" => "에픽 사냥·견제: 마지막으로 본 적이 유효한 시간(틱, 원본 120). vw_check와 같은 축의 재현측 값입니다. -1=원본",
 "nx_enable" => "이 탭의 값들을 실제로 적용할지 정하는 마스터 스위치. 0(기본)=게임 원본 그대로 / 1=아래 nx_dn_*·nx_an_* 값 적용. 이 탭은 팀 전술과 무관합니다. 다만 넥서스 공격·방어 판단 자체가 경기 상황(팀 모드)으로 정해지므로, 우리 팀이 공격 국면이 아니면 공격 쪽 값은 쓰이지 않습니다",
 "nx_dn_nexus_hp" => "아군 넥서스의 체력이 이 값 이하면 적극 수비로 전환(%, 원본 50). ★이 탭에서 실제로 넥서스 체력을 재는 유일한 값입니다(나머지 nx_dn_* 체력값은 챔피언 체력). ↑=일찍 수비 전환",
 "nx_dn_hp_crit" => "넥서스 방어 중 '위급' 판정 체력(%, 원본 21). ⚠재는 것은 챔피언 자신의 체력이지 넥서스 체력이 아닙니다(넥서스 체력은 nx_dn_nexus_hp). ↑=조금만 다쳐도 위급으로 봄",
 "nx_dn_hp_low" => "넥서스 방어 중 '체력 낮음' 판정 체력(%, 원본 31). ⚠위와 같이 챔피언 자신의 체력입니다. 위급(21)보다 한 단계 위의 경계",
 "nx_dn_near_dist" => "넥서스 근접 판정 거리(원본 120000, 코드가 제곱해서 씀). 적이 이 안에 들어오면 넥서스가 위협받는다고 봅니다. ↑=더 멀리서부터 위협으로 판정",
 "nx_dn_pred_dist" => "넥서스 위협 예측 거리(원본 240000). 근접 판정보다 넓은 바깥 띠로, \"곧 올 것 같다\"를 미리 봅니다. ↑=더 일찍 대비",
 "nx_dn_vision_mem" => "적을 본 기억이 남는 시간(틱, 원본 120 ≈ 4초). ↑하면 오래된 목격도 위협으로 쳐서 넥서스 방어가 더 자주 걸립니다. ⚠이름의 'lane'은 잘못 붙은 것으로 레인 진척과는 무관합니다(키 이름은 기존 설정 파일 호환 때문에 유지)",
 "nx_an_finish_hp" => "내 체력%가 이 값 이상이면 타워 규율 검사를 건너뛰고 적 넥서스를 칩니다(원본 56). ⚠키 이름이 finish_hp지만 대상의 체력이 아니라 내 체력입니다. ↓낮추면 체력이 적어도 무리하게 밀어붙이고, ↑높이면 건강할 때만 넥서스를 칩니다",
 "nx_an_cull_dist" => "넥서스 공세에 동원할 아군을 거리로 걸러내는 기준(원본 390624 ≈ 2.5칸). 이보다 멀면 그 아군은 후보에서 빠집니다. ↑높이면 먼 아군까지 동원하고, ↓낮추면 근접한 아군만 참여합니다",
 "gb_enable" => "로밍 판단 값들을 적용할지 정하는 마스터 스위치. 0(기본)=게임 원본 그대로 / 1=아래 gb_* 값 적용(각 값이 -1이면 그 항목만 원본 유지). ⚠gb_join_phase는 현재 동작하지 않습니다",
 "gb_join_dist" => "합류/근접 전환거리(유닛, 원본 60000). 대상까지 이 거리 이내면 '근접/합류 모드'로 전환하는 지배 게이트(라인range 세팅도 여기서). ↑키우면 더 먼 거리에서도 합류/근접 모드=적극 합류, ↓줄이면 바짝 붙어야 합류. -1=원본. (거리²로 인코딩됨, 유닛으로 입력)",
 "gb_scout_radius" => "거점/타겟 후보수집 반경(유닛, 원본 120000)=로밍 탐색범위. 후보 유닛/거점을 이 반경 안에서만 채택. ↑키우면 더 먼 거점까지 로밍 대상=로밍 범위 확장, ↓줄이면 근처만. -1=원본. 0.5.3: 패치 자리 2곳→1곳 병합(게임이 같은 기준값을 루프 앞뒤 2곳에 복사하던 것을 루프 안 1회 사용으로 변경)·비교방향 반전은 모드가 보정 = 의미·체감 동일. 적용확인=gb_imm.txt.",
 "gb_close_radius" => "근접 판정 반경(유닛, 원본≈387). 근접 여부를 재는 반경 파라미터. ↑키우면 근접 판정 범위↑. -1=원본. (imm32 부호확장 제약: √값 46340 미만)",
 "gb_line_range" => "라인 판정 반경(유닛, 원본≈500). 근접모드 진입 시 세팅되는 라인 거리 임계. -1=원본. (imm32 제약: √값 46340 미만)",
 "gb_op_phase" => "운영 진입 phase 임계(원본 31, =경기진행 카운터>30). phase가 이 값 이상이면 운영 로직 진입. ↓낮추면 더 이른 시점부터 운영 시작, ↑높이면 늦게. -1=원본. (0~127 범위)",
 "gb_push_hp" => "라인 압박 HP% 임계(원본 30). 라인 대상 체력%가 이 값 미만이면 압박 오더(order state=3). ↑높이면 더 높은 체력에도 압박=공격적, ↓낮추면 빈사일 때만. -1=원본. (0~127 범위)",
 "gb_reach_cap" => "도달 판정 상한(원본 140000). ⚠구 표기 140052는 오류였습니다 — 정확한 원본 복원은 -1로만 됩니다. -1=원본",
 "gb_reach_margin" => "⚠전역 사거리 여유(유닛, 원본 25000). reach 2차판정의 이동-외삽 여유폭. gb_reach_cap과 마찬가지로 전 AI 공유(신중). ↑키우면 사거리 판정에 여유↑. -1=원본(무개입 권장).",
 "gk_wait" => "정글 전술에 따라 갈림. 이 값이 걸리는 자리가 성장·커버 / 라인개입 / 카정에 나뉘어 있습니다. 특히 라인개입일 때 가장 많이 발화합니다. 5사이트 중 1곳이 '라인 개입' 전술 전용(나머지 4곳은 전술 무관). ⟹ 부쉬 왕복이 라인 개입에서만 난다면 팀전술 초반 정글 = 라인 개입 위주로 두고 실험해야 이 값의 전용 사이트가 살아납니다. 이하 상세: 갱 셋업 부쉬 대기 timeout(초). 원본 = 사이트별 10/12/15초(5사이트). -1=원본. 2~72초 조합 근사(레지스터 배수 인코딩이라 요청값에 가장 가까운 조합 선택). ↑=한 번 자리 잡으면 오래 대기 = 취소→재시도 왕복 주기↓ / ↓=빨리 포기하고 정글 복귀. 정글 부쉬 왕복(와리가리) 튜닝의 1순위 레버. 적용확인=gank_imm.txt",
 "gk_hp_base_gank" => "정글 전술에 따라 갈림. 이 값이 걸리는 자리가 성장·커버 / 라인개입 / 카정에 나뉘어 있습니다. 특히 라인개입일 때 가장 많이 발화합니다. 라인개입(jng=1) 갱 발동 HP 게이트 base(원본 70). 실효 임계 = base − 정글러 스탯/5 (스탯100→50%). 정글러 체력%가 실효 임계 이상이어야 갱 리드액션 발동. ↑=발동 억제(왕복 빈도↓·전술도 약해짐) ↓=저체력에도 시도. jng=1 분기 전용 카피만 패치 = 성장/커버·카운터정글엔 무영향. -1=원본",
 "gk_window_margin" => "정글 전술에 따라 갈림. 이 값이 걸리는 자리가 성장·커버 / 라인개입 / 카정에 나뉘어 있습니다. 특히 라인개입일 때 가장 많이 발화합니다. 갱 윈도우 최소 여유 배수(원본 ×5, 허용 2/3/5/9 — 다른 값은 최근접 매핑). 그 라인의 갱 윈도우 마감까지 이 배수×1초 이상 남아야 시도. ↑(9)=마감 임박 재시도 억제 = 시도 횟수↓ / ↓(2)=임박에도 시도. -1=원본. 전술 조건: 3사이트가 정글 전술 갈래별로 1곳씩(성장/커버·라인개입·카운터정글) ⟹ 어느 전술이든 1곳은 적용됩니다",
 "ex_judge_floor" => "판단력 오판 게이트의 문턱 하한(원본 150). 문턱 = min(판단력,cap)×slope÷10 + floor. ★이 문턱 하나가 두 곳을 동시에, 반대 방향으로 움직입니다 — ①주사위가 문턱보다 크면 최선 후보 대신 무작위 후보 선택 ②주사위가 문턱 이하일 때만 후보 점수 검사(cs_score_floor)를 수행. 즉 판단력이 낮으면 나쁜 후보가 걸러지지 않은 채 남고(84.9%가 검사 면제) 그중 하나가 무작위로 뽑혀 효과가 곱해집니다. 원본은 판단력 0인 챔프가 85% 확률로 오판. ↑=전 챔프 오판 감소 + 점수 검사도 강화(400이면 최대 60%, 1000이면 오판 0·검사 100%). -1=원본",
 "ex_judge_slope" => "판단력이 문턱에 미치는 기울기(원본 85). 문턱 = min(판단력, 상한)×이값÷10 + 하한. ⚠라인 수비 판단에서만 작동하고, 모드가 127로 잘라 넣습니다. 또 내부에 16비트 절단이 있어 상한×기울기 ≥ 65536이면 값이 감깁니다. -1=원본",
 "ex_judge_cap" => "판단력을 세는 상한(원본 100). ⚠라인 수비 판단 전용이며 모드가 127로 잘라 넣습니다. 상한×기울기 ≥ 65536이면 16비트 절단으로 값이 감깁니다(기울기 85 기준 상한 771 이하). -1=원본",
 "ex_order_hold" => "오더 유지 최소 경과(틱, 원본 10). 한 번 고른 행동을 최소 이만큼 유지한 뒤에야 재선정합니다. ↑=행동이 뚝심 있어짐(우왕좌왕·왕복 완화 후보) / ↓=상황 변화에 민감. -1=원본",
 "sc_turret_radius" => "구조물을 인식하는 반경(유닛, 원본 150000, 13곳 동시). 아군 포탑이 나를 지켜주는지, 적 포탑이 나를 위협하는지 판단할 때 이 반경 안의 구조물만 봅니다. ↑=먼 포탑까지 계산에 넣음 / ↓=코앞 포탑만. ⚠제곱 인코딩이라 모드가 자동 변환합니다. -1=원본",
 "sc_engage_radius" => "적을 '근접해 있다'고 보는 반경(유닛, 원본 약 122474, 5곳 동시). 포탑 지원 보너스를 줄지 판단할 때 씁니다. ↑=멀리 있는 적도 교전 중으로 간주. -1=원본",
 "sc_cell_dist" => "셀 위협 페널티가 걸리기 시작하는 거리(유닛, 원본 35000). 대상이 이보다 멀 때만 '가려는 자리가 위험한가'를 계산합니다. ↑=웬만한 거리에선 자리 위험을 무시(저돌적) / ↓=가까운 대상에도 자리를 따짐. -1=원본",
 "sc_dive_margin" => "적 포탑 위협 사거리의 여유분(유닛, 원본 15000). 포탑 사거리에 이만큼을 더한 범위를 '다이브 위험'으로 봅니다. ↑=다이브를 훨씬 꺼림(안전 지향) / ↓=포탑 밑까지 들어감. -1=원본",
 "sc_score_vision" => "점수 계산의 '마지막으로 본 적' 유효 시간(틱, 원본 120). ⚠같은 축이 1곳인데 이 키는 그중 1곳만 잡습니다 — 나머지 1곳은 vw_score가 담당하고 있어, 여기서 또 잡으면 두 값이 같은 자리를 다투게 됩니다. vw_score를 쓰세요. -1=원본",
 "sc_risk_dmg" => "위험 판정 기본선(%, 원본 49). 예상 피해가 내 체력의 이 비율을 넘으면 체력과 무관하게 위험으로 봅니다. ↑=웬만해선 위험으로 안 봄(공격적) / ↓=조금만 아파도 물러남. -1=원본",
 "sc_risk_hp1" => "1단 체력 경계(%, 원본 65 → 체력 66% 미만). 이 밑으로 내려가면 아래 sc_risk_dmg1 기준이 적용됩니다. -1=원본",
 "sc_risk_dmg1" => "1단 피해 기준(%, 원본 29). 체력이 1단 경계 밑일 때, 예상 피해가 이 비율을 넘으면 위험. ↓=더 일찍 겁먹음. -1=원본",
 "sc_risk_hp2" => "2단 체력 경계(%, 원본 40 → 체력 41% 미만). -1=원본",
 "sc_risk_dmg2" => "2단 피해 기준(%, 원본 17). 체력이 절반 아래일 때의 위험 민감도. -1=원본",
 "sc_risk_hp3" => "3단 체력 경계(%, 원본 25 → 체력 26% 미만). -1=원본",
 "sc_risk_dmg3" => "3단 피해 기준(%, 원본 10). 빈사 상태의 위험 민감도. 원본이 10이라 거의 모든 피해를 위험으로 봅니다. -1=원본",
 "sc_focus_cap" => "집중포화 보너스 상한(원본 80). 아군들이 이미 노리고 있는 대상에게 얼마나 더 끌릴지의 최대치. ↑=포커스가 강해짐(한 명에게 몰림) / ↓=각자 다른 대상. -1=원본",
 "sc_kill_cap" => "처치각 보너스 상한(원본 80). 결함 수정 완료 — 예전에는 상한 비교만 고치고 대입을 안 고쳐서, 올리면 *그 값 미만은 무제한 통과, 이상은 80으로 추락*하는 역전이 났습니다. 지금은 3쌍 6곳을 모두 잡습니다. ↑=마무리 각을 더 크게 침. -1=원본",
 "sc_kill_pct" => "부분 처치각 인정 기준(%, 원본 60). 가진 기술을 다 써서 대상 체력의 이 비율 이상을 깎을 수 있으면 부분 보너스를 줍니다. ↓=더 쉽게 킬각으로 인정. -1=원본",
 "sc_null_score" => "실익이 0일 때 대신 쓰는 점수(원본 −10). 아무 이득이 없는 공격을 약하게 억제합니다. 0으로 하면 억제가 사라지고, 더 낮추면 무의미한 공격을 강하게 피합니다. 음수 입력 가능. -1은 여기선 값으로 취급되니 원본을 쓰려면 −9999를 넣으세요",
 "sc_adv_lo" => "주변에 아군이 2명 이상 많을 때의 배율(%, 원본 30 = 0.3배). AI는 추적 계열 행동 점수에 '주변 적 수 − 아군 수'로 정해지는 배율을 곱합니다. 헷갈리기 쉬운데, 원본이 30(가장 낮음)인 쪽은 아군이 많을 때입니다 — 즉 아군이 많으면 굳이 한 명을 쫓아가지 않고, 적이 많을수록 추적 가치가 커집니다. 이 다섯 값(sc_adv_*)이 그 곡선 전체입니다",
 "sc_adv_m1" => "아군이 1명 많을 때의 배율(%, 원본 60 = 0.6배)",
 "sc_adv_0" => "주변 머릿수가 같을 때의 배율(%, 원본 80 = 0.8배). 원본이 100 미만이라 동수에서는 추적 가치가 한 단계 깎입니다. 100으로 올리면 5:5 상황에서 훨씬 끈질기게 쫓습니다 — 체감이 가장 큰 값 중 하나",
 "sc_adv_p1" => "적이 1명 많을 때의 배율(%, 원본 150 = 1.5배)",
 "sc_adv_hi" => "적이 2명 이상 많을 때의 배율(%, 원본 200 = 2.0배). 곡선의 상한. ↑=적이 몰려 있을수록 더 달라붙음 / ↓=적이 많으면 덜 쫓음",
 "sc_ally_radius" => "적을 세는 반경(유닛, 원본 150000). ⚠키 이름이 ally지만 실제로 세는 것은 적입니다(이름은 기존 설정 파일 호환 때문에 그대로 둡니다). 이 안의 '보이는 적' 수를 세어 위 배율을 정합니다. ↑=멀리 있는 적까지 머릿수에 넣음 / ↓=코앞의 적만 셈. ⚠제곱으로 저장되는 값이라 모드가 자동 변환합니다",
 "sc_enemy_radius" => "아군을 세는 반경(유닛, 원본 100000). ⚠키 이름이 enemy지만 실제로 세는 것은 아군입니다(자기 자신 포함, 시야 조건 없음). 원본이 적 반경(150000)보다 작으므로 AI는 아군보다 적을 더 넓게 셉니다 — 기본적으로 상황을 불리하게 보는 편향입니다",
 "mv0_adv_lo" => "도망 점수 — 아군이 2명 이상 많을 때의 배율(%, 원본 40). 도망·귀환에는 쫓아가기와 다른 배율표가 쓰입니다. 이쪽이 폭이 더 넓어(40~300) 머릿수에 훨씬 민감합니다",
 "mv0_adv_m1" => "도망 점수 — 아군이 1명 많을 때(%, 원본 75)",
 "mv0_adv_0" => "도망 점수 — 머릿수가 같을 때(%, 원본 100). 쫓아가기 쪽은 같은 상황에서 80인데 도망은 100이라, 대등하면 도망 쪽이 상대적으로 더 잘 뽑힙니다",
 "mv0_adv_p1" => "도망 점수 — 적이 1명 많을 때(%, 원본 200)",
 "mv0_adv_hi" => "도망 점수 — 적이 2명 이상 많을 때(%, 원본 300 = 3배). 다섯 값 중 가장 극단적입니다. ↓=포위당해도 안 빠짐 / ↑=조금만 몰려도 즉시 도주",
 "mv0_risk_shift" => "'위험이 줄어드는 만큼'을 얼마나 크게 볼지(원본 2 = ÷4). 작을수록 도망 점수가 커집니다 — 1로 내리면 2배, 3으로 올리면 절반. 도망 성향을 통째로 조절하는 가장 굵은 값입니다",
 "mv0_engage_shift" => "'맞을 것 같은 양·포탑 위험'을 얼마나 크게 볼지(원본 9 = ÷800). 위와 같은 방식이며 이쪽에만 머릿수 배율이 곱해집니다. 8로 내리면 2배. ⚠6 이하는 내부 계산과 충돌할 수 있으니 피하세요",
 "mv0_base_penalty" => "도망에 붙는 기본 감점(원본 −2). 다른 조건이 같으면 도망을 살짝 손해로 만들어 둡니다. 0으로 하면 그 억제가 사라집니다. 음수 입력 가능 — 원본으로 되돌리려면 −9999",
 "mv0_near_bonus" => "도주 목적지가 가까울 때의 가산점(원본 10). 붙는 조건이 까다로워(스킬 쿨·레벨 등) 체감은 작습니다",
 "mv0_near_gate" => "위 가산점을 무효로 만드는 임계(원본 950). 내부 판정값이 이 이상이면 보너스를 안 줍니다",
 "mv_tower_margin" => "포탑 사거리에서 빼는 여유분(원본 30000). AI는 '포탑이 대상을 때릴 수 있는가'를 볼 때 실제 사거리에서 이만큼을 깎아서 봅니다. ↑=포탑 지원·위협을 더 좁게 인정(포탑을 덜 의식) / ↓=넓게 인정(포탑 근처를 더 신경 씀)",
 "mv_tower_cap" => "포탑 지원·위협 점수의 상한(원본 100). 아군 포탑이 지켜줄 때의 가산점과 적 포탑에 노출될 때의 감점 둘 다 이 값에서 잘립니다. ↑=포탑 유무가 판단을 더 크게 좌우",
 "mv2_gain_shift" => "접근 행동의 목표 이득을 얼마나 반영할지(원본 7 = ÷200). 작을수록 크게 반영 — 6이면 2배입니다. 접근(자리 잡기) 계열의 적극성을 조절합니다",
 "mv_engage_thr" => "'맞을 것 같은 양'을 셀 때 포함할 위협의 상한(원본 9999 = 사실상 전부 포함). 내리면 예상 피격량이 줄어 도망·추적 점수가 함께 낮아집니다",
 "vis_mem_global" => "적을 마지막으로 본 뒤 그 위치를 기억하는 시간(틱, 원본 120). ↑=사라진 적을 오래 경계. -1=원본. 0.5.4부터 판단 전체에 공통 적용됩니다(1곳).",
 "ld_around_range" => "라인에서 목표물에 얼마나 가까이 붙을지(원본 80000). 타워·미니언·아군에게 접근할 때 '이 거리까지만' 다가갑니다. ↓=바짝 붙음(공격적, 위험) / ↑=멀찍이 서성임. 7군데에 같은 값이 쓰여 라인전 거리감 전체를 바꿉니다",
 "ld_around_delay" => "접근 액션을 유지하는 시간(틱, 원본 5, 1곳 동시). 에 1곳→1곳으로 보정 — ld_around_range 1사이트와 짝입니다. ↑=한 번 정한 접근 목표를 더 오래 유지. -1=원본",
 "ld_mode_mask" => "자리 평가에 '특수 목적'을 주는 경기 페이즈 비트마스크(원본 417 = 페이즈 0·5·7·8, 4곳 동시). ⚠게임 모드가 아니라 경기 진행 페이즈(0~8)입니다 — 게임 모드는 세 값뿐인데 이 마스크는 8번 비트까지 씁니다. lt_phase_mask·pl_serpen_phase_mask와 같은 필드입니다. 에 4곳→4곳 보정. -1=원본",
 "ld_move_pct" => "이동거리 예측식의 기준 퍼센트(원본 100, 1곳 동시). 에 1곳→1곳으로 보정. ↑=적이 더 멀리 올 수 있다고 보아 위협 반경이 커집니다. -1=원본",
 "ld_threat_state" => "위협 스캔이 반응하는 대상 상태값(원본 13, 2곳 동시). 에 2곳→2곳으로 보정. 이 상태인 적만 위협 후보로 삼습니다. 통과 뒤 세부 상태로 한 번 더 갈리는데 그 의미는 미확정입니다. -1=원본",
 "ld_rand_min" => "무작위 대체가 발동할 최소 후보 수(원본 2). 판단력 굴림에 실패하면 후보를 다 버리고 하나만 무작위로 남기는데, 후보가 이 수 이상일 때만 그렇게 합니다. 3 이상으로 올리면 그 사고가 확 줄어듭니다",
 "dm_lookahead" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. 데스매치에서 '곧 닿는다'고 보는 선행 틱(원본 30). 사거리 판정에 선행틱 × 속도만큼 여유를 줍니다. ↑=아직 멀어도 달려듦 / ↓=확실히 사거리에 들어와야 움직임",
 "dm_ult_lookahead" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. 궁 경로의 선행 틱(원본 60). 위와 같은 방식이며 궁에만 두 배가 적용됩니다",
 "dm_near_ally" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. '곁에 아군이 있다'고 보는 거리(원본 150000). 몇몇 스킬 사용 조건이 이 판정을 요구합니다. ↑=멀리 있는 아군도 같이 싸운다고 계산 → 적극적",
 "dm_near_enemy" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. '교전 중이다'라고 보는 거리(원본 150000). ↑=멀리 있는 적도 교전으로 인식",
 "dm_execute_hp" => "처형 사정권 체력%(원본 20). 보정 — 예전에는 64비트 나눗셈 경로만 잡아서 통상 상황(32비트)에서는 값이 안 먹었습니다. ⚠이 값이 속한 결사전 판단은 일반 경기에서 발화하지 않습니다. -1=원본",
 "dm_lasthit" => "미니언 막타로 인정하는 타격 수(원본 2 = 3방 이내). 보정(32비트 경로 추가). ⚠일반 경기에서 발화하지 않는 판단입니다. -1=원본",
 "dm_skill_hp" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. 스킬을 조건 없이 허용하는 대상 체력(%, 원본 79). 대상이 이보다 건강하면 '사거리 안에 다친 아군이 있는가' 같은 추가 조건을 통과해야 합니다. ↑=풀피 적에게도 스킬을 난사",
 "dm_ult_rally" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. 궁을 쓰려면 대상이 팀 지정 지점에서 얼마나 가까워야 하는지(원본 6000 — 사실상 '바로 그 자리'). 올리면 궁을 훨씬 자유롭게 씁니다",
 "dm_ult_rally2" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. 맵 전역을 노리는 궁의 완화된 기준(원본 90000)",
 "dm_ult_range" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. ⚠궁 총사거리 임계(원본 150000). 이하면 대상 후보를 A 방식으로, 넘으면 B 방식으로 모읍니다 — 바꾸면 대상 선정 방식 자체가 바뀌므로 신중히",
 "dm_ult_mask_rally" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. 위 '지정 지점 근처' 요구를 적용할 팀 작전 묶음(원본 111). 0으로 두면 모든 상황에서 요구가 사라져 궁을 남발합니다",
 "dm_ult_mask_focus" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. 팀이 지목한 대상에게 예외를 주는 작전 묶음(원본 78)",
 "dm_ult_mask_safe" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. 궁에도 안전 판정을 강제하는 작전 묶음(원본 33). 해당 작전에서의 신중함을 조절합니다",
 "dm_skill2_level" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. 스킬2 해금 레벨(원본 3). 낮추면 더 이른 시점부터 스킬2를 후보에 올립니다",
 "dm_ult_level" => "⛔일반 경기에서는 작동하지 않습니다(결사전 판단이 정규전에서 발화하지 않습니다). 데스매치 모드 전용 — 일반 경기에서는 발동하지 않습니다. 궁 해금 레벨(원본 5)",
 "sf_margin" => "데스매치·일반 경기 공통. '때려도 되나' 판정의 겁 많음(원본 15000). 적 사거리를 실제보다 이만큼 넓게 잡아 반격 위험을 계산합니다. ↓=겁을 덜 냄(과감) / ↑=더 몸을 사림. 라인전 단계의 과감함을 직접 좌우하는 값입니다",
 "sf_radius" => "안전 판정에서 주변 머릿수를 세는 반경(원본 120000, 5곳 동시). 에 3곳→5곳으로 보정. 거의 모든 판단이 공유하는 안전 판정이라 파급이 넓습니다. -1=원본",
 "sf_mem" => "데스매치·일반 경기 공통. 안전 판정에서 안 보이는 적을 '아직 거기 있다'고 세는 시간(틱, 원본 120)",
 "pe_collect_radius" => "주변 인물을 위험·이득 계산에 넣는 반경(원본 200000). 이 안의 적과 아군만 자리 평가에 반영됩니다. 가장 바깥쪽 그물이라, 좁히면 AI가 먼 상황을 아예 못 보고 넓히면 계산량이 늘어납니다",
 "pe_champ_threat" => "적 챔피언을 실제로 무서워하기 시작하는 거리(원본 100000). 위 반경 안에 있어도 이 거리를 넘으면 위험으로 안 칩니다. ↑=멀리서부터 겁을 냄(소극적) / ↓=코앞까지 와야 위험 인식(과감) — 자리 판단에서 체감이 가장 큰 값 중 하나입니다",
 "pe_minion_add" => "미니언·중립을 위험으로 세기 시작하는 거리(원본 64000). ⚠단 이 계산은 라인전 계열에서만 돌아갑니다 — 도망·추적·전투 판단은 미니언 위험을 아예 안 봅니다. ↑=미니언 근처를 더 피함(라인 유지가 소극적)",
 "pe_filter_radius" => "구조물·미니언 후보를 훑는 반경(원본 150000). 위험 계산 대상을 모으는 1차 그물입니다",
 "pe_near_cut" => "근접 판정 컷(원본 70000). 구조물·미니언 목록을 훑을 때의 내부 경계입니다",
 "pe_field_radius" => "장판·지속 효과를 훑는 반경(원본 250000). 다른 값보다 넓은 이유는 장판이 크기 때문입니다",
 "pe_count_radius" => "고립 판정에 쓰는 머릿수 반경(원본 120000). 이 안에 아군이 0명이고 적이 2명 이상이면 위험에 큰 가산이 붙습니다(적 위협 총합 × 인원수 × 0.25). ↑=넓게 보므로 고립 판정이 잘 안 뜸 / ↓=조금만 떨어져도 고립으로 침",
 "pe_reach_bonus" => "스킬이 '닿는다'고 보는 여유 거리(원본 80000, 21군데). 내 공격 가치와 적 위협 양쪽에 쓰입니다. ↑=더 멀리서도 닿는다고 판단해 적극적이지만 위험 인식도 함께 커집니다",
 "pe_skillshot_width" => "적이 쓰는 스킬 궤적을 피해야 할 폭(원본 20000). 이 안에 들어가면 그 자리는 '스킬샷 위험 지대'로 표시됩니다. ↑=스킬을 크게 돌아 피함 / ↓=아슬아슬하게 지나감",
 "pe_bodyblock_width" => "아군이 대신 맞아준다고 인정하는 폭(원본 28000). 적 스킬 선상에 아군이 이만큼 안에 있으면 내 위험이 절반이 됩니다. ↑=아군 뒤에 숨기 쉬워짐",
 "pe_outer_band" => "투사체·중립의 외곽 감쇠 띠(원본 32000). 사거리 밖이어도 이 띠 안이면 위험을 절반~1/3로 셉니다. ↑=여유 있게 피함",
 "pe_tower_margin" => "타워·미니언 사거리에 더하는 여유(원본 18000). ↑=포탑을 더 멀리서부터 의식",
 "pe_source_cap" => "위협 하나가 낼 수 있는 위험의 상한(원본 150 = 내 체력의 150%). 22군데에 쌍으로 박혀 있습니다. ↑=강한 적 한 명이 판단을 독점 / ↓=여러 위협이 고르게 반영",
 "pe_predict_cap" => "예상 피격량의 상한(원본 140). 위와 같은 %체력 단위입니다",
 "pe_tower_far" => "원거리 타워 기여 계수(원본 656). 사거리 밖 타워를 얼마나 신경 쓸지입니다. ↑=먼 타워도 무서워함",
 "pe_kind_scale" => "상황별 배율(원본 120 = ×1.2). 어떤 판단에서는 위험을, 어떤 판단에서는 이득을 1.2배로 키웁니다",
 "pe_wall_risk" => "지형 벽 칸의 위험값(원본 9999 = 사실상 무한대). 갈 수 없는 칸을 나타냅니다 — 건드리면 벽을 통과하려 들 수 있으니 권장하지 않습니다",
 "pe_well_risk" => "적 본진(우물) 안의 위험값(원본 9999). 낮추면 적 본진으로 들어가려 합니다 — 실험용",
 "pe_ally_gain_cut" => "아군 스킬을 이득으로 셀 때의 컷(원본 1200)",
 "pe_state_gate" => "특정 상태 게이트(원본 180). 이득 계산의 슬롯별 조건에 쓰이며, 추적에서도 '버틸 만하다'는 감산이 걸립니다",
 "pe_noise_exempt" => "**위치 판단 노이즈 면제선** — 포지셔닝 스탯이 이 값 이상인 선수는 위험도 계산이 흔들리지 않습니다. **기본 100000**. ⚠단위는 옛날 그대로입니다 — 모드가 내부에서 1000으로 나눕니다. `100000`을 넣어야 원본이고, `100`을 넣으면 0이 되어 **전원 면제 = 노이즈가 통째로 꺼집니다**. 유효 입력 0~127000(1000 단위). ⚠판단력이 아니라 **포지셔닝** 스탯입니다(08-05 정정). 2곳 동시 패치. -1=원본",
 "pe_noise_amp" => "자리 판단 흔들림의 최대 폭(원본 2000). 판단력이 낮을수록 이 폭에 가까워집니다. 0으로 하면 흔들림이 사라져 모든 선수가 위험을 정확히 봅니다. ⚠단 포탑 위험에는 원래 흔들림이 안 걸립니다 — 확정된 위협은 누구나 정확히 본다는 설계입니다",
 "pe_noise_amp_mode2" => "⛔일반 경기에서는 작동하지 않습니다(게임 모드가 2일 때만 쓰이는 값). 흔들림 폭은 pe_noise_amp를 쓰세요. 특정 모드에서의 흔들림 폭(원본 1000). 위와 같은 장치의 다른 경로입니다",
 "ldsc_lost_target" => "대상이 사라진 공격 후보에 주는 감점(원본 −99999). 라인 수비에서 후보가 잘려 나가는 가장 흔한 이유가 이것입니다. 절댓값을 줄이면(예: −20) 대상이 사라진 후보도 살아남습니다. 음수 입력 가능 — 원본으로 되돌리려면 −9999",
 "ldsc_skill_factor" => "스킬 가치 계수 기저(원본 100 = 100%). 라인 수비에서 공격 후보의 가치 전체를 스케일합니다",
 "ldsc_vision_mem" => "이 판정 안에서 적을 기억하는 시간(틱, 원본 120). 공유 헬퍼 쪽은 전역 영향이라 제외하고 3군데만 노출했습니다",
 "ldsc_early_mask" => "조기 반환 태그 묶음(원본 129123 — 0.5.4에서 비트 9가 추가돼 128611에서 바뀌었습니다). 이 묶음에 든 행동만 정식 점수 계산을 거치고, 나머지(접근·미니언 자리 등)는 다른 함수 값이 그대로 최종 점수가 됩니다. 비트를 세우면 그 행동을 정식 경로로 되돌릴 수 있습니다 — 파급이 크니 실험용",
 "sc_near_bonus" => "근접 보너스(원본 10). 대상이 적 인식 반경 안일 때 점수에 더해집니다. ↑=가까운 대상을 더 선호",
 "sc_obj_bonus" => "오브젝트 확인 판단 보너스(원본 10). 에픽·세르펜을 '확인하러 가는' 판단에만 붙습니다. ↑=오브젝트를 더 자주 보러 감",
 "sc_keep_thr" => "라인 수비에서 후보를 살려둘 점수 하한(원본 −30). 이보다 낮은 점수의 행동은 후보에서 탈락합니다. ↑(예 0)=엄격해져 어중간한 행동이 사라짐 / ↓(예 −100)=거의 다 살아남아 선택지가 넓어짐. 음수 입력 가능",
 "lw_wait_dist" => "대기와 전진을 가르는 거리(유닛, 원본 180000). 적이 기준점에서 이 거리 안이면 그 자리에서 대기하고, 밖이면 경로를 따라 앞으로 나가 대기합니다. ↑=더 멀리 있는 적에게도 제자리 대기(소극적) / ↓=자꾸 앞으로 나감. ⚠제곱 인코딩이라 모드가 자동 변환합니다",
 "lw_back" => "대기 지점을 정할 때 물러날 거리(유닛, 원본 180000). **경로를 따라간 누적 길이에서 이만큼 뺀 지점**에 섭니다(0 미만이면 0). ↑=더 뒤에서 대기(안전) / ↓=바짝 붙어 대기. ⚠기준 대상이 적인지 아군인지는 아직 미확정입니다. 옛 이름 ex_wait_back 도 같은 값으로 동작합니다",
 "lw_radius" => "대기 중 배회 반경(유닛, 원본 80000, 1곳 동시). 대기 지점 주변을 이 반경 안에서 돌아다닙니다. ↓=제자리에 가깝게 고정(왕복 완화 후보) / ↑=넓게 배회",
 "ls_radius" => "라인 안전 상태의 배회 반경(유닛, 원본 80000, 1곳 동시). 물러나 있을 때 얼마나 넓게 움직일지. ↓=한자리에 머무름 / ↑=넓게 움직임",
 "mv_bush_arrive" => "수풀 도착 판정 반경(유닛, 원본 16000). 목표 수풀에 이 거리까지 오면 '도착'으로 보고 이동을 끝냅니다. ↑=덜 정확하게 도착(수풀 밖에서 멈출 수 있음) / ↓=정확히 들어감. ⚠제곱 인코딩 자동 변환",
 "mv_hide_near" => "은신 이동의 근접 판정 거리(유닛, 원본 12000). ↑=더 멀리서도 근접으로 판단. ⚠제곱 인코딩 자동 변환",
 "mv_trace_dist" => "추격 거리 임계(유닛, 원본 120000). 추격 행동이 이 거리를 기준으로 갈립니다. ↑=더 멀리까지 쫓음 / ↓=금방 포기. ⚠제곱 인코딩 자동 변환",
 "cs_lead_attack" => "기본공격을 시작하는 선행 예측 틱(원본 30, 2곳 동시). 사거리 판정이 실제거리 ≤ 사거리 + 예측틱×접근속도라, 이 값이 클수록 아직 사거리 밖인 적에게도 미리 달려들어 공격을 겁니다. ↓=사거리에 확실히 들어와야 반응(신중). -1=원본",
 "cs_lead_skill" => "스킬을 시작하는 선행 예측 틱(원본 30). ↑=더 먼 거리에서 스킬 후보를 올림(적극적) / ↓=확실히 닿을 때만. -1=원본",
 "cs_lead_skill2" => "스킬2를 시작하는 선행 예측 틱(원본 30). 위와 동일. -1=원본",
 "cs_lead_ult" => "궁을 시작하는 선행 예측 틱(원본 60, 2곳 동시 — 적 대상/아군 대상). 원본부터 다른 행동의 2배라 궁은 원래 멀리서도 노립니다. ↑=더 멀리서 궁 시도. 상한 127. -1=원본",
 "cs_lead_steal" => "막타·스틸을 노릴 때의 선행 예측 틱(원본 30). 중립 몬스터·처치 직전 적을 향해 미리 움직이는 거리에 영향. ↑=더 멀리서 스틸 시도. -1=원본",
 "cs_ult_range" => "궁 사용 허용 반경(유닛, 원본 6,000). 궁 후보는 팀이 지정한 지점 근처에 있어야만 올라가는데, 원본 6,000은 사실상 '바로 그 자리'라 궁이 잘 안 나갑니다. 올리면 궁을 훨씬 자유롭게 씁니다(예: 60000 = 한 라인 규모). ⚠제곱으로 저장되는 값이라 모드가 자동 변환합니다 — 그냥 원하는 거리를 넣으세요. -1=원본",
 "cs_ult_range_global" => "맵 전역형 궁의 허용 반경(유닛, 원본 90,000 ≈ 맵 전체). 일부 궁만 이 값을 씁니다. 낮추면 전역 궁도 근처에서만 쓰게 됩니다. -1=원본",
 "cs_ult_mode_mask" => "궁의 '지정 지점 근처' 요구를 적용할 팀 작전 비트마스크(원본 0x6f=111 → 넥서스 공격 작전만 면제). 0으로 두면 모든 상황에서 근접 요구가 사라져 궁을 조건 없이 씁니다(가장 효과 큰 한 방, 단 남발 주의). -1=원본",
 "cs_steal_hp" => "중립 몬스터 막타를 노리는 체력%(원본 20). 몬스터 체력이 이 값 이하면 평타·스킬로 스틸을 시도합니다. ↑=더 건강한 몬스터에게도 달려듦(오브젝트 다툼 적극적) / ↓=거의 안 노림. 상한 127. -1=원본",
 "cs_unit_hits" => "적 유닛(미니언 등)을 때릴 판단 기준(원본 2). 대상 체력 ÷ 내 예상 피해 ≤ 이 값일 때만 공격 후보가 됩니다 — 원본 2는 '3방 안에 죽일 수 있으면'. ↑=더 단단한 유닛도 공격(라인 정리 적극적) / ↓=거의 안 때림. -1=원본",
 "cs_minion_vision" => "미니언을 '아직 거기 있다'고 믿는 시간(틱, 원본 120, 상한 127). 시야에서 사라진 미니언도 이 시간 동안은 공격 후보로 유지합니다. -1=원본",
 "cs_ally_hp" => "아군 지원 스킬의 체력 경계(%, 원본 79, 1곳 동시). 아군 체력이 이 값 이하일 때와 이상일 때 판단 경로가 갈립니다. ↑=더 건강한 아군에게도 지원 스킬을 씀. -1=원본",
 "cs_ally_radius" => "아군 지원 판단의 밀집 반경(유닛, 원본 120,000, 8곳 동시). 이 반경 안의 아군·적 수를 세어 지원 스킬을 쓸지 정합니다. ↑=더 넓게 보고 지원 / ↓=바로 옆 아군만. ⚠제곱 인코딩이라 모드가 자동 변환합니다. -1=원본",
 "cs_cc_mask" => "'아직 움직일 수 있는' 상태이상 비트마스크(원본 0x3B8, 4곳 동시). 이 마스크에 없는 상태이상에 걸린 적은 '못 움직인다'고 보고, 상대 속도 대신 내 전속력으로 거리를 계산합니다. 0으로 두면 모든 군중제어를 이동불가로 취급 → 추격·진입이 공격적이 됩니다. -1=원본",
 "ex_skill2_level" => "스킬2가 열리는 레벨(원본 3). 이 값은 게임 데이터 파일이 아니라 코드에 박혀 있어 여기서만 바꿀 수 있습니다. ↑=늦게 열림(성장 곡선이 느려짐). ⚠낮추는 것은 위험합니다 — 스킬2가 없는 챔피언(일부 3슬롯 챔프)에서 게임이 죽을 수 있습니다. -1=원본 권장",
 "ex_ult_level" => "궁이 열리는 레벨(원본 5). 위와 같이 코드 하드코딩 값입니다. ↑=궁이 늦게 나와 초중반 교전 양상이 크게 바뀜. ⚠낮추는 것은 위험(슬롯이 비면 게임이 죽을 수 있음). -1=원본 권장",
 "ex_attack_margin" => "기본공격 접근 여유(유닛, 원본 15,000). 대상에게 다가갈 때 사거리 − 이 값까지 붙고 멈춥니다. ↑=더 바짝 붙음(사거리 이점을 못 살림) / ↓=사거리 끝에서 정지(카이팅 성향). -1=원본",
 "ex_attack_margin_sp" => "특수 대상용 접근 여유(유닛, 원본 2,000). 구조물 등 일부 대상에 적용되는 별도 값. ↑=더 바짝 붙음. -1=원본",
 "ex_attack_seek" => "평타 사거리 배율의 절편(원본 100). 실제 배율(%) = 이 값 + 대상 탐색 스탯이고 최소 1로 보정됩니다. ⚠콜리 내부에서 정말 %로 곱해지는지는 미확인(추정). -1=원본",
 "ex_fail_min_ticks" => "실패한 행동을 기록할 최소 지속 틱(원본 119). 이보다 짧게 끝난 행동은 '실패'로 기록하지 않습니다. 기록된 행동은 한동안 다시 안 고릅니다. ↓=실패를 더 자주 기록해 같은 행동 반복을 억제(왕복 완화 후보) / ↑=억제 약해짐. 상한 127. -1=원본",
 "ex_think_min" => "재판단 간격의 하한 base(원본 400). 선수는 매 틱 판단하지 않고 (이 값 + 3×판단력)÷100 틱마다 한 번 생각합니다(원본 기준 약 4~7틱). ↑=반응이 느려짐(멍해짐) / ↓=자주 판단(민감하지만 연산 부담↑). -1=원본",
 "ex_think_max" => "판단 간격의 상한 계수(원본 800). 실제 간격 = (이 값 + 4×(100−판단력)) ÷ 100 틱. 최종 주기는 하한~상한 사이 무작위라 원본 기준 4~12틱입니다. ⚠판단력이 높을수록 간격이 짧아집니다(구 설명의 '+4×판단력'은 항이 반대였습니다). -1=원본",
 "au_noise_off" => "행동 성향 흔들림 끄기(1=끔 / -1=원본). 게임은 행동 부류 11개(이동·교전·귀환·스킬 4종 등)의 가중치를 판단이 바뀔 때마다 부류별로 주사위를 굴려 새로 정합니다 — 판단력이 낮을수록 폭이 커져(최대 ±45%) 같은 상황에서도 성향이 제멋대로 바뀝니다. 1로 두면 주사위를 없애 모든 선수가 판단력과 무관하게 일관된 성향이 됩니다. 우왕좌왕·부쉬 왕복 완화 1순위 후보. ⚠판단력 스탯의 체감 차이가 크게 줄어듭니다",
 "au_noise_amp" => "흔들림 진폭 상수(원본 900). 흔들림폭 = (이 값 − 9×판단력) ÷ 2. ↑=판단력이 높아도 성향이 흔들림(무작위성 증가). ⚠900 미만 금지 — 계산이 음수로 넘어가 점수 체계가 깨집니다(모드가 900으로 자동 보정). 흔들림을 줄이려면 이 값이 아니라 au_noise_off를 쓰세요. -1=원본",
 "au_score_center" => "행동 점수 가중치 중심(원본 1000 = ×1.000). 올리면 모든 행동 점수가 일괄 증폭됩니다(부류 간 상대비는 유지). 다른 층에서 점수 임계와 비교하는 지점이 있어, 크게 올리면 '무엇이든 일단 행동' 쪽으로 기웁니다. -1=원본",
 "bt_hp_flee" => "교전 판단의 후퇴/추격 체력 임계(%, 원본 21). 대상 체력이 이 값 이상일 때만 원거리 추격 오더가 나갑니다. ↑=더 건강한 적에게도 추격 판단 / ↓=거의 안 나감. -1=원본",
 "bt_hp_gate" => "도주 판단의 우물 귀환 체력 임계(%, 원본 41). 내 체력이 이 값 이상이면 아군 수를 2명 더 있는 셈 쳐서 버팁니다(적 수 ≥ 보정 아군 수면 귀환). ↑=더 일찍 물러남 / ↓=더 오래 버팀. -1=원본",
 "bt_chase_stop" => "교전 시 접근 정지 반경(유닛, 원본 15000, 2곳 동시). 적에게 이만큼 다가가면 멈춥니다. ↑=멀리서 멈춤(소극적) / ↓=끝까지 붙음. ⚠2곳 중 한 곳은 정지 반경이 아니라 '적이 나에게 닿는가' 판정에 더하는 여유라, 올리면 적을 더 일찍 위협으로 세기도 합니다. -1=원본",
 "bt_chase_keep" => "교전 추격 유지 거리(유닛, 원본 80000, 1곳 동시). 이 거리까지는 대상을 계속 쫓습니다. ↑=끈질기게 물고 늘어짐 / ↓=금방 포기(다이브 억제). -1=원본",
 "bt_vision_mem" => "교전 판단의 '마지막으로 본 적' 유효 시간(틱, 원본 120, 1곳 동시·상한 127). 시야에서 사라진 적을 이 시간 동안은 그 자리에 있다고 간주합니다. ↑=기억이 오래감(유령 추격 가능) / ↓=금방 잊음. -1=원본",
 "ld_chase_stop" => "라인 수비 2차 평가의 접근 정지 반경(유닛, 원본 15000, 1곳 동시). 위 bt_chase_stop의 라인전 판. ↑=소극적. -1=원본",
 "ld_ally_near" => "아군 '근접' 판정 거리(유닛, 원본 160000, 5곳 동시). 아군 4명 각각이 이 거리 안이면 '붙어있다'고 세어, 그 수를 적 수와 비교해 교전 여부를 정합니다. ↑=아군이 멀어도 함께 있다고 판단 → 더 공격적 / ↓=혼자라고 느껴 소극적. -1=원본",
 "ld_intervene" => "개입 최소 거리(유닛, 원본 50000). 기준 유닛과 이 거리 이상 떨어져 있을 때만 특수 추적 경로로 갑니다. ↑=개입 판정 범위가 넓어짐. -1=원본",
 "ld_vision_mem" => "라인 수비의 '마지막으로 본 적' 유효 시간(틱, 원본 120, 상한 127). ↑=기억 오래감 / ↓=금방 잊음. -1=원본",
 "ld_est_base" => "AI 추정 오차 하한(원본 10). AI는 사거리·피해를 어림잡을 때 무작위 오차를 섞는데, 이 값이 그 정확도 바닥입니다. ↑=AI의 거리·피해 추정이 정확해져 헛발질이 줄어듭니다(상한 127). ↓=더 자주 잘못 어림잡음. -1=원본",
 "tm_cancel_mask" => "자동 취소 대상 팀 작전 비트마스크(원본 0xb00=2816 → 갱·갱다이브·컴백픽). 여기 포함된 작전은 조건이 어긋나면 팀이 스스로 취소합니다. 비트를 더하면 그 작전도 자동 취소 대상이 되고, 빼면 한번 잡은 작전을 끝까지 밀어붙입니다. 1곳 동시. -1=원본",
 "path_orth_cost" => "상하좌우 한 칸 이동 비용(원본 640). 위험 회피 비용과의 <b>상대 크기</b>가 경로를 정합니다 — 이걸 올리면 위험을 감수하고 지름길로 가고, 내리면 멀리 돌아갑니다. ⚠640보다 <b>낮추면</b> 최단경로 보장이 깨집니다(크래시 아님). 올리는 쪽은 안전. <b>76곳 동시</b>. -1=원본",
 "path_diag_cost" => "대각선 한 칸 이동 비용(원본 896 ≈ 640×√2). 올리면 대각선을 꺼려 <b>계단식(맨해튼)</b>으로 움직이고, 640까지 내리면 대각선이 직교와 같은 값이라 매우 자유롭게 씁니다. ⚠896보다 낮추면 최단경로 보장이 깨집니다. <b>20곳 동시</b>. -1=원본",
 "path_greedy" => "경로 탐색의 <b>탐욕도</b>(원본 7 = 휴리스틱 ×128). 올리면 목적지 쪽으로 밀어붙여 빠르지만 경로가 나빠지고, <b>0이면 완전탐색</b>이라 최적 경로 대신 CPU를 많이 씁니다(시뮬 속도 저하 주의). 0~9. <b>54곳 동시</b>. -1=원본",
 "path_threat_floor" => "타워 등 <b>위험지대 위를 지날 때 최소 몇 칸 돌아갈지</b>(원본 2). 0으로 두면 위험을 거의 무시하고 직진합니다. 0~60. -1=원본",
 "path_threat_cap" => "위험 회피 우회의 <b>상한 칸 수</b>(원본 60). 아무리 위험해도 이보다 더 돌지는 않습니다. <b>2곳 동시</b>(비교문과 대입문). 0~127. -1=원본",
 "path_threat_scale" => "체력 대비 받을 피해에 얼마나 <b>민감</b>할지(원본 30). 우회 칸 수 = 하한 + 이 값 × (한 칸에서 받을 피해 ÷ 체력). 0이면 피해량과 무관하게 늘 하한만 돕니다. <b>2곳 동시</b>. -1=원본",
 "path_threat_default" => "위험원을 찾지 못했을 때 쓰는 <b>기본 우회 칸 수</b>(원본 2). <b>2곳 동시</b>. 0~60. -1=원본",
 "path_danger_cost" => "<b>미니언 웨이브가 죽는 자리</b>를 얼마나 피할지(원본 1281 = 2칸 우회×640 + 1). 0이면 그 자리를 신경 쓰지 않습니다. <b>50곳 동시</b>. -1=원본",
 "path_wave_risk_ret" => "위협원 <b>안에 서 있을 때</b> 매기는 위험등급(원본 3 = 최고). 낮추면 위험지대 한복판에서도 태연해집니다. -1=원본",
 "auc_flee_version_gate" => "경매 중 강제 귀환의 <b>주 스위치</b>(원본 1). 게임 내부의 AI 사양 버전이 이 값보다 커야 발동합니다. 그 버전의 실제 값은 아직 확인되지 않아, 지금 이 판단이 켜져 있는지 자체가 미확인입니다. <b>0으로 낮추면 버전과 무관하게 항상 켜집니다.</b> 크게 올리면 완전히 잠급니다. 0~127. -1=원본",
 "auc_flee_undying_gate" => "불사 상태 특례 판정값(원본 0). 불사면 거리 계산을 건너뛰고 '기지 근처'로 칩니다. ⚠발동 여부에는 영향이 없고 기록용 분기에만 쓰입니다 — 사실상 만질 일이 없습니다. -1=원본",
 "auc_flee_hp_field" => "'도망 도중 맞을 피해'를 <b>무엇과 비교할지</b>(원본 1624 = 현재 체력). 1552로 바꾸면 최대 체력과 비교하므로 훨씬 드물게 발동합니다. ⚠값이 아니라 읽을 자리를 바꾸는 노브라, 이 두 값 외에는 넣지 마세요. -1=원본",
 "auc_flee_nexus_mask" => "'우리 넥서스가 실제로 맞는 중이면 도망 취소' 조건(원본 256). 0이면 취소 조항이 사라져 넥서스가 깨지는 중이어도 물러납니다. 65537로 넓히면 더 자주 취소됩니다. -1=원본",
 "auc_flee_goal_far" => "도망 목적지의 먼 쪽 좌표(원본 928000). 맵 한 변이 약 960000이라 사실상 맵 끝, 즉 기지 코너입니다. 줄이면 덜 깊이 물러납니다. -1=원본",
 "auc_flee_goal_near_a" => "도망 목적지의 가까운 쪽 좌표(원본 32000). ⚠<b>`auc_flee_goal_near_b`와 반드시 같은 값으로 바꾸세요</b> — 한쪽만 바꾸면 팀 사이드에 따라 목적지가 달라집니다. -1=원본",
 "auc_flee_goal_near_b" => "위와 같은 값의 반대 팀 사이드용 사본(원본 32000). ⚠<b>`auc_flee_goal_near_a`와 항상 같이</b>. -1=원본",
 "auc_flee_end_delay" => "기지 코너에 도착한 뒤 이 명령을 몇 틱 더 붙잡고 있을지(원본 5). 크게 하면 더 오래 웅크리고, 0이면 도착 즉시 다음 판단으로 넘어갑니다. -1=원본",
 "auc_flee_pathfinder" => "경로탐색 사용 여부(원본 2 = 사용 안 함). ⚠<b>바꾸지 마세요</b> — 2 외의 값은 초기화되지 않은 경로탐색 자료를 읽습니다. -1=원본",
 "auc_flee_with_skill" => "도망치는 동안 무엇을 허용할지(원본 1 = 스킬만). 바이트 단위로 스킬·궁·궤적 회피·목표 확정이 켜집니다. 0=아무것도 안 씀, 257=스킬+궁. -1=원본",
 "auc_flee_score" => "이 강제 귀환에 매기는 점수(원본 99999 = 사실상 무조건 1위라 다른 판단을 전부 이깁니다). 낮추면 다른 판단과 <b>경쟁</b>하게 되어, 정말 급할 때만 채택되게 만들 수 있습니다. -1=원본",
 "auc_flee_action_tag" => "강제 귀환이 실제로 내리는 행동(원본 3 = 도주). ⚠<b>태그만 바꾸면 딸려가는 값들의 자리가 안 맞아 오작동합니다</b> — 3 외에는 권장하지 않습니다. -1=원본",
 "an_tower_gate" => "넥서스를 직접 치러 갈 수 있는 <b>적 타워 잔여 수</b>(원본 0 = 전부 밀어야 넥서스로 갑니다). ⚠비교가 '정확히 N개'라 값을 올리면 '딱 그 수일 때만' 넥서스로 갑니다. -1=원본",
 "an_attack_sub" => "넥서스를 칠 때 실제로 수행할 하위 판단(원본 18). -1=원본",
 "an_home_wait" => "넥서스로 갈 조건이 안 됐고 분수에서 대기할 때의 하위 판단(원본 7 = 귀환). -1=원본",
 "an_fallback" => "타워가 남아 넥서스로 못 갈 때 대신 할 판단(원본 2 = 라인 방어). 값을 바꾸면 마무리 국면의 성향이 통째로 바뀝니다. -1=원본",
 "an_fallback_wave" => "위 폴백에서의 미니언 웨이브 성향(원본 2, <b>추정</b> = 밀기). -1=원본",
 "an_fallback_style" => "위 폴백의 스타일 바이트(원본 0). 의미 미확정. -1=원본",
 "pe_kind_mask" => "자리 판단에서 <b>어떤 종류의 이득을 셈에 넣을지</b> 고르는 비트마스크. 비트를 빼면 그 종류(예: 구조물 압박, 아군 보호)를 자리 선택에서 아예 무시합니다. ⚠종류별 비트 대응은 아직 확정되지 않았습니다 — 한 비트씩 바꿔가며 확인하세요. (원본 771) -1=원본",
 "pe_mode_mask" => "자리 판단을 <b>어떤 팀 작전에서 적용할지</b> 고르는 비트마스크. 비트를 빼면 그 작전 중에는 자리 재평가를 하지 않고 원래 위치를 지킵니다. ⚠작전별 비트 대응 미확정. (원본 417) -1=원본",
 "d19_ally_hp" => "아군 넥서스가 <b>이 체력% 아래</b>로 떨어져야 위기로 보고 지원을 갑니다(원본 값은 편집기 기본 -1 = 원본). 올리면 조금만 깎여도 우르르 수비하러 오고, 내리면 거의 끝장날 때까지 각자 할 일을 합니다. -1=원본",
 "d19_sev_hp_1" => "넥서스 방어 <b>위험도 1단계</b>로 올리는 체력% 경계(원본 66). 아래 2·3단계와 함께 사다리를 이룹니다. -1=원본",
 "d19_sev_hp_2" => "넥서스 방어 <b>위험도 2단계</b> 체력% 경계(원본 41). -1=원본",
 "d19_sev_hp_3" => "넥서스 방어 <b>위험도 3단계</b> 체력% 경계(원본 26). 여기까지 내려오면 가장 급한 상태로 봅니다. -1=원본",
 "d19_sev_ratio_0" => "위험도 <b>0단계</b>(체력과 무관한 기본) 문턱(원본 49). 이 사다리는 '적 전력이 우리 몇 %를 넘으면 위험으로 칠지'를 단계별로 정합니다. 낮출수록 쉽게 위험으로 봅니다. -1=원본",
 "d19_sev_ratio_1" => "위험도 <b>1단계</b> 문턱(원본 29) — 체력이 1단계 경계 아래일 때 적용. -1=원본",
 "d19_sev_ratio_2" => "위험도 <b>2단계</b> 문턱(원본 17). -1=원본",
 "d19_sev_ratio_3" => "위험도 <b>3단계</b> 문턱(원본 9). 체력이 바닥이면 적이 조금만 있어도 위험으로 봅니다. -1=원본",
 "disc16_home_hp" => "기지로 돌아가 회복할지 정하는 체력% 기준. 올리면 조금만 다쳐도 귀환하고, 내리면 끝까지 버팁니다. -1=원본",
 "jungle_retreat_threat" => "<b>정글러가 교전에서 물러나는 민감도</b>(%, 100=원본). 100보다 <b>작으면 잘 후퇴</b>하고, 크면 덜 후퇴합니다. 0~200. -1=원본",
 "nx_an_count_gate" => "넥서스 공격으로 넘어갈 때 보는 <b>인원 게이트</b>. 올리면 더 많이 모여야 넥서스를 칩니다. -1=원본",
 "nx_dn_count_gate" => "넥서스 수비로 넘어갈 때 보는 <b>인원 게이트</b>. 올리면 더 많이 모여야 수비 대형을 잡습니다. -1=원본",
 "self_team_only" => "선수별 오버라이드를 <b>우리 팀에만</b> 적용할지(원본 1 = 우리 팀만). 0으로 두면 상대 팀 선수에게도 똑같이 적용됩니다. -1=원본",
 "pl_obj_role" => "에픽·세르펜·정글 계열 판단을 가질 역할 슬롯(원본 1 = 정글러 전용). 2사이트 동시 패치. 값을 바꾸면 그 역할의 챔프가 에픽/세르펜 사냥·견제와 정글 판단을 취득합니다(예: 0 = 탑, 2 = 미드). ⚠오브젝트 운영 전반이 바뀌는 큰 레버 — 한 번에 하나씩 실험. -1=원본",
 "pl_ganker_gate" => "갱(LineGanker) 판단 생성 게이트(원본 11). 이 값이 맞을 때만 정글러가 갱 플랜을 취득합니다. 부쉬 왕복이 갱 셋업에서 온다면, gk_*(대기·재시도)로 완화가 안 될 때 갱 자체를 덜 뜨게 하는 상위 수단. ⚠의미 = 내부 상태값 비교라 임의값은 '갱 봉인'에 가깝게 동작할 수 있음. -1=원본",
 "pl_serpen_phase_mask" => "세르펜 판단 허용 페이즈 비트마스크(원본 0x1a1=417, 비트 0·5·7·8). 경기 진행 페이즈 중 어느 구간에서 세르펜 사냥·견제 판단을 만들지. 비트를 더하면 더 넓은 구간에서 세르펜을 노립니다(예: 511=0~8 전 구간). 4사이트 동시. -1=원본",
 "pl_epic_phase_min" => "⚠에픽 판단 허용 페이즈 경계(원본 249). 내부 인코딩이라 의미가 아직 확정되지 않았습니다 — 값↔효과 대응이 직관적이지 않을 수 있으니 리플레이 A/B로 확인하며 쓰세요. 2사이트 동시. -1=원본",
 // ★[신규 09-01] 개시 게이트 + 사냥 세부 (인게임 검증 완료)
 "gk2_gank_radius" => "갱크 개시: 갱크 판정 근접 반경(유닛, 원본 250000). 아군·적 카운트 공통. ↓=코앞만 갱크, ↑=넓게 갱크. -1=원본",
 "gk2_gank_hp" => "갱크 개시: 갱크 적격 적 HP%(원본 40, 10사이트). ↑=더 다친 적만 갱크(신중), ↓=풀피도 갱크(과감). -1=원본",
 "eng_camp_radius" => "교전 개시: 아군 집결 반경(유닛, 원본≈140060). 이 반경 안에 아군이 모여야 교전 개시. ↓=바짝 모여야 개시, ↑=흩어져도 개시. -1=원본",
 "db_retreat_margin" => "결사전 후퇴 인원마진(원본 2 = 아군 생존수 ≥ 적수+2 일 때만 후퇴 허용). ↓=쉽게 후퇴, ↑=올인 고집. -1=원본",
 "eh_fin_mode" => "에픽/세르펜 사냥 킬타깃 추격 게이트(branch-patch 4사이트). -1=원본 / 0=항상 킬타깃 추격(적극) / 1=항상 무시.",
 "eh_band_off" => "사냥 접근밴드 오프셋(원본 10000, 고급). -1=원본",
 "eh_commit_margin" => "사냥 커밋 거리여유(원본 30000, 고급). -1=원본",
 "eh_dist_clamp" => "사냥 거리 클램프 상한(원본 100000, 고급). -1=원본",
 "eh_clamp2" => "사냥 2차 거리 클램프(원본 80000, 고급). -1=원본",
 "eh_engage_dist" => "사냥 교전거리(유닛, 원본 12000, 제곱저장, 고급). -1=원본",
 "eh_dist_shift" => "사냥 거리²>>10 임계(원본 172265625 raw, 고급). -1=원본",
 "eh_power_weight" => "사냥 팀파워 편차 가중계수(원본 103, 고급). -1=원본",
 "eh_power_neutral" => "사냥 팀파워 중립점%(원본 50, 고급). -1=원본",
 "eh_power_sub" => "사냥 파워 2번째 항 byte(원본 206=−50, eh_power_neutral 짝, 고급). -1=원본",
 "eh_time_slope" => "사냥 파워%→시간창 계수(원본 99, 고급). -1=원본",
 "eh_window_cap" => "사냥 시간창 상한(원본 2000, 고급). -1=원본",
 "eh_score_floor" => "사냥 fight_check 점수 하한(원본 1000, 고급). -1=원본",
 "eh_score_gate" => "사냥 점수 보조 게이트(원본 10, 고급). -1=원본",
 "eh_helper_a" => "사냥 보조 헬퍼 파라미터(원본 40, 고급). -1=원본",
 "eh_helper_b" => "사냥 보조 헬퍼 파라미터2(원본 60, 고급). -1=원본",
 "eh_hp_gate2" => "사냥 HP% 2차 게이트(원본 36, 고급). -1=원본",
 "eh_grid_cost" => "사냥 그리드 탐색 비용 가산(원본 10000, 고급). -1=원본",
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
fn read_utf8(path: &Path) -> Option<String> { read_utf8_checked(path).map(|(s, _)| s) }
// ★[09-01 견고화] 유효성도 반환: 무효 UTF-8이면 (lossy문자열, false). 호출부가 원본백업·경고 가능.
//   구 read_utf8은 lossy로 조용히 U+FFFD 치환 → 그 상태로 저장하면 손상(주석 등)이 영구 고착됐다.
fn read_utf8_checked(path: &Path) -> Option<(String, bool)> {
 let bytes = std::fs::read(path).ok()?;
 let bytes = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) { &bytes[3..] } else { &bytes[..] };
 let valid = std::str::from_utf8(bytes).is_ok();
 Some((String::from_utf8_lossy(bytes).into_owned(), valid))
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
 active_class: i8, // -1 = 기본(전역) / 0..4 = 클래스(melee/range/magician/util/assassin)
 // ★[08-07] 클래스 편집 중일 때 **지정 불가 항목까지 볼지**. 기본 꺼짐 = 가능한 것만 보인다.
 //   불가 항목이 332개라 다 늘어놓으면 정작 손댈 수 있는 123개를 찾기가 어렵다.
 class_show_all: bool,
 config_list: Vec<String>,
 selected_config: String,
 show_save_as: bool,
 save_as_name: String,
 toast: String,
 toast_err: bool,
 toast_until: f64,
 // ── 순서도 보기(실험) ──
 view_flow: bool,
 flow_open: Vec<bool>,
 flow_gopen: std::collections::HashSet<(usize, usize)>, // 중분류(묶음)별 펼침 상태
 flow_sel: Option<String>,
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

 // ★[09-01 견고화] active cfg 유효성 검사. 무효 UTF-8이면 원본 raw를 .bak_invalid 로 백업(저장 시 lossy 손상 고착 방지) + 경고.
 let (active_text, active_valid) = read_utf8_checked(&active_path)
 .unwrap_or_else(|| (default_text.clone(), true));
 if !active_valid {
 if let Ok(raw) = std::fs::read(&active_path) {
 let bak = active_path.with_file_name("tfm2_ai_adjust.cfg.bak_invalid");
 let _ = std::fs::write(&bak, &raw);
 }
 }

 let mut app = App {
 folder, active_path, cfg_dir, default_path,
 default_text, defaults,
 model: Model::default(),
 active_tab: 0,
 active_class: -1,
 class_show_all: false,
 config_list: Vec::new(),
 selected_config: ACTIVE_NAME.to_string(),
 show_save_as: false,
 save_as_name: String::new(),
 toast: if active_valid { String::new() } else { "⚠ tfm2_ai_adjust.cfg 가 무효 UTF-8입니다 — 원본을 tfm2_ai_adjust.cfg.bak_invalid 로 백업했습니다. 저장하면 손상된 부분(주석 등)만 정리되고 값은 보존됩니다. (편집기·게임은 UTF-8 BOM없음으로 저장합니다)".to_string() },
 toast_err: !active_valid, toast_until: if active_valid { 0.0 } else { 1e18 },
 view_flow: false, flow_open: vec![false; FLOW.len()],
 flow_gopen: std::collections::HashSet::new(), flow_sel: None,
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
 // ★[08-06 유저 규칙] **내부값은 -1, 표시만 원본값.**
 //   -1 = "그 바이트를 안 건드림" / 숫자 = "원본과 같다고 믿는 값으로 덮어씀".
 //   내가 채운 기본값이 틀리면 그대로 오패치이고, 그 오패치는 applied=N/N·blocked=0 이라
 //   지표상 전부 정상으로 보인다(08-06 pe_noise_exempt·-1 펼치기 실사고). 그래서
 //   **원본값과 같은 입력은 -1 로 저장**해 "안 건드림" 상태를 유지한다.
 //   토글은 제외 — 0/1 규약에 -1 이 없다.
 let v: &str = if !is_toggle(k) && orig_val(k).map_or(false, |o| o == v) { "-1" } else { v };
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
 // ★[08-05] 설명은 원래 태그 제거를 안 거쳐서 <b>·** 가 화면에 그대로 새어 나왔다.
    //   본문은 평문으로 정규화했지만, 앞으로 태그가 섞여도 안 새도록 여기서도 거른다.
    if let Some(d) = desc_static(k) { return html_to_text(d); }
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

 /// 값 편집 컨트롤 1개. 탭 보기와 순서도 보기가 **같은 위젯을 공유**한다
 /// (콤보/토글/폐기잠금/클래스 오버라이드 규칙이 두 곳에서 갈라지지 않게).
 fn value_ctl(&mut self, ui: &mut egui::Ui, k: &str, width: f32) {
 let cur = self.get_val(k);
 if self.active_class >= 0 {
 let pos = self.active_class as usize;
 let pk = format!("{}_class_{}", k, CLASS_EN[pos]);
 let was = self.model.map.contains_key(&pk);
 let gval = cur.clone();
 let mut inherit = !was;
 let mut new_val: Option<String> = None;
 ui.horizontal(|ui| {
 ui.checkbox(&mut inherit, "기본 따름");
 if inherit {
 let mut gv = gval.clone();
 ui.add_enabled(false, egui::TextEdit::singleline(&mut gv)
 .desired_width(width - 120.0).font(egui::TextStyle::Monospace));
 } else {
 let mut v = if was { self.get_val(&pk) }
 else if gval.is_empty() { "0".to_string() } else { gval.clone() };
 let resp = ui.add_sized([width - 120.0, 24.0],
 egui::TextEdit::singleline(&mut v).font(egui::TextStyle::Monospace));
 if resp.changed() || !was { new_val = Some(v.trim().to_string()); }
 }
 });
 if inherit { if was { self.remove_key(&pk); } }
 else if let Some(v) = new_val { self.set_val(&pk, &v); }
 } else if let Some(opts) = select_opts(k) {
 let mut sel = cur.clone();
 let shown = opts.iter().find(|(v, _)| *v == sel).map(|(_, l)| *l).unwrap_or(&sel).to_string();
 egui::ComboBox::from_id_salt(k).selected_text(shown).width(width).show_ui(ui, |ui| {
 for (v, l) in opts { ui.selectable_value(&mut sel, v.to_string(), *l); }
 });
 if sel != cur { self.set_val(k, &sel); }
 } else if is_toggle(k) {
 let mut on = cur == "1" || cur == "true";
 // ★라벨은 checkbox **뒤에서** 만든다 — checkbox 가 on 을 제자리에서 뒤집으므로,
 //   앞에서 계산하면 클릭한 프레임에 "체크됨 + 꺼짐"이 같이 보인다(2026-08-06 유저 신고).
 if ui.checkbox(&mut on, "").changed() {
 self.set_val(k, if on { "1" } else { "0" });
 }
 ui.label(if on { "켜짐" } else { "꺼짐" });
 } else if is_dead(k) || is_unfired(k) {
 let mut v = shown_val(k, &cur);
 ui.add_enabled(false, egui::TextEdit::singleline(&mut v)
 .desired_width(width).font(egui::TextStyle::Monospace));
 } else {
 // ★[08-06] 입력 중에는 자동 채움 금지 — 포커스가 빠졌을 때만 기본값을 보여준다.
 let tid = ui.make_persistent_id(("valbox", k));
 let focused = ui.ctx().memory(|m| m.has_focus(tid));
 let mut v = if focused { cur.clone() } else { shown_val(k, &cur) };
 let resp = ui.add_sized([width, 24.0],
     egui::TextEdit::singleline(&mut v).id(tid).font(egui::TextStyle::Monospace));
 if resp.changed() { self.set_val(k, v.trim()); }
 }
 }

 /// 순서도 한 층에 속한 키 목록. **TABS 를 접두로 훑어 만든다** —
 /// 새 설정값을 탭에 추가하면 순서도에도 자동으로 나타난다(목록 이중관리 금지).
 fn flow_keys(&self, prefixes: &[&str], used: &mut std::collections::HashSet<&'static str>) -> Vec<&'static str> {
 let mut out = Vec::new();
 for t in TABS.iter() {
 for &k in t.keys {
 if k.starts_with('§') || used.contains(k) { continue; }
 if prefixes.iter().any(|p| *p == "*" || if p.ends_with('_') { k.starts_with(*p) } else { k == *p }) {
 used.insert(k);
 out.push(k);
 }
 }
 }
 out
 }

 // ── 순서도 색 ──
 // 판단흐름도.html 과 같은 인상을 주도록 금색 강조 + 카드 2단 명도로 맞췄다.
 const F_GOLD: egui::Color32 = egui::Color32::from_rgb(0xd0, 0xa0, 0x4a);
 const F_HEAD: egui::Color32 = egui::Color32::from_rgb(0xcd, 0xd3, 0xdf);
 const F_DIM: egui::Color32 = egui::Color32::from_rgb(0x9a, 0xa3, 0xb2);
 const F_CARD: egui::Color32 = egui::Color32::from_rgb(0x1a, 0x1d, 0x25);
 const F_SUB: egui::Color32 = egui::Color32::from_rgb(0x22, 0x26, 0x30);
 const F_EDGE: egui::Color32 = egui::Color32::from_rgb(0x33, 0x39, 0x47);

 /// 단계 번호 원형 배지 — 순서도 노드처럼 보이게 직접 그린다.
 /// 설명문 전용 라벨 — **행간을 넓혀서** 그린다.
 /// egui는 글자 크기를 키워도 줄 높이가 폰트 기본값 그대로라 긴 문단이 빽빽하게 붙는다.
 /// `TextFormat::line_height`는 `LayoutJob`으로만 줄 수 있어서 여기서 직접 조립한다.
 fn para(ui: &mut egui::Ui, text: &str, color: egui::Color32, size: Option<f32>) {
 let mut font = egui::TextStyle::Body.resolve(ui.style());
 if let Some(s) = size { font.size = s; }
 let mut job = egui::text::LayoutJob::default();
 job.wrap.max_width = ui.available_width();
 job.append(text, 0.0, egui::TextFormat {
 font_id: font.clone(),
 color,
 line_height: Some(font.size * 1.6), // 1.0 = 붙음 / 1.6 = 읽기 편한 정도
 ..Default::default()
 });
 ui.label(job);
 }

 fn flow_badge(ui: &mut egui::Ui, txt: &str, key: bool) {
 let d = 28.0;
 let (rect, _) = ui.allocate_exact_size(egui::vec2(d, d), egui::Sense::hover());
 let col = if key { Self::F_GOLD } else { Self::F_DIM };
 ui.painter().circle_stroke(rect.center(), d * 0.5 - 2.0,
 egui::Stroke::new(if key { 2.0 } else { 1.3 }, col));
 ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, txt,
 egui::FontId::monospace(13.0), col);
 }

 /// 단계 사이 화살표 — 세로선 + 화살촉.
 fn flow_arrow(ui: &mut egui::Ui) {
 let w = ui.available_width();
 let (rect, _) = ui.allocate_exact_size(egui::vec2(w, 24.0), egui::Sense::hover());
 let x = rect.center().x;
 let (t, b) = (rect.top() + 1.0, rect.bottom() - 2.0);
 let st = egui::Stroke::new(1.6, Self::F_EDGE);
 let p = ui.painter();
 p.line_segment([egui::pos2(x, t), egui::pos2(x, b)], st);
 p.line_segment([egui::pos2(x - 4.5, b - 6.0), egui::pos2(x, b)], st);
 p.line_segment([egui::pos2(x + 4.5, b - 6.0), egui::pos2(x, b)], st);
 }

 // ── 순서도 보기 본체 ──
 // 대분류(단계) → 중분류(묶음) → 소분류(설정값) 3단으로 접힌다.
 // 단계를 열면 묶음 제목만 뜨고, 묶음을 눌러야 값이 펼쳐진다.
 fn flow_ui(&mut self, ui: &mut egui::Ui) {
 ui.horizontal(|ui| {
 if ui.button("단계 모두 펼치기").clicked() { for o in self.flow_open.iter_mut() { *o = true; } }
 if ui.button("모두 접기").clicked() {
 for o in self.flow_open.iter_mut() { *o = false; }
 self.flow_gopen.clear();
 }
 ui.separator();
 // 설명은 오른쪽 사이드바로 — 값을 보면서 동시에 읽을 수 있게(페이지 이동 없음)

 });
 ui.add_space(10.0);

 egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
 let mut used: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
 for (i, node) in FLOW.iter().enumerate() {
 // 층별 키 = TABS 를 접두로 훑어 계산(목록 이중관리 없음)
 let groups: Vec<(&'static str, &'static str, Vec<&'static str>)> = node.groups.iter()
 .map(|g| (g.label, g.note, self.flow_keys(g.prefixes, &mut used)))
 .collect();
 let total: usize = groups.iter().map(|(_, _, v)| v.len()).sum();
 let mut ch = 0usize;
 for (_, _, v) in &groups {
 for &k in v {
 if self.defaults.get(k).map_or(false, |d| d != &self.get_val(k)) { ch += 1; }
 }
 }
 let open = self.flow_open[i];
 let key_layer = i == 4 || i == 5; // 판단력이 개입하는 두 단계 = 강조

 egui::Frame::NONE
 .fill(Self::F_CARD)
 .stroke(egui::Stroke::new(if key_layer { 1.6 } else { 1.0 },
 if key_layer { Self::F_GOLD } else { Self::F_EDGE }))
 .corner_radius(6)
 .inner_margin(egui::Margin::symmetric(14, 12))
 .show(ui, |ui| {
 ui.set_width(ui.available_width());
 // ── 대분류 헤더 ──
 let hdr = ui.horizontal(|ui| {
 Self::flow_badge(ui, node.no, key_layer);
 ui.add_space(8.0);
 ui.vertical(|ui| {
 ui.label(egui::RichText::new(node.title).strong().size(18.0).color(Self::F_HEAD));
 ui.label(egui::RichText::new(node.sub).small().color(Self::F_DIM));
 });
 ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
 ui.label(egui::RichText::new(if open { "\u{25bc}" } else { "\u{25b6}" }).color(Self::F_DIM));
 ui.add_space(6.0);
 if total > 0 {
 let t = if ch > 0 {
 egui::RichText::new(format!("설정값 {} · 변경 {}", total, ch)).small().color(BLUE)
 } else {
 egui::RichText::new(format!("설정값 {}", total)).small().color(Self::F_DIM)
 };
 ui.label(t);
 }
 });
 }).response.rect;
 // ⚠`InnerResponse::interact()` 는 자식 라벨이 hover 를 가져가면 글자 위 클릭을 놓친다.
 // → 헤더 영역을 **직접 히트테스트**해서 글자든 빈칸이든 어디를 눌러도 펼쳐지게 한다.
 let hdr = ui.interact(hdr, ui.make_persistent_id(("flow_node", i)), egui::Sense::click())
 .on_hover_cursor(egui::CursorIcon::PointingHand);
 if hdr.clicked() { self.flow_open[i] = !open; }

 if open {
 ui.add_space(8.0);
 ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
 Self::para(ui, node.body, Self::F_DIM, None);
 ui.style_mut().wrap_mode = None;
 if total == 0 {
 ui.add_space(6.0);
 ui.label(egui::RichText::new("이 단계에는 조절할 설정값이 없습니다.").small().weak());
 }
 // ── 중분류(묶음) ──
 for (gi, (label, note, keys)) in groups.iter().enumerate() {
 if keys.is_empty() { continue; }
 ui.add_space(7.0);
 let gopen = self.flow_gopen.contains(&(i, gi));
 let gch = keys.iter()
 .filter(|k| self.defaults.get(**k).map_or(false, |d| d != &self.get_val(k)))
 .count();
 egui::Frame::NONE
 .fill(Self::F_SUB)
 .stroke(egui::Stroke::new(1.0, Self::F_EDGE))
 .corner_radius(4)
 .inner_margin(egui::Margin::symmetric(11, 8))
 .show(ui, |ui| {
 ui.set_width(ui.available_width());
 // 묶음 설명은 **접힌 상태에서도** 보이게 제목 아래에 함께 둔다.
 let gh = ui.vertical(|ui| {
 ui.horizontal(|ui| {
 ui.label(egui::RichText::new(if gopen { "\u{25be}" } else { "\u{25b8}" }).color(Self::F_DIM));
 ui.add_space(2.0);
 ui.label(egui::RichText::new(*label).strong().size(15.5).color(Self::F_HEAD));
 ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
 let t = if gch > 0 {
 egui::RichText::new(format!("{}개 · 변경 {}", keys.len(), gch)).small().color(BLUE)
 } else {
 egui::RichText::new(format!("{}개", keys.len())).small().color(Self::F_DIM)
 };
 ui.label(t);
 });
 });
 if !note.is_empty() {
 ui.add_space(3.0);
 ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
 Self::para(ui, note, Self::F_DIM, Some(13.5));
 ui.style_mut().wrap_mode = None;
 }
 }).response.rect;
 // 헤더(제목 + 설명문) 전체를 직접 히트테스트 — 글자 위 클릭도 먹게.
 let gh = ui.interact(gh, ui.make_persistent_id(("flow_grp", i, gi)), egui::Sense::click())
 .on_hover_cursor(egui::CursorIcon::PointingHand);
 if gh.clicked() {
 if gopen { self.flow_gopen.remove(&(i, gi)); }
 else { self.flow_gopen.insert((i, gi)); }
 }
 // ── 소분류(설정값) ── (설명은 위 헤더에 이미 있으므로 반복하지 않는다)
 if gopen {
 ui.add_space(8.0);
 egui::Grid::new(format!("flow{}_{}", i, gi))
 .num_columns(3).striped(true).spacing([12.0, 8.0]).show(ui, |ui| {
 for &k in keys.iter() {
 let cur = self.get_val(k);
 let def = self.defaults.get(k).cloned();
 // ★[08-06] baseline 뿐 아니라 **실제 기본값**과도 비교한다.
 //   cfg 의 `-1` 을 실제 기본값으로 펼치면서 baseline("-1")과 달라져 멀쩡한 값이 전부 파랑으로 잡혔다.
 let changed = def.as_ref().map_or(false, |d| d != &cur)
     && orig_val(k).map_or(true, |o| o != cur);
 ui.vertical(|ui| {
 ui.set_min_width(180.0); ui.set_max_width(180.0);
 ui.horizontal(|ui| {
 let mut t = egui::RichText::new(disp_key(k)).monospace().strong();
 if changed { t = t.color(BLUE); }
 if is_dead(k) {
 ui.label(egui::RichText::new("폐기").small().strong()
 .color(egui::Color32::from_rgb(0xe0,0x6c,0x6c)));
 } else if is_unfired(k) {
 ui.label(egui::RichText::new("미확인").small().weak());
 }
 if ui.selectable_label(self.flow_sel.as_deref() == Some(k), t).clicked() {
 self.flow_sel = Some(k.to_string());
 }
 });
 ui.label(egui::RichText::new(
 base_line(k, &def))
 .small().weak());
 });
 self.value_ctl(ui, k, 220.0);
 // 설명을 바로 옆에 그린다 — 버튼을 눌러야 보이면 훑어보기가 안 된다.
 if let Some(d) = desc_plain(k) {
 ui.vertical(|ui| {
 ui.set_max_width(430.0);
 Self::para(ui, &d, Self::F_DIM, Some(12.5));
 });
 } else { ui.label(""); }
 ui.end_row();
 }
 });
 }
 });
 }
 }
 });

 if i + 1 < FLOW.len() { Self::flow_arrow(ui); }
 }
 ui.add_space(24.0);
 });
 }

 // ── 순서도 안의 '설명' 탭 ──
}

// ============================ 순서도 보기 ============================
struct FlowGroup { label: &'static str, note: &'static str, prefixes: &'static [&'static str] }
struct FlowNode {
 no: &'static str,
 title: &'static str,
 sub: &'static str,
 body: &'static str,
 groups: &'static [FlowGroup],
}

static FLOW: &[FlowNode] = &[
 FlowNode{ no:"0", title:"팀 전술", sub:"경기 시작 전에 정해지는 배경",
 body:"유저가 지정한 10개 항목이 경기 시작 때 복사되어 이후 모든 단계의 배경이 됩니다.\n\
 단, 전술을 읽는 판단 함수는 23개뿐이라 대부분의 설정값은 전술과 무관하게 항상 작동합니다.",
 groups:&[] },

 FlowNode{ no:"1", title:"팀 목표를 잡는다", sub:"채팅 46종 → 팀 작전 하나",
 body:"팀원이 보낸 메시지를 받아 '지금 팀이 뭘 하기로 했는가'를 딱 하나 정합니다(배타적).\n\
 모르가드·세르펜·넥서스 공수·압박·귀환·갱·갱다이브·포탑압박·컴백픽 중 하나이며,\n\
 채팅 없이 AI가 스스로 시작할 수 있는 건 갱뿐이고 그것도 팀이 아무 작전도 안 잡았을 때만입니다.",
 groups:&[
 FlowGroup{ label:"작전 자동 취소",
 note:"한번 잡은 작전을 조건이 어긋났을 때 팀이 스스로 접을지 정합니다. 갱·갱다이브·컴백픽만 자동 취소 대상이고, 나머지는 끝까지 밀어붙입니다.",
 prefixes:&["tm_"] } ] },

 FlowNode{ no:"2", title:"선수가 판단을 만든다", sub:"팀 목표 + 상황 → 이 선수의 판단",
 body:"팀 목표가 개인 판단으로 번역되는 지점입니다. 넥서스 방어/공격, 귀환, 갱커 같은 판단이 여기서 생깁니다.\n\
 팀 목표는 항상 판단 재계산보다 먼저 세팅되므로, 목표가 바뀌면 그 즉시 판단이 다시 만들어집니다.",
 groups:&[
 FlowGroup{ label:"물러날지 — 머릿수·포탑·능력치",
 note:"싸울지 물러설지를 정하는 값들입니다. 근처 아군·적과 포탑을 전력으로 환산해 승산을 보고, 선수 능력치(공격성·에고·판단력)로 그 판단을 흔듭니다. `_move`가 붙은 것은 라인전 전용 값이고, −1이면 한타값을 그대로 따릅니다.",
 prefixes:&["numbers_","ally_tower_","tower_","stat_","adv_prof","adv_prof_min","adv_prof_seg"] },
 FlowGroup{ label:"교전에 들어갈지 — 확률·합류",
 note:"교전 대상 우선순위별 진입 확률과, 다른 곳에서 벌어진 싸움에 합류할지를 정합니다.",
 prefixes:&["eng_role2","eng_role3","eng_role4","eng_role_def","engage_base","engage_thr_mult","t_engage","rc_join_"] },
 FlowGroup{ label:"안 보이는 적이 어디 있을지",
 note:"마지막으로 본 시각·위치·이동속도로 \"지금 이쯤 있겠다\"는 원판을 그립니다. 이 원판이 판단의 입력이 됩니다.",
 prefixes:&["eg_"] },
 FlowGroup{ label:"넥서스를 칠지 / 기지로 갈지",
 note:"적 타워 잔여 수를 보고 넥서스로 밀어붙일지, 아직 이르면 대신 무엇을 할지. 그리고 회복하러 돌아갈 체력 기준입니다.",
 prefixes:&["an_","disc16_home_hp","jungle_retreat_threat","aggr_lane"] },
 FlowGroup{ label:"어떤 판단을 만들지",
 note:"이 단계는 '그 판단이 떴을 때 어떻게 행동하나'가 아니라 애초에 그 판단을 만들지 말지를 정합니다. 오브젝트 운영 전반이 한 번에 바뀌니 한 번에 하나씩 실험하세요.",
 prefixes:&["pl_"] } ] },

 FlowNode{ no:"3", title:"판단을 실행 단위로 바꾼다", sub:"같은 판단도 상황따라 갈라짐",
 body:"같은 '라인전' 판단이라도 라인 수비 / 안전 / 대기 / 귀환 중 무엇을 할지로 갈라집니다.\n\
 경기 중 가장 자주 돌아가는 경로라, 체감을 바꾸려면 여기부터 보는 게 좋습니다.",
 groups:&[
 FlowGroup{ label:"라인전",
 note:"라인에 서 있을 때의 전진·후퇴 판단입니다. 한 경기에서 가장 자주 지나가는 경로라 값 하나가 경기 전체 인상을 바꿉니다.",
 prefixes:&["dd_"] },
 FlowGroup{ label:"라인 대기·안전",
 note:"밀고 나간 뒤 어디서 기다릴지, 물러나 있을 때 얼마나 넓게 움직일지입니다. 배회 반경을 줄이면 한자리에 머물러 왔다갔다가 줄어듭니다.",
 prefixes:&["lw_", "ls_"] },
 FlowGroup{ label:"귀환",
 note:"체력·거리·아군 상황을 보고 우물로 돌아갈지 정합니다. 돌아가는 시점이 이르면 라인을 자주 비우고, 늦으면 자주 죽습니다.",
 prefixes:&["rc_", "d7_", "t_recall"] },
 FlowGroup{ label:"교전·갱커버",
 note:"싸움에 합류할지, 견제만 할지 가르는 판단입니다.",
 prefixes:&["pf_", "poke_"] },
 FlowGroup{ label:"에픽 사냥·견제",
 note:"에픽 오브젝트를 노릴 때의 판단입니다. 정글 포지션에만 붙는 경우가 많습니다.",
 prefixes:&["ec_"] },
 FlowGroup{ label:"세르펜 사냥·견제",
 note:"에픽 오브젝트를 견제할 때의 판단입니다. 회복존 안에서 다쳤을 때 나가지 않고 대기하는 규칙이 여기 들어 있습니다.
⚠키 앞의 sn_ 은 세르펜처럼 보이지만 **에픽**입니다 — 예전 함수 이름이 잘못 붙어 있던 잔재이고, 실제 동작은 에픽 견제입니다(키 이름은 기존 설정 파일 호환 때문에 그대로 둡니다).",
 prefixes:&["sn_"] },
 FlowGroup{ label:"넥서스 방어",
 note:"세르펜 오브젝트를 견제할 때의 판단입니다. 진척도와 거리로 물러날지 밀어붙일지 정합니다.
⚠키 앞의 nxd_ · nx_repl 은 넥서스처럼 보이지만 **세르펜**입니다 — 예전 이름 잔재이고, 넥서스 관련 값은 아래 「넥서스 공수 실행」에 따로 있습니다.",
 prefixes:&["nxd_", "nx_repl"] },
 FlowGroup{ label:"라인 배정 · 봇 듀오",
 note:"어느 라인에 서고 언제 물러날지를 정합니다. 레인 번호는 0=탑 · 1=미드 · 2=바텀이고, 특수 처리가 붙는 것은 바텀(봇 듀오)입니다. 체력 기준을 올리면 조금만 아파도 뒤로 빠집니다.",
 prefixes:&["d4_"] },
 FlowGroup{ label:"라인 총력전",
 note:"한 라인에 다 같이 밀어붙일 때의 판단입니다. 아군에게 붙기 시작하는 거리를 내리면 잘 안 모입니다.",
 prefixes:&["lt_"] },
 FlowGroup{ label:"후퇴 트리거 · 정글 진행",
 note:"'내가 곧 죽는다'를 따로 감시하는 장치와, 정글을 계속 돌지 정하는 체력 기준입니다. 후퇴 기준 세 개는 고정값이 아니라 판단력이 높을수록 한쪽으로 밀리는 직선입니다.",
 prefixes:&["rt_", "jg_"] },
 ] },

 FlowNode{ no:"4", title:"할 수 있는 행동 후보를 만든다", sub:"판단마다 전담 함수가 목록을 만듦",
 body:"판단마다 전담 함수가 있고, 각자 '지금 할 수 있는 행동 목록'을 만듭니다. 한 번에 여러 개가 나옵니다.\n\
 여기서 판단력이 1차로 개입합니다 — 후보를 걸러내는 필터에 주사위가 붙어 있어,\n\
 판단력이 낮으면 필터가 아예 안 걸려 점수 낮은 행동이 그대로 남습니다.",
 groups:&[
 FlowGroup{ label:"전역 궁",
 note:"아군이 요청한 전역 궁을 쓸지. 근처에 적이 보이면 억제되기 때문에 원본은 교전 중에 거의 나가지 않습니다.",
 prefixes:&["gu_"] },
 FlowGroup{ label:"판단력 오판 게이트",
 note:"게임의 '판단력' 스탯이 실제로 작동하는 방식 중 하나입니다. 주사위가 문턱을 넘으면 최선 후보 대신 무작위 후보를 고릅니다. 원본 기준 판단력 0인 선수는 85% 확률로 엉뚱한 행동을 합니다.",
 prefixes:&["ex_judge_"] },
 FlowGroup{ label:"평타·스킬 사거리와 조건",
 note:"교전 중 '누구에게 평타·스킬을 쓸지' 후보를 고릅니다. 사거리 판정에 선행 예측이 들어가서, 예측 틱을 올리면 아직 사거리 밖인 적에게도 미리 달려듭니다. 궁은 지정 지점 근처에서만 후보가 됩니다.",
 prefixes:&["cs_"] },
 FlowGroup{ label:"데스매치의 전투 행동 만들기",
 note:"⚠데스매치 모드 전용 판단입니다 — 일반 경기에서는 발동하지 않습니다. 데스매치에서 평타·스킬·스킬2·궁 후보를 실제로 만들어 내는 가장 큰 함수입니다. 여기서는 이동 계열 행동이 전혀 안 나옵니다. 적 체력이 마무리 기준 밑으로 떨어지면 별도 경로가 열려 몰아치고, 스킬은 대상이 일정 체력 밑이면 조건 없이 허용됩니다.",
 prefixes:&["dm_lookahead", "dm_ult_lookahead", "dm_near_", "dm_execute_hp", "dm_lasthit", "dm_skill_hp", "dm_ult_", "dm_skill2_level"] },
 FlowGroup{ label:"때려도 되는지 판정 (겁 많음·적음)",
 note:"후보를 만들 때 '이걸 때리면 반격당해 죽지 않나'를 검사하는 관문입니다. 막타가 확실하면 무조건 통과하고, 라인전에는 적 사거리를 실제보다 넓게 잡아 몸을 사립니다. 이 여유분을 줄이는 것이 라인전 과감함을 올리는 가장 직접적인 방법입니다.",
 prefixes:&["sf_"] },
 FlowGroup{ label:"교전 판단",
 note:"실제 싸움을 담당하는 층의 체력·거리 임계입니다. 접근 정지 반경을 올리면 멀찍이서 멈추고, 추격 유지 거리를 올리면 끈질기게 쫓습니다.",
 prefixes:&["bt_"] },
 FlowGroup{ label:"라인 수비",
 note:"라인 수비 판단입니다. 서로 다른 두 평가가 나란히 항상 실행되므로 같은 손잡이가 두 군데에 동시에 걸립니다. 목표물 접근 거리를 줄이면 바짝 붙어 공격적이 되고, 무작위 대체 최소 후보 수를 올리면 판단력 낮은 선수의 엉뚱한 행동이 줄어듭니다.",
 prefixes:&["ld_"] },
 FlowGroup{ label:"대기 위치",
 note:"밀고 나간 아군 뒤에서 기다릴 때의 거리입니다.",
 prefixes:&["ex_wait_"] },
 FlowGroup{ label:"위협 민감도",
 note:"주변 위협을 얼마나 심각하게 볼지의 사다리입니다. 체력 대비 위협 비율로 단계가 갈리며, 낮출수록 겁이 없어집니다.",
 prefixes:&["sv_"] },
 FlowGroup{ label:"시야",
 note:"시야에서 사라진 적을 얼마나 오래 '아직 거기 있다'고 믿을지입니다. 길면 유령을 쫓고, 짧으면 금방 잊습니다.",
 prefixes:&["vw_", "vis_window"] },
 FlowGroup{ label:"정글 갱 셋업",
 note:"정글러가 갱을 준비할 때의 대기·재시도 타이밍입니다. 팀 전술이 라인 개입일 때만 발화하는 항목이 섞여 있습니다.",
 prefixes:&["gk_"] },
 FlowGroup{ label:"운영 전환·로밍",
 note:"라인을 떠나 다른 곳으로 움직이는 판단의 거리·타이밍입니다. 마스터 스위치를 켜야 아래 값이 반영됩니다.",
 prefixes:&["gb_"] },
 FlowGroup{ label:"넥서스 공수 실행",
 note:"넥서스를 치거나 지킬 때의 체력·거리 임계입니다. **여기만 진짜 넥서스**이고, 에픽·세르펜(오브젝트)은 위의 별도 그룹에 있습니다.
주의: nx_dn_hp_crit · nx_dn_hp_low 는 이름이 넥서스 계열이지만 재는 것은 **챔피언 자신의 체력**이고, nx_dn_nexus_hp 만 넥서스 체력입니다.",
 prefixes:&["nx_", "d19_", "d19i_"] },
 FlowGroup{ label:"숨기",
 note:"수풀로 숨을지, 어디로 물러날지를 정합니다. 후보 선별 거리가 이 판단에서 압도적으로 많이 쓰이는 값이라, 여기만 바꿔도 은신 동선이 크게 달라집니다.",
 prefixes:&["hd_"] },
 FlowGroup{ label:"아군 지원스킬 낭비 방지",
 note:"체력이 넉넉한 아군에게 지원 스킬을 허비하지 않게 거르는 필터입니다. 체력 상한을 내리면 진짜 위급할 때만 씁니다. 대상 주변에 적이 없으면 아예 안 씁니다.",
 prefixes:&["c3_"] },
 FlowGroup{ label:"모르가드 · 세르펜 사냥",
 note:"에픽 몬스터를 사냥할 때 붙는 거리와 포기 조건입니다. 이 판단은 팀 전술 '마무리'를 읽는 단 두 곳 중 하나이기도 합니다.",
 prefixes:&["eh_"] },
 ] },

 FlowNode{ no:"5", title:"후보 중 하나를 고른다 — 경매", sub:"거르기 → 점수 → 배율 → 뽑기",
 body:"후보를 한 틱 앞서 돌려보고 의미 없는 것을 버린 뒤, 점수를 매기고 배율을 곱해 하나를 뽑습니다.\n\
 여기서 판단력이 2차로 개입합니다 — 행동 부류 11개의 가중치가 판단이 바뀔 때마다 새로 굴려집니다.\n\
 판단력 100이면 흔들림 0, 0이면 부류마다 0.55~1.45배로 제멋대로가 됩니다.",
 groups:&[
 FlowGroup{ label:"시전 후보 검열",
 note:"평타·스킬 후보를 경매에 올리기 전에 거릅니다 — 갈 자리가 위험한가, 곧 받을 피해가 큰가, 사거리가 닿는가.",
 prefixes:&["cf_"] },
 FlowGroup{ label:"재경매 — 골라놓고 다시 겨루기",
 note:"접근·추격을 골랐어도 조건이 맞으면 공격 후보와 한 번 더 겨룹니다.",
 prefixes:&["re_"] },
 FlowGroup{ label:"경매 중 강제 귀환 (0.5.4 신설)",
 note:"경매가 도는 동안 \"지금 도망칠 피해를 못 견딘다\"고 보이면 다른 모든 후보를 제치고 기지 코너로 물러납니다.",
 prefixes:&["auc_"] },
 FlowGroup{ label:"주변 머릿수 배율 — 쫓아가기",
 note:"주변의 적 수에서 아군 수를 뺀 값에 따라 점수에 배율을 곱합니다. 방향을 헷갈리기 쉬운데 배율이 가장 낮은 0.30배 쪽이 '아군이 많을 때'입니다 — 아군이 넉넉하면 굳이 한 명을 물고 늘어지지 않고, 적이 몰려 있을수록 달라붙는다는 뜻입니다. 다섯 값 중 대등(0.80)을 올리는 것이 체감이 가장 큽니다.",
 prefixes:&["sc_adv_"] },
 FlowGroup{ label:"주변 머릿수 배율 — 도망·귀환 (별도 표)",
 note:"도망과 귀환에는 위와 다른 배율표가 쓰입니다. 원본은 40 / 75 / 100 / 200 / 300으로 폭이 훨씬 넓어 머릿수에 민감합니다. 특히 적이 2명 이상 많을 때가 3배라, 이 값 하나가 '포위당하면 즉시 뺀다'를 만듭니다.",
 prefixes:&["mv0_adv_"] },
 FlowGroup{ label:"몇 명을 이 싸움으로 셀지",
 note:"위 배율을 정할 때의 반경입니다. 키 이름과 실제가 반대라 주의하세요 — 적은 150000까지(보이는 적만), 아군은 100000까지 셉니다. 즉 적을 아군보다 넓게 세므로 AI는 기본적으로 상황을 불리하게 봅니다.",
 prefixes:&["sc_ally_radius", "sc_enemy_radius"] },
 FlowGroup{ label:"도망 점수의 가중치와 보너스",
 note:"도망 점수는 위험이 줄어드는 양과 맞을 것 같은 양을 각각 나눠서 더합니다. 나누는 값이 작을수록 그 항이 커지므로, 두 shift 값이 도망 성향을 통째로 좌우하는 가장 굵은 손잡이입니다. 기본 감점(−2)을 0으로 하면 도망을 억제하던 장치가 사라집니다.",
 prefixes:&["mv0_risk_shift", "mv0_engage_shift", "mv0_base_penalty", "mv0_near_"] },
 FlowGroup{ label:"포탑이 점수에 끼치는 영향",
 note:"아군 포탑이 대상을 때려줄 수 있으면 가산점, 적 포탑에 노출되면 감점입니다. 사거리 여유분을 올리면 포탑을 덜 의식하고, 상한을 올리면 포탑 유무가 판단을 더 크게 좌우합니다.",
 prefixes:&["mv_tower_", "mv2_gain_shift", "mv_engage_thr", "vis_mem_global"] },
 FlowGroup{ label:"행동의 실익을 어떻게 계산하나",
 note:"점수는 네 가지를 더한 값입니다 — 아군 포탑이 지켜주는가(+) · 내가 위험해지는가(−) · 가려는 자리가 위험한가(−) · 이 행동의 실익. 실익은 (피해 + 보정) × 목표 우선순위 ÷ 대상 체력이라 체력이 적은 적일수록 점수가 급격히 커집니다. 여기 값들은 그 계산에 쓰이는 인식 범위입니다.",
 prefixes:&["sc_turret_radius", "sc_engage_radius", "sc_cell_dist", "sc_dive_margin", "sc_score_vision"] },
 FlowGroup{ label:"위험하다고 보는 기준",
 note:"체력이 낮을수록 적은 피해에도 위험으로 판정하는 계단식 규칙입니다. 원본은 체력 66% 밑이면 예상 피해 29%, 41% 밑이면 17%, 26% 밑이면 9%만 넘어도 위험으로 봅니다. 기준을 올리면 웬만해선 위험으로 안 봐서 공격적이 되고, 내리면 조금만 아파도 물러납니다.",
 prefixes:&["sc_risk_"] },
 FlowGroup{ label:"보너스 상한",
 note:"죽일 수 있는 적과 아군이 이미 노리는 적에게 얼마나 더 끌릴지의 최대치입니다. 집중포화를 올리면 한 명에게 몰리고, 처치각을 올리면 마무리에 집착합니다.",
 prefixes:&["sc_focus_cap", "sc_kill_cap", "sc_kill_pct", "sc_null_score"] },
 FlowGroup{ label:"점수 보정",
 note:"근접 대상 선호, 오브젝트 확인 선호, 그리고 후보를 살려둘 점수 하한입니다. 하한을 낮추면 어중간한 행동도 후보로 남아 선택지가 넓어집니다.",
 prefixes:&["sc_"] },
 FlowGroup{ label:"자리가 얼마나 위험한가 (자리 평가)",
 note:"도망·추적·접근 점수의 재료가 여기서 만들어집니다. 지형·포탑·미니언·투사체·적 스킬 궤적을 모아 '내 체력의 몇 %가 날아갈 자리인가'로 환산합니다. 무엇까지 셀지(반경)와 위협 하나의 상한이 성향을 크게 좌우합니다. 특히 적 챔피언을 무서워하기 시작하는 거리가 체감이 큽니다.",
 prefixes:&["pe_collect_radius", "pe_champ_threat", "pe_minion_add", "pe_filter_radius", "pe_near_cut", "pe_field_radius", "pe_count_radius"] },
 FlowGroup{ label:"자리 평가 — 거리 여유와 감쇠",
 note:"스킬이 닿는다고 보는 여유, 스킬 궤적을 피할 폭, 아군이 대신 맞아준다고 인정하는 폭입니다. 궤적 폭을 키우면 스킬을 크게 돌아 피하고, 몸빵 폭을 키우면 아군 뒤에 숨기 쉬워집니다.",
 prefixes:&["pe_reach_bonus", "pe_skillshot_width", "pe_bodyblock_width", "pe_outer_band", "pe_tower_margin"] },
 FlowGroup{ label:"자리 평가 — 상한과 배율",
 note:"위협 하나가 낼 수 있는 위험의 상한과 각종 계수입니다. 상한을 올리면 강한 적 한 명이 판단을 독점하고, 내리면 여러 위협이 고르게 반영됩니다. 벽과 적 본진 값은 건드리지 않는 편이 좋습니다.",
 prefixes:&["pe_source_cap", "pe_predict_cap", "pe_tower_far", "pe_kind_scale", "pe_wall_risk", "pe_well_risk", "pe_ally_gain_cut", "pe_state_gate", "pe_mode_mask", "pe_kind_mask"] },
 FlowGroup{ label:"자리 판단의 흔들림 (판단력 3번째 장치)",
 note:"판단력이 낮으면 자리의 위험을 잘못 봅니다. 오판 게이트·성향 흔들림과는 또 다른 세 번째 장치입니다. 흔들림 폭을 0으로 하면 모든 선수가 위험을 정확히 봅니다. 다만 포탑 위험에는 원래 흔들림이 안 걸립니다 — 확정된 위협은 누구나 정확히 본다는 설계입니다.",
 prefixes:&["pe_noise_"] },
 FlowGroup{ label:"라인 수비 후보 점수",
 note:"라인 수비에서 만든 후보를 점수로 걸러냅니다. 후보가 잘려 나가는 가장 흔한 이유는 포지셔닝이 나빠서가 아니라 대상이 이미 사라졌기 때문입니다. 그 감점의 절댓값을 줄이면 사라진 대상을 노리던 후보도 살아남습니다.",
 prefixes:&["ldsc_"] },
 FlowGroup{ label:"위험 수치를 만드는 값",
 note:"자리 평가가 쓰는 재료를 만드는 곳입니다. 주변의 적을 훑어 평타·스킬 사거리의 띠를 만들고, 피해는 미리 만들어 둔 표에서 꺼내 '내 체력의 몇 %'로 환산한 뒤 적 하나당 상한에서 자릅니다. 훑는 반경을 줄이면 시야 밖 위협에 둔감해져 과감해지고, 상한을 내리면 강한 적 한 명을 덜 무서워합니다.",
 prefixes:&["th_"] },
 FlowGroup{ label:"라인 수비 후보 점수 — 접근·미니언 자리",
 note:"이동 계열 후보를 직접 채점합니다. 확실히 처치할 수 있는 대상이 있으면 가장 큰 가산이 붙어 막타 집착을 좌우합니다. 위험·이득을 얼마나 줄여서 반영할지도 여기서 정합니다.",
 prefixes:&["ae_"] },
 FlowGroup{ label:"전투 실익 계산",
 note:"스킬 한 방의 가치를 매깁니다. 상한에서 잘리고, 반경 안 아군 수에 따라 집중포화 배율이 붙고, 다른 아군이 이미 죽일 수 있는 만큼은 감점됩니다. 피해로 값을 매길 수 없는 특수 효과는 별도의 고정 점수표를 씁니다.",
 prefixes:&["bv_"] },
 FlowGroup{ label:"게임 원본의 결함 고치기",
 note:"원본 게임의 계산 결함을 선택적으로 고칩니다. 체크를 풀면 원본과 완전히 같습니다. 두 가지가 있습니다 — 적의 두 번째 스킬 피해가 위험 계산에서 빠지는 문제(켜면 전체적으로 조금 더 몸을 사립니다), 그리고 체력 비례 스킬의 피해가 예측에서 빠지는 문제(실제 피해량은 그대로이고 AI 판단만 바뀌며, 뱀파이어 등 소수 선수에게만 영향).",
 prefixes:&["fix_"] },
 FlowGroup{ label:"행동 성향 흔들림",
 note:"판단력이 낮은 선수의 성향이 들쭉날쭉한 이유입니다. 행동 부류별 가중치를 판단이 바뀔 때마다 새로 굴리는데, 이걸 끄면 모든 선수가 판단력과 무관하게 일관된 성향이 됩니다.",
 prefixes:&["au_"] },
 ] },

 FlowNode{ no:"6", title:"고른 행동을 실행한다", sub:"입력으로 바꿔 실제 움직임",
 body:"선수는 매 틱 판단하지 않습니다. 판단력에 따라 4~8틱마다 한 번 생각하고,\n\
 고른 행동도 최소 10틱은 유지합니다. 단 맞아서 체력이 줄거나 주변 적 수가 바뀌거나\n\
 시전이 끝나면 그 자리에서 다시 뽑습니다.",
 groups:&[
 FlowGroup{ label:"어디로 걸어갈지 (0.5.4 신설)",
 note:"목적지가 정해진 뒤 실제 경로를 찾는 층입니다. 직교·대각 한 칸 비용과 위험지대 우회 칸 수가 동선을 정합니다. ⚠비용을 원본보다 낮추면 최단경로 보장이 깨집니다(올리는 쪽은 안전).",
 prefixes:&["path_"] },
 FlowGroup{ label:"실제 이동 만들기 (모든 이동이 통과)",
 note:"고른 행동이 이동이면 목적지 좌표만 정해진 채 이 한 곳을 통과해 실제 이동 입력이 됩니다. 그래서 도망·추적·접근·자리잡기에 전부 동시에 영향을 줍니다. 도착 판정 거리를 올리면 막판에 파고들고, 내리면 끝까지 남을 피해 돌아갑니다. 우물 탈출은 판단보다 먼저 걸립니다.",
 prefixes:&["mv2_arrive_snap", "mv2_avoid_", "mv2_well_", "mv2_pos_mode_thr"] },
 FlowGroup{ label:"행동을 얼마나 자주·오래 붙잡나",
 note:"재판단 주기와 행동 유지 시간입니다. 유지 시간을 늘리면 뚝심이 생겨 왔다갔다가 줄지만 상황 변화에 둔해집니다. 재판단 주기를 줄이면 민감해지는 대신 연산이 늘어납니다.",
 prefixes:&["ex_order_hold", "ex_think_", "ex_fail_"] },
 FlowGroup{ label:"스킬 해금 레벨",
 note:"스킬2는 3레벨, 궁은 5레벨에 열립니다. 이 두 숫자는 게임 데이터가 아니라 코드에 박혀 있어 여기서만 바꿀 수 있습니다. 올리면 성장 곡선이 느려집니다. ⚠낮추는 것은 위험합니다 — 그 슬롯이 빈 챔피언에서 게임이 죽을 수 있습니다.",
 prefixes:&["ex_skill2_level", "ex_ult_level", "ex_skill2_level_x", "ex_ult_level_x"] },
 FlowGroup{ label:"기본공격 접근·대상 선택",
 note:"기본공격은 스킬과 다른 경로로 갑니다. 사거리 검사 없이 대상을 자동으로 고르고 사거리 안까지 붙는데, 접근 여유를 줄이면 사거리 끝에서 멈춰 카이팅처럼 움직입니다.",
 prefixes:&["ex_attack_"] },
 FlowGroup{ label:"이동 도착·추격 판정",
 note:"수풀에 '도착했다'고 볼 거리, 은신 근접 판정, 추격을 유지할 거리입니다.",
 prefixes:&["mv_"] },
 ] },

 FlowNode{ no:"—", title:"그 밖의 설정", sub:"위 단계에 배정되지 않은 항목",
 body:"엔진·진단·대체 스택 등 판단 흐름 바깥의 항목입니다.",
 groups:&[
 FlowGroup{ label:"대체 스택 — 어느 판단을 모드가 대신할지",
 note:"켜면 그 판단을 게임 원본 대신 모드의 재구현이 처리합니다. 끄면 게임 원본이 그대로 돕니다. \
⚠대체를 끄면 그 판단에 딸린 노브들도 함께 무효가 됩니다.",
 prefixes:&["mp_repl","dd7_repl","recall_repl","engage_repl","cond_repl","d12_repl","d14_repl","poke_repl","nx_repl","d4_repl","d7_repl","d15_repl","e9jt"] },
 FlowGroup{ label:"진단 · 계측",
 note:"개발용입니다. 켜면 진단 파일이 쌓이고 경기가 느려질 수 있습니다. 배포 기본은 전부 꺼짐입니다.",
 prefixes:&["perf_measure","read_bench","probe","replay_reset","sp_seen","sp_seen_tag","hang_","judge_dump","log","skip_untuned"] },
 FlowGroup{ label:"엔진 · 기타",
 note:"읽기 경로 같은 엔진 설정과, 어느 단계에도 속하지 않는 항목입니다.",
 prefixes:&["fast_","self_team_only"] },
 FlowGroup{ label:"아직 배정되지 않음",
 note:"여기가 비어 있는 것이 정상입니다 — 항목이 보이면 위 단계 중 하나에 배정해야 한다는 뜻입니다.",
 prefixes:&["*"] } ] },
];

// ============================ 렌더 ============================
const BLUE: egui::Color32 = egui::Color32::from_rgb(0x5b, 0x9d, 0xff);
const GREEN: egui::Color32 = egui::Color32::from_rgb(0x48, 0xc7, 0x8e);

impl eframe::App for App {
  fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    ctx.set_visuals(egui::Visuals::dark());   // OS 라이트테마 추종 무시, 항상 다크
    // 기본 글자가 작아 눈이 피로하다는 지적 → 전역으로 한 단계 키움(탭·순서도 공통)
    ctx.style_mut(|st| {
      use egui::{FontId, TextStyle};
      st.text_styles.insert(TextStyle::Body,      FontId::proportional(15.5));
      st.text_styles.insert(TextStyle::Button,    FontId::proportional(15.5));
      st.text_styles.insert(TextStyle::Small,     FontId::proportional(13.0));
      st.text_styles.insert(TextStyle::Monospace, FontId::monospace(14.0));
      st.text_styles.insert(TextStyle::Heading,   FontId::proportional(21.0));
      // 글자만 키우고 간격을 그대로 두면 줄이 붙어 보인다 → 위젯 간격·행 높이도 같이 올린다.
      st.spacing.item_spacing    = egui::vec2(9.0, 7.0);
      st.spacing.button_padding  = egui::vec2(8.0, 4.0);
      st.spacing.interact_size.y = 26.0;
      st.spacing.indent          = 22.0;
    });
    // ── 상단 헤더 ──
    egui::TopBottomPanel::top("hdr").show_inside(root, |ui| {
      ui.add_space(4.0);
      ui.horizontal(|ui| {
        ui.heading("tfm2_ai_adjust 설정 편집기");
        ui.add_space(10.0);
        ui.label(egui::RichText::new(format!("변경 {}", self.changed_count())).color(BLUE));
        ui.add_space(16.0);
        ui.label(egui::RichText::new("보기").weak());
        if ui.selectable_label(!self.view_flow, "탭").clicked() { self.view_flow = false; }
        if ui.selectable_label(self.view_flow, "순서도")
          .on_hover_text("판단이 흘러가는 순서대로 보고, 그 자리에서 값을 고칩니다 (실험)").clicked() { self.view_flow = true; }
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

    // ── 순서도 보기(실험): 탭 대신 판단 흐름 순서대로 ──
    if self.view_flow {
      egui::CentralPanel::default().show_inside(root, |ui| { self.flow_ui(ui); });
      self.overlays(ctx);
      return;
    }

    // ── 좌측 탭 ──
    egui::SidePanel::left("nav").resizable(false).exact_width(220.0).show_inside(root, |ui| {
      ui.add_space(6.0);
      ui.label(egui::RichText::new("탭").weak());
      ui.add_space(2.0);
      // ★탭이 많아 창이 작으면 아래쪽이 잘린다 → 목록 자체를 스크롤 영역으로.
      egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        // ★[08-07] 클래스 편집 중에는 **그 탭에서 지정할 수 있는 항목 수**를 함께 보여준다.
        //   없는 탭은 흐리게 — 들어가 봐야 비어 있는 걸 아는 헛클릭을 막는다.
        let cls_mode = self.active_class >= 0 && !self.class_show_all;
        for (i, t) in TABS.iter().enumerate() {
          let title = html_to_text(t.title);
          let label = if cls_mode {
            let n = t.keys.iter().filter(|k| !k.starts_with('§') && class_capable(k)).count();
            if n == 0 {
              egui::RichText::new(format!("{}  ·", title)).weak()
            } else {
              egui::RichText::new(format!("{}  {}", title, n))
            }
          } else {
            egui::RichText::new(title)
          };
          if ui.selectable_label(self.active_tab == i, label).clicked() {
            self.active_tab = i;
          }
        }
      });
    });

    // ── 본문 ──
    egui::CentralPanel::default().show_inside(root, |ui| {
      let tab: &'static Tab = &TABS[self.active_tab];
      egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgb(0x1c, 0x1f, 0x27))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(0x33,0x39,0x47)))
        .show(ui, |ui| {
          // ★[08-05] 안내문이 길면 설정 목록을 아래로 밀어낸다 → **10줄까지만** 보이고 그 안에서 스크롤.
          //   줄 높이는 현재 테마의 본문 글꼴에서 뽑는다(글꼴 크기를 바꿔도 10줄이 유지되게).
          let lh = egui::TextStyle::Body.resolve(ui.style()).size * 1.45;
          egui::ScrollArea::vertical()
            .id_salt(("tabnote", tab.id))
            .max_height(lh * NOTE_MAX_LINES)
            .auto_shrink([false, true])
            .show(ui, |ui| {
              ui.label(egui::RichText::new(html_to_text(tab.note)).color(egui::Color32::from_rgb(0x9a,0xa3,0xb2)));
            });
        });
      // ★[08-07] 클래스 편집 중에는 **지정할 수 있는 항목만** 보여준다.
      //   불가 항목(게임 코드 상수를 직접 고치는 방식이라 선수별로 못 나눔)이 332개나 되어,
      //   전부 늘어놓으면 정작 손댈 수 있는 123개를 찾을 수가 없다.
      //   §머리글은 그 아래에 보여줄 항목이 하나라도 있을 때만 남긴다(빈 머리글 방지).
      let vis: Vec<&str> = if self.active_class < 0 || self.class_show_all {
        tab.keys.to_vec()
      } else {
        let mut out: Vec<&str> = Vec::new();
        for (i, &k) in tab.keys.iter().enumerate() {
          if k.starts_with('§') {
            let has = tab.keys[i + 1..].iter()
              .take_while(|x| !x.starts_with('§'))
              .any(|x| class_capable(x));
            if has { out.push(k); }
          } else if class_capable(k) {
            out.push(k);
          }
        }
        out
      };
      let hidden = tab.keys.iter().filter(|k| !k.starts_with('§')).count()
                 - vis.iter().filter(|k| !k.starts_with('§')).count();

      if self.active_class >= 0 {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
          ui.label(egui::RichText::new(format!(
            "▶ '{}' 클래스 편집 중 — 항목의 '기본 따름'을 끄면 그 항목만 이 클래스 전용 값. (켜짐=전역값 상속, 저장 안 됨)",
            CLASS_KR[self.active_class as usize])).color(GREEN));
          if hidden > 0 || self.class_show_all {
            ui.checkbox(&mut self.class_show_all, "지정 불가 항목도 보기")
              .on_hover_text("이 클래스에서 값을 줄 수 없는 항목까지 함께 표시합니다(읽기 전용으로 보입니다).");
          }
        });
        if hidden > 0 && !self.class_show_all {
          ui.label(egui::RichText::new(format!(
            "   이 탭에서 클래스별로 지정할 수 있는 항목만 보이는 중입니다 (지정 불가 {}개 숨김).", hidden))
            .small().weak());
        }
      }
      ui.add_space(8.0);

      if self.active_class >= 0 && vis.iter().all(|k| k.starts_with('§')) {
        ui.add_space(12.0);
        ui.label(egui::RichText::new("이 탭에는 클래스별로 지정할 수 있는 항목이 없습니다.")
          .color(egui::Color32::from_rgb(150, 150, 150)));
        ui.label(egui::RichText::new(
          "여기 있는 값들은 게임 코드의 상수를 직접 고치는 방식이라, 선수마다 다른 값을 줄 수 없습니다.\n\
           '기본(전역)' 탭에서 전체 공통 값으로는 바꿀 수 있습니다.").small().weak());
        return;
      }

      egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        egui::Grid::new("fields").num_columns(3).striped(true).spacing([14.0, 8.0]).show(ui, |ui| {
          for &k in &vis {
            if let Some(h) = k.strip_prefix('§') {
              // ★위 간격용 빈 행 (Grid 셀 내 add_space는 같은 행이라 안 벌어짐 → 별도 행으로)
              ui.add_space(18.0);
              ui.end_row();
              // ★[08-05] 머리글도 **1열 폭에 못 박는다**. 예전엔 그냥 label 이라
              //   긴 머리글이 1열을 늘려 탭마다 입력칸·설명칸 x좌표가 달라졌다.
              ui.scope(|ui| {
                ui.set_min_width(COL1_W);
                ui.set_max_width(COL1_W);
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                ui.label(egui::RichText::new(h).strong().color(egui::Color32::from_rgb(0xcd,0xd3,0xdf)));
              });
              ui.label("");
              ui.label("");
              ui.end_row();
              continue;
            }
            let cur = self.get_val(k);
            let def = self.defaults.get(k).cloned();
            let changed = def.as_ref().map_or(false, |d| d != &cur)
                && orig_val(k).map_or(true, |o| o != cur);

            // 1열: 키 + 기본값 + 신규배지 (고정폭 — "기본 …" 큰 숫자도 안 잘리게)
            ui.vertical(|ui| {
              ui.set_min_width(COL1_W);
              ui.set_max_width(COL1_W);
              ui.horizontal(|ui| {
                let mut t = egui::RichText::new(disp_key(k)).strong().monospace();
                if changed { t = t.color(BLUE); }
                ui.label(t);
                // 상태 뱃지만 표시(값 무반영/미발화 격리). ★"신규" 뱃지는 달지 않는다 —
                //   유저 지시(2026-08-03): 추가분을 구분 표시하지 말 것. 새 키도 기존 키와 동등하게 보여야 한다.
                if is_dead(k) {
                  ui.label(egui::RichText::new("폐기").small().strong().color(egui::Color32::from_rgb(0xe0,0x6c,0x6c)));
                } else if is_unfired(k) {
                  ui.label(egui::RichText::new("미확인").small().weak());
                }
              });
              ui.label(egui::RichText::new(base_line(k, &def)).small().weak());
            });

            // 2열: 컨트롤 — ★[08-05] 컨트롤 종류마다 폭이 달라 탭마다 열이 밀렸다.
            //   1열처럼 **고정폭 컨테이너**로 감싸 어느 탭에서도 같은 x에 오게 한다.
            ui.vertical(|ui| {
            ui.set_min_width(COL2_W);
            ui.set_max_width(COL2_W);
            if self.active_class >= 0 && !class_capable(k) {
              // ★[08-07] 이 노브는 바이트패치 전용 = 클래스별 값이 원리상 적용될 수 없다.
              //   칸을 내주면 값이 들어가고, 그 값은 효과 없이 skip_untuned 최적화만 꺼서
              //   재생을 멈춘다(08-06 사고). 그래서 입력 자체를 막고 이유를 보여준다.
              ui.label(egui::RichText::new("클래스별 지정 불가 (전체 공통)")
                .color(egui::Color32::from_rgb(150, 150, 150)).italics())
                .on_hover_text("이 항목은 게임 코드의 상수를 직접 고치는 방식이라 선수마다 다른 값을 줄 수 없습니다. 전체 공통 값만 쓰입니다.");
            } else if self.active_class >= 0 {
              // ── 클래스 오버라이드 모드: '기본 따름' 토글 + 전용값 입력 ──
              let pos = self.active_class as usize;
              let pk = format!("{}_class_{}", k, CLASS_EN[pos]);
              let was = self.model.map.contains_key(&pk);
              // ★[08-07] 상속칸이 **공백으로 보이던 문제**. 전역 모드는 `shown_val()` 로
              //   "cfg 에 줄이 없거나 -1(=원본 유지)" 일 때 **게임 원본값을 대신 보여주는데**,
              //   여기만 `cur`(=cfg 원문)을 그대로 써서 빈칸이 나왔다(교전 진입·합류 탭처럼
              //   cfg 에 줄이 없는 항목이 많은 탭에서 전부 빈칸).
              //   ⚠더 위험했던 건 아래 '기본 따름' 해제 시 초깃값 — 빈칸이면 **"0" 을 넣고** 있었다.
              //   0 은 게임 원본이 아니라 그냥 0 이라, 체크만 풀어도 회피계수 0 같은 값이 저장된다.
              let gval = shown_val(k, &cur);          // 전역 현재값(없으면 원본값으로 폴백)
              let mut inherit = !was;
              let mut new_val: Option<String> = None;
              ui.horizontal(|ui| {
                ui.checkbox(&mut inherit, "기본 따름");
                if inherit {
                  let mut gv = gval.clone();
                  ui.add_enabled(false, egui::TextEdit::singleline(&mut gv)
                    .desired_width(COL2_W - 96.0).font(egui::TextStyle::Monospace));
                } else {
                  // ★체크를 막 푼 순간의 초깃값 = **상속받던 그 값**(위 gval, 원본값 폴백 포함).
                  //   예전엔 gval 이 비면 "0" 을 넣어, 체크만 풀어도 원본과 무관한 0 이 저장됐다.
                  let mut v = if was { self.get_val(&pk) } else { gval.clone() };
                  let resp = ui.add_sized([COL2_W - 96.0, 24.0],
                    egui::TextEdit::singleline(&mut v).font(egui::TextStyle::Monospace));
                  if resp.changed() || !was { new_val = Some(v.trim().to_string()); }
                }
              });
              if inherit { if was { self.remove_key(&pk); } }
              else if let Some(v) = new_val { self.set_val(&pk, &v); }
            } else if let Some(opts) = select_opts(k) {
              let mut sel = cur.clone();
              let shown = opts.iter().find(|(v, _)| *v == sel).map(|(_, l)| *l).unwrap_or(&sel).to_string();
              egui::ComboBox::from_id_salt(k).selected_text(shown).width(COL2_W - 6.0).show_ui(ui, |ui| {
                for (v, l) in opts { ui.selectable_value(&mut sel, v.to_string(), *l); }
              });
              if sel != cur { self.set_val(k, &sel); }
            } else if is_toggle(k) {
              let mut on = cur == "1" || cur == "true";
              // ★라벨은 checkbox 뒤에서 만든다(위 동일 사유).
              if ui.checkbox(&mut on, "").changed() {
                self.set_val(k, if on { "1" } else { "0" });
              }
              ui.label(if on { "켜짐" } else { "꺼짐" });
            } else if is_dead(k) || is_unfired(k) {
              // 값 무반영(폐기)/미발화(미확인) — 편집 불가(회색), set_val 호출 안 함.
              let mut v = shown_val(k, &cur);
              ui.add_enabled(false, egui::TextEdit::singleline(&mut v)
                .desired_width(COL2_W - 6.0).font(egui::TextStyle::Monospace));
            } else {
              // ★[08-06] 입력 중에는 자동 채움 금지(위와 동일 사유).
              let tid = ui.make_persistent_id(("valbox_cls", k));
              let focused = ui.ctx().memory(|m| m.has_focus(tid));
              let mut v = if focused { cur.clone() } else { shown_val(k, &cur) };
              let resp = ui.add_sized([COL2_W - 6.0, 24.0],
                  egui::TextEdit::singleline(&mut v).id(tid).font(egui::TextStyle::Monospace));
              if resp.changed() { self.set_val(k, v.trim()); }
            }
            });

            // 3열: 설명 — min/max 를 같이 줘야 폭이 흔들리지 않는다.
            ui.scope(|ui| {
              ui.set_min_width(COL3_W);
              ui.set_max_width(COL3_W);
              ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
              Self::para(ui, &self.desc_of(k), egui::Color32::from_rgb(0x9a,0xa3,0xb2), None);
            });
            ui.end_row();
          }
        });
      });
    });

    self.overlays(ctx);
  }
}

impl App {
  /// 저장 모달 + 토스트 — 탭 보기와 순서도 보기가 공유.
  fn overlays(&mut self, ctx: &egui::Context) {
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
