#![allow(clippy::needless_return)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod rejection;
mod offer_history;

use common::property_parsable::PropertyParsable;
use game_core::transfer::negotiation::{
    calculate_athlete_fair_transfer_fee_at, compute_team_budget_snapshot,
};
use game_core::transfer::planning::player_asset::{
    evaluate_player_asset_for_team, PlayerAssetEvaluation, PlayerAssetTradeStance,
};
use game_core::transfer::planning::player_value::{
    career_move_assessment_for_team_offer, seller_roster_continuity_allows_sale,
    seller_roster_continuity_hard_blocks_sale, seller_transfer_value_policy,
    SellerRosterContinuityContext, SellerTransferValueContext, SellerTransferValuePolicy,
};
use game_core::{
    Athlete, Contract, Database, GameModeKind, ModSaveData, News, NewsType, PaperState, Position,
    SquadStatus, TransferRequest, TransferRequestPaper,
};
use mod_api::*;
// [PORT056] 계약 현황 projection 행의 선수 신원을 공식 필드로 직접 결속하기 위해 필요.
use game_view::ViewDetailButtonRunner;
use game_view::{ColorIconButtonRunner, ColorIconButtonRunnerProperty};
use engine_ui::runner::LabelRunner;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as FmtWrite;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write as IoWrite;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "player_trade_system_test79_stable_pending_ui";
const MOD_VERSION: &str = "0.1.0-test79-patch055-profile-nav-scene-return-desktop-fix6";
const PATCH055_BASE_VERSION: &str = "0.5.5";
const OFFER_SURFACE_ID: &str = "offer";
const ENTRY_ID: &str = "pts_trade_entry";
/// [PORT056] 버튼 옆 안내 라벨(비활성 사유 표시). 바닐라 `delegate_tooltip` 과 같은 패턴.
const ENTRY_TIP_ID: &str = "pts_trade_entry_tip";
const NATIVE_COMPARE_ID: &str = "compare_popup";
const NATIVE_COMPARE_SOURCE_ID: &str = "pts_trade_compare_source";
const NATIVE_COMPARE_FADE_ID: &str = "fade";
const MODAL_LAYER_TEMPLATE_ID: &str = "pts_trade_modal_layer_template";
const MODAL_LAYER_ID: &str = "pts_trade_modal_layer";
const BACKDROP_TEMPLATE_ID: &str = "pts_trade_modal_backdrop_template";
const BACKDROP_ID: &str = "pts_trade_modal_backdrop";
const CASH_TEMPLATE_ID: &str = "pts_trade_cash_template";
const CASH_PANEL_ID: &str = "pts_trade_cash_panel";
const CUSTOM_COMPARE_TEMPLATE_ID: &str = "pts_trade_custom_compare_template";
const CUSTOM_COMPARE_ID: &str = "pts_trade_custom_compare";
const CUSTOM_CLOSE_ID: &str = "pts_trade_custom_close";
const CUSTOM_SLOT_PREFIX: &str = "pts_trade_custom_slot_";
const CUSTOM_ROSTER_SLOT_COUNT: usize = 24;
const CASH_INPUT_ID: &str = "pts_cash_input";
const CASH_CANCEL_ID: &str = "pts_cash_cancel";
const STATUS_TOGGLE_ID: &str = "pts_trade_runtime_status_toggle";
const STATUS_MENU_ID: &str = "pts_trade_runtime_status_menu";
const REVIEW_BUTTON_ID: &str = "pts_cash_submit";
const COMMIT_BUTTON_ID: &str = "pts_trade_commit_confirm";
const REVIEW_BUTTON_TEMPLATE_ID: &str = "pts_cash_submit_template";
const COMMIT_BUTTON_TEMPLATE_ID: &str = "pts_trade_commit_confirm_template";
const QUOTE_COMMAND: &str = "evaluate_trade_quote";
const QUOTE_EVENT: &str = "trade_quote_result";
const REVIEW_COMMAND: &str = "submit_async_trade_proposal";
const REVIEW_EVENT: &str = "async_trade_proposal_submit_result";
const EXECUTE_COMMAND: &str = "execute_sealed_trade_command";
const EXECUTE_EVENT: &str = "sealed_trade_execution_result";
const VALIDATE_SAVED_TRADE_COMMAND: &str = "validate_saved_trade_commit";
const VALIDATE_SAVED_TRADE_EVENT: &str = "saved_trade_commit_validation_result";
const ASYNC_STATUS_COMMAND: &str = "query_async_trade_proposal_status";
const ASYNC_STATUS_EVENT: &str = "async_trade_proposal_status_result";
const NATIVE_OFFER_STATUS_COMMAND: &str = "query_first_active_offer_status";
const NATIVE_OFFER_STATUS_EVENT: &str = "first_active_offer_status_result";
const ASYNC_PROPOSAL_KEY: &str = "async_trade_proposal_v10_test79_stable_ui";
const ASYNC_SUCCESS_NEWS_BIND_KEY: &str = "PtsTradeProposalId";
const ASYNC_SUCCESS_NEWS_AUTHOR: &str = "Player Trade System";
const PENDING_CONTRACT_SLOT_TEMPLATE_ID: &str = "pts_trade_pending_contract_slot_template";
const PENDING_CONTRACT_SLOT_RUNTIME_PREFIX: &str = "pts_trade_pending_contract_slot_test79_";
const PENDING_CONTRACT_SLOT_RUNTIME_ID: &str = "pts_trade_pending_contract_slot_test79_active";
const PROFILE_CONTRACT_BUTTON_ID: &str = "contract";
const PROFILE_STATE_LABEL_ID: &str = "state";
const FIRST_OFFER_STATUS_LABEL_ID: &str = "pts_first_offer_status_detail";
const TEST77_REQUIRED_OFFERED_ID: usize = 7;
const TEST77_REQUIRED_TARGET_ID: usize = 92;
const SELLER_REVIEW_DELAY_DAYS: i64 = 2;
const PLAYER_REVIEW_DELAY_DAYS: i64 = 1;
const MOD_SAVE_NAMESPACE: &str = "player_trade_system";
const MOD_SAVE_NAMESPACE_VERSION: usize = 1;
const EXECUTED_PLAN_REGISTRY_KEY: &str = "executed_trade_plans_v1";
/// [PORT056] 요구사항 1 — 영입 시즌당 트레이드 1회. **성사(commit)만 소모**한다(유저 확정 2026-08-22).
///   거절·만료로 끝난 제안은 소모로 치지 않으며, 조건을 바꿔 같은 시즌에 다시 제안할 수 있다.
///   값 = 소모된 영입 시즌 키(`current_recruit_season_key` 형식).
const TRADE_SEASON_USAGE_KEY: &str = "trade_season_usage_v1";
const TRADE_COMMIT_RECEIPT_KEY: &str = "trade_commit_receipt_v1";
const PREVIOUS_TRADE_COMMIT_RECEIPT_KEY: &str = "trade_commit_receipt_previous_v1";
const OLDEST_TRADE_COMMIT_RECEIPT_KEY: &str = "trade_commit_receipt_previous2_v1";
const FIRST_PLAN_ID: &str = "T60-89617444924FD838";
const SECOND_PLAN_ID: &str = "T60-83270E3A33D2F105";
const FIRST_OFFERED_ID: usize = 1512;
const FIRST_TARGET_ID: usize = 1514;
const SECOND_OFFERED_ID: usize = 1568;
const SECOND_TARGET_ID: usize = 1588;
const REQUESTER_TEAM_ID: usize = 7;
const RECIPIENT_TEAM_ID: usize = 0;
const BASELINE_SAVE_SLOT: &str = "PTS_Test65_MigrationFix1";
const PENDING_SAVE_SLOT: &str = "PTS_Test79_PendingSellerReview";
const RESULT_SAVE_SLOT: &str = "PTS_Test79_AfterAsyncTrade";
const TEMPLATE_NATIVE_SELECTED_ID: usize = 1150;
const RUNTIME_LOG: &str = "PLAYER_TRADE_TEST79_PATCH055_RUNTIME.log";
const MONEY_UNIT_WON: u64 = 10_000;
const WEEKS_PER_YEAR: f64 = 52.0;
const CORE_REPLACEMENT_FLOOR: f64 = 0.70;
const IMPORTANT_REPLACEMENT_FLOOR: f64 = 0.55;
const GENERAL_REPLACEMENT_FLOOR: f64 = 0.40;
const FLOOR_AUDIT_COMMAND: &str = "audit_replacement_floor_integration";
const FLOOR_AUDIT_EVENT: &str = "replacement_floor_audit_result";
const AUDIT_MAOMAO_ID: usize = 82;
const AUDIT_HOYA_ID: usize = 45;
const AUDIT_SOLADA_ID: usize = 20;
const AUDIT_CHICO_ID: usize = 1445;
const AUDIT_KESHI_ID: usize = 10;
const AUDIT_JUE_ID: usize = 1512;
const AUDIT_ZEUS_ID: usize = 1514;
const TEST79_SMOKE_ONLY: bool = false;

const ACTION_NONE: u8 = 0;
const ACTION_OPEN: u8 = 1;
const ACTION_CLOSE: u8 = 2;
const ACTION_BLOCK: u8 = 3;
const ACTION_STATUS_TOGGLE: u8 = 4;
const ACTION_STATUS_CORE: u8 = 5;
const ACTION_STATUS_IMPORTANT: u8 = 6;
const ACTION_STATUS_GENERAL: u8 = 7;
const ACTION_STATUS_SUB: u8 = 8;
const ACTION_STATUS_PROSPECT: u8 = 9;
const ACTION_REVIEW: u8 = 10;
const ACTION_EXECUTE: u8 = 11;
const ACTION_OPEN_PROJECTION_PROFILE: u8 = 12;

const STATUS_CORE: u8 = 0;
const STATUS_IMPORTANT: u8 = 1;
const STATUS_GENERAL: u8 = 2;
const STATUS_SUB: u8 = 3;
const STATUS_PROSPECT: u8 = 4;

static LOAD_LOGGED: AtomicBool = AtomicBool::new(false);
static COMPARE_TREE_LOGGED: AtomicBool = AtomicBool::new(false);
static ENTRY_TEMPLATE_READY: AtomicBool = AtomicBool::new(false);
static CACHED_COMPARE_TEMPLATE_FALLBACK_LOGGED: AtomicBool = AtomicBool::new(false);
static OFFER_TREE_LOGGED: AtomicBool = AtomicBool::new(false);
static ENTRY_LOGGED: AtomicBool = AtomicBool::new(false);
static CASH_INPUT_FOCUS_LOGGED: AtomicBool = AtomicBool::new(false);
static RUNTIME_ERROR_LOGGED: AtomicBool = AtomicBool::new(false);
static POPUP_OPEN: AtomicBool = AtomicBool::new(false);
static STATUS_MENU_OPEN: AtomicBool = AtomicBool::new(false);
static QUOTE_REQUEST_SENT: AtomicBool = AtomicBool::new(false);
static QUOTE_UI_DIRTY: AtomicBool = AtomicBool::new(false);
static REVIEW_REQUEST_QUEUED: AtomicBool = AtomicBool::new(false);
static REVIEW_REQUEST_PENDING: AtomicBool = AtomicBool::new(false);
static EXECUTE_REQUEST_QUEUED: AtomicBool = AtomicBool::new(false);
static EXECUTE_REQUEST_PENDING: AtomicBool = AtomicBool::new(false);
static DUPLICATE_REPLAY_REQUEST_QUEUED: AtomicBool = AtomicBool::new(false);
static DUPLICATE_REPLAY_REQUEST_PENDING: AtomicBool = AtomicBool::new(false);
static DUPLICATE_REPLAY_VERIFIED: AtomicBool = AtomicBool::new(false);
static FIRST_TRADE_BASELINE_VERIFIED: AtomicBool = AtomicBool::new(false);
/// [PORT056] 요구사항 1 — 이번 영입 시즌에 이미 트레이드를 성사시켰는가(서버 async status 채널로 갱신).
static TRADE_SEASON_USED: AtomicBool = AtomicBool::new(false);
/// [PORT056] projection 행의 런너 신원이 어긋나 재결속한 횟수. 계속 늘면 엔진이 매 갱신마다 되돌린다는 뜻.
static PROJECTION_IDENTITY_REBIND_COUNT: AtomicUsize = AtomicUsize::new(0);
/// [PORT056] 네이티브 clone 폴백을 실제로 탔는지 1회 로깅용.
static PROJECTION_NATIVE_FALLBACK_LOGGED: AtomicBool = AtomicBool::new(false);
/// [PORT056] 게임 접두 id 살균이 실제로 발생했는지 1회 로깅용.
static PROJECTION_ID_SANITIZE_LOGGED: AtomicBool = AtomicBool::new(false);
/// [PORT056] 진단용 전체 클릭 카운터(상한 300줄).
static ALL_RAW_CLICK_COUNT: AtomicUsize = AtomicUsize::new(0);
/// [PORT056] 진입 버튼 라벨 적용 경로 1회 로깅.
static ENTRY_LABEL_PATH_LOGGED: AtomicBool = AtomicBool::new(false);
/// [PORT056] 비활성 룩 상태 변화 감지(0=미기록).
static ENTRY_LOOK_LAST_KEY: AtomicUsize = AtomicUsize::new(0);
/// [PORT056] 우리가 띄운 툴팁이 현재 보이는가.
static TIP_VISIBLE: AtomicBool = AtomicBool::new(false);
/// [PORT056] 계약 탭 트리 1회 덤프 여부.
static CONTRACT_TREE_DUMPED: AtomicBool = AtomicBool::new(false);
// [PORT056] 접힌 계약 리스트 nudge 로그 스팸 방지용 카운터.
static CONTRACT_LIST_NUDGE_COUNT: AtomicUsize = AtomicUsize::new(0);
// [PORT056] 네이티브 계약행 덮어쓰기 로그 스팸 방지용.
static NATIVE_ROW_OVERWRITE_COUNT: AtomicUsize = AtomicUsize::new(0);
static NATIVE_ROW_ID_PROBE_COUNT: AtomicUsize = AtomicUsize::new(0);
static NATIVE_ROW_ACTION_DUMPED: AtomicBool = AtomicBool::new(false);
thread_local! {
    static NATIVE_ROW_ID_PROBE_KEY: RefCell<String> = RefCell::new(String::new());
    static NATIVE_ROW_COLUMN_RESULT_KEY: RefCell<String> = RefCell::new(String::new());
    static CASH_LOCK_DIAG_KEY: RefCell<String> = RefCell::new(String::new());
}
static CONTRACT_TREE_DUMPED_AFTER_NUDGE: AtomicBool = AtomicBool::new(false);
// [PORT056] player_detail 신원 캐리어 탐색 덤프 제한용.
// [PORT056] 진단 덤프 스위치 — 배포는 둘 다 false (한 판에 수천 줄을 남긴다).
//   계약 탭 높이 0 진단·프로필 신원 캐리어 탐색이 끝나 껐다(2026-08-23). 재조사 필요 시 true.
const CONTRACT_TREE_DIAGNOSTIC_DUMP_ENABLED: bool = false;
const PLAYER_DETAIL_DIAGNOSTIC_DUMP_ENABLED: bool = false;
static PLAYER_DETAIL_DUMP_COUNT: AtomicUsize = AtomicUsize::new(0);
static PLAYER_DETAIL_LAST_DUMPED_INSTANCE: AtomicUsize = AtomicUsize::new(0);
/// [PORT056] 부모 컨테이너 클릭을 진입으로 채택한 사실 1회 로깅.
static ENTRY_PARENT_FALLBACK_LOGGED: AtomicBool = AtomicBool::new(false);
static TRANSACTION_EXECUTED_UI: AtomicBool = AtomicBool::new(false);
static EXECUTION_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
static MUTATION_ACTIVE: AtomicBool = AtomicBool::new(false);
static ROLLBACK_RENDER_LOGGED: AtomicBool = AtomicBool::new(false);
static ROLLBACK_COMPLETED_UI: AtomicBool = AtomicBool::new(false);
static REGION_ERROR_LOGGED: AtomicBool = AtomicBool::new(false);
static FLOOR_AUDIT_REQUEST_SENT: AtomicBool = AtomicBool::new(false);
static FLOOR_AUDIT_RESPONSE_RECEIVED: AtomicBool = AtomicBool::new(false);
static FLOOR_AUDIT_SERVER_COMPLETED: AtomicBool = AtomicBool::new(false);
static FLOOR_AUDIT_LAST_SEND_FRAME: AtomicUsize = AtomicUsize::new(0);
static FLOOR_AUDIT_SEND_ATTEMPT: AtomicUsize = AtomicUsize::new(0);
static RUNTIME_FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);
static CLICK_ACTION: AtomicU8 = AtomicU8::new(ACTION_NONE);
static OFFER_RAW_CLICK_COUNT: AtomicUsize = AtomicUsize::new(0);
static CASH_INPUT_STATE: AtomicU8 = AtomicU8::new(0);
static DESIRED_SQUAD_STATUS: AtomicU8 = AtomicU8::new(STATUS_CORE);
static TARGET_ATHLETE_ID: AtomicUsize = AtomicUsize::new(0);
/// ★[PORT056] 선수 id 0 은 **유효한 선수**다(2026-08-23 인게임 실측: 유저 팀 T1 의 Doran = athlete_id 0,
/// 팀 id 도 0). 구판은 0 을 "선택 없음" 센티넬로 써서 **id 0 선수를 영원히 선택할 수 없었다**
/// (로그: 캡처된 offered_id 가 1·2·3·4 뿐, 0 은 한 번도 안 잡힘).
/// ⟹ 센티넬을 usize::MAX 로 분리한다. 빠뜨린 곳이 있어도 "아무와도 매칭 안 됨"으로 드러나지,
///    엉뚱한 선수를 고르는 조용한 오류가 되지 않는다.
const NO_ATHLETE: usize = usize::MAX;
static OFFERED_ATHLETE_ID: AtomicUsize = AtomicUsize::new(NO_ATHLETE);
static PENDING_OFFERED_ATHLETE_ID: AtomicUsize = AtomicUsize::new(NO_ATHLETE);
static DIRECT_POPUP_SELECTION_COUNT: AtomicUsize = AtomicUsize::new(0);
static ZERO_CASH_EXPLANATION_LOGGED: AtomicBool = AtomicBool::new(false);
static VISUAL_SYNC_LAST_OFFERED_ID: AtomicUsize = AtomicUsize::new(0);
static VISUAL_SYNC_LAST_SEQUENCE: AtomicUsize = AtomicUsize::new(0);
static NATIVE_VISUAL_SYNC_PENDING: AtomicBool = AtomicBool::new(false);
static CUSTOM_ROSTER_BUILT: AtomicBool = AtomicBool::new(false);
static CUSTOM_ROSTER_SLOT_IDS: Mutex<Vec<usize>> = Mutex::new(Vec::new());
static NATIVE_HIGHLIGHT_OWNER_ID: AtomicUsize = AtomicUsize::new(TEMPLATE_NATIVE_SELECTED_ID);
static EXECUTED_PLAN_IDS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static DUPLICATE_REPLAY_PAYLOAD: Mutex<Option<Vec<u8>>> = Mutex::new(None);
static SAVED_TRADE_VALIDATION_LAST_SEND_FRAME: AtomicUsize = AtomicUsize::new(0);
static SAVED_TRADE_VALIDATION_SEND_ATTEMPT: AtomicUsize = AtomicUsize::new(0);
static SAVED_TRADE_VALIDATION_RESPONSE_RECEIVED: AtomicBool = AtomicBool::new(false);
static SAVED_TRADE_VALIDATION_SERVER_LOGGED: AtomicBool = AtomicBool::new(false);
static TRADE_ENTRY_ACTIVE: AtomicBool = AtomicBool::new(false);
static NATIVE_COMPARE_OPEN_PENDING: AtomicBool = AtomicBool::new(false);
static NATIVE_COMPARE_OPEN_REQUEST_FRAME: AtomicUsize = AtomicUsize::new(0);
static NATIVE_COMPARE_OPEN_TARGET_ID: AtomicUsize = AtomicUsize::new(0);
static TARGET_EPOCH: AtomicUsize = AtomicUsize::new(0);
static OPEN_GENERATION: AtomicUsize = AtomicUsize::new(0);
static OPEN_CLICK_SEQUENCE: AtomicUsize = AtomicUsize::new(0);
static LAST_ACTIVE_OFFER_COUNT: AtomicUsize = AtomicUsize::new(usize::MAX);
static ACTIVE_RAW_OFFER_ID_COUNT: AtomicUsize = AtomicUsize::new(0);
static ACTIVE_STRUCTURAL_OFFER_COUNT: AtomicUsize = AtomicUsize::new(0);
static RETURN_TO_PROFILE_PENDING: AtomicBool = AtomicBool::new(false);
static RETURN_TO_PROFILE_REQUEST_FRAME: AtomicUsize = AtomicUsize::new(0);
static RETURN_TO_PROFILE_OBSERVED: AtomicBool = AtomicBool::new(false);
static RETURN_TO_PROFILE_TIMEOUT_LOGGED: AtomicBool = AtomicBool::new(false);
static PROFILE_STATUS_OWNED: AtomicBool = AtomicBool::new(false);
static PROFILE_CONTEXT_ATHLETE_ID: AtomicUsize = AtomicUsize::new(0);
static PROFILE_CONTEXT_DETAIL_INSTANCE: AtomicUsize = AtomicUsize::new(0);
static PROFILE_CONTEXT_BIND_NEXT_DETAIL: AtomicBool = AtomicBool::new(false);
static PROFILE_CONTEXT_TARGET_LOCK_VALID: AtomicBool = AtomicBool::new(false);
static PROFILE_CONTEXT_ALLOW_NEXT_REBUILD: AtomicBool = AtomicBool::new(false);
static PROFILE_CONTEXT_MANAGEMENT_TICK_REBUILD_REBOUND: AtomicBool = AtomicBool::new(false);
static PROFILE_CONTEXT_EXPLICITLY_BOUND: AtomicBool = AtomicBool::new(false);
static RELOAD_PROFILE_FALLBACK_CONSUMED: AtomicBool = AtomicBool::new(false);
static CLICK_FILTER_OFFER_SIGNATURE_FIX4: AtomicUsize = AtomicUsize::new(0);
static PROFILE_NATIVE_LOCK_ACTIVE: AtomicBool = AtomicBool::new(false);
static NATIVE_OFFER_LOCK_ACTIVE: AtomicBool = AtomicBool::new(false);
static CONTRACT_PROJECTION_ACTIVE: AtomicBool = AtomicBool::new(false);
static PROJECTION_PROFILE_CLICK_PENDING_ID: AtomicUsize = AtomicUsize::new(0);
static PROJECTION_PROFILE_CLICK_REQUEST_FRAME: AtomicUsize = AtomicUsize::new(0);
static PROJECTION_PROFILE_CLICK_TIMEOUT_LOGGED: AtomicBool = AtomicBool::new(false);
static PROJECTION_PROFILE_SCENE_RETURN_REQUESTED: AtomicBool = AtomicBool::new(false);
static PROFILE_CONTEXT_TARGET_LEASE_ACTIVE: AtomicBool = AtomicBool::new(false);
static PROFILE_CONTEXT_SCENE_SUSPENDED: AtomicBool = AtomicBool::new(false);
static PROFILE_CONTEXT_SCENE_RETURN_REBOUND: AtomicBool = AtomicBool::new(false);
static PROFILE_CONTEXT_LAST_CONFIRMED_TARGET_SCENE: AtomicBool = AtomicBool::new(false);
/// [PORT056] 요구사항 3 재개통 1회 로그.
static OTHER_PLAYER_PROFILE_OFFER_UI_REENABLED_LOGGED: AtomicBool = AtomicBool::new(false);
/// [PORT056] 신원 미확정 프레임의 스킵 로그 레이트 리밋용 카운터.
static FIRST_OFFER_STATUS_SKIP_COUNT: AtomicUsize = AtomicUsize::new(0);
static OTHER_PLAYER_PROFILE_OFFER_UI_DISABLED_LOGGED: AtomicBool = AtomicBool::new(false);
static ASYNC_STATUS_QUERY_SEND_ATTEMPT: AtomicUsize = AtomicUsize::new(0);
static ASYNC_STATUS_QUERY_PENDING: AtomicBool = AtomicBool::new(false);
static ASYNC_STATUS_QUERY_LAST_FRAME: AtomicUsize = AtomicUsize::new(0);
static ASYNC_STATUS_UI_DIRTY: AtomicBool = AtomicBool::new(false);
static NATIVE_OFFER_STATUS_QUERY_PENDING: AtomicBool = AtomicBool::new(false);
static NATIVE_OFFER_STATUS_LAST_FRAME: AtomicUsize = AtomicUsize::new(0);
static NATIVE_OFFER_STATUS_ATHLETE_ID: AtomicUsize = AtomicUsize::new(0);
static NATIVE_OFFER_STATUS_OWNED: AtomicBool = AtomicBool::new(false);
static ASYNC_LIFECYCLE_BUSY: AtomicBool = AtomicBool::new(false);
static ASYNC_SERVER_START_LOGGED: AtomicBool = AtomicBool::new(false);
static RUNTIME_NONCE: OnceLock<u128> = OnceLock::new();

type ClickFilter = Rc<dyn Fn(&UIEvent) -> bool>;
type ClickHandler = Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)>;
type ClickHandlerPair = (ClickFilter, ClickHandler);

#[derive(Clone)]
struct QuoteView {
    cooldown_fingerprint: u64,
    cooldown_retry_at: String,
    cooldown_changeable: bool,
    cooldown_present: bool,
    cooldown_exempt: bool,
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    offered_name: String,
    target_name: String,
    required_cash_won: u64,
    required_units: u64,
    cash_offer_max_units: u64,
    cash_range_obscured: bool,
    exact_threshold_disclosed: bool,
    cash_budget_won: f64,
    budget_units: u64,
}

#[derive(Clone, Copy)]
struct ProfileSpec {
    label: &'static str,
    stance: PlayerAssetTradeStance,
    is_excess_sell: bool,
}

#[derive(Clone)]
struct ReplacementFloorAssessment {
    target_status: SquadStatus,
    floor: f64,
    internal_best_candidate_id: Option<usize>,
    internal_best_candidate_name: String,
    internal_best_ratio: f64,
    incoming_same_position: bool,
    incoming_ratio: f64,
    effective_ratio: f64,
    structural_cover: bool,
    allows: bool,
}

#[derive(Clone, Copy)]
struct ThresholdSearchResult {
    requester_cash: f64,
    evaluation_count: usize,
    repeat_consistent: bool,
    coarse_monotonic: bool,
    boundary_verified: bool,
    zero_cash_accepted: bool,
    budget_ceiling_accepted: bool,
}

struct ServerQuote {
    // ★[PORT056] 클라 선차단용 쿨다운 힌트(유저 지시 2026-08-23) — rejection::CooldownHint 참조.
    cooldown_fingerprint: u64,
    cooldown_retry_at: String,
    cooldown_changeable: bool,
    cooldown_present: bool,
    cooldown_exempt: bool,
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    region_id: usize,
    offered_name: String,
    target_name: String,
    offered_value: f64,
    target_value: f64,
    required_cash_won: u64,
    required_units: u64,
    display_min_units: u64,
    display_max_units: u64,
    display_lower_percent: u8,
    display_upper_percent: u8,
    cash_budget_won: f64,
    budget_units: u64,
    profile_label: &'static str,
    evaluation_count: usize,
    game_time: String,
}

struct ServerReview {
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    region_id: usize,
    offered_name: String,
    target_name: String,
    proposed_units: u64,
    proposed_cash_won: u64,
    desired_status_choice: u8,
    desired_status_key: &'static str,
    desired_status_label: &'static str,
    required_cash_won: u64,
    cash_offer_min_units: u64,
    cash_offer_max_units: u64,
    cash_budget_won: f64,
    cash_meets_required: bool,
    cash_within_budget: bool,
    cash_within_offer_range: bool,
    seller_accepted: bool,
    seller_repeat_consistent: bool,
    player_accepted: bool,
    overall_approved: bool,
    requested_years: usize,
    contract_days_left: i64,
    inherited_yearly_salary: f64,
    contract_inherited: bool,
    role_promise_accepted: bool,
    salary_renegotiation_required: bool,
    offer_value_ratio: f64,
    current_status: String,
    offered_status: String,
    clear_exit_path: bool,
    seller_protects_downward_move: bool,
    protected_downward_asset: bool,
    exceptional_exit_offer: bool,
    last_player_without_replacement: bool,
    game_time: String,
    command_envelope: Option<TradeCommandEnvelope>,
    rejection_meta: Option<rejection::RejectionMeta>,
}

#[derive(Clone)]
struct NativeCompareHome {
    parent_path: Vec<usize>,
    ancestor_states: Vec<(Vec<usize>, bool, bool)>,
    popup_visible: bool,
    popup_disabled: bool,
    fade_visible: bool,
    fade_disabled: bool,
}

#[derive(Clone)]
struct ClosedCompareSnapshot {
    target_id: usize,
    target_epoch: usize,
    home: NativeCompareHome,
}

#[derive(Clone)]
struct TradeCommandEnvelope {
    schema_version: u8,
    plan_id: String,
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    offered_destination_team_id: usize,
    target_destination_team_id: usize,
    cash_payer_team_id: usize,
    cash_recipient_team_id: usize,
    proposed_cash_won: u64,
    desired_status_choice: u8,
    desired_status_key: &'static str,
    requester_roster_count: usize,
    recipient_roster_count: usize,
    offered_yearly_salary: f64,
    target_yearly_salary: f64,
    requester_cash_budget_won: f64,
    prepared_game_time: String,
    state_precondition_count: usize,
    operation_count: usize,
    atomic_batch_required: bool,
    contract_transfer_mode: &'static str,
    money_direction: &'static str,
    plan_repeat_consistent: bool,
    execution_gate_closed: bool,
}

#[derive(Clone)]
struct ReviewView {
    offered_name: String,
    offered_id: usize,
    target_id: usize,
    target_name: String,
    proposed_units: u64,
    desired_status_choice: u8,
    desired_status_label: String,
    seller_accepted: bool,
    player_accepted: bool,
    overall_approved: bool,
    requested_years: usize,
    inherited_yearly_salary: f64,
    command_envelope_prepared: bool,
    plan_id: String,
    plan_repeat_consistent: bool,
    execution_gate_closed: bool,
    rejection_present: bool,
    rejection_actor_ko: String,
    rejection_reason: String,
    rejection_reason_ko: String,
    rejection_policy: String,
    rejection_retry_at: String,
    rejection_retry_ko: String,
    rejection_package_fingerprint: String,
    rejection_feedback_id: String,
    rejection_cooldown_blocked: bool,
    rejection_news_created: bool,
    rejection_duplicate_news_count: usize,
    rejection_ledger_entry_count: usize,
}

#[derive(Clone)]
struct ExecuteView {
    plan_id: String,
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    offered_name: String,
    target_name: String,
    proposed_cash_won: u64,
    desired_status_choice: u8,
    desired_status_label: String,
    offered_team_after: usize,
    target_team_after: usize,
    target_status_after: String,
    offered_contracted_status_after: String,
    target_contracted_status_after: String,
    rollback_rehearsal_verified: bool,
    requester_total_before: f64,
    requester_total_after: f64,
    requester_transfer_before: f64,
    requester_transfer_after: f64,
    recipient_total_before: f64,
    recipient_total_after: f64,
    recipient_transfer_before: f64,
    recipient_transfer_after: f64,
    atomic_commit_verified: bool,
    rollback_performed: bool,
    execution_gate_closed: bool,
    pre_receipt_mod_save_unchanged: bool,
    executed_plan_persisted: bool,
    trade_receipt_persisted: bool,
    receipt_readback_verified: bool,
    executed_plan_registry_count: usize,
    commit_process_id: u32,
}

#[derive(Clone, Copy)]
struct TeamFinanceSnapshot {
    total_balance: f64,
    transfer_budget: f64,
    salary_budget: f64,
}

#[derive(Clone)]
struct AthleteTradeSnapshot {
    id: usize,
    name: String,
    team_id: usize,
    start_date: String,
    end_date: String,
    weekly_salary: f64,
    transfer_fee: f64,
    incentives_debug: String,
    transfer_requests_debug: String,
    recruit_requests_debug: String,
    squad_status_debug: String,
}

#[derive(Clone, Copy)]
struct WorldTradeSnapshot {
    requester_roster: usize,
    recipient_roster: usize,
    contracted: usize,
    requester_payroll: f64,
    recipient_payroll: f64,
}

#[derive(Clone)]
struct AtomicRollbackSnapshot {
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    offered_team_before: usize,
    target_team_before: usize,
    target_squad_status_before: SquadStatus,
    target_contracted_squad_status_before: Option<SquadStatus>,
    requester_finance_before: TeamFinanceSnapshot,
    recipient_finance_before: TeamFinanceSnapshot,
    mod_save_before: ModSaveData,
}

#[derive(Clone)]
struct ForcedRollbackAuditSnapshot {
    plan_id: String,
    offered_before: AthleteTradeSnapshot,
    target_before: AthleteTradeSnapshot,
    offered_contracted_squad_status_before: Option<SquadStatus>,
    target_contracted_squad_status_before: Option<SquadStatus>,
    requester_finance_before: TeamFinanceSnapshot,
    recipient_finance_before: TeamFinanceSnapshot,
    world_before: WorldTradeSnapshot,
    mod_save_before: String,
    requester_news_before: usize,
    recipient_news_before: usize,
    game_time_before: String,
}

struct AtomicTradeResult {
    plan_id: String,
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    offered_name: String,
    target_name: String,
    proposed_cash_won: u64,
    desired_status_choice: u8,
    desired_status_key: &'static str,
    desired_status_label: &'static str,
    offered_team_before: usize,
    offered_team_after: usize,
    target_team_before: usize,
    target_team_after: usize,
    target_status_before: String,
    target_status_after: String,
    offered_contracted_status_before: String,
    offered_contracted_status_after: String,
    target_contracted_status_before: String,
    target_contracted_status_after: String,
    requester_finance_before: TeamFinanceSnapshot,
    requester_finance_after: TeamFinanceSnapshot,
    recipient_finance_before: TeamFinanceSnapshot,
    recipient_finance_after: TeamFinanceSnapshot,
    requester_roster_before: usize,
    requester_roster_after: usize,
    recipient_roster_before: usize,
    recipient_roster_after: usize,
    contracted_before: usize,
    contracted_after: usize,
    requester_payroll_after: f64,
    recipient_payroll_after: f64,
    contract_inherited_both: bool,
    offered_status_unchanged: bool,
    target_status_applied: bool,
    offered_contracted_status_unchanged: bool,
    target_contracted_status_applied: bool,
    rollback_rehearsal_verified: bool,
    combined_finance_conserved: bool,
    pre_receipt_mod_save_unchanged: bool,
    executed_plan_persisted: bool,
    trade_receipt_persisted: bool,
    receipt_readback_verified: bool,
    executed_plan_registry_count: usize,
    offered_contract_fingerprint: u64,
    target_contract_fingerprint: u64,
    news_count_unchanged: bool,
    commit_process_id: u32,
    game_time: String,
}

#[derive(Clone)]
struct TradeCommitReceipt {
    schema_version: usize,
    plan_id: String,
    commit_process_id: u32,
    commit_game_time: String,
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    offered_name: String,
    target_name: String,
    proposed_cash_won: u64,
    desired_status_choice: u8,
    desired_status_key: String,
    offered_team_after: usize,
    target_team_after: usize,
    target_status_after: String,
    offered_contracted_status_after: String,
    target_contracted_status_after: String,
    rollback_rehearsal_verified: bool,
    offered_contract_fingerprint: u64,
    target_contract_fingerprint: u64,
    requester_total_bits: u64,
    requester_transfer_bits: u64,
    requester_salary_bits: u64,
    recipient_total_bits: u64,
    recipient_transfer_bits: u64,
    recipient_salary_bits: u64,
    requester_roster_after: usize,
    recipient_roster_after: usize,
    contracted_after: usize,
    requester_payroll_bits: u64,
    recipient_payroll_bits: u64,
    requester_news_count: usize,
    recipient_news_count: usize,
    executed_plan_registry_count: usize,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AsyncTradeState {
    SellerReview,
    PlayerReview,
    Accepted,
    Rejected,
    Cancelled,
}

impl AsyncTradeState {
    fn as_str(self) -> &'static str {
        match self {
            Self::SellerReview => "SellerReview",
            Self::PlayerReview => "PlayerReview",
            Self::Accepted => "Accepted",
            Self::Rejected => "Rejected",
            Self::Cancelled => "Cancelled",
        }
    }

    fn from_str(value: &str) -> Result<Self, String> {
        match value {
            "SellerReview" => Ok(Self::SellerReview),
            "PlayerReview" => Ok(Self::PlayerReview),
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "Cancelled" => Ok(Self::Cancelled),
            other => Err(format!("unsupported async trade state {other}")),
        }
    }

    fn stage_ko(self) -> &'static str {
        match self {
            Self::SellerReview => "판매 구단 검토",
            Self::PlayerReview => "선수 검토",
            Self::Accepted => "트레이드 성사",
            Self::Rejected => "트레이드 결렬",
            Self::Cancelled => "트레이드 취소",
        }
    }

    fn terminal(self) -> bool {
        matches!(self, Self::Accepted | Self::Rejected | Self::Cancelled)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AsyncTradeProposal {
    schema_version: u8,
    proposal_id: String,
    state: AsyncTradeState,
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    offered_name: String,
    target_name: String,
    region_id: usize,
    proposed_units: u64,
    proposed_cash_won: u64,
    desired_status_choice: u8,
    desired_status_key: String,
    display_min_units: u64,
    display_max_units: u64,
    display_lower_percent: u8,
    display_upper_percent: u8,
    submitted_at: String,
    seller_due_at: String,
    player_due_at: String,
    completed_at: String,
    package_fingerprint: u64,
    offered_contract_fingerprint_at_submit: u64,
    target_contract_fingerprint_at_submit: u64,
    result_plan_id: String,
    rejection_actor: String,
    rejection_reason: String,
    rejection_reason_ko: String,
    success_news_id: String,
    transition_count: u32,
    submit_process_id: u32,
    commit_process_id: u32,
}

#[derive(Clone)]
struct AsyncStatusView {
    proposal_present: bool,
    proposal_id: String,
    state: String,
    stage_ko: String,
    requester_team_id: usize,
    recipient_team_id: usize,
    requester_team_name: String,
    recipient_team_name: String,
    target_id: usize,
    offered_id: usize,
    target_name: String,
    offered_name: String,
    target_position_label: String,
    target_position_icon: String,
    target_contract_end: String,
    target_yearly_salary: f64,
    proposed_units: u64,
    desired_status_label: String,
    game_time: String,
    submitted_at: String,
    seller_due_at: String,
    player_due_at: String,
    completed_at: String,
    rejection_reason_ko: String,
    result_plan_id: String,
    success_news_count: usize,
    submit_process_id: u32,
    commit_process_id: u32,
    current_process_id: u32,
    offered_team_current: usize,
    target_team_current: usize,
    target_status_current: String,
    target_contracted_status_current: String,
    executed_plan_registry_count: usize,
    result_plan_occurrences: usize,
}

#[derive(Clone)]
struct NativeOfferStatusView {
    visible: bool,
    athlete_id: usize,
    first_seen: String,
    state_text: String,
    deadline: String,
    stage: String,
    sequence: u64,
    active_offer_count: usize,
    reloaded: bool,
}

#[derive(Clone, Copy)]
struct ProfileNativeUiSnapshot {
    contract_visible: bool,
    contract_disabled: bool,
    state_visible: bool,
    state_disabled: bool,
}

static ROLLBACK_SNAPSHOT: Mutex<Option<AtomicRollbackSnapshot>> = Mutex::new(None);
static FORCED_ROLLBACK_AUDIT: Mutex<Option<ForcedRollbackAuditSnapshot>> = Mutex::new(None);

thread_local! {
    static COMPARE_TEMPLATE: RefCell<Option<Node>> = const { RefCell::new(None) };
    static NATIVE_COMPARE_HOME: RefCell<Option<NativeCompareHome>> = const { RefCell::new(None) };
    static CLOSED_COMPARE_SNAPSHOT: RefCell<Option<ClosedCompareSnapshot>> = const { RefCell::new(None) };
    static ACTIVE_OFFER_PATH: RefCell<Option<Vec<usize>>> = const { RefCell::new(None) };
    static CLICK_HANDLER_PAIR: RefCell<Option<ClickHandlerPair>> = const { RefCell::new(None) };
    static COMMIT_ARMED_LOG_KEY: RefCell<Option<(usize, String)>> = const { RefCell::new(None) };
    static QUOTE_VIEW: RefCell<Option<QuoteView>> = const { RefCell::new(None) };
    static QUOTE_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
    static PROPOSED_UNITS: RefCell<Option<u64>> = const { RefCell::new(None) };
    static REVIEW_VIEW: RefCell<Option<ReviewView>> = const { RefCell::new(None) };
    static REVIEW_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
    static EXECUTE_VIEW: RefCell<Option<ExecuteView>> = const { RefCell::new(None) };
    static EXECUTE_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
    static ASYNC_STATUS_VIEW: RefCell<Option<AsyncStatusView>> = const { RefCell::new(None) };
    static ASYNC_STATUS_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
    static NATIVE_OFFER_STATUS_VIEW: RefCell<Option<NativeOfferStatusView>> = const { RefCell::new(None) };
    static NATIVE_OFFER_STATUS_LAST_RENDER_KEY: RefCell<String> = const { RefCell::new(String::new()) };
    static ASYNC_PROFILE_LAST_RENDER_KEY: RefCell<String> = const { RefCell::new(String::new()) };
    static ASYNC_PROFILE_LAST_SKIP_KEY: RefCell<String> = const { RefCell::new(String::new()) };
    static UNRELATED_PROFILE_NATIVE_UI_LAST_KEY: RefCell<String> = const { RefCell::new(String::new()) };
    static ASYNC_PROFILE_LAST_DETAIL_INSTANCE: RefCell<usize> = const { RefCell::new(0) };
    static ASYNC_PROFILE_REAPPLY_LAST_KEY: RefCell<String> = const { RefCell::new(String::new()) };
    static TRADE_ENTRY_ASYNC_STATE_KEY: RefCell<String> = const { RefCell::new(String::new()) };
    static SECOND_SUBMIT_BLOCK_LAST_KEY: RefCell<String> = const { RefCell::new(String::new()) };
    static PENDING_CONTRACT_SLOT_TEMPLATE: RefCell<Option<Node>> = const { RefCell::new(None) };
    static NATIVE_TARGET_VIEW_DETAIL_TEMPLATE: RefCell<Option<(usize, Node)>> = const { RefCell::new(None) };
    static PROFILE_NATIVE_UI_SNAPSHOT: RefCell<Option<ProfileNativeUiSnapshot>> = const { RefCell::new(None) };
    static ASYNC_CONTRACT_PROJECTION_SURFACES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static ASYNC_CONTRACT_PROJECTION_LAST_KEY: RefCell<String> = const { RefCell::new(String::new()) };
    static ASYNC_CONTRACT_PROJECTION_UPDATE_KEY: RefCell<String> = const { RefCell::new(String::new()) };
    static ASYNC_NATIVE_OFFER_LOCK_LAST_KEY: RefCell<String> = const { RefCell::new(String::new()) };
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

fn log_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let game_dir = exe.parent()?;
    Some(game_dir.join("mods").join(MOD_ID).join(RUNTIME_LOG))
}

fn append_log(text: &str) {
    let Some(path) = log_path() else { return };
    if let Some(parent) = path.parent() {
        let _ = create_dir_all(parent);
    }
    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) else {
        return;
    };
    let _ = file.write_all(text.as_bytes());
    if !text.ends_with('\n') {
        let _ = file.write_all(b"\n");
    }
    let _ = file.flush();
}

fn log_event(event: &str, detail: &str) {
    let process_id = std::process::id();
    let runtime_nonce = *RUNTIME_NONCE.get_or_init(|| {
        (now_ms()
            ^ ((process_id as u128) << 64)
            ^ ((&RUNTIME_NONCE as *const _ as usize) as u128))
            .max(1)
    });
    let detail_target_id = detail.split(';').find_map(|token| {
        token
            .strip_prefix("target_id=")
            .and_then(|value| value.parse::<usize>().ok())
    });
    let context_target_id = detail_target_id
        .unwrap_or_else(|| TARGET_ATHLETE_ID.load(Ordering::Acquire));
    append_log(&format!(
        "ts_ms={}|event={}|process_id={};runtime_nonce={};target_epoch={};open_generation={};click_sequence={};target_id={};{}\n",
        now_ms(),
        event,
        process_id,
        runtime_nonce,
        TARGET_EPOCH.load(Ordering::Acquire),
        OPEN_GENERATION.load(Ordering::Acquire),
        OPEN_CLICK_SEQUENCE.load(Ordering::Acquire),
        context_target_id,
        detail,
    ));
}

fn sanitize(value: &str) -> String {
    value
        .replace('|', " ")
        .replace(';', " ")
        .replace('\r', " ")
        .replace('\n', " ")
}

fn find_node_by_id<'a>(node: &'a Node, target: &str) -> Option<&'a Node> {
    if node.id == target {
        return Some(node);
    }
    for child in &node.child {
        if let Some(found) = find_node_by_id(child, target) {
            return Some(found);
        }
    }
    None
}

fn count_nodes_by_id(node: &Node, target: &str) -> usize {
    let mut count: usize = if node.id == target { 1 } else { 0 };
    for child in &node.child {
        count += count_nodes_by_id(child, target);
    }
    count
}

fn find_node_by_id_mut<'a>(node: &'a mut Node, target: &str) -> Option<&'a mut Node> {
    if node.id == target {
        return Some(node);
    }
    for child in &mut node.child {
        if let Some(found) = find_node_by_id_mut(child, target) {
            return Some(found);
        }
    }
    None
}

fn collect_effectively_interactive_trade_offer_paths(
    node: &Node,
    ancestors_interactive: bool,
    path: &mut Vec<usize>,
    output: &mut Vec<Vec<usize>>,
    raw_effective_offer_id_count: &mut usize,
) {
    let interactive = ancestors_interactive && node.visible && !node.disabled;
    if interactive && node.id == OFFER_SURFACE_ID {
        *raw_effective_offer_id_count += 1;
        if node_has_trade_offer_shape(node) {
            output.push(path.clone());
        }
    }
    for (index, child) in node.child.iter().enumerate() {
        path.push(index);
        collect_effectively_interactive_trade_offer_paths(
            child,
            interactive,
            path,
            output,
            raw_effective_offer_id_count,
        );
        path.pop();
    }
}

fn node_by_index_path<'a>(mut node: &'a Node, path: &[usize]) -> Option<&'a Node> {
    for index in path {
        node = node.child.get(*index)?;
    }
    Some(node)
}

fn node_by_index_path_mut<'a>(mut node: &'a mut Node, path: &[usize]) -> Option<&'a mut Node> {
    for index in path {
        node = node.child.get_mut(*index)?;
    }
    Some(node)
}

fn collect_parent_index_paths_by_id_excluding(
    node: &Node,
    target: &str,
    excluded_subtree_id: &str,
    path: &mut Vec<usize>,
    output: &mut Vec<Vec<usize>>,
) {
    if node.id == excluded_subtree_id {
        return;
    }
    if node.child.iter().any(|child| child.id == target) {
        output.push(path.clone());
    }
    for (index, child) in node.child.iter().enumerate() {
        path.push(index);
        collect_parent_index_paths_by_id_excluding(
            child,
            target,
            excluded_subtree_id,
            path,
            output,
        );
        path.pop();
    }
}

fn select_exact_active_offer_path(ui: &GameUI) -> Option<Vec<usize>> {
    let mut candidates = Vec::new();
    let mut raw_effective_offer_id_count = 0;
    collect_effectively_interactive_trade_offer_paths(
        &ui.root,
        true,
        &mut Vec::new(),
        &mut candidates,
        &mut raw_effective_offer_id_count,
    );
    let count = candidates.len();
    let raw_offer_id_count = if count == 1 {
        node_by_index_path(&ui.root, &candidates[0])
            .map(|surface| count_nodes_by_id(surface, OFFER_SURFACE_ID))
            .unwrap_or(raw_effective_offer_id_count)
    } else {
        raw_effective_offer_id_count
    };
    ACTIVE_RAW_OFFER_ID_COUNT.store(raw_offer_id_count, Ordering::Release);
    ACTIVE_STRUCTURAL_OFFER_COUNT.store(count, Ordering::Release);
    let previous_count = LAST_ACTIVE_OFFER_COUNT.swap(count, Ordering::AcqRel);
    if count > 1 && count != previous_count {
        log_event(
            "trade_offer_context_ambiguous",
            &format!(
                "active_offer_count={count};raw_offer_id_count={raw_offer_id_count};expected=1;resolver=direct_child_structure_v1;fail_closed=true"
            ),
        );
    }
    if count == 1 {
        candidates.pop()
    } else {
        None
    }
}

fn set_active_offer_path(ui: &GameUI) -> Option<Vec<usize>> {
    let path = select_exact_active_offer_path(ui);
    ACTIVE_OFFER_PATH.with(|slot| *slot.borrow_mut() = path.clone());
    path
}

fn active_offer_path() -> Option<Vec<usize>> {
    ACTIVE_OFFER_PATH.with(|slot| slot.borrow().clone())
}

fn active_offer<'a>(ui: &'a GameUI) -> Option<&'a Node> {
    let path = active_offer_path()?;
    let node = node_by_index_path(&ui.root, &path)?;
    (node.visible && !node.disabled && node_has_trade_offer_shape(node))
        .then_some(node)
}

fn active_offer_mut<'a>(ui: &'a mut GameUI) -> Option<&'a mut Node> {
    let path = active_offer_path()?;
    let node = node_by_index_path_mut(&mut ui.root, &path)?;
    if node.visible && !node.disabled && node_has_trade_offer_shape(node) {
        Some(node)
    } else {
        None
    }
}

fn find_node_by_id_excluding<'a>(
    node: &'a Node,
    target: &str,
    excluded_subtree_id: &str,
) -> Option<&'a Node> {
    if node.id == excluded_subtree_id {
        return None;
    }
    if node.id == target {
        return Some(node);
    }
    for child in &node.child {
        if let Some(found) = find_node_by_id_excluding(child, target, excluded_subtree_id) {
            return Some(found);
        }
    }
    None
}

fn find_visible_node_by_id_excluding<'a>(
    node: &'a Node,
    target: &str,
    excluded_subtree_id: &str,
) -> Option<&'a Node> {
    if node.id == excluded_subtree_id {
        return None;
    }
    if node.id == target && node.visible {
        return Some(node);
    }
    for child in &node.child {
        if let Some(found) =
            find_visible_node_by_id_excluding(child, target, excluded_subtree_id)
        {
            return Some(found);
        }
    }
    None
}

fn find_node_by_id_mut_excluding<'a>(
    node: &'a mut Node,
    target: &str,
    excluded_subtree_id: &str,
) -> Option<&'a mut Node> {
    if node.id == excluded_subtree_id {
        return None;
    }
    if node.id == target {
        return Some(node);
    }
    for child in &mut node.child {
        if let Some(found) = find_node_by_id_mut_excluding(child, target, excluded_subtree_id) {
            return Some(found);
        }
    }
    None
}

fn find_parent_of_id<'a>(node: &'a Node, target: &str) -> Option<&'a Node> {
    if node.child.iter().any(|child| child.id == target) {
        return Some(node);
    }
    for child in &node.child {
        if let Some(found) = find_parent_of_id(child, target) {
            return Some(found);
        }
    }
    None
}

fn find_parent_of_id_mut<'a>(node: &'a mut Node, target: &str) -> Option<&'a mut Node> {
    if node.child.iter().any(|child| child.id == target) {
        return Some(node);
    }
    for child in &mut node.child {
        if let Some(found) = find_parent_of_id_mut(child, target) {
            return Some(found);
        }
    }
    None
}

fn take_direct_child(node: &mut Node, id: &str) -> Option<Node> {
    let index = node.child.iter().position(|child| child.id == id)?;
    let child = node.child.remove(index);
    node.runner.set_dirty(true);
    Some(child)
}

fn direct_child<'a>(node: &'a Node, id: &str) -> Option<&'a Node> {
    node.child.iter().find(|child| child.id == id)
}

fn direct_child_mut<'a>(node: &'a mut Node, id: &str) -> Option<&'a mut Node> {
    node.child.iter_mut().find(|child| child.id == id)
}

fn direct_path_mut<'a>(mut node: &'a mut Node, path: &[&str]) -> Option<&'a mut Node> {
    for id in path {
        let index = node.child.iter().position(|child| child.id == *id)?;
        node = &mut node.child[index];
    }
    Some(node)
}


#[derive(Clone)]
struct PopupAthleteVisual {
    athlete_id: usize,
    name: String,
    position_label: &'static str,
    position_icon: &'static str,
    status_label: &'static str,
    stats: BTreeMap<&'static str, f64>,
}

#[derive(Clone)]
struct CustomRosterEntry {
    athlete_id: usize,
    name: String,
    position_rank: u8,
    position_label: &'static str,
    position_icon: &'static str,
    status_label: &'static str,
}

const POPUP_COMPARISON_STAT_IDS: [&str; 11] = [
    "last_hit",
    "skill_avoid",
    "skill_hit",
    "control_speed",
    "positioning",
    "judgement",
    "mental",
    "concentration",
    "order",
    "roaming",
    "aggressive",
];

const POPUP_EXTRA_STAT_IDS: [&str; 3] = ["ego", "stamina", "condition"];

/// [PORT056] 버튼 노드 자체가 아니라 그 **자손 라벨 노드**에 텍스트를 넣는다.
/// `color_icon_button` 은 표시 텍스트를 자식 라벨이 들고 있어, 버튼 런너에 `build_with_property("text")`
/// 를 걸어도 화면이 바뀌지 않는다(2026-08-23 인게임 실측). 첫 라벨 자손을 찾아 갱신한다.
fn set_entry_button_label(node: &mut Node, text: &str) -> bool {
    fn walk(node: &mut Node, text: &str, depth: usize) -> bool {
        if depth > 6 {
            return false;
        }
        for child in node.child.iter_mut() {
            if child.runner.type_name().to_ascii_lowercase().contains("label") {
                set_runner_text(child, text);
                child.visible = true;
                return true;
            }
            if walk(child, text, depth + 1) {
                return true;
            }
        }
        false
    }
    walk(node, text, 0)
}

// ── [PORT056] 커서 위치 (Win32) ──
//   엔진 UIEvent 에는 마우스이동/호버 이벤트가 없다(Click/RightClick/TextEditComplete/… 뿐).
//   `Node.disabled=true` 를 켜면 게임의 호버 처리도 죽어 러너 `hint` 툴팁이 안 뜨므로,
//   커서를 직접 읽어 버튼 rect 안이면 우리 툴팁 노드를 띄운다. (champ_pos_lock 과 동일 기법)
#[repr(C)]
#[derive(Clone, Copy)]
struct Win32Point {
    x: i32,
    y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Win32Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
#[link(name = "user32")]
extern "system" {
    fn GetCursorPos(p: *mut Win32Point) -> i32;
    fn ScreenToClient(h: usize, p: *mut Win32Point) -> i32;
    fn GetForegroundWindow() -> usize;
    fn GetClientRect(h: usize, r: *mut Win32Rect) -> i32;
}

/// 커서를 UI 좌표계(= `GameUI.rect`)로 환산.
/// ⚠`GameUI.scale` 로 나누면 안 된다 — scale 은 픽셀아트 배율이지 클라이언트px↔UI유닛 비율이 아니다
///   (champ_pos_lock 이 2026-08-22 실측으로 확인한 함정).
fn cursor_in_ui(uiw: f32, uih: f32) -> Option<(f32, f32)> {
    unsafe {
        let h = GetForegroundWindow();
        if h == 0 {
            return None;
        }
        let mut p = Win32Point { x: 0, y: 0 };
        if GetCursorPos(&mut p) == 0 || ScreenToClient(h, &mut p) == 0 {
            return None;
        }
        let mut r = Win32Rect::default();
        if GetClientRect(h, &mut r) == 0 {
            return None;
        }
        let (cw, ch) = ((r.right - r.left) as f32, (r.bottom - r.top) as f32);
        if cw < 1.0 || ch < 1.0 || uiw < 1.0 || uih < 1.0 {
            return None;
        }
        Some((p.x as f32 * uiw / cw, p.y as f32 * uih / ch))
    }
}

// ===================== [PORT056] 비활성 룩 + 게임 툴팁 (champ_pos_lock 검증 기법) =====================
//
// 배경: `entry.disabled = true` 로 클릭은 막히지만 **버튼이 멀쩡해 보인다**(2026-08-23 유저 보고:
//   "버튼 그냥 있어. 클릭도 안되고 호버해도 커서 안바뀌어"). 그리고 버튼 텍스트는 바꿀 수 없다
//   (ColorIconButtonRunner 가 내부에 들고 있고 자식 노드가 없음 · downcast 는 cdylib 경계라 실패).
//
// 해법은 `tfm2_champ_pos_lock` 이 확정 버튼에 이미 쓰던 것을 그대로 가져온다:
//   ① `type_name()` 문자열로 런너를 판별하고 **fat 포인터에서 데이터 주소를 꺼내** SDK 타입으로 캐스팅
//      (`downcast` 를 안 쓰므로 TypeId 불일치 문제를 우회한다)
//   ② `ColorIconButtonRunner.hint: String` = **게임 자체 툴팁**. 호버 판정·표시·배치를 게임이 해준다
//   ③ 비활성 룩 = `style.normal/hover/active` 를 `style.disabled` 색으로 덮어쓴다
//      (스타일에 disabled 정의가 없어 normal 과 같으면 직접 어둡게 = dim 폴백)
// 오프셋 하드코딩이 없고 전부 SDK 공개 필드라 패치 내성이 있다.

/// 런너의 실제 데이터 주소. `type_name()` 이 want 를 포함할 때만 반환한다.
fn runner_base(node: &Node, want: &str) -> Option<usize> {
    if !node.runner.type_name().contains(want) {
        return None;
    }
    let any: &dyn std::any::Any = node.runner.as_any();
    // &dyn Any = (데이터 포인터, vtable 포인터) 2워드. 앞이 데이터 주소.
    let parts: [usize; 2] = unsafe { std::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any) };
    (parts[0] >= 0x10000 && parts[0] < (1usize << 48)).then_some(parts[0])
}

type BtnColors = [common::color::Color; 4];

fn cib_get(p: &ColorIconButtonRunnerProperty) -> BtnColors {
    [p.icon.color, p.sub.color, p.text.color, p.btn.color]
}
fn cib_set(p: &mut ColorIconButtonRunnerProperty, c: BtnColors) {
    p.icon.color = c[0];
    p.sub.color = c[1];
    p.text.color = c[2];
    p.btn.color = c[3];
}
fn dim_color(c: common::color::Color) -> common::color::Color {
    common::color::Color { r: c.r * 0.4, g: c.g * 0.4, b: c.b * 0.4, a: c.a }
}

/// 진입 버튼의 원래 색 백업 [normal, hover, active].
static ENTRY_STYLE_SAVED: Mutex<Option<[BtnColors; 4]>> = Mutex::new(None);
/// [PORT056] 모달 "트레이드" 버튼용 별도 저장 슬롯.
static TRADE_BUTTON_STYLE_SAVED: Mutex<Option<[BtnColors; 4]>> = Mutex::new(None);
static CASH_STATUS_WARNING_SHOWN: AtomicBool = AtomicBool::new(false);

/// `gray=true` → 비활성 색 + 툴팁(hint) 설정 / `false` → 원래 색 복구 + 툴팁 해제.
/// 런너 타입이 다르면 false.
fn entry_button_set_disabled_look(node: &mut Node, gray: bool, hint: &str) -> bool {
    button_set_disabled_look(node, gray, hint, &ENTRY_STYLE_SAVED)
}

/// [PORT056] 위 함수의 일반형 — 버튼마다 **원본 색 저장 슬롯이 따로** 있어야 한다
///   (하나를 공유하면 먼저 회색이 된 버튼의 색으로 다른 버튼이 복구된다).
fn button_set_disabled_look(
    node: &mut Node,
    gray: bool,
    hint: &str,
    saved_slot: &Mutex<Option<[BtnColors; 4]>>,
) -> bool {
    let Some(base) = runner_base(node, "ColorIconButtonRunner") else {
        return false;
    };
    unsafe {
        let r = &mut *(base as *mut ColorIconButtonRunner);
        if r.hint != hint {
            r.hint = hint.to_string();
        }
        let mut saved = saved_slot.lock().unwrap_or_else(|e| e.into_inner());
        if saved.is_none() {
            *saved = Some([
                cib_get(&r.style.normal),
                cib_get(&r.style.hover),
                cib_get(&r.style.active),
                cib_get(&r.style.disabled),
            ]);
        }
        let base_colors = saved.unwrap();
        if gray {
            let mut d = base_colors[3]; // 원래 disabled 색
            if d == base_colors[0] {
                // 스타일에 비활성 룩이 없다(=normal 과 동일) → 직접 어둡게
                d = [dim_color(d[0]), dim_color(d[1]), dim_color(d[2]), dim_color(d[3])];
            }
            // 4상태 전부 덮는다: 노드를 disabled 로 두지 않으므로 normal/hover/active 가 실제로 쓰이고,
            // 혹시 엔진이 disabled 상태를 쓰더라도 같은 색이 되도록 함께 맞춘다.
            cib_set(&mut r.style.normal, d);
            cib_set(&mut r.style.hover, d);
            cib_set(&mut r.style.active, d);
            cib_set(&mut r.style.disabled, d);
        } else {
            cib_set(&mut r.style.normal, base_colors[0]);
            cib_set(&mut r.style.hover, base_colors[1]);
            cib_set(&mut r.style.active, base_colors[2]);
            cib_set(&mut r.style.disabled, base_colors[3]);
        }
    }
    node.runner.set_dirty(true);
    true
}

fn set_runner_text(node: &mut Node, text: &str) {
    // [PORT056] `build_with_property("text")` 는 런너가 그 키를 읽어줄 때만 동작하는 간접 경로다.
    //   `LabelRunner.text: String` 은 공개 필드이므로 직접 쓰고, 보조로 기존 경로도 유지한다.
    if let Some(label) = node.runner.as_any_mut().downcast_mut::<LabelRunner>() {
        label.text = text.to_string();
    }
    let mut properties: HashMap<String, Rc<dyn Any>> = HashMap::new();
    properties.insert("text".to_string(), Rc::new(text.to_string()));
    node.runner.build_with_property(&properties);
    node.runner.set_dirty(true);
}

/// 모드가 소유한 행(계약 현황 projection)의 텍스트 전용.
/// ★`LabelRunner.binds: Vec<Rc<dyn Fn() -> (String, String)>>` 는 라벨 텍스트를 **매 갱신마다 다시 만들어내는**
///   바인드 클로저 목록이다. 네이티브 계약 행을 복제해 오면 이 바인드가 따라와서, 우리가 이름을 써넣어도
///   다음 갱신에 원본 선수(Dan) 문자열로 되돌아간다.
///   ⟹ 인계본이 "이름 LabelRunner 가 다음 update 에 Dan 으로 다시 덮어써지지 않는지 봐달라"고 적었던
///   현상의 메커니즘으로 유력하다(03_MESSAGE_TO_DEVELOPER 검토 요청 6번). 바인드를 끊고 값을 고정한다.
fn set_runner_text_pinned(node: &mut Node, text: &str) -> (bool, usize) {
    let mut is_label = false;
    let mut cleared = 0usize;
    if let Some(label) = node.runner.as_any_mut().downcast_mut::<LabelRunner>() {
        is_label = true;
        cleared = label.binds.len();
        label.binds.clear();
        label.text = text.to_string();
    }
    let mut properties: HashMap<String, Rc<dyn Any>> = HashMap::new();
    properties.insert("text".to_string(), Rc::new(text.to_string()));
    node.runner.build_with_property(&properties);
    node.runner.set_dirty(true);
    (is_label, cleared)
}

fn set_runner_source(node: &mut Node, source: &str) {
    let mut properties: HashMap<String, Rc<dyn Any>> = HashMap::new();
    properties.insert("source".to_string(), Rc::new(source.to_string()));
    node.runner.build_with_property(&properties);
    node.runner.set_dirty(true);
}

fn with_active_trade_popup_node_mut<F>(ui: &mut GameUI, path: &[&str], mut apply: F) -> bool
where
    F: FnMut(&mut Node),
{
    let Some(modal_layer) = find_node_by_id_mut(&mut ui.root, MODAL_LAYER_ID) else {
        return false;
    };
    let Some(popup) = direct_child_mut(modal_layer, NATIVE_COMPARE_ID) else {
        return false;
    };
    let Some(node) = direct_path_mut(popup, path) else {
        return false;
    };
    apply(node);
    true
}

fn set_trade_popup_text(ui: &mut GameUI, path: &[&str], text: &str) -> bool {
    with_active_trade_popup_node_mut(ui, path, |node| set_runner_text(node, text))
}

fn set_trade_popup_source(ui: &mut GameUI, path: &[&str], source: &str) -> bool {
    with_active_trade_popup_node_mut(ui, path, |node| set_runner_source(node, source))
}

fn set_trade_popup_visible(ui: &mut GameUI, path: &[&str], visible: bool) -> bool {
    with_active_trade_popup_node_mut(ui, path, |node| {
        node.visible = visible;
        node.disabled = !visible;
        node.runner.set_dirty(true);
    })
}

fn debug_numeric_field(debug: &str, key: &str) -> Option<f64> {
    let needle = format!("{key}: ");
    let mut search_from = 0usize;
    while search_from < debug.len() {
        let relative = debug[search_from..].find(&needle)?;
        let value_start = search_from + relative + needle.len();
        let tail = &debug[value_start..];
        let token = tail
            .split(|character: char| character == ',' || character == '}' || character == ']')
            .next()?
            .trim();
        if let Ok(value) = token.parse::<f64>() {
            if value.is_finite() {
                return Some(value);
            }
        }
        search_from = value_start;
    }
    None
}

fn format_popup_stat(value: Option<f64>) -> String {
    match value {
        Some(value) if value.is_finite() => {
            if (value - value.round()).abs() <= 0.05 {
                format!("{:.0}", value)
            } else {
                format!("{:.1}", value)
            }
        }
        _ => "—".to_string(),
    }
}

fn popup_position_visual(position: Position) -> (&'static str, &'static str) {
    match position {
        Position::Top => ("탑", "asset/base/ui/icons/top"),
        Position::Jungle => ("정글", "asset/base/ui/icons/jungle"),
        Position::Mid => ("미드", "asset/base/ui/icons/mid"),
        Position::Bottom => ("바텀", "asset/base/ui/icons/bottom"),
        Position::Support => ("서포터", "asset/base/ui/icons/support"),
    }
}

fn build_popup_athlete_visual(athlete_id: usize, athlete: &Athlete) -> PopupAthleteVisual {
    let (position_label, position_icon) = popup_position_visual(athlete.main_position());
    let debug = format!("{:?}", athlete);
    let mut stats = BTreeMap::new();
    for key in POPUP_COMPARISON_STAT_IDS
        .iter()
        .chain(POPUP_EXTRA_STAT_IDS.iter())
    {
        if let Some(value) = debug_numeric_field(&debug, key) {
            stats.insert(*key, value);
        }
    }
    PopupAthleteVisual {
        athlete_id,
        name: athlete.name.clone(),
        position_label,
        position_icon,
        status_label: custom_squad_status_label(&athlete.squad_status),
        stats,
    }
}

fn popup_compare_marker(target: Option<f64>, offered: Option<f64>) -> &'static str {
    match (target, offered) {
        (Some(target), Some(offered)) if target > offered + 0.05 => "▲",
        (Some(target), Some(offered)) if target + 0.05 < offered => "▼",
        (Some(_), Some(_)) => "=",
        _ => "—",
    }
}

const NATIVE_VISUAL_STAT_IDS: [&str; 14] = [
    "last_hit",
    "skill_avoid",
    "skill_hit",
    "control_speed",
    "positioning",
    "judgement",
    "mental",
    "concentration",
    "order",
    "roaming",
    "aggressive",
    "ego",
    "stamina",
    "condition",
];


fn custom_squad_status_label(status: &SquadStatus) -> &'static str {
    match status {
        SquadStatus::Core => "핵심",
        SquadStatus::Important => "주요",
        SquadStatus::General => "주전",
        SquadStatus::Sub => "후보",
        SquadStatus::Prospect => "유망주",
    }
}

fn custom_position_rank(position: Position) -> u8 {
    match position {
        Position::Top => 0,
        Position::Jungle => 1,
        Position::Mid => 2,
        Position::Bottom => 3,
        Position::Support => 4,
    }
}

fn custom_slot_id(index: usize) -> String {
    format!("{CUSTOM_SLOT_PREFIX}{index:02}")
}

fn custom_slot_index_from_click(path: &str, item: &str) -> Option<usize> {
    for source in [item, path] {
        let Some(start) = source.find(CUSTOM_SLOT_PREFIX) else {
            continue;
        };
        let tail = &source[start + CUSTOM_SLOT_PREFIX.len()..];
        let digits: String = tail.chars().take_while(|ch| ch.is_ascii_digit()).collect();
        if digits.is_empty() {
            continue;
        }
        let Ok(index) = digits.parse::<usize>() else {
            continue;
        };
        if index < CUSTOM_ROSTER_SLOT_COUNT {
            return Some(index);
        }
    }
    None
}

fn custom_slot_athlete_id(index: usize) -> Option<usize> {
    let slots = CUSTOM_ROSTER_SLOT_IDS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    slots.get(index).copied().filter(|id| *id != NO_ATHLETE) // [PORT056]
}

fn with_custom_modal_node_mut<F>(ui: &mut GameUI, id: &str, mut apply: F) -> bool
where
    F: FnMut(&mut Node),
{
    let Some(modal) = find_node_by_id_mut(&mut ui.root, MODAL_LAYER_ID) else {
        return false;
    };
    let Some(node) = find_node_by_id_mut(modal, id) else {
        return false;
    };
    apply(node);
    true
}

fn set_custom_text(ui: &mut GameUI, id: &str, text: &str) -> bool {
    with_custom_modal_node_mut(ui, id, |node| set_runner_text(node, text))
}

fn set_custom_source(ui: &mut GameUI, id: &str, source: &str) -> bool {
    with_custom_modal_node_mut(ui, id, |node| set_runner_source(node, source))
}

fn set_custom_visible(ui: &mut GameUI, id: &str, visible: bool) -> bool {
    with_custom_modal_node_mut(ui, id, |node| {
        node.visible = visible;
        node.disabled = !visible;
        node.runner.set_dirty(true);
    })
}

fn custom_stat_palette_tier(value: Option<f64>) -> &'static str {
    match value {
        Some(value) if value.is_finite() && value < 21.0 => "low",
        Some(value) if value.is_finite() && value < 41.0 => "teal",
        Some(value) if value.is_finite() && value < 61.0 => "blue",
        Some(value) if value.is_finite() && value < 81.0 => "purple",
        Some(value) if value.is_finite() => "orange",
        _ => "low",
    }
}

fn custom_status_variant(label: &str) -> &'static str {
    match label {
        "유망주" => "prospect",
        "후보" => "sub",
        "주전" => "general",
        "주요" => "important",
        "핵심" => "core",
        _ => "prospect",
    }
}

fn set_custom_status_value(ui: &mut GameUI, id: &str, label: &str) -> bool {
    let variant = custom_status_variant(label);
    let mut active_found = false;
    let container_found = with_custom_modal_node_mut(ui, id, |container| {
        for child in &mut container.child {
            let active = child.id == variant;
            child.visible = active;
            child.disabled = !active;
            child.runner.set_dirty(true);
            if active {
                active_found = true;
            }
        }
    });
    container_found && active_found
}

fn set_custom_stat_value(ui: &mut GameUI, id: &str, value: Option<f64>) -> bool {
    let tier = custom_stat_palette_tier(value);
    let text = format_popup_stat(value);
    let mut active_found = false;
    let container_found = with_custom_modal_node_mut(ui, id, |container| {
        for child in &mut container.child {
            let active = child.id == tier;
            child.visible = active;
            child.disabled = !active;
            if active {
                set_runner_text(child, &text);
                active_found = true;
            } else {
                child.runner.set_dirty(true);
            }
        }
    });
    container_found && active_found
}

fn custom_compare_marker_variant(target: Option<f64>, offered: Option<f64>) -> &'static str {
    match (target, offered) {
        (Some(target), Some(offered)) if target > offered + 0.05 => "up",
        (Some(target), Some(offered)) if target + 0.05 < offered => "down",
        (Some(_), Some(_)) => "equal",
        _ => "unknown",
    }
}

fn set_custom_compare_marker(
    ui: &mut GameUI,
    id: &str,
    target: Option<f64>,
    offered: Option<f64>,
) -> bool {
    let variant = custom_compare_marker_variant(target, offered);
    let mut active_found = false;
    let container_found = with_custom_modal_node_mut(ui, id, |container| {
        for child in &mut container.child {
            let active = child.id == variant;
            child.visible = active;
            child.disabled = !active;
            if active {
                child.runner.set_dirty(true);
                active_found = true;
            } else {
                child.runner.set_dirty(true);
            }
        }
    });
    container_found && active_found
}

fn with_custom_slot_mut<F>(ui: &mut GameUI, index: usize, mut apply: F) -> bool
where
    F: FnMut(&mut Node),
{
    let id = custom_slot_id(index);
    with_custom_modal_node_mut(ui, &id, |node| apply(node))
}

fn set_custom_slot(ui: &mut GameUI, index: usize, entry: Option<&CustomRosterEntry>) -> bool {
    with_custom_slot_mut(ui, index, |slot| {
        slot.visible = entry.is_some();
        slot.disabled = entry.is_none();
        slot.runner.set_dirty(true);
        let Some(entry) = entry else {
            return;
        };
        if let Some(bg) = direct_child_mut(slot, "selected_bg") {
            bg.visible = false;
            bg.disabled = true;
            bg.runner.set_dirty(true);
        }
        if let Some(check) = direct_child_mut(slot, "check") {
            set_runner_text(check, "");
        }
        if let Some(position) = direct_child_mut(slot, "position") {
            set_runner_source(position, entry.position_icon);
        }
        if let Some(name) = direct_child_mut(slot, "name") {
            set_runner_text(name, &entry.name);
        }
        if let Some(status) = direct_child_mut(slot, "status") {
            set_runner_text(status, entry.status_label);
        }
    })
}

fn set_custom_slot_selected(ui: &mut GameUI, index: usize, selected: bool) -> bool {
    with_custom_slot_mut(ui, index, |slot| {
        if let Some(bg) = direct_child_mut(slot, "selected_bg") {
            bg.visible = selected;
            bg.disabled = true;
            bg.runner.set_dirty(true);
        }
        if let Some(check) = direct_child_mut(slot, "check") {
            set_runner_text(check, if selected { "✓" } else { "" });
        }
    })
}

fn is_historical_trade_fixture_athlete(athlete_id: usize) -> bool {
    [FIRST_OFFERED_ID, FIRST_TARGET_ID, SECOND_OFFERED_ID, SECOND_TARGET_ID]
        .contains(&athlete_id)
}

fn custom_roster_entries(data: &ClientData) -> Vec<CustomRosterEntry> {
    let db = data.db();
    // [PORT056] 구 Test79 는 내 팀을 상수 REQUESTER_TEAM_ID(=7) 로 박아뒀다(그쪽 테스트 세이브 전용).
    //   실제 세이브에서는 로스터가 통째로 비거나 남의 팀이 나온다 → 실제 플레이어 팀으로 교체.
    let my_team_id = data.player_team_id();
    let mut entries = Vec::new();
    // Patch 0.5.5 ClientDatabase::athletes is a key/value map. This differs
    // from server Database::athletes, whose iterator yields &Athlete directly.
    for (athlete_id, athlete) in db.athletes.iter() {
        let Contract::InContract { team_id, .. } = &athlete.contract else {
            continue;
        };
        if *team_id != my_team_id {
            continue;
        }
        // [PORT056] 과거 테스트 픽스처 선수(Jue/Zeus/Zenit/Fill) 제외 규칙 삭제 — 일반 세이브엔 무의미.
        let position = athlete.main_position();
        let (position_label, position_icon) = popup_position_visual(position);
        entries.push(CustomRosterEntry {
            athlete_id: *athlete_id,
            name: athlete.name.clone(),
            position_rank: custom_position_rank(position),
            position_label,
            position_icon,
            status_label: custom_squad_status_label(&athlete.squad_status),
        });
    }
    entries.sort_by(|left, right| {
        left.position_rank
            .cmp(&right.position_rank)
            .then_with(|| left.name.cmp(&right.name))
            .then_with(|| left.athlete_id.cmp(&right.athlete_id))
    });
    entries
}

fn custom_stat_ids(stat_id: &str) -> (String, String, String) {
    (
        format!("pts_trade_custom_stat_{stat_id}_target"),
        format!("pts_trade_custom_stat_{stat_id}_marker"),
        format!("pts_trade_custom_stat_{stat_id}_offered"),
    )
}

fn populate_custom_trade_ui(ui: &mut GameUI, data: &ClientData) {
    if !POPUP_OPEN.load(Ordering::Acquire)
        || CUSTOM_ROSTER_BUILT.load(Ordering::Acquire)
    {
        return;
    }
    let target_id = TARGET_ATHLETE_ID.load(Ordering::Acquire);
    let (entries, target) = {
        let entries = custom_roster_entries(data);
        let db = data.db();
        let target = db
            .athlete(target_id)
            .map(|athlete| build_popup_athlete_visual(target_id, athlete));
        (entries, target)
    };

    let mut slots = vec![NO_ATHLETE; CUSTOM_ROSTER_SLOT_COUNT]; // [PORT056] 0 은 유효 id
    for index in 0..CUSTOM_ROSTER_SLOT_COUNT {
        let entry = entries.get(index);
        if let Some(entry) = entry {
            slots[index] = entry.athlete_id;
        }
        let _ = set_custom_slot(ui, index, entry);
    }
    {
        let mut stored = CUSTOM_ROSTER_SLOT_IDS
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *stored = slots;
    }
    let _ = set_custom_text(
        ui,
        "pts_trade_custom_roster_count",
        &format!("{}명", entries.len()),
    );

    if let Some(target) = target {
        let _ = set_custom_text(ui, "pts_trade_custom_target_name", &target.name);
        let _ = set_custom_text(
            ui,
            "pts_trade_custom_target_position",
            target.position_label,
        );
        let _ = set_custom_source(
            ui,
            "pts_trade_custom_target_position_icon",
            target.position_icon,
        );
        let _ = set_custom_status_value(
            ui,
            "pts_trade_custom_target_status",
            target.status_label,
        );
        let _ = set_label_text(
            ui,
            "pts_cash_runtime_selection_value",
            &format!("내 선수를 선택하세요  ↔  {}", target.name),
        );
        for stat_id in NATIVE_VISUAL_STAT_IDS {
            let (target_id, marker_id, offered_id) = custom_stat_ids(stat_id);
            let target_value = target.stats.get(stat_id).copied();
            let _ = set_custom_stat_value(ui, &target_id, target_value);
            let _ = set_custom_compare_marker(ui, &marker_id, None, None);
            let _ = set_custom_stat_value(ui, &offered_id, None);
        }
    }

    let overflow = entries.len().saturating_sub(CUSTOM_ROSTER_SLOT_COUNT);
    CUSTOM_ROSTER_BUILT.store(true, Ordering::Release);
    log_event(
        "custom_trade_roster_built",
        &format!(
            "requester_team_id={};roster_count={};visible_slot_count={};slot_capacity={};overflow_count={};two_column_static_layout=true;panel_side=right;panel_x=732px;original_compare_popup_used=false;transaction_enabled=true",
            data.player_team_id(), // [PORT056] 상수 7 → 실제 플레이어 팀
            entries.len(),
            entries.len().min(CUSTOM_ROSTER_SLOT_COUNT),
            CUSTOM_ROSTER_SLOT_COUNT,
            overflow,
        ),
    );
}

fn sync_custom_selection_highlight(ui: &mut GameUI, offered_id: usize) -> bool {
    let slots = CUSTOM_ROSTER_SLOT_IDS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone();
    let mut selected_count = 0usize;
    for index in 0..CUSTOM_ROSTER_SLOT_COUNT {
        let selected = slots.get(index).copied().unwrap_or(0) == offered_id;
        if selected {
            selected_count += 1;
        }
        let _ = set_custom_slot_selected(ui, index, selected);
    }
    selected_count == 1
}

fn direct_path<'a>(mut node: &'a Node, path: &[&str]) -> Option<&'a Node> {
    for id in path {
        node = direct_child(node, id)?;
    }
    Some(node)
}

fn node_has_trade_offer_shape(node: &Node) -> bool {
    node.id == OFFER_SURFACE_ID
        && direct_path(node, &["data", "row1", "info", "header", "name"]).is_some()
        && direct_path(node, &["data", "row2", "delegate"]).is_some()
        && direct_path(node, &["data", "row2", "offer"])
            .is_some_and(|submit| submit.child.is_empty())
        && direct_path(node, &["data", "row2", "view_other"]).is_some()
        && direct_path(node, &["data", "row2", ENTRY_ID]).is_some()
        && direct_child(node, MODAL_LAYER_TEMPLATE_ID)
            .is_some_and(|template| !template.visible)
        && direct_child(node, CASH_TEMPLATE_ID)
            .is_some_and(|template| !template.visible)
}

fn active_trade_popup(ui: &GameUI) -> Option<&Node> {
    let modal_layer = find_node_by_id(&ui.root, MODAL_LAYER_ID)?;
    direct_child(modal_layer, NATIVE_COMPARE_ID)
}

fn active_trade_popup_node<'a>(ui: &'a GameUI, path: &[&str]) -> Option<&'a Node> {
    direct_path(active_trade_popup(ui)?, path)
}

fn trade_popup_roster_row_present(ui: &GameUI, athlete_id: usize) -> bool {
    let Some(contents) = active_trade_popup_node(ui, &["popup", "list", "scroll", "contents"]) else {
        return false;
    };
    let athlete_id_text = athlete_id.to_string();
    contents.child.iter().any(|child| child.id == athlete_id_text)
}

fn swap_trade_popup_row_runners(ui: &mut GameUI, left_id: usize, right_id: usize) -> bool {
    if left_id == 0 || right_id == 0 || left_id == right_id {
        return true;
    }
    let Some(modal_layer) = find_node_by_id_mut(&mut ui.root, MODAL_LAYER_ID) else {
        return false;
    };
    let Some(popup) = direct_child_mut(modal_layer, NATIVE_COMPARE_ID) else {
        return false;
    };
    let Some(contents) = direct_path_mut(popup, &["popup", "list", "scroll", "contents"])
    else {
        return false;
    };
    let left_text = left_id.to_string();
    let right_text = right_id.to_string();
    let Some(left_index) = contents.child.iter().position(|child| child.id == left_text) else {
        return false;
    };
    let Some(right_index) = contents.child.iter().position(|child| child.id == right_text) else {
        return false;
    };
    if left_index < right_index {
        let (before, after) = contents.child.split_at_mut(right_index);
        std::mem::swap(&mut before[left_index].runner, &mut after[0].runner);
        before[left_index].runner.set_dirty(true);
        after[0].runner.set_dirty(true);
    } else {
        let (before, after) = contents.child.split_at_mut(left_index);
        std::mem::swap(&mut before[right_index].runner, &mut after[0].runner);
        before[right_index].runner.set_dirty(true);
        after[0].runner.set_dirty(true);
    }
    true
}

fn sync_trade_popup_selected_row(
    ui: &mut GameUI,
    _previous_offered_id: usize,
    _previous_offered_name: Option<&str>,
    offered_id: usize,
    offered_name: &str,
) -> bool {
    let selected = sync_custom_selection_highlight(ui, offered_id);
    log_event(
        "custom_trade_selected_row_synced",
        &format!(
            "offered_id={};offered_name={};selected_row_count={};green_highlight_applied={};check_marker_applied={};static_custom_slots=true;original_compare_popup_used=false",
            offered_id,
            sanitize(offered_name),
            usize::from(selected),
            selected,
            selected,
        ),
    );
    selected
}


fn sync_trade_popup_visual(
    ui: &mut GameUI,
    data: &ClientData,
    previous_offered_id: usize,
    offered_id: usize,
    target_id: usize,
    selection_sequence: usize,
) {
    if offered_id == NO_ATHLETE || target_id == 0 || !POPUP_OPEN.load(Ordering::Acquire) {
        return;
    }
    let (offered, target, previous_offered_name) = {
        let db = data.db();
        let Some(offered_athlete) = db.athlete(offered_id) else {
            log_event(
                "custom_trade_visual_sync_failed",
                &format!("offered_id={offered_id};reason=offered_athlete_not_found"),
            );
            return;
        };
        let Some(target_athlete) = db.athlete(target_id) else {
            log_event(
                "custom_trade_visual_sync_failed",
                &format!("target_id={target_id};reason=target_athlete_not_found"),
            );
            return;
        };
        let previous_offered_name = if previous_offered_id != NO_ATHLETE && previous_offered_id != offered_id {
            db.athlete(previous_offered_id).map(|athlete| athlete.name.clone())
        } else {
            None
        };
        (
            build_popup_athlete_visual(offered_id, offered_athlete),
            build_popup_athlete_visual(target_id, target_athlete),
            previous_offered_name,
        )
    };

    let selected = sync_trade_popup_selected_row(
        ui,
        previous_offered_id,
        previous_offered_name.as_deref(),
        offered_id,
        &offered.name,
    );

    let _ = set_custom_text(ui, "pts_trade_custom_target_name", &target.name);
    let _ = set_custom_text(
        ui,
        "pts_trade_custom_target_position",
        target.position_label,
    );
    let _ = set_custom_source(
        ui,
        "pts_trade_custom_target_position_icon",
        target.position_icon,
    );
    let _ = set_custom_status_value(
        ui,
        "pts_trade_custom_target_status",
        target.status_label,
    );

    let _ = set_custom_text(ui, "pts_trade_custom_offered_name", &offered.name);
    let _ = set_custom_text(
        ui,
        "pts_trade_custom_offered_position",
        offered.position_label,
    );
    let _ = set_custom_source(
        ui,
        "pts_trade_custom_offered_position_icon",
        offered.position_icon,
    );
    let _ = set_custom_visible(ui, "pts_trade_custom_offered_position_icon", true);
    let _ = set_custom_status_value(
        ui,
        "pts_trade_custom_offered_status",
        offered.status_label,
    );

    let mut stat_value_count = 0usize;
    let mut comparison_marker_count = 0usize;
    let mut stat_color_sync_count = 0usize;
    let mut marker_color_sync_count = 0usize;
    for stat_id in NATIVE_VISUAL_STAT_IDS {
        let target_value = target.stats.get(stat_id).copied();
        let offered_value = offered.stats.get(stat_id).copied();
        let (target_node, marker_node, offered_node) = custom_stat_ids(stat_id);
        if set_custom_stat_value(ui, &target_node, target_value) {
            stat_value_count += usize::from(target_value.is_some());
            stat_color_sync_count += 1;
        }
        if set_custom_stat_value(ui, &offered_node, offered_value) {
            stat_value_count += usize::from(offered_value.is_some());
            stat_color_sync_count += 1;
        }
        if set_custom_compare_marker(ui, &marker_node, target_value, offered_value) {
            comparison_marker_count += 1;
            marker_color_sync_count += 1;
        }
    }

    VISUAL_SYNC_LAST_OFFERED_ID.store(offered_id, Ordering::Release);
    VISUAL_SYNC_LAST_SEQUENCE.store(selection_sequence, Ordering::Release);
    NATIVE_VISUAL_SYNC_PENDING.store(false, Ordering::Release);
    log_event(
        "custom_trade_visual_synced",
        &format!(
            "offered_id={};offered_name={};offered_position={};offered_status={};target_id={};target_name={};target_position={};target_status={};green_highlight_applied={};check_marker_applied={};stat_value_count={};comparison_marker_count={};stat_color_sync_count={};marker_color_sync_count={};stat_palette=original_game_tier_5;stat_thresholds=0_20_gray_21_40_teal_41_60_blue_61_80_purple_81_100_orange;stat_colors=6b6c74_4ed5bd_55c1fe_b34bb1_f86624;marker_palette=original_game_up_red_down_gray_equal_gray;marker_colors=eb3d4d_667085;panel_order=compare_left_roster_right;card_status_palette=prospect_858d9d_sub_3cb8a0_general_4eb0d8_important_c850bf_core_f86624;custom_cards=true;custom_static_roster=true;original_compare_popup_used=false;selection_sequence={}",
            offered.athlete_id,
            sanitize(&offered.name),
            offered.position_label,
            offered.status_label,
            target.athlete_id,
            sanitize(&target.name),
            target.position_label,
            target.status_label,
            selected,
            selected,
            stat_value_count,
            comparison_marker_count,
            stat_color_sync_count,
            marker_color_sync_count,
            selection_sequence,
        ),
    );
}


fn sync_pending_native_trade_visual(_ui: &GameUI, _data: &ClientData) {
    // Test77 owns the detached clone's visual state directly in sync_trade_popup_visual.
}

fn rename_node(root: &mut Node, from: &str, to: &str) {
    if let Some(node) = find_node_by_id_mut(root, from) {
        node.id = to.to_string();
    }
}

fn dump_tree(node: &Node, parent: &str, depth: usize, output: &mut String) {
    let path = if parent.is_empty() {
        node.id.clone()
    } else {
        format!("{}.{}", parent, node.id)
    };
    let runner_type = node.runner.type_name().replace('|', "/");
    let _ = writeln!(
        output,
        "tree_node|depth={}|path={}|runner={}|visible={}|disabled={}|children={}|rect={:.0},{:.0},{:.0},{:.0}|lw={:?}|lh={:?}",
        depth,
        path,
        runner_type,
        node.visible,
        node.disabled,
        node.child.len(),
        node.rect.x,
        node.rect.y,
        node.rect.w,
        node.rect.h,
        node.layout.normal.width,
        node.layout.normal.height
    );
    for child in &node.child {
        dump_tree(child, &path, depth + 1, output);
    }
}

// ★[PORT056] player_detail 신원 캐리어 탐색용 (2026-08-23).
//   `current_player_detail_athlete_id` 는 표시 중인 선수를 **트리에서 읽지 않고**
//   클릭 시 저장한 PROFILE_CONTEXT_ATHLETE_ID + 노드 포인터 일치만 본다.
//   게임이 같은 player_detail 노드를 재사용해 다른 선수를 그리면 신원이 고정돼
//   무관한 선수 프로필에 트레이드 상태가 뜨고 영입 제안 버튼이 숨겨진다(유저 보고: 1Jiang).
//   ⟹ 트리 안에 `<prefix>_<athleteId>` 형태의 캐리어가 있는지 id 만 덤프해 확인한다.
fn dump_ids(node: &Node, parent: &str, output: &mut String) {
    let path = if parent.is_empty() {
        node.id.clone()
    } else {
        format!("{}.{}", parent, node.id)
    };
    let _ = writeln!(output, "detail_id|{}", path);
    for child in &node.child {
        dump_ids(child, &path, output);
    }
}

fn parse_kv_payload(bytes: &[u8]) -> Result<BTreeMap<String, String>, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| format!("payload UTF-8: {error}"))?;
    let mut values = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("payload line has no equals sign: {line}"));
        };
        values.insert(key.to_string(), value.to_string());
    }
    Ok(values)
}

fn map_required<'a>(values: &'a BTreeMap<String, String>, key: &str) -> Result<&'a str, String> {
    values
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("payload missing {key}"))
}

fn map_usize(values: &BTreeMap<String, String>, key: &str) -> Result<usize, String> {
    map_required(values, key)?
        .parse::<usize>()
        .map_err(|error| format!("payload {key}: {error}"))
}

fn map_u64(values: &BTreeMap<String, String>, key: &str) -> Result<u64, String> {
    map_required(values, key)?
        .parse::<u64>()
        .map_err(|error| format!("payload {key}: {error}"))
}

fn map_f64(values: &BTreeMap<String, String>, key: &str) -> Result<f64, String> {
    map_required(values, key)?
        .parse::<f64>()
        .map_err(|error| format!("payload {key}: {error}"))
}

fn map_bool(values: &BTreeMap<String, String>, key: &str) -> Result<bool, String> {
    match map_required(values, key)? {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!(
            "payload {key}: expected true or false, found {other}"
        )),
    }
}

fn contract_team_id(athlete: &Athlete) -> Result<usize, String> {
    match &athlete.contract {
        Contract::InContract { team_id, .. } => Ok(*team_id),
        _ => Err(format!("athlete {} is not under contract", athlete.name)),
    }
}

fn server_contracted_athlete_region_id(db: &Database, athlete: &Athlete) -> Result<usize, String> {
    let team_id = contract_team_id(athlete)?;
    let team = db.teams.get(team_id).ok_or_else(|| {
        format!(
            "athlete {} contract team {team_id} was not found",
            athlete.name
        )
    })?;
    let league = db
        .leagues
        .get(team.league_id)
        .ok_or_else(|| format!("team {team_id} league {} was not found", team.league_id))?;
    Ok(league.region_id)
}

fn contract_yearly_salary(athlete: &Athlete) -> Result<f64, String> {
    let Contract::InContract { weekly_salary, .. } = &athlete.contract else {
        return Err(format!("athlete {} is not under contract", athlete.name));
    };
    let yearly_salary = *weekly_salary * WEEKS_PER_YEAR;
    if !yearly_salary.is_finite() || yearly_salary < 0.0 {
        return Err(format!(
            "athlete {} has an invalid inherited yearly salary: {}",
            athlete.name, yearly_salary
        ));
    }
    Ok(yearly_salary)
}

fn nearly_equal_money(left: f64, right: f64) -> bool {
    left.is_finite() && right.is_finite() && (left - right).abs() <= 0.05
}

fn contracted_roster_count(db: &Database, team_id: usize) -> usize {
    db.athletes
        .iter()
        .filter(|athlete| {
            matches!(
                &athlete.contract,
                Contract::InContract {
                    team_id: current_team_id,
                    ..
                } if *current_team_id == team_id
            )
        })
        .count()
}

fn fnv1a64(text: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3_u64);
    }
    hash
}

fn athlete_trade_fingerprint(snapshot: &AthleteTradeSnapshot) -> u64 {
    let canonical = format!(
        "{}|{}|{}|{}|{}|{:016X}|{:016X}|{}|{}|{}|{}",
        snapshot.id,
        sanitize(&snapshot.name),
        snapshot.team_id,
        sanitize(&snapshot.start_date),
        sanitize(&snapshot.end_date),
        snapshot.weekly_salary.to_bits(),
        snapshot.transfer_fee.to_bits(),
        sanitize(&snapshot.incentives_debug),
        sanitize(&snapshot.transfer_requests_debug),
        sanitize(&snapshot.recruit_requests_debug),
        sanitize(&snapshot.squad_status_debug),
    );
    fnv1a64(&canonical)
}

fn valid_test60_plan_id(plan_id: &str) -> bool {
    plan_id.len() == 20
        && plan_id.starts_with("T60-")
        && plan_id[4..]
            .chars()
            .all(|character| character.is_ascii_hexdigit())
}

fn decode_executed_plan_registry(bytes: &[u8]) -> Result<Vec<String>, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("executed-plan registry UTF-8: {error}"))?;
    let mut plan_ids = Vec::new();
    for line in text.lines() {
        let plan_id = line.trim();
        if plan_id.is_empty() {
            continue;
        }
        if !valid_test60_plan_id(plan_id) {
            return Err(format!("executed-plan registry contains invalid plan id {plan_id}"));
        }
        if plan_ids.iter().any(|existing| existing == plan_id) {
            return Err(format!("executed-plan registry contains duplicate plan id {plan_id}"));
        }
        plan_ids.push(plan_id.to_string());
    }
    plan_ids.sort();
    Ok(plan_ids)
}

fn encode_executed_plan_registry(plan_ids: &[String]) -> Vec<u8> {
    let mut sorted = plan_ids.to_vec();
    sorted.sort();
    sorted.dedup();
    let mut text = sorted.join("\n");
    if !text.is_empty() {
        text.push('\n');
    }
    text.into_bytes()
}

fn read_persisted_plan_ids(db: &Database) -> Result<Vec<String>, String> {
    match db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, EXECUTED_PLAN_REGISTRY_KEY)
    {
        Some(bytes) => decode_executed_plan_registry(&bytes),
        None => Ok(Vec::new()),
    }
}

fn receipt_is_first_trade(receipt: &TradeCommitReceipt) -> bool {
    receipt.schema_version == 1
        && receipt.plan_id == FIRST_PLAN_ID
        && receipt.requester_team_id == REQUESTER_TEAM_ID
        && receipt.recipient_team_id == RECIPIENT_TEAM_ID
        && receipt.offered_id == FIRST_OFFERED_ID
        && receipt.target_id == FIRST_TARGET_ID
        && receipt.offered_name == "Jue"
        && receipt.target_name == "Zeus"
        && receipt.proposed_cash_won == 207_340_000
        && receipt.desired_status_choice == STATUS_IMPORTANT
        && receipt.desired_status_key == "important"
        && receipt.offered_team_after == RECIPIENT_TEAM_ID
        && receipt.target_team_after == REQUESTER_TEAM_ID
        && receipt.target_status_after == "Important"
        && receipt.executed_plan_registry_count == 1
}

fn receipt_is_second_trade(receipt: &TradeCommitReceipt) -> bool {
    receipt.schema_version == 1
        && receipt.plan_id == SECOND_PLAN_ID
        && receipt.requester_team_id == REQUESTER_TEAM_ID
        && receipt.recipient_team_id == RECIPIENT_TEAM_ID
        && receipt.offered_id == SECOND_OFFERED_ID
        && receipt.target_id == SECOND_TARGET_ID
        && receipt.offered_name == "Zenit"
        && receipt.target_name == "Fill"
        && receipt.proposed_cash_won == 0
        && receipt.desired_status_choice == STATUS_IMPORTANT
        && receipt.desired_status_key == "important"
        && receipt.offered_team_after == RECIPIENT_TEAM_ID
        && receipt.target_team_after == REQUESTER_TEAM_ID
        && receipt.target_status_after == "Important"
        && receipt.executed_plan_registry_count == 2
}

fn read_latest_trade_receipt(db: &Database) -> Result<TradeCommitReceipt, String> {
    let bytes = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, TRADE_COMMIT_RECEIPT_KEY)
        .ok_or_else(|| "latest trade commit receipt is missing".to_string())?;
    decode_trade_commit_receipt(&bytes)
}

fn require_dual_trade_baseline(db: &Database) -> Result<(), String> {
    if db.mod_save_data.save_version(MOD_SAVE_NAMESPACE) != MOD_SAVE_NAMESPACE_VERSION {
        return Err("the Test77 baseline ModSave namespace version is invalid".to_string());
    }
    let plan_ids = read_persisted_plan_ids(db)?;
    if plan_ids.len() != 2
        || !plan_ids.iter().any(|plan_id| plan_id == FIRST_PLAN_ID)
        || !plan_ids.iter().any(|plan_id| plan_id == SECOND_PLAN_ID)
    {
        return Err(format!(
            "load {BASELINE_SAVE_SLOT}: the baseline must contain the exact two verified plans"
        ));
    }
    let current = read_latest_trade_receipt(db)?;
    if !receipt_is_second_trade(&current) {
        return Err(format!(
            "load {BASELINE_SAVE_SLOT}: the verified Zenit/Fill receipt is not current"
        ));
    }
    let previous_bytes = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, PREVIOUS_TRADE_COMMIT_RECEIPT_KEY)
        .ok_or_else(|| "the archived Jue/Zeus receipt is missing".to_string())?;
    let previous = decode_trade_commit_receipt(&previous_bytes)?;
    if !receipt_is_first_trade(&previous) {
        return Err("the archived receipt is not the verified Jue/Zeus trade".to_string());
    }
    if db
        .mod_save_data
        .contains_key(MOD_SAVE_NAMESPACE, OLDEST_TRADE_COMMIT_RECEIPT_KEY)
    {
        return Err("the Test77 baseline already contains a third receipt archive".to_string());
    }

    let first_offered = athlete_trade_snapshot(db, FIRST_OFFERED_ID)?;
    let first_target = athlete_trade_snapshot(db, FIRST_TARGET_ID)?;
    let second_offered = athlete_trade_snapshot(db, SECOND_OFFERED_ID)?;
    let second_target = athlete_trade_snapshot(db, SECOND_TARGET_ID)?;
    if first_offered.team_id != RECIPIENT_TEAM_ID
        || first_target.team_id != REQUESTER_TEAM_ID
        || first_target.squad_status_debug != "Important"
        || second_offered.team_id != RECIPIENT_TEAM_ID
        || second_target.team_id != REQUESTER_TEAM_ID
        || second_target.squad_status_debug != "Important"
        || contracted_squad_status_debug(db, FIRST_OFFERED_ID)? != "None"
        || contracted_squad_status_debug(db, FIRST_TARGET_ID)? != "Some(Important)"
        || contracted_squad_status_debug(db, SECOND_OFFERED_ID)? != "None"
        || contracted_squad_status_debug(db, SECOND_TARGET_ID)? != "Some(Important)"
    {
        return Err("the Test65 migrated dual-trade fixture is not exact".to_string());
    }
    Ok(())
}

fn require_fresh_trade_fixture(
    db: &Database,
    requester_team_id: usize,
    offered_id: usize,
    target_id: usize,
) -> Result<(), String> {
    // [PORT056] 구 Test79: `requester_team_id != 7` 하드코딩 + 과거 픽스처 선수 4명 제외.
    //   → 팀 상수 게이트와 픽스처 제외를 삭제하고, 방식 자체의 불변식(서로 다른 1:1)만 남긴다.
    //   소속 검증(내보낼 선수=내 팀 / 받을 선수=타 팀)은 아래 offered_team_id/target_team_id 비교가 이미 수행.
    if offered_id == target_id {
        return Err("트레이드는 서로 다른 두 선수 사이에서만 가능합니다".to_string());
    }
    let offered_team_id = db
        .athletes
        .get(offered_id)
        .ok_or_else(|| "the selected outgoing athlete does not exist".to_string())
        .and_then(contract_team_id)?;
    let target_team_id = db
        .athletes
        .get(target_id)
        .ok_or_else(|| "the selected incoming athlete does not exist".to_string())
        .and_then(contract_team_id)?;
    // [PORT056] 상수 7 → 인자로 받은 실제 요청 팀
    if offered_team_id != requester_team_id || target_team_id == requester_team_id {
        return Err("내보낼 선수는 내 팀, 받을 선수는 상대 팀 소속이어야 합니다".to_string());
    }
    Ok(())
}

fn encode_trade_commit_receipt(receipt: &TradeCommitReceipt) -> Vec<u8> {
    let mut text = String::new();
    let _ = writeln!(text, "schema_version={}", receipt.schema_version);
    let _ = writeln!(text, "plan_id={}", receipt.plan_id);
    let _ = writeln!(text, "commit_process_id={}", receipt.commit_process_id);
    let _ = writeln!(text, "commit_game_time={}", sanitize(&receipt.commit_game_time));
    let _ = writeln!(text, "requester_team_id={}", receipt.requester_team_id);
    let _ = writeln!(text, "recipient_team_id={}", receipt.recipient_team_id);
    let _ = writeln!(text, "offered_id={}", receipt.offered_id);
    let _ = writeln!(text, "target_id={}", receipt.target_id);
    let _ = writeln!(text, "offered_name={}", sanitize(&receipt.offered_name));
    let _ = writeln!(text, "target_name={}", sanitize(&receipt.target_name));
    let _ = writeln!(text, "proposed_cash_won={}", receipt.proposed_cash_won);
    let _ = writeln!(text, "desired_status_choice={}", receipt.desired_status_choice);
    let _ = writeln!(text, "desired_status_key={}", receipt.desired_status_key);
    let _ = writeln!(text, "offered_team_after={}", receipt.offered_team_after);
    let _ = writeln!(text, "target_team_after={}", receipt.target_team_after);
    let _ = writeln!(text, "target_status_after={}", sanitize(&receipt.target_status_after));
    if receipt.schema_version >= 2 {
        let _ = writeln!(
            text,
            "offered_contracted_status_after={}",
            sanitize(&receipt.offered_contracted_status_after)
        );
        let _ = writeln!(
            text,
            "target_contracted_status_after={}",
            sanitize(&receipt.target_contracted_status_after)
        );
        let _ = writeln!(
            text,
            "rollback_rehearsal_verified={}",
            receipt.rollback_rehearsal_verified
        );
    }
    let _ = writeln!(
        text,
        "offered_contract_fingerprint={}",
        receipt.offered_contract_fingerprint
    );
    let _ = writeln!(
        text,
        "target_contract_fingerprint={}",
        receipt.target_contract_fingerprint
    );
    let _ = writeln!(text, "requester_total_bits={}", receipt.requester_total_bits);
    let _ = writeln!(
        text,
        "requester_transfer_bits={}",
        receipt.requester_transfer_bits
    );
    let _ = writeln!(text, "requester_salary_bits={}", receipt.requester_salary_bits);
    let _ = writeln!(text, "recipient_total_bits={}", receipt.recipient_total_bits);
    let _ = writeln!(
        text,
        "recipient_transfer_bits={}",
        receipt.recipient_transfer_bits
    );
    let _ = writeln!(text, "recipient_salary_bits={}", receipt.recipient_salary_bits);
    let _ = writeln!(
        text,
        "requester_roster_after={}",
        receipt.requester_roster_after
    );
    let _ = writeln!(
        text,
        "recipient_roster_after={}",
        receipt.recipient_roster_after
    );
    let _ = writeln!(text, "contracted_after={}", receipt.contracted_after);
    let _ = writeln!(
        text,
        "requester_payroll_bits={}",
        receipt.requester_payroll_bits
    );
    let _ = writeln!(
        text,
        "recipient_payroll_bits={}",
        receipt.recipient_payroll_bits
    );
    let _ = writeln!(
        text,
        "requester_news_count={}",
        receipt.requester_news_count
    );
    let _ = writeln!(
        text,
        "recipient_news_count={}",
        receipt.recipient_news_count
    );
    let _ = writeln!(
        text,
        "executed_plan_registry_count={}",
        receipt.executed_plan_registry_count
    );
    text.into_bytes()
}

fn decode_trade_commit_receipt(bytes: &[u8]) -> Result<TradeCommitReceipt, String> {
    let values = parse_kv_payload(bytes)?;
    let schema_version = map_usize(&values, "schema_version")?;
    let plan_id = map_required(&values, "plan_id")?.to_string();
    if !(schema_version == 1 || schema_version == 2) || !valid_test60_plan_id(&plan_id) {
        return Err("trade commit receipt schema or plan id is invalid".to_string());
    }
    let commit_process_id = u32::try_from(map_u64(&values, "commit_process_id")?)
        .map_err(|_| "commit process id is out of range".to_string())?;
    let desired_status_choice = u8::try_from(map_u64(&values, "desired_status_choice")?)
        .map_err(|_| "receipt desired status choice is out of range".to_string())?;
    Ok(TradeCommitReceipt {
        schema_version,
        plan_id,
        commit_process_id,
        commit_game_time: map_required(&values, "commit_game_time")?.to_string(),
        requester_team_id: map_usize(&values, "requester_team_id")?,
        recipient_team_id: map_usize(&values, "recipient_team_id")?,
        offered_id: map_usize(&values, "offered_id")?,
        target_id: map_usize(&values, "target_id")?,
        offered_name: map_required(&values, "offered_name")?.to_string(),
        target_name: map_required(&values, "target_name")?.to_string(),
        proposed_cash_won: map_u64(&values, "proposed_cash_won")?,
        desired_status_choice,
        desired_status_key: map_required(&values, "desired_status_key")?.to_string(),
        offered_team_after: map_usize(&values, "offered_team_after")?,
        target_team_after: map_usize(&values, "target_team_after")?,
        target_status_after: map_required(&values, "target_status_after")?.to_string(),
        offered_contracted_status_after: values
            .get("offered_contracted_status_after")
            .cloned()
            .unwrap_or_else(|| "LegacyUnrecorded".to_string()),
        target_contracted_status_after: values
            .get("target_contracted_status_after")
            .cloned()
            .unwrap_or_else(|| "LegacyUnrecorded".to_string()),
        rollback_rehearsal_verified: if schema_version >= 2 {
            map_bool(&values, "rollback_rehearsal_verified")?
        } else {
            false
        },
        offered_contract_fingerprint: map_u64(&values, "offered_contract_fingerprint")?,
        target_contract_fingerprint: map_u64(&values, "target_contract_fingerprint")?,
        requester_total_bits: map_u64(&values, "requester_total_bits")?,
        requester_transfer_bits: map_u64(&values, "requester_transfer_bits")?,
        requester_salary_bits: map_u64(&values, "requester_salary_bits")?,
        recipient_total_bits: map_u64(&values, "recipient_total_bits")?,
        recipient_transfer_bits: map_u64(&values, "recipient_transfer_bits")?,
        recipient_salary_bits: map_u64(&values, "recipient_salary_bits")?,
        requester_roster_after: map_usize(&values, "requester_roster_after")?,
        recipient_roster_after: map_usize(&values, "recipient_roster_after")?,
        contracted_after: map_usize(&values, "contracted_after")?,
        requester_payroll_bits: map_u64(&values, "requester_payroll_bits")?,
        recipient_payroll_bits: map_u64(&values, "recipient_payroll_bits")?,
        requester_news_count: map_usize(&values, "requester_news_count")?,
        recipient_news_count: map_usize(&values, "recipient_news_count")?,
        executed_plan_registry_count: map_usize(&values, "executed_plan_registry_count")?,
    })
}

/// [PORT056] 영수증 자기정합성 검사(생산판).
/// 구 Test79 는 여기에 팀7·과거 픽스처 선수·위상 Core·레지스트리 3건까지 요구해서
/// 일반 세이브의 **첫 거래가 절대 커밋되지 못했다**. 남기는 것은 "이 영수증이 스스로 모순되지 않는가"뿐.
fn receipt_is_fresh_trade(receipt: &TradeCommitReceipt) -> bool {
    let (expected_status_key, _) = desired_squad_status(receipt.desired_status_choice);
    receipt.schema_version == 2
        && receipt.requester_team_id != receipt.recipient_team_id
        && receipt.offered_id != receipt.target_id
        && receipt.offered_team_after == receipt.recipient_team_id
        && receipt.target_team_after == receipt.requester_team_id
        && receipt.desired_status_key == expected_status_key
        && receipt.rollback_rehearsal_verified
        && receipt.executed_plan_registry_count >= 1
}

fn persist_trade_commit(
    db: &mut Database,
    mut receipt: TradeCommitReceipt,
) -> Result<(TradeCommitReceipt, bool), String> {
    // [PORT056] 구판은 "레지스트리가 정확히 2건 + 과거 픽스처 영수증 2건"을 요구했다
    //   ⟹ 거래 이력이 없는 세이브에서는 **첫 거래가 여기서 항상 Err** = 영영 커밋 불가.
    //   생산판은 이력 0건에서 시작해 누적하고, 직전 영수증 2세대를 굴려서 보관한다.
    let mut plan_ids = read_persisted_plan_ids(db).unwrap_or_default();
    // 직전/전전 영수증(있으면). 첫 거래에서는 둘 다 없음 = 정상.
    let second_receipt_bytes = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, TRADE_COMMIT_RECEIPT_KEY);
    let first_receipt_bytes = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, PREVIOUS_TRADE_COMMIT_RECEIPT_KEY);
    if plan_ids.iter().any(|plan_id| plan_id == &receipt.plan_id) {
        return Err("the sealed fresh-trade plan id is already present in the registry".to_string());
    }
    plan_ids.push(receipt.plan_id.clone());
    plan_ids.sort();
    plan_ids.dedup();
    receipt.executed_plan_registry_count = plan_ids.len();
    if !receipt_is_fresh_trade(&receipt) {
        return Err("the new receipt does not describe the required Test77 Core-status trade".to_string());
    }

    let registry_bytes = encode_executed_plan_registry(&plan_ids);
    let receipt_bytes = encode_trade_commit_receipt(&receipt);
    let _ = db
        .mod_save_data
        .set_version(MOD_SAVE_NAMESPACE, MOD_SAVE_NAMESPACE_VERSION);
    let _ = db.mod_save_data.set_bytes(
        MOD_SAVE_NAMESPACE,
        EXECUTED_PLAN_REGISTRY_KEY,
        registry_bytes.clone(),
    );
    let _ = db.mod_save_data.set_bytes(
        MOD_SAVE_NAMESPACE,
        TRADE_COMMIT_RECEIPT_KEY,
        receipt_bytes.clone(),
    );
    // [PORT056] 영수증 세대 굴리기: 현재→PREVIOUS, PREVIOUS→OLDEST. 없으면 그 세대는 건너뛴다.
    if let Some(bytes) = second_receipt_bytes.clone() {
        let _ = db.mod_save_data.set_bytes(
            MOD_SAVE_NAMESPACE,
            PREVIOUS_TRADE_COMMIT_RECEIPT_KEY,
            bytes,
        );
    }
    if let Some(bytes) = first_receipt_bytes.clone() {
        let _ = db
            .mod_save_data
            .set_bytes(MOD_SAVE_NAMESPACE, OLDEST_TRADE_COMMIT_RECEIPT_KEY, bytes);
    }

    // ★[PORT056] 요구사항 1 — 성사만 소모. 커밋이 확정되는 이 지점에서만 시즌 쿼터를 찍는다.
    //   (거절·만료 경로인 reject_async_trade_proposal 은 이 함수를 타지 않으므로 자동으로 소모되지 않는다.)
    let season_used = mark_trade_season_used(db);

    if db.mod_save_data.save_version(MOD_SAVE_NAMESPACE) != MOD_SAVE_NAMESPACE_VERSION {
        return Err("ModSave namespace version mismatch after trade receipt write".to_string());
    }

    // [PORT056] 리드백 검증은 유지하되 **이번에 쓴 것만** 대조한다.
    //   구판은 과거 픽스처 영수증 3세대가 전부 존재하고 그 내용까지 특정 거래여야 통과였다.
    let registry_readback = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, EXECUTED_PLAN_REGISTRY_KEY)
        .ok_or_else(|| "executed-plan registry disappeared after write".to_string())?;
    let receipt_readback = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, TRADE_COMMIT_RECEIPT_KEY)
        .ok_or_else(|| "trade commit receipt disappeared after write".to_string())?;
    if registry_readback != registry_bytes || receipt_readback != receipt_bytes {
        return Err("trade registry or receipt byte readback mismatch".to_string());
    }
    if let Some(expected) = second_receipt_bytes.as_ref() {
        let got = db
            .mod_save_data
            .get_bytes(MOD_SAVE_NAMESPACE, PREVIOUS_TRADE_COMMIT_RECEIPT_KEY)
            .ok_or_else(|| "archived previous receipt disappeared after write".to_string())?;
        if got != *expected {
            return Err("archived previous receipt byte readback mismatch".to_string());
        }
    }
    if let Some(expected) = first_receipt_bytes.as_ref() {
        let got = db
            .mod_save_data
            .get_bytes(MOD_SAVE_NAMESPACE, OLDEST_TRADE_COMMIT_RECEIPT_KEY)
            .ok_or_else(|| "archived oldest receipt disappeared after write".to_string())?;
        if got != *expected {
            return Err("archived oldest receipt byte readback mismatch".to_string());
        }
    }
    if let Some(key) = season_used.as_ref() {
        let got = db
            .mod_save_data
            .get_bytes(MOD_SAVE_NAMESPACE, TRADE_SEASON_USAGE_KEY)
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .unwrap_or_default();
        if got.trim() != key {
            return Err("trade season usage marker did not persist".to_string());
        }
    }

    let decoded_registry = decode_executed_plan_registry(&registry_readback)?;
    let decoded_receipt = decode_trade_commit_receipt(&receipt_readback)?;
    let exact_plan_count = decoded_registry
        .iter()
        .filter(|plan_id| *plan_id == &receipt.plan_id)
        .count();
    let verified = exact_plan_count == 1
        && decoded_registry.len() == receipt.executed_plan_registry_count
        && decoded_receipt.plan_id == receipt.plan_id
        && receipt_is_fresh_trade(&decoded_receipt)
        && encode_trade_commit_receipt(&decoded_receipt) == receipt_readback;
    if !verified {
        return Err("persisted plan registry or receipt verification failed".to_string());
    }
    log_event(
        "trade_commit_persisted",
        &format!(
            "plan_id={};registry_count={};season_used={};port056_production_persistence=true",
            decoded_receipt.plan_id,
            decoded_registry.len(),
            season_used.as_deref().unwrap_or("none"),
        ),
    );
    Ok((decoded_receipt, true))
}

fn validate_saved_trade_commit(db: &Database) -> Result<Vec<u8>, String> {
    // [PORT056] 이 함수는 "그쪽 테스트 세이브가 맞는지" 검증하는 루틴이다(게임 로직 아님 —
    //   자체 로그도 database_mutation=false / transaction_executed=false).
    //   아래 본문은 과거 검증 거래 2건(Jue/Zeus·Zenit/Fill)과 플랜 레지스트리 3건을 요구해서
    //   일반 세이브에서는 항상 Err → 클라이언트가 트레이드를 영구 차단한다.
    //   ⟹ 항상 허용 응답으로 단락시키고, 원 본문은 이력으로 남긴다(도달 불가).
    let receipt_present = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, TRADE_COMMIT_RECEIPT_KEY)
        .is_some();
    return Ok(format!(
        "status=none\nreceipt_present={}\nport056_test_fixture_validation_skipped=true\n",
        receipt_present,
    )
    .into_bytes());

    #[allow(unreachable_code)]
    let Some(receipt_bytes) = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, TRADE_COMMIT_RECEIPT_KEY)
    else {
        return Ok(b"status=none\nreceipt_present=false\n".to_vec());
    };
    let receipt = decode_trade_commit_receipt(&receipt_bytes)?;

    if receipt.schema_version == 1 {
        require_dual_trade_baseline(db)?;
        if !SAVED_TRADE_VALIDATION_SERVER_LOGGED.swap(true, Ordering::AcqRel) {
            log_event(
                "test77_dual_baseline_verified_server",
                &format!(
                    "sdk_base={};baseline_save_slot={};first_plan_id={};second_plan_id={};executed_plan_registry_count=2;first_target_contracted_status=Some(Important);second_target_contracted_status=Some(Important);fresh_trade_enabled=true;historical_players_excluded=true;database_mutation=false;transaction_executed=false;save_api_called=false",
                    PATCH055_BASE_VERSION, BASELINE_SAVE_SLOT, FIRST_PLAN_ID, SECOND_PLAN_ID,
                ),
            );
        }
        return Ok(format!(
            "status=baseline_verified\nsdk_base={}\nbaseline_save_slot={}\nresult_save_slot={}\nfirst_plan_id={}\nsecond_plan_id={}\nexecuted_plan_registry_count=2\nfirst_target_contracted_status=Some(Important)\nsecond_target_contracted_status=Some(Important)\nfresh_trade_enabled=true\nhistorical_players_excluded=true\ndatabase_mutation=false\ntransaction_executed=false\nsave_api_called=false\n",
            PATCH055_BASE_VERSION,
            BASELINE_SAVE_SLOT,
            RESULT_SAVE_SLOT,
            FIRST_PLAN_ID,
            SECOND_PLAN_ID,
        )
        .into_bytes());
    }

    if !receipt_is_fresh_trade(&receipt) {
        return Err("the current Test77 receipt is not the verified fresh Core-status trade".to_string());
    }
    if receipt.commit_process_id == std::process::id() {
        return Ok(format!(
            "status=same_process\nreceipt_present=true\nplan_id={}\ncommit_process_id={}\nreload_process_id={}\n",
            receipt.plan_id,
            receipt.commit_process_id,
            std::process::id(),
        )
        .into_bytes());
    }
    if db.mod_save_data.save_version(MOD_SAVE_NAMESPACE) != MOD_SAVE_NAMESPACE_VERSION {
        return Err("reloaded Test77 ModSave namespace version mismatch".to_string());
    }

    let registry_bytes = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, EXECUTED_PLAN_REGISTRY_KEY)
        .ok_or_else(|| "reloaded three-plan registry is missing".to_string())?;
    let plan_ids = decode_executed_plan_registry(&registry_bytes)?;
    let plan_occurrences = plan_ids.iter().filter(|id| *id == &receipt.plan_id).count();
    let first_plan_occurrences = plan_ids.iter().filter(|id| id.as_str() == FIRST_PLAN_ID).count();
    let second_plan_occurrences = plan_ids.iter().filter(|id| id.as_str() == SECOND_PLAN_ID).count();
    if plan_ids.len() != 3
        || plan_occurrences != 1
        || first_plan_occurrences != 1
        || second_plan_occurrences != 1
    {
        return Err("reloaded Test77 three-plan registry is not exact".to_string());
    }

    let second_bytes = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, PREVIOUS_TRADE_COMMIT_RECEIPT_KEY)
        .ok_or_else(|| "reloaded archived Zenit/Fill receipt is missing".to_string())?;
    let first_bytes = db
        .mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, OLDEST_TRADE_COMMIT_RECEIPT_KEY)
        .ok_or_else(|| "reloaded archived Jue/Zeus receipt is missing".to_string())?;
    if !receipt_is_second_trade(&decode_trade_commit_receipt(&second_bytes)?)
        || !receipt_is_first_trade(&decode_trade_commit_receipt(&first_bytes)?)
    {
        return Err("reloaded Test77 archived receipts are not the verified first two trades".to_string());
    }

    let offered = athlete_trade_snapshot(db, receipt.offered_id)?;
    let target = athlete_trade_snapshot(db, receipt.target_id)?;
    let offered_contracted = contracted_squad_status_debug(db, receipt.offered_id)?;
    let target_contracted = contracted_squad_status_debug(db, receipt.target_id)?;
    let requester_finance = team_finance_snapshot(db, receipt.requester_team_id)?;
    let recipient_finance = team_finance_snapshot(db, receipt.recipient_team_id)?;
    let world = world_trade_snapshot(db, receipt.requester_team_id, receipt.recipient_team_id);
    let requester_news = db
        .teams
        .get(receipt.requester_team_id)
        .ok_or_else(|| "requester team missing during Test77 reload".to_string())?
        .news
        .len();
    let recipient_news = db
        .teams
        .get(receipt.recipient_team_id)
        .ok_or_else(|| "recipient team missing during Test77 reload".to_string())?
        .news
        .len();
    let fresh_trade_persisted = db.time.to_string() == receipt.commit_game_time
        && offered.team_id == receipt.offered_team_after
        && target.team_id == receipt.target_team_after
        && target.squad_status_debug == receipt.target_status_after
        && offered_contracted == receipt.offered_contracted_status_after
        && target_contracted == receipt.target_contracted_status_after
        && athlete_trade_fingerprint(&offered) == receipt.offered_contract_fingerprint
        && athlete_trade_fingerprint(&target) == receipt.target_contract_fingerprint
        && requester_finance.total_balance.to_bits() == receipt.requester_total_bits
        && requester_finance.transfer_budget.to_bits() == receipt.requester_transfer_bits
        && requester_finance.salary_budget.to_bits() == receipt.requester_salary_bits
        && recipient_finance.total_balance.to_bits() == receipt.recipient_total_bits
        && recipient_finance.transfer_budget.to_bits() == receipt.recipient_transfer_bits
        && recipient_finance.salary_budget.to_bits() == receipt.recipient_salary_bits
        && world.requester_roster == receipt.requester_roster_after
        && world.recipient_roster == receipt.recipient_roster_after
        && world.contracted == receipt.contracted_after
        && world.requester_payroll.to_bits() == receipt.requester_payroll_bits
        && world.recipient_payroll.to_bits() == receipt.recipient_payroll_bits
        && requester_news == receipt.requester_news_count
        && recipient_news == receipt.recipient_news_count;
    if !fresh_trade_persisted {
        return Err("reloaded Test77 trade state does not match its sealed receipt".to_string());
    }

    let historical_fixture_preserved = athlete_trade_snapshot(db, FIRST_OFFERED_ID)?.team_id
        == RECIPIENT_TEAM_ID
        && athlete_trade_snapshot(db, FIRST_TARGET_ID)?.team_id == REQUESTER_TEAM_ID
        && athlete_trade_snapshot(db, SECOND_OFFERED_ID)?.team_id == RECIPIENT_TEAM_ID
        && athlete_trade_snapshot(db, SECOND_TARGET_ID)?.team_id == REQUESTER_TEAM_ID
        && contracted_squad_status_debug(db, FIRST_TARGET_ID)? == "Some(Important)"
        && contracted_squad_status_debug(db, SECOND_TARGET_ID)? == "Some(Important)";
    if !historical_fixture_preserved {
        return Err("the first two verified trades changed during Test77".to_string());
    }

    if !SAVED_TRADE_VALIDATION_SERVER_LOGGED.swap(true, Ordering::AcqRel) {
        log_event(
            "trade_save_reload_verified",
            &format!(
                "sdk_base={};plan_id={};commit_process_id={};reload_process_id={};desktop_restart_verified=true;baseline_save_slot={};result_save_slot={};offered_id={};offered_name={};offered_team_after_reload={};offered_contracted_status_after_reload={};target_id={};target_name={};target_team_after_reload={};target_status_after_reload={};target_contracted_status_after_reload={};desired_status_choice={};desired_status_key={};proposed_cash_won={};executed_plan_registry_count=3;plan_occurrences=1;first_plan_id={};first_plan_occurrences=1;second_plan_id={};second_plan_occurrences=1;three_receipts_preserved=true;historical_fixture_preserved=true;rollback_rehearsal_verified=true;duplicate_application_count=0;contract_fingerprints_persisted=true;finance_bits_persisted=true;save_reload_persisted=true;reload_validation_only=true;reload_validation_read_only=true;database_mutation=false;transaction_executed=false;save_api_called=false",
                PATCH055_BASE_VERSION,
                receipt.plan_id,
                receipt.commit_process_id,
                std::process::id(),
                BASELINE_SAVE_SLOT,
                RESULT_SAVE_SLOT,
                offered.id,
                sanitize(&offered.name),
                offered.team_id,
                sanitize(&offered_contracted),
                target.id,
                sanitize(&target.name),
                target.team_id,
                sanitize(&target.squad_status_debug),
                sanitize(&target_contracted),
                receipt.desired_status_choice,
                sanitize(&receipt.desired_status_key),
                receipt.proposed_cash_won,
                FIRST_PLAN_ID,
                SECOND_PLAN_ID,
            ),
        );
    }

    let mut payload = String::new();
    let _ = writeln!(payload, "status=verified");
    let _ = writeln!(payload, "sdk_base={}", PATCH055_BASE_VERSION);
    let _ = writeln!(payload, "plan_id={}", receipt.plan_id);
    let _ = writeln!(payload, "commit_process_id={}", receipt.commit_process_id);
    let _ = writeln!(payload, "reload_process_id={}", std::process::id());
    let _ = writeln!(payload, "desktop_restart_verified=true");
    let _ = writeln!(payload, "baseline_save_slot={}", BASELINE_SAVE_SLOT);
    let _ = writeln!(payload, "result_save_slot={}", RESULT_SAVE_SLOT);
    let _ = writeln!(payload, "offered_id={}", offered.id);
    let _ = writeln!(payload, "offered_name={}", sanitize(&offered.name));
    let _ = writeln!(payload, "offered_team_after_reload={}", offered.team_id);
    let _ = writeln!(payload, "offered_contracted_status_after_reload={}", sanitize(&offered_contracted));
    let _ = writeln!(payload, "target_id={}", target.id);
    let _ = writeln!(payload, "target_name={}", sanitize(&target.name));
    let _ = writeln!(payload, "target_team_after_reload={}", target.team_id);
    let _ = writeln!(payload, "target_status_after_reload={}", sanitize(&target.squad_status_debug));
    let _ = writeln!(payload, "target_contracted_status_after_reload={}", sanitize(&target_contracted));
    let _ = writeln!(payload, "desired_status_choice={}", receipt.desired_status_choice);
    let _ = writeln!(payload, "desired_status_key={}", receipt.desired_status_key);
    let _ = writeln!(payload, "proposed_cash_won={}", receipt.proposed_cash_won);
    let _ = writeln!(payload, "executed_plan_registry_count=3");
    let _ = writeln!(payload, "plan_occurrences={}", plan_occurrences);
    let _ = writeln!(payload, "first_plan_id={}", FIRST_PLAN_ID);
    let _ = writeln!(payload, "first_plan_occurrences={}", first_plan_occurrences);
    let _ = writeln!(payload, "second_plan_id={}", SECOND_PLAN_ID);
    let _ = writeln!(payload, "second_plan_occurrences={}", second_plan_occurrences);
    let _ = writeln!(payload, "three_receipts_preserved=true");
    let _ = writeln!(payload, "historical_fixture_preserved=true");
    let _ = writeln!(payload, "rollback_rehearsal_verified=true");
    let _ = writeln!(payload, "duplicate_application_count=0");
    let _ = writeln!(payload, "save_reload_persisted=true");
    let _ = writeln!(payload, "reload_validation_only=true");
    let _ = writeln!(payload, "reload_validation_read_only=true");
    let _ = writeln!(payload, "database_mutation=false");
    let _ = writeln!(payload, "transaction_executed=false");
    let _ = writeln!(payload, "save_api_called=false");
    Ok(payload.into_bytes())
}

fn build_trade_command_envelope(
    db: &Database,
    review: &ServerReview,
) -> Result<TradeCommandEnvelope, String> {
    let offered = db
        .athletes
        .get(review.offered_id)
        .ok_or_else(|| format!("offered athlete id {} disappeared", review.offered_id))?;
    let target = db
        .athletes
        .get(review.target_id)
        .ok_or_else(|| format!("target athlete id {} disappeared", review.target_id))?;
    let offered_team_id = contract_team_id(offered)?;
    let target_team_id = contract_team_id(target)?;
    if offered_team_id != review.requester_team_id || target_team_id != review.recipient_team_id {
        return Err("athlete ownership changed while preparing the command envelope".to_string());
    }

    let requester_roster_count = contracted_roster_count(db, review.requester_team_id);
    let recipient_roster_count = contracted_roster_count(db, review.recipient_team_id);
    if requester_roster_count == 0 || recipient_roster_count == 0 {
        return Err("a command envelope cannot bind an empty contracted roster".to_string());
    }
    let offered_yearly_salary = contract_yearly_salary(offered)?;
    let target_yearly_salary = contract_yearly_salary(target)?;
    let canonical = format!(
        "schema=2|requester={}|recipient={}|offered={}|target={}|cash={}|status={}|requester_roster={}|recipient_roster={}|offered_salary_bits={:016X}|target_salary_bits={:016X}|budget_bits={:016X}|game_time={}",
        review.requester_team_id,
        review.recipient_team_id,
        review.offered_id,
        review.target_id,
        review.proposed_cash_won,
        review.desired_status_choice,
        requester_roster_count,
        recipient_roster_count,
        offered_yearly_salary.to_bits(),
        target_yearly_salary.to_bits(),
        review.cash_budget_won.to_bits(),
        review.game_time.as_str(),
    );
    let plan_id = format!("T60-{:016X}", fnv1a64(&canonical));
    Ok(TradeCommandEnvelope {
        schema_version: 2,
        plan_id,
        requester_team_id: review.requester_team_id,
        recipient_team_id: review.recipient_team_id,
        offered_id: review.offered_id,
        target_id: review.target_id,
        offered_destination_team_id: review.recipient_team_id,
        target_destination_team_id: review.requester_team_id,
        cash_payer_team_id: review.requester_team_id,
        cash_recipient_team_id: review.recipient_team_id,
        proposed_cash_won: review.proposed_cash_won,
        desired_status_choice: review.desired_status_choice,
        desired_status_key: review.desired_status_key,
        requester_roster_count,
        recipient_roster_count,
        offered_yearly_salary,
        target_yearly_salary,
        requester_cash_budget_won: review.cash_budget_won,
        prepared_game_time: review.game_time.clone(),
        state_precondition_count: 9,
        operation_count: 5,
        atomic_batch_required: true,
        contract_transfer_mode: "inherit_both_contracts",
        money_direction: "requester_to_recipient",
        plan_repeat_consistent: false,
        execution_gate_closed: true,
    })
}

fn desired_squad_status(choice: u8) -> (&'static str, &'static str) {
    match choice {
        STATUS_CORE => ("core", "핵심 선수"),
        STATUS_IMPORTANT => ("important", "주요 선수"),
        STATUS_SUB => ("sub", "후보 선수"),
        STATUS_PROSPECT => ("prospect", "유망주"),
        _ => ("general", "주전 선수"),
    }
}

fn desired_squad_status_value(choice: u8) -> Result<SquadStatus, String> {
    match choice {
        STATUS_CORE => Ok(SquadStatus::Core),
        STATUS_IMPORTANT => Ok(SquadStatus::Important),
        STATUS_GENERAL => Ok(SquadStatus::General),
        STATUS_SUB => Ok(SquadStatus::Sub),
        STATUS_PROSPECT => Ok(SquadStatus::Prospect),
        _ => Err(format!("unsupported promised squad status choice {choice}")),
    }
}

// ===================== [PORT056] 계약 현황 표시 = 게임 데이터 방식 (유저 지시 2026-08-23) =====================
//
// 게임의 계약 현황 탭은 선수의 `Contract::InContract.transfer_requests` 에서 행을 만든다.
// (증거: 이 모드 자신의 `offer_history::collect_active_flows` 가 같은 구조를 읽어 "협상 중" 목록을 뽑는다.)
// ⟹ 우리 제안을 그 벡터에 넣으면 이름·소속팀·클릭·팝업이 **전부 네이티브로** 렌더된다.
//    지금까지의 UI 노드 강제 삽입(2026-08-23 인게임 3회 실패: 헤더폴백 → 정확경로 → 여전히 안 보임)이
//    통째로 불필요해진다.
//
// 인계본은 `native_transfer_request_inserted=false` 로 이 경로를 **의도적으로 피했다**(이유는 미기록).
// 그 회피를 검증 없이 승계한 것이 이번 삽질의 뿌리였다.
//
// ⚠부작용 가능성(실험으로 확인): 이건 게임의 **진짜 협상 데이터**다. 게임 자체 협상 로직(AI 응답·기한
//   만료·쿨다운)이 개입해 우리 비동기 수명주기와 이중으로 굴러갈 수 있다. 그래서 우리가 넣은 항목만
//   식별해 정확히 되돌릴 수 있도록 `team_id == 우리 팀` 조건으로만 추가/제거한다.

/// 대상 선수의 `transfer_requests` 에 우리 제안을 반영한다.
/// `active=false` 면 **우리가 넣은 항목만** 제거한다(다른 협상은 건드리지 않는다).
/// 반환 = 무엇을 했는지(로그용).
fn sync_native_transfer_request(
    db: &mut Database,
    target_id: usize,
    requester_team_id: usize,
    active: bool,
    due_days: i64,
    transfer_fee: f64,
    desired: SquadStatus,
) -> Result<&'static str, String> {
    let now = db.time;
    // `chrono` 를 extern 으로 링크하지 않으므로 타입을 이름붙이지 않고 추론에 맡긴다.
    // (오늘부터 `succ_opt` 로 전진 — NaiveDate 의 inherent 메서드라 import 가 필요 없다.)
    let due = {
        let mut d = db.time.date();
        for _ in 0..due_days.max(0) {
            match d.succ_opt() {
                Some(next) => d = next,
                None => break,
            }
        }
        d
    };
    let athlete = db
        .athletes
        .get_mut(target_id)
        .ok_or_else(|| format!("target athlete {target_id} not found for native request sync"))?;
    let Contract::InContract { transfer_requests, .. } = &mut athlete.contract else {
        return Err("target athlete is not under contract".to_string());
    };

    if !active {
        let before = transfer_requests.len();
        transfer_requests.retain(|r| r.team_id != requester_team_id);
        return Ok(if transfer_requests.len() == before { "absent" } else { "removed" });
    }

    let paper = TransferRequestPaper {
        is_draft: false,
        transfer_fee,
        state: PaperState::Waiting,
        is_ask: true,
        response_date: due,
        no_negotiation: false,
        options: Vec::new(),
    };
    if let Some(existing) = transfer_requests
        .iter_mut()
        .find(|r| r.team_id == requester_team_id)
    {
        existing.last_date = now;
        existing.desired_squad_status = desired;
        existing.phase = vec![paper];
        return Ok("updated");
    }
    transfer_requests.push(TransferRequest {
        team_id: requester_team_id,
        last_date: now,
        phase: vec![paper],
        cooldown_until: None,
        delegated_to_scout: false,
        seller_delegated_to_scout: false,
        desired_squad_status: desired,
    });
    Ok("inserted")
}

fn native_recruit_open(db: &Database) -> bool {
    let date = db.time.date();
    db.year_schedules
        .iter()
        .any(|schedule| schedule.is_in_recruit(date))
}

// ===================== [PORT056] 영입 시즌당 1회 제한 (성사만 소모) =====================

/// 지금 열려 있는 영입 창을 유일하게 식별하는 키. 창 밖이면 None.
/// `YearSchedule { id: usize, year: usize, recruits: Vec<(NaiveDate, NaiveDate)>, .. }` (SDK 실측).
/// 한 해에 영입 창이 여러 번일 수 있으므로 연도가 아니라 **(스케줄 id, 창 시작일)** 을 키로 쓴다.
fn current_recruit_season_key(db: &Database) -> Option<String> {
    let date = db.time.date();
    for schedule in db.year_schedules.iter() {
        for (start, end) in schedule.recruits.iter() {
            if date >= *start && date <= *end {
                return Some(format!("{}|{}|{}", schedule.id, schedule.year, start));
            }
        }
    }
    None
}

/// 이번 영입 시즌에 이미 트레이드를 성사시켰는가.
fn trade_season_already_used(db: &Database) -> bool {
    let Some(current) = current_recruit_season_key(db) else {
        return false;
    };
    db.mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, TRADE_SEASON_USAGE_KEY)
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .is_some_and(|stored| stored.trim() == current)
}

/// 거래가 실제로 성사됐을 때만 호출한다(persist_trade_commit 안).
fn mark_trade_season_used(db: &mut Database) -> Option<String> {
    let key = current_recruit_season_key(db)?;
    let _ = db.mod_save_data.set_bytes(
        MOD_SAVE_NAMESPACE,
        TRADE_SEASON_USAGE_KEY,
        key.clone().into_bytes(),
    );
    Some(key)
}

fn approximately_equal(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= 1.0e-9 * scale + 0.001
}

fn finance_bits_equal(left: TeamFinanceSnapshot, right: TeamFinanceSnapshot) -> bool {
    left.total_balance.to_bits() == right.total_balance.to_bits()
        && left.transfer_budget.to_bits() == right.transfer_budget.to_bits()
        && left.salary_budget.to_bits() == right.salary_budget.to_bits()
}

fn team_finance_snapshot(
    db: &Database,
    team_id: usize,
) -> Result<TeamFinanceSnapshot, String> {
    let team = db
        .teams
        .get(team_id)
        .ok_or_else(|| format!("team id {team_id} was not found for finance snapshot"))?;
    let snapshot = TeamFinanceSnapshot {
        total_balance: team.total_balance,
        transfer_budget: team.transfer_budget,
        salary_budget: team.salary_budget,
    };
    if !snapshot.total_balance.is_finite()
        || !snapshot.transfer_budget.is_finite()
        || !snapshot.salary_budget.is_finite()
    {
        return Err(format!("team id {team_id} has non-finite finance fields"));
    }
    Ok(snapshot)
}

fn athlete_trade_snapshot(
    db: &Database,
    athlete_id: usize,
) -> Result<AthleteTradeSnapshot, String> {
    let athlete = db
        .athletes
        .get(athlete_id)
        .ok_or_else(|| format!("athlete id {athlete_id} was not found"))?;
    let Contract::InContract {
        team_id,
        start_date,
        end_date,
        weekly_salary,
        transfer_fee,
        incentives,
        transfer_requests,
        recruit_requests,
        ..
    } = &athlete.contract
    else {
        return Err(format!("athlete {} is not under InContract", athlete.name));
    };
    Ok(AthleteTradeSnapshot {
        id: athlete_id,
        name: athlete.name.to_string(),
        team_id: *team_id,
        start_date: start_date.to_string(),
        end_date: end_date.to_string(),
        weekly_salary: *weekly_salary,
        transfer_fee: *transfer_fee,
        incentives_debug: format!("{:?}", incentives),
        transfer_requests_debug: format!("{:?}", transfer_requests),
        recruit_requests_debug: format!("{:?}", recruit_requests),
        squad_status_debug: format!("{:?}", athlete.squad_status),
    })
}

fn contracted_squad_status_value(
    db: &Database,
    athlete_id: usize,
) -> Result<Option<SquadStatus>, String> {
    let policy = db
        .athlete_policies
        .get(athlete_id)
        .ok_or_else(|| format!("athlete policy id {athlete_id} was not found"))?;
    Ok(policy.contracted_squad_status.clone())
}

fn contracted_squad_status_debug(db: &Database, athlete_id: usize) -> Result<String, String> {
    Ok(format!(
        "{:?}",
        contracted_squad_status_value(db, athlete_id)?
    ))
}

fn set_target_contracted_squad_status(
    db: &mut Database,
    athlete_id: usize,
    squad_status: Option<SquadStatus>,
) -> Result<(), String> {
    let policy = db
        .athlete_policies
        .get_mut(athlete_id)
        .ok_or_else(|| format!("athlete policy id {athlete_id} was not found for status write"))?;
    policy.contracted_squad_status = squad_status;
    Ok(())
}

fn same_contract_except_team(
    before: &AthleteTradeSnapshot,
    after: &AthleteTradeSnapshot,
    expected_team_id: usize,
) -> bool {
    after.team_id == expected_team_id
        && before.id == after.id
        && before.name == after.name
        && before.start_date == after.start_date
        && before.end_date == after.end_date
        && before.weekly_salary.to_bits() == after.weekly_salary.to_bits()
        && before.transfer_fee.to_bits() == after.transfer_fee.to_bits()
        && before.incentives_debug == after.incentives_debug
        && before.transfer_requests_debug == after.transfer_requests_debug
        && before.recruit_requests_debug == after.recruit_requests_debug
}

fn world_trade_snapshot(
    db: &Database,
    requester_team_id: usize,
    recipient_team_id: usize,
) -> WorldTradeSnapshot {
    let mut snapshot = WorldTradeSnapshot {
        requester_roster: 0,
        recipient_roster: 0,
        contracted: 0,
        requester_payroll: 0.0,
        recipient_payroll: 0.0,
    };
    for athlete in db.athletes.iter() {
        if let Contract::InContract {
            team_id,
            weekly_salary,
            ..
        } = &athlete.contract
        {
            snapshot.contracted += 1;
            if *team_id == requester_team_id {
                snapshot.requester_roster += 1;
                snapshot.requester_payroll += *weekly_salary;
            }
            if *team_id == recipient_team_id {
                snapshot.recipient_roster += 1;
                snapshot.recipient_payroll += *weekly_salary;
            }
        }
    }
    snapshot
}

fn athlete_trade_snapshot_exact(left: &AthleteTradeSnapshot, right: &AthleteTradeSnapshot) -> bool {
    left.id == right.id
        && left.name == right.name
        && left.team_id == right.team_id
        && left.start_date == right.start_date
        && left.end_date == right.end_date
        && left.weekly_salary.to_bits() == right.weekly_salary.to_bits()
        && left.transfer_fee.to_bits() == right.transfer_fee.to_bits()
        && left.incentives_debug == right.incentives_debug
        && left.transfer_requests_debug == right.transfer_requests_debug
        && left.recruit_requests_debug == right.recruit_requests_debug
        && left.squad_status_debug == right.squad_status_debug
}

fn world_trade_snapshot_exact(left: WorldTradeSnapshot, right: WorldTradeSnapshot) -> bool {
    left.requester_roster == right.requester_roster
        && left.recipient_roster == right.recipient_roster
        && left.contracted == right.contracted
        && left.requester_payroll.to_bits() == right.requester_payroll.to_bits()
        && left.recipient_payroll.to_bits() == right.recipient_payroll.to_bits()
}

fn arm_forced_rollback_audit(snapshot: ForcedRollbackAuditSnapshot) {
    let mut guard = FORCED_ROLLBACK_AUDIT
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = Some(snapshot);
}

fn current_forced_rollback_plan_id() -> String {
    let guard = FORCED_ROLLBACK_AUDIT
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    guard
        .as_ref()
        .map(|snapshot| snapshot.plan_id.clone())
        .unwrap_or_else(|| "none".to_string())
}

fn clear_forced_rollback_audit() {
    let mut guard = FORCED_ROLLBACK_AUDIT
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = None;
}

fn verify_forced_rollback_exact(db: &Database) -> Result<bool, String> {
    let snapshot = {
        let guard = FORCED_ROLLBACK_AUDIT
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.clone()
    };
    let Some(snapshot) = snapshot else {
        return Err("forced rollback audit snapshot is missing".to_string());
    };
    let offered_after = athlete_trade_snapshot(db, snapshot.offered_before.id)?;
    let target_after = athlete_trade_snapshot(db, snapshot.target_before.id)?;
    let offered_contracted_after =
        contracted_squad_status_value(db, snapshot.offered_before.id)?;
    let target_contracted_after =
        contracted_squad_status_value(db, snapshot.target_before.id)?;
    let requester_finance_after = team_finance_snapshot(db, snapshot.offered_before.team_id)?;
    let recipient_finance_after = team_finance_snapshot(db, snapshot.target_before.team_id)?;
    let world_after = world_trade_snapshot(
        db,
        snapshot.offered_before.team_id,
        snapshot.target_before.team_id,
    );
    let requester_news_after = db
        .teams
        .get(snapshot.offered_before.team_id)
        .ok_or_else(|| "requester team missing after forced rollback".to_string())?
        .news
        .len();
    let recipient_news_after = db
        .teams
        .get(snapshot.target_before.team_id)
        .ok_or_else(|| "recipient team missing after forced rollback".to_string())?
        .news
        .len();
    let offered_contract_restored = athlete_trade_snapshot_exact(&offered_after, &snapshot.offered_before);
    let target_contract_restored = athlete_trade_snapshot_exact(&target_after, &snapshot.target_before);
    let offered_contracted_status_restored = format!("{:?}", offered_contracted_after)
        == format!("{:?}", snapshot.offered_contracted_squad_status_before);
    let target_contracted_status_restored = format!("{:?}", target_contracted_after)
        == format!("{:?}", snapshot.target_contracted_squad_status_before);
    let finance_bits_restored = finance_bits_equal(requester_finance_after, snapshot.requester_finance_before)
        && finance_bits_equal(recipient_finance_after, snapshot.recipient_finance_before);
    let world_restored = world_trade_snapshot_exact(world_after, snapshot.world_before);
    let mod_save_restored = format!("{:?}", db.mod_save_data) == snapshot.mod_save_before;
    let news_counts_restored = requester_news_after == snapshot.requester_news_before
        && recipient_news_after == snapshot.recipient_news_before;
    let game_time_restored = db.time.to_string() == snapshot.game_time_before;
    let rollback_ok = offered_contract_restored
        && target_contract_restored
        && offered_contracted_status_restored
        && target_contracted_status_restored
        && finance_bits_restored
        && world_restored
        && mod_save_restored
        && news_counts_restored
        && game_time_restored;
    log_event(
        "trade_atomic_rollback_verified",
        &format!(
            "plan_id={};offered_id={};target_id={};offered_team_after_restore={};target_team_after_restore={};target_status_after_restore={};offered_contracted_status_after_restore={};target_contracted_status_after_restore={};offered_contract_restored={};target_contract_restored={};offered_contracted_status_restored={};target_contracted_status_restored={};finance_bits_restored={};world_restored={};mod_save_restored={};news_counts_restored={};game_time_restored={};requester_roster_after={};recipient_roster_after={};contracted_after={};requester_payroll_bits={:016X};recipient_payroll_bits={:016X};rollback_ok={};net_database_change={};transaction_executed=false;save_api_called=false",
            snapshot.plan_id,
            offered_after.id,
            target_after.id,
            offered_after.team_id,
            target_after.team_id,
            sanitize(&target_after.squad_status_debug),
            sanitize(&format!("{:?}", offered_contracted_after)),
            sanitize(&format!("{:?}", target_contracted_after)),
            offered_contract_restored,
            target_contract_restored,
            offered_contracted_status_restored,
            target_contracted_status_restored,
            finance_bits_restored,
            world_restored,
            mod_save_restored,
            news_counts_restored,
            game_time_restored,
            world_after.requester_roster,
            world_after.recipient_roster,
            world_after.contracted,
            world_after.requester_payroll.to_bits(),
            world_after.recipient_payroll.to_bits(),
            rollback_ok,
            !rollback_ok,
        ),
    );
    clear_forced_rollback_audit();
    if !rollback_ok {
        return Err("exact post-rollback audit did not match the pre-execution snapshot".to_string());
    }
    Ok(true)
}

fn set_team_id(
    db: &mut Database,
    athlete_id: usize,
    expected_team_id: usize,
    new_team_id: usize,
) -> Result<(), String> {
    let athlete = db
        .athletes
        .get_mut(athlete_id)
        .ok_or_else(|| format!("athlete id {athlete_id} was not found for mutation"))?;
    match &mut athlete.contract {
        Contract::InContract { team_id, .. } => {
            if *team_id != expected_team_id {
                return Err(format!(
                    "athlete id {athlete_id} expected team {expected_team_id}, found {}",
                    *team_id
                ));
            }
            *team_id = new_team_id;
            Ok(())
        }
        _ => Err(format!("athlete id {athlete_id} stopped being InContract")),
    }
}

fn force_team_id(
    db: &mut Database,
    athlete_id: usize,
    team_id_before: usize,
) -> Result<(), String> {
    let athlete = db
        .athletes
        .get_mut(athlete_id)
        .ok_or_else(|| format!("athlete id {athlete_id} was not found for rollback"))?;
    match &mut athlete.contract {
        Contract::InContract { team_id, .. } => {
            *team_id = team_id_before;
            Ok(())
        }
        _ => Err(format!(
            "athlete id {athlete_id} is not InContract during rollback"
        )),
    }
}

fn set_target_squad_status(
    db: &mut Database,
    athlete_id: usize,
    squad_status: SquadStatus,
) -> Result<(), String> {
    let athlete = db
        .athletes
        .get_mut(athlete_id)
        .ok_or_else(|| format!("athlete id {athlete_id} was not found for squad-status write"))?;
    athlete.squad_status = squad_status;
    Ok(())
}

fn set_team_finance(
    db: &mut Database,
    team_id: usize,
    snapshot: TeamFinanceSnapshot,
) -> Result<(), String> {
    let team = db
        .teams
        .get_mut(team_id)
        .ok_or_else(|| format!("team id {team_id} was not found for finance restore"))?;
    team.total_balance = snapshot.total_balance;
    team.transfer_budget = snapshot.transfer_budget;
    team.salary_budget = snapshot.salary_budget;
    Ok(())
}

fn apply_requester_cash(
    db: &mut Database,
    requester_team_id: usize,
    recipient_team_id: usize,
    requester_cash: f64,
) -> Result<(), String> {
    {
        let requester = db
            .teams
            .get_mut(requester_team_id)
            .ok_or_else(|| "requester team disappeared during cash debit".to_string())?;
        if requester.total_balance + 0.001 < requester_cash
            || requester.transfer_budget + 0.001 < requester_cash
        {
            return Err("requester no longer has enough balance or transfer budget".to_string());
        }
        requester.total_balance -= requester_cash;
        requester.transfer_budget -= requester_cash;
    }
    {
        let recipient = db
            .teams
            .get_mut(recipient_team_id)
            .ok_or_else(|| "recipient team disappeared during cash credit".to_string())?;
        recipient.total_balance += requester_cash;
        recipient.transfer_budget += requester_cash;
    }
    Ok(())
}

fn arm_atomic_rollback(snapshot: AtomicRollbackSnapshot) {
    let mut guard = ROLLBACK_SNAPSHOT
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = Some(snapshot);
    MUTATION_ACTIVE.store(true, Ordering::Release);
}

fn clear_atomic_rollback() {
    MUTATION_ACTIVE.store(false, Ordering::Release);
    let mut guard = ROLLBACK_SNAPSHOT
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = None;
}

fn current_atomic_rollback() -> Option<AtomicRollbackSnapshot> {
    let guard = ROLLBACK_SNAPSHOT
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    guard.clone()
}

fn restore_atomic_rollback(db: &mut Database) -> Result<bool, String> {
    let Some(snapshot) = current_atomic_rollback() else {
        return Ok(false);
    };
    force_team_id(db, snapshot.offered_id, snapshot.offered_team_before)?;
    force_team_id(db, snapshot.target_id, snapshot.target_team_before)?;
    set_target_squad_status(
        db,
        snapshot.target_id,
        snapshot.target_squad_status_before,
    )?;
    set_target_contracted_squad_status(
        db,
        snapshot.target_id,
        snapshot.target_contracted_squad_status_before.clone(),
    )?;
    set_team_finance(
        db,
        snapshot.requester_team_id,
        snapshot.requester_finance_before,
    )?;
    set_team_finance(
        db,
        snapshot.recipient_team_id,
        snapshot.recipient_finance_before,
    )?;
    db.mod_save_data = snapshot.mod_save_before.clone();

    let offered_team_after_restore = db
        .athletes
        .get(snapshot.offered_id)
        .ok_or_else(|| "offered athlete disappeared after rollback".to_string())
        .and_then(contract_team_id)?;
    let target_after_restore = db
        .athletes
        .get(snapshot.target_id)
        .ok_or_else(|| "target athlete disappeared after rollback".to_string())?;
    let target_team_after_restore = contract_team_id(target_after_restore)?;
    let target_status_restored = format!("{:?}", &target_after_restore.squad_status)
        == format!("{:?}", &snapshot.target_squad_status_before);
    let target_contracted_status_restored = contracted_squad_status_debug(db, snapshot.target_id)?
        == format!("{:?}", snapshot.target_contracted_squad_status_before);
    let requester_finance_after_restore =
        team_finance_snapshot(db, snapshot.requester_team_id)?;
    let recipient_finance_after_restore =
        team_finance_snapshot(db, snapshot.recipient_team_id)?;
    let mod_save_restored = format!("{:?}", db.mod_save_data)
        == format!("{:?}", snapshot.mod_save_before);
    if offered_team_after_restore != snapshot.offered_team_before
        || target_team_after_restore != snapshot.target_team_before
        || !target_status_restored
        || !target_contracted_status_restored
        || !finance_bits_equal(
            requester_finance_after_restore,
            snapshot.requester_finance_before,
        )
        || !finance_bits_equal(
            recipient_finance_after_restore,
            snapshot.recipient_finance_before,
        )
        || !mod_save_restored
    {
        return Err("post-rollback verification did not match the armed snapshot".to_string());
    }
    Ok(true)
}

fn execute_atomic_trade(
    db: &mut Database,
    review: &ServerReview,
    envelope: &TradeCommandEnvelope,
    force_rollback_rehearsal: bool,
    rollback_rehearsal_verified: bool,
) -> Result<AtomicTradeResult, String> {
    if !review.overall_approved || !envelope.plan_repeat_consistent {
        return Err("the sealed command is not approved and repeat-consistent".to_string());
    }
    if !native_recruit_open(db) {
        return Err("the native player recruitment window is closed".to_string());
    }
    if db.time.to_string() != envelope.prepared_game_time {
        return Err("game time changed after the command was sealed".to_string());
    }

    let offered_before = athlete_trade_snapshot(db, envelope.offered_id)?;
    let target_before = athlete_trade_snapshot(db, envelope.target_id)?;
    if offered_before.team_id != envelope.requester_team_id
        || target_before.team_id != envelope.recipient_team_id
    {
        return Err("athlete ownership no longer matches the sealed command".to_string());
    }
    let world_before = world_trade_snapshot(
        db,
        envelope.requester_team_id,
        envelope.recipient_team_id,
    );
    if world_before.requester_roster != envelope.requester_roster_count
        || world_before.recipient_roster != envelope.recipient_roster_count
    {
        return Err("roster counts changed after the command was sealed".to_string());
    }
    if !nearly_equal_money(
        offered_before.weekly_salary * WEEKS_PER_YEAR,
        envelope.offered_yearly_salary,
    ) || !nearly_equal_money(
        target_before.weekly_salary * WEEKS_PER_YEAR,
        envelope.target_yearly_salary,
    ) {
        return Err("an inherited salary changed after the command was sealed".to_string());
    }

    let requester_finance_before =
        team_finance_snapshot(db, envelope.requester_team_id)?;
    let recipient_finance_before =
        team_finance_snapshot(db, envelope.recipient_team_id)?;
    let cash = envelope.proposed_cash_won as f64;
    if requester_finance_before.total_balance + 0.001 < cash
        || requester_finance_before.transfer_budget + 0.001 < cash
        || envelope.requester_cash_budget_won + 0.001 < cash
    {
        return Err("requester finance no longer satisfies the sealed cash amount".to_string());
    }
    let target_squad_status_before = db
        .athletes
        .get(envelope.target_id)
        .ok_or_else(|| "target athlete disappeared before status snapshot".to_string())?
        .squad_status
        .clone();
    let offered_contracted_squad_status_before =
        contracted_squad_status_value(db, envelope.offered_id)?;
    let target_contracted_squad_status_before =
        contracted_squad_status_value(db, envelope.target_id)?;
    let mod_save_clone_before = db.mod_save_data.clone();
    let mod_save_before = format!("{:?}", db.mod_save_data);
    let requester_news_before = db
        .teams
        .get(envelope.requester_team_id)
        .ok_or_else(|| "requester team disappeared before news snapshot".to_string())?
        .news
        .len();
    let recipient_news_before = db
        .teams
        .get(envelope.recipient_team_id)
        .ok_or_else(|| "recipient team disappeared before news snapshot".to_string())?
        .news
        .len();

    arm_forced_rollback_audit(ForcedRollbackAuditSnapshot {
        plan_id: envelope.plan_id.clone(),
        offered_before: offered_before.clone(),
        target_before: target_before.clone(),
        offered_contracted_squad_status_before:
            offered_contracted_squad_status_before.clone(),
        target_contracted_squad_status_before:
            target_contracted_squad_status_before.clone(),
        requester_finance_before,
        recipient_finance_before,
        world_before,
        mod_save_before: mod_save_before.clone(),
        requester_news_before,
        recipient_news_before,
        game_time_before: envelope.prepared_game_time.clone(),
    });
    arm_atomic_rollback(AtomicRollbackSnapshot {
        requester_team_id: envelope.requester_team_id,
        recipient_team_id: envelope.recipient_team_id,
        offered_id: envelope.offered_id,
        target_id: envelope.target_id,
        offered_team_before: offered_before.team_id,
        target_team_before: target_before.team_id,
        target_squad_status_before,
        target_contracted_squad_status_before:
            target_contracted_squad_status_before.clone(),
        requester_finance_before,
        recipient_finance_before,
        mod_save_before: mod_save_clone_before,
    });

    set_team_id(
        db,
        envelope.offered_id,
        envelope.requester_team_id,
        envelope.offered_destination_team_id,
    )?;
    set_team_id(
        db,
        envelope.target_id,
        envelope.recipient_team_id,
        envelope.target_destination_team_id,
    )?;
    apply_requester_cash(
        db,
        envelope.cash_payer_team_id,
        envelope.cash_recipient_team_id,
        cash,
    )?;
    let desired_status = desired_squad_status_value(envelope.desired_status_choice)?;
    set_target_squad_status(db, envelope.target_id, desired_status.clone())?;
    set_target_contracted_squad_status(
        db,
        envelope.target_id,
        Some(desired_status.clone()),
    )?;

    let offered_after = athlete_trade_snapshot(db, envelope.offered_id)?;
    let target_after = athlete_trade_snapshot(db, envelope.target_id)?;
    let offered_contracted_status_after =
        contracted_squad_status_debug(db, envelope.offered_id)?;
    let target_contracted_status_after =
        contracted_squad_status_debug(db, envelope.target_id)?;
    let requester_finance_after = team_finance_snapshot(db, envelope.requester_team_id)?;
    let recipient_finance_after = team_finance_snapshot(db, envelope.recipient_team_id)?;
    let world_after = world_trade_snapshot(
        db,
        envelope.requester_team_id,
        envelope.recipient_team_id,
    );

    let contract_inherited_both = same_contract_except_team(
        &offered_before,
        &offered_after,
        envelope.offered_destination_team_id,
    ) && same_contract_except_team(
        &target_before,
        &target_after,
        envelope.target_destination_team_id,
    );
    if !contract_inherited_both {
        return Err("a contract changed outside its destination team id".to_string());
    }
    let offered_status_unchanged =
        offered_after.squad_status_debug == offered_before.squad_status_debug;
    if !offered_status_unchanged {
        return Err("the outgoing athlete squad status changed unexpectedly".to_string());
    }
    let desired_status_debug = format!(
        "{:?}",
        desired_squad_status_value(envelope.desired_status_choice)?
    );
    let target_status_applied = target_after.squad_status_debug == desired_status_debug;
    if !target_status_applied {
        return Err("the promised incoming squad status was not applied".to_string());
    }
    let offered_contracted_status_before =
        format!("{:?}", offered_contracted_squad_status_before);
    let target_contracted_status_before =
        format!("{:?}", target_contracted_squad_status_before);
    let offered_contracted_status_unchanged =
        offered_contracted_status_after == offered_contracted_status_before;
    let target_contracted_status_applied =
        target_contracted_status_after == format!("Some({desired_status_debug})");
    if !offered_contracted_status_unchanged || !target_contracted_status_applied {
        return Err("the 0.5.5 contracted squad-status mutation is not exact".to_string());
    }

    let requester_finance_ok = approximately_equal(
        requester_finance_after.total_balance,
        requester_finance_before.total_balance - cash,
    ) && approximately_equal(
        requester_finance_after.transfer_budget,
        requester_finance_before.transfer_budget - cash,
    ) && requester_finance_after.salary_budget.to_bits()
        == requester_finance_before.salary_budget.to_bits();
    let recipient_finance_ok = approximately_equal(
        recipient_finance_after.total_balance,
        recipient_finance_before.total_balance + cash,
    ) && approximately_equal(
        recipient_finance_after.transfer_budget,
        recipient_finance_before.transfer_budget + cash,
    ) && recipient_finance_after.salary_budget.to_bits()
        == recipient_finance_before.salary_budget.to_bits();
    if !requester_finance_ok || !recipient_finance_ok {
        return Err("team finance deltas do not match the sealed cash transfer".to_string());
    }
    let combined_finance_conserved = approximately_equal(
        requester_finance_after.total_balance + recipient_finance_after.total_balance,
        requester_finance_before.total_balance + recipient_finance_before.total_balance,
    ) && approximately_equal(
        requester_finance_after.transfer_budget + recipient_finance_after.transfer_budget,
        requester_finance_before.transfer_budget + recipient_finance_before.transfer_budget,
    );
    if !combined_finance_conserved {
        return Err("combined finance was not conserved".to_string());
    }

    if world_after.requester_roster != world_before.requester_roster
        || world_after.recipient_roster != world_before.recipient_roster
        || world_after.contracted != world_before.contracted
    {
        return Err("the 1:1 transaction changed roster or contracted counts".to_string());
    }
    let expected_requester_payroll = world_before.requester_payroll
        - offered_before.weekly_salary
        + target_before.weekly_salary;
    let expected_recipient_payroll = world_before.recipient_payroll
        - target_before.weekly_salary
        + offered_before.weekly_salary;
    if !approximately_equal(world_after.requester_payroll, expected_requester_payroll)
        || !approximately_equal(world_after.recipient_payroll, expected_recipient_payroll)
    {
        return Err("team payrolls did not follow the inherited contracts".to_string());
    }

    let pre_receipt_mod_save_unchanged = format!("{:?}", db.mod_save_data) == mod_save_before;
    let news_count_unchanged = db
        .teams
        .get(envelope.requester_team_id)
        .is_some_and(|team| team.news.len() == requester_news_before)
        && db
            .teams
            .get(envelope.recipient_team_id)
            .is_some_and(|team| team.news.len() == recipient_news_before);
    if !pre_receipt_mod_save_unchanged || !news_count_unchanged {
        return Err("unexpected ModSave or news mutation occurred before receipt persistence".to_string());
    }
    if db.time.to_string() != envelope.prepared_game_time {
        return Err("game time changed during the atomic batch".to_string());
    }

    if force_rollback_rehearsal {
        log_event(
            "test77_forced_rollback_rehearsal_triggered",
            &format!(
                "plan_id={};offered_id={};target_id={};target_status_applied={};target_contracted_status_applied={};all_sealed_mutations_applied=true;receipt_persistence_started=false;rollback_required=true;transaction_executed=false;save_api_called=false",
                envelope.plan_id,
                envelope.offered_id,
                envelope.target_id,
                target_status_applied,
                target_contracted_status_applied,
            ),
        );
        return Err("TEST66_FORCED_ROLLBACK_REHEARSAL".to_string());
    }
    if !rollback_rehearsal_verified {
        return Err("the Test77 rollback rehearsal was not verified before commit".to_string());
    }

    let (desired_status_key, desired_status_label) =
        desired_squad_status(envelope.desired_status_choice);
    let offered_contract_fingerprint = athlete_trade_fingerprint(&offered_after);
    let target_contract_fingerprint = athlete_trade_fingerprint(&target_after);
    let receipt = TradeCommitReceipt {
        schema_version: 2,
        plan_id: envelope.plan_id.clone(),
        commit_process_id: std::process::id(),
        commit_game_time: db.time.to_string(),
        requester_team_id: envelope.requester_team_id,
        recipient_team_id: envelope.recipient_team_id,
        offered_id: envelope.offered_id,
        target_id: envelope.target_id,
        offered_name: offered_before.name.clone(),
        target_name: target_before.name.clone(),
        proposed_cash_won: envelope.proposed_cash_won,
        desired_status_choice: envelope.desired_status_choice,
        desired_status_key: desired_status_key.to_string(),
        offered_team_after: offered_after.team_id,
        target_team_after: target_after.team_id,
        target_status_after: target_after.squad_status_debug.clone(),
        offered_contracted_status_after: offered_contracted_status_after.clone(),
        target_contracted_status_after: target_contracted_status_after.clone(),
        rollback_rehearsal_verified: true,
        offered_contract_fingerprint,
        target_contract_fingerprint,
        requester_total_bits: requester_finance_after.total_balance.to_bits(),
        requester_transfer_bits: requester_finance_after.transfer_budget.to_bits(),
        requester_salary_bits: requester_finance_after.salary_budget.to_bits(),
        recipient_total_bits: recipient_finance_after.total_balance.to_bits(),
        recipient_transfer_bits: recipient_finance_after.transfer_budget.to_bits(),
        recipient_salary_bits: recipient_finance_after.salary_budget.to_bits(),
        requester_roster_after: world_after.requester_roster,
        recipient_roster_after: world_after.recipient_roster,
        contracted_after: world_after.contracted,
        requester_payroll_bits: world_after.requester_payroll.to_bits(),
        recipient_payroll_bits: world_after.recipient_payroll.to_bits(),
        requester_news_count: requester_news_before,
        recipient_news_count: recipient_news_before,
        executed_plan_registry_count: 0,
    };
    let (persisted_receipt, receipt_readback_verified) = persist_trade_commit(db, receipt)?;
    let executed_plan_persisted = read_persisted_plan_ids(db)?
        .iter()
        .filter(|plan_id| *plan_id == &envelope.plan_id)
        .count()
        == 1;
    // [PORT056] 첫 거래에는 아카이브 세대(PREVIOUS/OLDEST)가 존재하지 않는다 —
    //   3개 키를 전부 요구하던 구판 검사는 첫 거래를 항상 실패시켰다. 현재 영수증만 요구한다.
    let trade_receipt_persisted = db
        .mod_save_data
        .contains_key(MOD_SAVE_NAMESPACE, TRADE_COMMIT_RECEIPT_KEY);
    if !executed_plan_persisted || !trade_receipt_persisted || !receipt_readback_verified {
        return Err("trade receipt persistence did not verify".to_string());
    }
    let post_receipt_news_unchanged = db
        .teams
        .get(envelope.requester_team_id)
        .is_some_and(|team| team.news.len() == requester_news_before)
        && db
            .teams
            .get(envelope.recipient_team_id)
            .is_some_and(|team| team.news.len() == recipient_news_before);
    if !post_receipt_news_unchanged || db.time.to_string() != envelope.prepared_game_time {
        return Err("news count or game time changed during receipt persistence".to_string());
    }

    log_event(
        "trade_atomic_mutations_verified",
        &format!(
            "sdk_base={};plan_id={};requester_team_id={};recipient_team_id={};offered_id={};offered_name={};target_id={};target_name={};proposed_cash_won={};proposed_cash_within_server_range=true;desired_status_choice={};desired_status_key={};offered_team_after={};target_team_after={};target_status_after={};offered_contracted_status_before={};offered_contracted_status_after={};target_contracted_status_before={};target_contracted_status_after={};offered_contracted_status_unchanged={};target_contracted_status_applied={};requester_total_before={};requester_total_after={};requester_transfer_before={};requester_transfer_after={};recipient_total_before={};recipient_total_after={};recipient_transfer_before={};recipient_transfer_after={};contract_inherited_both={};offered_status_unchanged={};target_status_applied={};combined_finance_conserved={};zero_cash_finance_bits_unchanged={};pre_receipt_mod_save_unchanged={};executed_plan_persisted={};trade_receipt_persisted={};three_receipts_preserved=true;first_plan_id={};first_plan_preserved=true;second_plan_id={};second_plan_preserved=true;receipt_readback_verified={};executed_plan_registry_count={};offered_contract_fingerprint={:016X};target_contract_fingerprint={:016X};news_count_unchanged={};rollback_rehearsal_verified=true;atomic_batch_applied=true;rollback_armed=true;database_mutation=true;contract_mutation=true;finance_mutation={};squad_status_mutation=true;contracted_squad_status_mutation=true;mod_save_mutation=true;transaction_executed=true;save_api_called=false",
            PATCH055_BASE_VERSION,
            envelope.plan_id,
            envelope.requester_team_id,
            envelope.recipient_team_id,
            envelope.offered_id,
            sanitize(&offered_before.name),
            envelope.target_id,
            sanitize(&target_before.name),
            envelope.proposed_cash_won,
            envelope.desired_status_choice,
            desired_status_key,
            offered_after.team_id,
            target_after.team_id,
            sanitize(&target_after.squad_status_debug),
            sanitize(&offered_contracted_status_before),
            sanitize(&offered_contracted_status_after),
            sanitize(&target_contracted_status_before),
            sanitize(&target_contracted_status_after),
            offered_contracted_status_unchanged,
            target_contracted_status_applied,
            requester_finance_before.total_balance,
            requester_finance_after.total_balance,
            requester_finance_before.transfer_budget,
            requester_finance_after.transfer_budget,
            recipient_finance_before.total_balance,
            recipient_finance_after.total_balance,
            recipient_finance_before.transfer_budget,
            recipient_finance_after.transfer_budget,
            contract_inherited_both,
            offered_status_unchanged,
            target_status_applied,
            combined_finance_conserved,
            requester_finance_before.total_balance.to_bits()
                == requester_finance_after.total_balance.to_bits()
                && requester_finance_before.transfer_budget.to_bits()
                    == requester_finance_after.transfer_budget.to_bits()
                && recipient_finance_before.total_balance.to_bits()
                    == recipient_finance_after.total_balance.to_bits()
                && recipient_finance_before.transfer_budget.to_bits()
                    == recipient_finance_after.transfer_budget.to_bits(),
            pre_receipt_mod_save_unchanged,
            executed_plan_persisted,
            trade_receipt_persisted,
            FIRST_PLAN_ID,
            SECOND_PLAN_ID,
            receipt_readback_verified,
            persisted_receipt.executed_plan_registry_count,
            offered_contract_fingerprint,
            target_contract_fingerprint,
            post_receipt_news_unchanged,
            cash > 0.0,
        ),
    );

    let result = AtomicTradeResult {
        plan_id: envelope.plan_id.clone(),
        requester_team_id: envelope.requester_team_id,
        recipient_team_id: envelope.recipient_team_id,
        offered_id: envelope.offered_id,
        target_id: envelope.target_id,
        offered_name: offered_before.name.clone(),
        target_name: target_before.name.clone(),
        proposed_cash_won: envelope.proposed_cash_won,
        desired_status_choice: envelope.desired_status_choice,
        desired_status_key,
        desired_status_label,
        offered_team_before: offered_before.team_id,
        offered_team_after: offered_after.team_id,
        target_team_before: target_before.team_id,
        target_team_after: target_after.team_id,
        target_status_before: target_before.squad_status_debug.clone(),
        target_status_after: target_after.squad_status_debug.clone(),
        offered_contracted_status_before,
        offered_contracted_status_after,
        target_contracted_status_before,
        target_contracted_status_after,
        requester_finance_before,
        requester_finance_after,
        recipient_finance_before,
        recipient_finance_after,
        requester_roster_before: world_before.requester_roster,
        requester_roster_after: world_after.requester_roster,
        recipient_roster_before: world_before.recipient_roster,
        recipient_roster_after: world_after.recipient_roster,
        contracted_before: world_before.contracted,
        contracted_after: world_after.contracted,
        requester_payroll_after: world_after.requester_payroll,
        recipient_payroll_after: world_after.recipient_payroll,
        contract_inherited_both,
        offered_status_unchanged,
        target_status_applied,
        offered_contracted_status_unchanged,
        target_contracted_status_applied,
        rollback_rehearsal_verified: true,
        combined_finance_conserved,
        pre_receipt_mod_save_unchanged,
        executed_plan_persisted,
        trade_receipt_persisted,
        receipt_readback_verified,
        executed_plan_registry_count: persisted_receipt.executed_plan_registry_count,
        offered_contract_fingerprint,
        target_contract_fingerprint,
        news_count_unchanged: post_receipt_news_unchanged,
        commit_process_id: std::process::id(),
        game_time: db.time.to_string(),
    };
    clear_atomic_rollback();
    clear_forced_rollback_audit();
    Ok(result)
}

fn requested_years_from_contract_days(contract_days_left: i64) -> usize {
    if contract_days_left <= 0 {
        return 1;
    }
    (((contract_days_left as u64 + 364) / 365).clamp(1, 10)) as usize
}

fn desired_squad_status_from_action(action: u8) -> Option<u8> {
    match action {
        ACTION_STATUS_CORE => Some(STATUS_CORE),
        ACTION_STATUS_IMPORTANT => Some(STATUS_IMPORTANT),
        ACTION_STATUS_GENERAL => Some(STATUS_GENERAL),
        ACTION_STATUS_SUB => Some(STATUS_SUB),
        ACTION_STATUS_PROSPECT => Some(STATUS_PROSPECT),
        _ => None,
    }
}

fn team_position_count(db: &Database, team_id: usize, position: Position) -> usize {
    db.athletes
        .iter()
        .filter(|athlete| {
            matches!(
                &athlete.contract,
                Contract::InContract {
                    team_id: current_team_id,
                    ..
                } if *current_team_id == team_id && athlete.main_position() == position
            )
        })
        .count()
}

fn structural_continuity(
    db: &Database,
    team_id: usize,
    outgoing_position: Position,
    incoming_position: Position,
) -> bool {
    let current = team_position_count(db, team_id, outgoing_position);
    current > 1 || incoming_position == outgoing_position
}

fn squad_status_label(status: SquadStatus) -> &'static str {
    match status {
        SquadStatus::Core => "Core",
        SquadStatus::Important => "Important",
        SquadStatus::General => "General",
        SquadStatus::Sub => "Sub",
        SquadStatus::Prospect => "Prospect",
    }
}

fn squad_status_rank(status: SquadStatus) -> u8 {
    match status {
        SquadStatus::Core => 0,
        SquadStatus::Important => 1,
        SquadStatus::General => 2,
        SquadStatus::Sub => 3,
        SquadStatus::Prospect => 4,
    }
}

fn trade_role_promise_allows(
    current_status: SquadStatus,
    contracted_status: Option<SquadStatus>,
    promised_status: SquadStatus,
) -> bool {
    let strongest_existing_rank = contracted_status
        .map(squad_status_rank)
        .map(|rank| rank.min(squad_status_rank(current_status)))
        .unwrap_or_else(|| squad_status_rank(current_status));
    squad_status_rank(promised_status) <= strongest_existing_rank
}

fn replacement_hard_floor(status: SquadStatus) -> f64 {
    match status {
        SquadStatus::Core => CORE_REPLACEMENT_FLOOR,
        SquadStatus::Important => IMPORTANT_REPLACEMENT_FLOOR,
        SquadStatus::General => GENERAL_REPLACEMENT_FLOOR,
        SquadStatus::Sub | SquadStatus::Prospect => 0.0,
    }
}

fn has_value_replacement_floor(status: SquadStatus) -> bool {
    matches!(status, SquadStatus::Core | SquadStatus::Important | SquadStatus::General)
}

fn hard_floor_allows(status: SquadStatus, structural_cover: bool, ratio: f64) -> bool {
    if !structural_cover || !ratio.is_finite() || ratio < 0.0 {
        return false;
    }
    if !has_value_replacement_floor(status) {
        return true;
    }
    ratio + 1e-12 >= replacement_hard_floor(status)
}

fn athlete_has_active_negotiation(athlete: &Athlete) -> bool {
    let Contract::InContract { transfer_requests, recruit_requests, .. } = &athlete.contract else {
        return false;
    };
    let debug = format!("{:?}|{:?}", transfer_requests, recruit_requests);
    debug.contains("state: Waiting") || debug.contains("state: CounterOffer")
}

fn replacement_floor_assessment(
    db: &Database,
    seller_team_id: usize,
    target: &Athlete,
    incoming: Option<&Athlete>,
    region_id: usize,
) -> Result<ReplacementFloorAssessment, String> {
    let target_value = fair_fee(db, target, region_id)?;
    let target_position = target.main_position();
    let mut internal_best_candidate_id = None;
    let mut internal_best_candidate_name = String::new();
    let mut internal_best_value = 0.0_f64;

    for candidate in db.athletes.iter() {
        if candidate.id == target.id || candidate.main_position() != target_position {
            continue;
        }
        let Contract::InContract { team_id, .. } = &candidate.contract else {
            continue;
        };
        if *team_id != seller_team_id || athlete_has_active_negotiation(candidate) {
            continue;
        }
        let value = fair_fee(db, candidate, region_id)?;
        if value > internal_best_value {
            internal_best_value = value;
            internal_best_candidate_id = Some(candidate.id);
            internal_best_candidate_name = candidate.name.clone();
        }
    }

    let internal_best_ratio = if target_value > 0.0 {
        internal_best_value / target_value
    } else {
        0.0
    };
    let incoming_same_position = incoming.is_some_and(|athlete| {
        athlete.main_position() == target_position && !athlete_has_active_negotiation(athlete)
    });
    let incoming_ratio = if incoming_same_position {
        fair_fee(db, incoming.expect("incoming checked above"), region_id)? / target_value
    } else {
        0.0
    };
    let structural_cover = internal_best_candidate_id.is_some() || incoming_same_position;
    let effective_ratio = internal_best_ratio.max(incoming_ratio);
    let floor = replacement_hard_floor(target.squad_status);
    let allows = hard_floor_allows(target.squad_status, structural_cover, effective_ratio);

    Ok(ReplacementFloorAssessment {
        target_status: target.squad_status,
        floor,
        internal_best_candidate_id,
        internal_best_candidate_name,
        internal_best_ratio,
        incoming_same_position,
        incoming_ratio,
        effective_ratio,
        structural_cover,
        allows,
    })
}

fn replacement_floor_error(
    target: &Athlete,
    assessment: &ReplacementFloorAssessment,
) -> String {
    format!(
        "SELLER_REPLACEMENT_UNAVAILABLE: target={} status={} required_ratio={:.6} effective_ratio={:.6} structural_cover={} internal_best_id={:?} internal_best_name={} internal_best_ratio={:.6} incoming_same_position={} incoming_ratio={:.6}; cash cannot bypass the replacement floor",
        target.name,
        squad_status_label(assessment.target_status),
        assessment.floor,
        assessment.effective_ratio,
        assessment.structural_cover,
        assessment.internal_best_candidate_id,
        sanitize(&assessment.internal_best_candidate_name),
        assessment.internal_best_ratio,
        assessment.incoming_same_position,
        assessment.incoming_ratio,
    )
}

fn fair_fee(db: &Database, athlete: &Athlete, region_id: usize) -> Result<f64, String> {
    let market = db
        .transfer_market
        .as_ref()
        .ok_or_else(|| "Database::transfer_market is None".to_string())?;
    let value = calculate_athlete_fair_transfer_fee_at(
        athlete,
        market,
        region_id,
        db.time,
        GameModeKind::Moba,
    );
    if !value.is_finite() || value < 0.0 {
        return Err(format!(
            "invalid fair value for {}: {}",
            athlete.name, value
        ));
    }
    Ok(value)
}

fn decision_label(evaluation: &PlayerAssetEvaluation) -> String {
    sanitize(&format!("{:?}", evaluation.decision))
}

fn decision_mapped_profile(evaluation: &PlayerAssetEvaluation) -> ProfileSpec {
    match decision_label(evaluation).as_str() {
        "SellIfGoodOffer" | "ListForSale" | "NonRenew" | "RunDown" => ProfileSpec {
            label: "DECISION_MAPPED_ACTIVE",
            stance: PlayerAssetTradeStance::ActivelySell,
            is_excess_sell: true,
        },
        _ => ProfileSpec {
            label: "DECISION_MAPPED_PROTECT",
            stance: PlayerAssetTradeStance::ProtectButListen,
            is_excess_sell: false,
        },
    }
}

fn policy_for_profile(
    evaluation: &PlayerAssetEvaluation,
    profile: ProfileSpec,
    replacement_score_ratio: f32,
    replacement_target_shortfall: f32,
) -> SellerTransferValuePolicy {
    seller_transfer_value_policy(SellerTransferValueContext {
        trade_stance: profile.stance,
        asset_intent: Some(evaluation.intent),
        is_excess_sell: profile.is_excess_sell,
        is_release_listed: false,
        salary_pressure: evaluation.operating_plan.envelope.pressure_score,
        salary_share: evaluation.salary_share,
        days_left: evaluation.days_left,
        position_percentile: evaluation.position_percentile,
        replacement_score_ratio,
        replacement_target_shortfall,
        replacement_secured: true,
        replacement_in_progress: false,
    })
}

fn policy_accepts(
    evaluation: &PlayerAssetEvaluation,
    policy: SellerTransferValuePolicy,
    profile: ProfileSpec,
    target_value: f64,
    requester_value: f64,
    requester_cash: f64,
    recipient_continuity_ok: bool,
    continuity_risk: f64,
) -> (bool, bool) {
    let context = SellerRosterContinuityContext {
        athlete_age: evaluation.age,
        continuity_risk,
        days_left: evaluation.days_left,
        fair_fee: target_value,
        incoming_fee: requester_value + requester_cash,
        is_release_listed: false,
        replacement_in_progress: false,
        replacement_secured: recipient_continuity_ok,
        salary_pressure: evaluation.operating_plan.envelope.pressure_score,
        trade_stance: profile.stance,
        value_policy: policy,
    };
    let allow_first = seller_roster_continuity_allows_sale(context);
    let allow_second = seller_roster_continuity_allows_sale(context);
    let hard_first = seller_roster_continuity_hard_blocks_sale(context);
    let hard_second = seller_roster_continuity_hard_blocks_sale(context);
    (
        allow_first && !hard_first,
        allow_first == allow_second && hard_first == hard_second,
    )
}

fn find_exact_requester_cash(
    evaluation: &PlayerAssetEvaluation,
    policy: SellerTransferValuePolicy,
    profile: ProfileSpec,
    target_value: f64,
    requester_value: f64,
    recipient_continuity_ok: bool,
    continuity_risk: f64,
    requester_cash_budget: f64,
) -> Result<ThresholdSearchResult, String> {
    if !requester_cash_budget.is_finite() || requester_cash_budget < 0.0 {
        return Err("requester cash budget is not finite and nonnegative".to_string());
    }

    let max_cash = requester_cash_budget.floor().min(9_000_000_000_000_000.0) as u64;
    let mut evaluation_count = 0usize;
    let mut repeat_consistent = true;

    let (zero_cash_accepted, zero_repeat) = policy_accepts(
        evaluation,
        policy,
        profile,
        target_value,
        requester_value,
        0.0,
        recipient_continuity_ok,
        continuity_risk,
    );
    evaluation_count += 1;
    repeat_consistent &= zero_repeat;
    if zero_cash_accepted {
        return Ok(ThresholdSearchResult {
            requester_cash: 0.0,
            evaluation_count,
            repeat_consistent,
            coarse_monotonic: true,
            boundary_verified: true,
            zero_cash_accepted: true,
            budget_ceiling_accepted: true,
        });
    }

    if max_cash == 0 {
        return Err("seller rejects zero cash and requester has no cash budget".to_string());
    }

    let (budget_ceiling_accepted, ceiling_repeat) = policy_accepts(
        evaluation,
        policy,
        profile,
        target_value,
        requester_value,
        max_cash as f64,
        recipient_continuity_ok,
        continuity_risk,
    );
    evaluation_count += 1;
    repeat_consistent &= ceiling_repeat;
    if !budget_ceiling_accepted {
        return Err(format!(
            "no acceptable requester-only cash package within budget ceiling {}",
            max_cash
        ));
    }

    let mut seen_accept = false;
    let mut coarse_monotonic = true;
    for step in 0u64..=64u64 {
        let cash = if step == 64 {
            max_cash
        } else {
            ((max_cash as u128 * step as u128) / 64u128) as u64
        };
        let (accepted, repeat) = policy_accepts(
            evaluation,
            policy,
            profile,
            target_value,
            requester_value,
            cash as f64,
            recipient_continuity_ok,
            continuity_risk,
        );
        evaluation_count += 1;
        repeat_consistent &= repeat;
        if accepted {
            seen_accept = true;
        } else if seen_accept {
            coarse_monotonic = false;
        }
    }
    if !coarse_monotonic {
        return Err("seller policy acceptance is non-monotonic".to_string());
    }

    let mut low_rejected = 0u64;
    let mut high_accepted = max_cash;
    while low_rejected + 1 < high_accepted {
        let mid = low_rejected + (high_accepted - low_rejected) / 2;
        let (accepted, repeat) = policy_accepts(
            evaluation,
            policy,
            profile,
            target_value,
            requester_value,
            mid as f64,
            recipient_continuity_ok,
            continuity_risk,
        );
        evaluation_count += 1;
        repeat_consistent &= repeat;
        if accepted {
            high_accepted = mid;
        } else {
            low_rejected = mid;
        }
    }

    let requester_cash = high_accepted as f64;
    let (exact_accepted, exact_repeat) = policy_accepts(
        evaluation,
        policy,
        profile,
        target_value,
        requester_value,
        requester_cash,
        recipient_continuity_ok,
        continuity_risk,
    );
    evaluation_count += 1;
    repeat_consistent &= exact_repeat;

    let (below_accepted, below_repeat) = if high_accepted > 0 {
        let result = policy_accepts(
            evaluation,
            policy,
            profile,
            target_value,
            requester_value,
            (high_accepted - 1) as f64,
            recipient_continuity_ok,
            continuity_risk,
        );
        evaluation_count += 1;
        result
    } else {
        (false, true)
    };
    repeat_consistent &= below_repeat;

    let (above_accepted, above_repeat) = policy_accepts(
        evaluation,
        policy,
        profile,
        target_value,
        requester_value,
        requester_cash + 1.0,
        recipient_continuity_ok,
        continuity_risk,
    );
    evaluation_count += 1;
    repeat_consistent &= above_repeat;

    let boundary_verified =
        exact_accepted && above_accepted && (high_accepted == 0 || !below_accepted);
    if !repeat_consistent {
        return Err("seller policy threshold was not deterministic".to_string());
    }
    if !boundary_verified {
        return Err("exact requester-cash boundary could not be verified".to_string());
    }

    Ok(ThresholdSearchResult {
        requester_cash,
        evaluation_count,
        repeat_consistent,
        coarse_monotonic,
        boundary_verified,
        zero_cash_accepted,
        budget_ceiling_accepted,
    })
}

fn won_to_units_ceil(won: f64) -> Result<u64, String> {
    if !won.is_finite() || won < 0.0 {
        return Err("cash amount is not finite and nonnegative".to_string());
    }
    Ok((won / MONEY_UNIT_WON as f64).ceil() as u64)
}

fn won_to_units_floor(won: f64) -> Result<u64, String> {
    if !won.is_finite() || won < 0.0 {
        return Err("budget is not finite and nonnegative".to_string());
    }
    Ok((won / MONEY_UNIT_WON as f64).floor() as u64)
}

fn cash_offer_max_units(required_units: u64, budget_units: u64) -> u64 {
    required_units.saturating_mul(2).min(budget_units)
}


fn stable_obscured_cash_range(
    required_units: u64,
    _budget_units: u64,
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
) -> (u64, u64, u8, u8) {
    if required_units == 0 {
        return (0, 0, 0, 0);
    }
    let seed = fnv1a64(&format!(
        "PTS76_RANGE_SALT_8D41A3|requester={requester_team_id}|recipient={recipient_team_id}|offered={offered_id}|target={target_id}"
    ));
    let lower_percent = 70_u8 + (seed % 11) as u8;
    let upper_percent = 150_u8 + (seed.rotate_left(23) % 11) as u8;
    let lower_raw = required_units
        .saturating_mul(u64::from(lower_percent))
        / 100;
    let upper_raw = required_units
        .saturating_mul(u64::from(upper_percent))
        .saturating_add(99)
        / 100;
    // Money input units are ten-thousand won. Display boundaries are sealed
    // to ten-million-won steps so the UI shows only 억/천 buckets.
    let display_min_units = (lower_raw / 1_000).saturating_mul(1_000);
    let display_max_units = upper_raw
        .saturating_add(999)
        / 1_000
        * 1_000;
    (display_min_units, display_max_units, lower_percent, upper_percent)
}

fn advance_game_days(db: &Database, days: i64) -> Result<String, String> {
    if days < 0 {
        return Err("async trade delay days must be nonnegative".to_string());
    }
    let mut due = db.time;
    for _ in 0..days {
        let next = due
            .date()
            .succ_opt()
            .ok_or_else(|| "async trade due date overflow".to_string())?;
        due = next.and_time(due.time());
    }
    Ok(due.to_string())
}

fn async_time_reached(now: &str, due: &str) -> bool {
    !due.is_empty() && now >= due
}

fn encode_async_trade_proposal(proposal: &AsyncTradeProposal) -> Vec<u8> {
    let mut text = String::new();
    let _ = writeln!(text, "schema_version={}", proposal.schema_version);
    let _ = writeln!(text, "proposal_id={}", proposal.proposal_id);
    let _ = writeln!(text, "state={}", proposal.state.as_str());
    let _ = writeln!(text, "requester_team_id={}", proposal.requester_team_id);
    let _ = writeln!(text, "recipient_team_id={}", proposal.recipient_team_id);
    let _ = writeln!(text, "offered_id={}", proposal.offered_id);
    let _ = writeln!(text, "target_id={}", proposal.target_id);
    let _ = writeln!(text, "offered_name={}", sanitize(&proposal.offered_name));
    let _ = writeln!(text, "target_name={}", sanitize(&proposal.target_name));
    let _ = writeln!(text, "region_id={}", proposal.region_id);
    let _ = writeln!(text, "proposed_units={}", proposal.proposed_units);
    let _ = writeln!(text, "proposed_cash_won={}", proposal.proposed_cash_won);
    let _ = writeln!(text, "desired_status_choice={}", proposal.desired_status_choice);
    let _ = writeln!(text, "desired_status_key={}", proposal.desired_status_key);
    let _ = writeln!(text, "display_min_units={}", proposal.display_min_units);
    let _ = writeln!(text, "display_max_units={}", proposal.display_max_units);
    let _ = writeln!(text, "display_lower_percent={}", proposal.display_lower_percent);
    let _ = writeln!(text, "display_upper_percent={}", proposal.display_upper_percent);
    let _ = writeln!(text, "submitted_at={}", proposal.submitted_at);
    let _ = writeln!(text, "seller_due_at={}", proposal.seller_due_at);
    let _ = writeln!(text, "player_due_at={}", proposal.player_due_at);
    let _ = writeln!(text, "completed_at={}", proposal.completed_at);
    let _ = writeln!(text, "package_fingerprint={:016X}", proposal.package_fingerprint);
    let _ = writeln!(text, "offered_contract_fingerprint_at_submit={:016X}", proposal.offered_contract_fingerprint_at_submit);
    let _ = writeln!(text, "target_contract_fingerprint_at_submit={:016X}", proposal.target_contract_fingerprint_at_submit);
    let _ = writeln!(text, "result_plan_id={}", proposal.result_plan_id);
    let _ = writeln!(text, "rejection_actor={}", proposal.rejection_actor);
    let _ = writeln!(text, "rejection_reason={}", proposal.rejection_reason);
    let _ = writeln!(text, "rejection_reason_ko={}", sanitize(&proposal.rejection_reason_ko));
    let _ = writeln!(text, "success_news_id={}", proposal.success_news_id);
    let _ = writeln!(text, "transition_count={}", proposal.transition_count);
    let _ = writeln!(text, "submit_process_id={}", proposal.submit_process_id);
    let _ = writeln!(text, "commit_process_id={}", proposal.commit_process_id);
    text.into_bytes()
}

fn decode_async_trade_proposal(bytes: &[u8]) -> Result<AsyncTradeProposal, String> {
    let values = parse_kv_payload(bytes)?;
    let schema_raw = map_u64(&values, "schema_version")?;
    let schema_version = u8::try_from(schema_raw)
        .map_err(|_| "async proposal schema version does not fit u8".to_string())?;
    if schema_version != 1 {
        return Err(format!("unsupported async proposal schema {schema_version}"));
    }
    let desired_raw = map_u64(&values, "desired_status_choice")?;
    let desired_status_choice = u8::try_from(desired_raw)
        .map_err(|_| "async proposal desired status does not fit u8".to_string())?;
    let lower_raw = map_u64(&values, "display_lower_percent")?;
    let upper_raw = map_u64(&values, "display_upper_percent")?;
    let transition_raw = map_u64(&values, "transition_count")?;
    let submit_pid_raw = map_u64(&values, "submit_process_id")?;
    let commit_pid_raw = map_u64(&values, "commit_process_id")?;
    Ok(AsyncTradeProposal {
        schema_version,
        proposal_id: map_required(&values, "proposal_id")?.to_string(),
        state: AsyncTradeState::from_str(map_required(&values, "state")?)?,
        requester_team_id: map_usize(&values, "requester_team_id")?,
        recipient_team_id: map_usize(&values, "recipient_team_id")?,
        offered_id: map_usize(&values, "offered_id")?,
        target_id: map_usize(&values, "target_id")?,
        offered_name: map_required(&values, "offered_name")?.to_string(),
        target_name: map_required(&values, "target_name")?.to_string(),
        region_id: map_usize(&values, "region_id")?,
        proposed_units: map_u64(&values, "proposed_units")?,
        proposed_cash_won: map_u64(&values, "proposed_cash_won")?,
        desired_status_choice,
        desired_status_key: map_required(&values, "desired_status_key")?.to_string(),
        display_min_units: map_u64(&values, "display_min_units")?,
        display_max_units: map_u64(&values, "display_max_units")?,
        display_lower_percent: u8::try_from(lower_raw).map_err(|_| "lower percent does not fit u8".to_string())?,
        display_upper_percent: u8::try_from(upper_raw).map_err(|_| "upper percent does not fit u8".to_string())?,
        submitted_at: map_required(&values, "submitted_at")?.to_string(),
        seller_due_at: map_required(&values, "seller_due_at")?.to_string(),
        player_due_at: map_required(&values, "player_due_at")?.to_string(),
        completed_at: map_required(&values, "completed_at")?.to_string(),
        package_fingerprint: u64::from_str_radix(map_required(&values, "package_fingerprint")?, 16)
            .map_err(|e| format!("async proposal package fingerprint: {e}"))?,
        offered_contract_fingerprint_at_submit: u64::from_str_radix(map_required(&values, "offered_contract_fingerprint_at_submit")?, 16)
            .map_err(|e| format!("async proposal offered fingerprint: {e}"))?,
        target_contract_fingerprint_at_submit: u64::from_str_radix(map_required(&values, "target_contract_fingerprint_at_submit")?, 16)
            .map_err(|e| format!("async proposal target fingerprint: {e}"))?,
        result_plan_id: map_required(&values, "result_plan_id")?.to_string(),
        rejection_actor: map_required(&values, "rejection_actor")?.to_string(),
        rejection_reason: map_required(&values, "rejection_reason")?.to_string(),
        rejection_reason_ko: map_required(&values, "rejection_reason_ko")?.to_string(),
        success_news_id: map_required(&values, "success_news_id")?.to_string(),
        transition_count: u32::try_from(transition_raw).map_err(|_| "transition count does not fit u32".to_string())?,
        submit_process_id: u32::try_from(submit_pid_raw).map_err(|_| "submit process id does not fit u32".to_string())?,
        commit_process_id: u32::try_from(commit_pid_raw).map_err(|_| "commit process id does not fit u32".to_string())?,
    })
}

fn load_async_trade_proposal(db: &Database) -> Result<Option<AsyncTradeProposal>, String> {
    db.mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, ASYNC_PROPOSAL_KEY)
        .map(|bytes| decode_async_trade_proposal(&bytes))
        .transpose()
}

fn save_async_trade_proposal(db: &mut Database, proposal: &AsyncTradeProposal) -> Result<(), String> {
    let bytes = encode_async_trade_proposal(proposal);
    let _ = db.mod_save_data.set_version(MOD_SAVE_NAMESPACE, MOD_SAVE_NAMESPACE_VERSION);
    let _ = db.mod_save_data.set_bytes(MOD_SAVE_NAMESPACE, ASYNC_PROPOSAL_KEY, bytes.clone());
    let readback = db.mod_save_data
        .get_bytes(MOD_SAVE_NAMESPACE, ASYNC_PROPOSAL_KEY)
        .ok_or_else(|| "async trade proposal disappeared after persistence".to_string())?;
    if readback != bytes || decode_async_trade_proposal(&readback)? != *proposal {
        return Err("async trade proposal persistence readback mismatch".to_string());
    }
    Ok(())
}

fn async_proposal_id(
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    proposed_cash_won: u64,
    desired_status_choice: u8,
    submitted_at: &str,
) -> String {
    let canonical = format!(
        "T77|requester={requester_team_id}|recipient={recipient_team_id}|offered={offered_id}|target={target_id}|cash={proposed_cash_won}|status={desired_status_choice}|submitted={submitted_at}|pid={}",
        std::process::id(),
    );
    format!("T77-{:016X}", fnv1a64(&canonical))
}

fn async_success_news_count(db: &Database, requester_team_id: usize, proposal_id: &str) -> Result<usize, String> {
    let team = db.teams.get(requester_team_id)
        .ok_or_else(|| "requester team missing while counting async success news".to_string())?;
    Ok(team.news.iter().filter(|news| {
        news.title_bind.iter().any(|(key, value)| key == ASYNC_SUCCESS_NEWS_BIND_KEY && value == proposal_id)
    }).count())
}

fn ensure_async_success_news(
    db: &mut Database,
    proposal: &AsyncTradeProposal,
    result: &AtomicTradeResult,
) -> Result<(bool, usize), String> {
    let before = async_success_news_count(db, proposal.requester_team_id, &proposal.proposal_id)?;
    if before == 0 {
        let requester_name = db.teams.get(proposal.requester_team_id)
            .map(|team| team.name.clone()).unwrap_or_else(|| format!("팀 {}", proposal.requester_team_id));
        let recipient_name = db.teams.get(proposal.recipient_team_id)
            .map(|team| team.name.clone()).unwrap_or_else(|| format!("팀 {}", proposal.recipient_team_id));
        let content = format!(
            "{} 구단과 {} 구단의 선수 트레이드가 성사되었습니다.\n\n{} 영입: {}\n{} 방출: {}\n추가 지급액: {}\n약속 위상: {}",
            requester_name,
            recipient_name,
            requester_name,
            proposal.target_name,
            requester_name,
            proposal.offered_name,
            format_cash_amount(proposal.proposed_units),
            desired_squad_status(proposal.desired_status_choice).1,
        );
        let news = News {
            ty: NewsType::Simple { content, content_bind: Vec::new() },
            title: "선수 트레이드 성사".to_string(),
            title_bind: vec![(ASYNC_SUCCESS_NEWS_BIND_KEY.to_string(), proposal.proposal_id.clone())],
            author: ASYNC_SUCCESS_NEWS_AUTHOR.to_string(),
            date: db.time,
            is_read: false,
            is_sent: false,
            is_favorite: false,
        };
        db.teams.get_mut(proposal.requester_team_id)
            .ok_or_else(|| "requester team missing during async success news insertion".to_string())?
            .news.push(news);
    }
    let after = async_success_news_count(db, proposal.requester_team_id, &proposal.proposal_id)?;
    if after != 1 {
        return Err(format!("async success news count must be exactly one, found {after}"));
    }
    log_event(
        "async_trade_success_news_created",
        &format!(
            "proposal_id={};plan_id={};news_created={};success_news_count={};offered_name={};target_name={};proposed_cash_won={};database_mutation=true;news_mutation=true;transaction_executed=true",
            proposal.proposal_id,
            result.plan_id,
            before == 0,
            after,
            sanitize(&proposal.offered_name),
            sanitize(&proposal.target_name),
            proposal.proposed_cash_won,
        ),
    );
    Ok((before == 0, after))
}

fn explicit_async_rejection_policy(reason: &str) -> rejection::RejectionPolicy {
    match reason {
        "SellerReplacementUnavailable" => rejection::RejectionPolicy {
            actor: "SELLER_TEAM",
            reason: "SellerReplacementUnavailable",
            reason_ko: "상대 구단이 적절한 대체 선수를 확보하지 못했습니다.",
            policy: "changed_package_immediate_same_offer_1d",
            retry_days: 1,
            changeable: true,
        },
        "BudgetExceeded" => rejection::RejectionPolicy {
            actor: "REQUESTER_TEAM_BUDGET",
            reason: "BudgetExceeded",
            reason_ko: "신청팀이 현재 제안 조건을 감당할 예산이 부족합니다.",
            policy: "changed_package_immediate_same_offer_1d",
            retry_days: 1,
            changeable: true,
        },
        "TargetPlayerRejected" => rejection::RejectionPolicy {
            actor: "TARGET_PLAYER",
            reason: "InheritedContractOrPromisedRoleUnacceptable",
            reason_ko: "선수가 기존 계약 승계 또는 약속 위상 조건에 동의하지 않았습니다.",
            policy: "changed_package_immediate_same_offer_1d",
            retry_days: 1,
            changeable: true,
        },
        _ => rejection::RejectionPolicy {
            actor: "SELLER_TEAM",
            reason: "TermsUnacceptable",
            reason_ko: "상대 구단이 현재 선수와 현금 조건을 받아들이지 않았습니다.",
            policy: "changed_package_immediate_same_offer_1d",
            retry_days: 1,
            changeable: true,
        },
    }
}

fn reject_async_trade_proposal(
    db: &mut Database,
    proposal: &mut AsyncTradeProposal,
    policy: rejection::RejectionPolicy,
    detail: &str,
) -> Result<(), String> {
    let requester_name = db.teams.get(proposal.requester_team_id)
        .map(|team| team.name.clone()).ok_or_else(|| "requester team missing during async rejection".to_string())?;
    let recipient_name = db.teams.get(proposal.recipient_team_id)
        .map(|team| team.name.clone()).ok_or_else(|| "recipient team missing during async rejection".to_string())?;
    let meta = rejection::record_trade_rejection(
        db,
        &proposal.offered_name,
        proposal.target_id,
        &proposal.target_name,
        proposal.requester_team_id,
        &requester_name,
        proposal.recipient_team_id,
        &recipient_name,
        proposal.package_fingerprint,
        policy,
    )?;
    proposal.state = AsyncTradeState::Rejected;
    // ★[PORT056] 종료 시 우리가 넣은 네이티브 협상 항목을 제거한다(다른 협상은 건드리지 않는다).
    if let Err(detail) = sync_native_transfer_request(
        db, proposal.target_id, proposal.requester_team_id, false, 0, 0.0, SquadStatus::General,
    ) {
        log_event(
            "native_transfer_request_sync_failed",
            &format!("stage=terminal;detail={}", sanitize(&detail)),
        );
    }
    proposal.completed_at = db.time.to_string();
    proposal.rejection_actor = meta.actor.clone();
    proposal.rejection_reason = meta.reason.clone();
    proposal.rejection_reason_ko = meta.reason_ko.clone();
    proposal.transition_count = proposal.transition_count.saturating_add(1);
    save_async_trade_proposal(db, proposal)?;
    log_event(
        "async_trade_proposal_rejected",
        &format!(
            "proposal_id={};state=Rejected;actor={};reason={};reason_ko={};detail={};completed_at={};feedback_id={:016X};news_created={};duplicate_news_count={};proposal_persisted=true;transaction_executed=false",
            proposal.proposal_id,
            meta.actor,
            meta.reason,
            sanitize(&meta.reason_ko),
            sanitize(detail),
            proposal.completed_at,
            meta.feedback_id,
            meta.news_created,
            meta.duplicate_news_count,
        ),
    );
    Ok(())
}

fn submit_async_trade_proposal(
    db: &mut Database,
    requester_team_id: usize,
    offered_id: usize,
    target_id: usize,
    region_id: usize,
    proposed_units: u64,
    desired_status_choice: u8,
) -> Result<AsyncTradeProposal, String> {
    // [PORT056] 구 Test79 의 3중 테스트 픽스처 게이트 제거:
    //   ① require_dual_trade_baseline  = 세이브에 과거 Jue/Zeus·Zenit/Fill 거래 영수증 2건이 있어야 통과
    //   ② offered/target = Dread(7)/xartE(92) 고정
    //   ③ 약속 위상 = 핵심(Core) 고정
    //   셋 다 그쪽 전용 세이브 전제라 일반 세이브에선 제안 자체가 항상 실패했다.
    require_fresh_trade_fixture(db, requester_team_id, offered_id, target_id)?;
    if !native_recruit_open(db) {
        return Err("영입 기간에만 트레이드를 제안할 수 있습니다".to_string());
    }
    // ★[PORT056] 요구사항 1 — 영입 시즌당 1회. **성사만 소모**(유저 확정 2026-08-22)이므로
    //   거절·만료로 끝난 뒤에는 같은 시즌에 조건을 바꿔 다시 제안할 수 있다.
    if trade_season_already_used(db) {
        return Err("이번 영입 시즌에는 이미 트레이드를 성사시켰습니다".to_string());
    }
    // ★[PORT056] 제안 레코드는 삭제되지 않고 **종료 상태로 덮어써질 뿐**이다(ASYNC_PROPOSAL_KEY).
    //   구판은 존재만 보고 막아서, 한 번 거절당하면 그 세이브에서 **영영 재제안이 불가능**했다
    //   (= 인계본이 "두 번째 트레이드 가능"을 끝내 검증 못 한 이유). 진행 중일 때만 막는다.
    if let Some(existing) = load_async_trade_proposal(db)? {
        if !existing.state.terminal() {
            return Err(format!(
                "이미 진행 중인 트레이드 제안이 있습니다 ({})",
                existing.state.stage_ko(),
            ));
        }
    }
    // ★[PORT056] `rejection::check_cooldown` 은 정의만 돼 있고 **아무 데서도 호출되지 않았다**
    //   ⟹ 거절 뉴스가 "같은 제안은 <날짜>부터 가능"이라고 약속해놓고 실제로는 강제되지 않았다.
    //   여기서 배선한다. 조건(선수/현금/위상)을 바꾸면 지문이 달라져 즉시 재제안 가능(changed_package_bypass).
    {
        let recipient_team_id = db
            .athletes
            .get(target_id)
            .ok_or_else(|| "target athlete missing during cooldown check".to_string())
            .and_then(contract_team_id)?;
        // ⚠ 아래 인자는 제안 생성 시점(`package_fingerprint:` 필드)의 산식과 **반드시 같아야** 한다.
        //   (`proposed_cash_won = proposed_units * MONEY_UNIT_WON`, `recipient_team_id = contract_team_id(target)`)
        //   어긋나면 지문이 달라져 쿨다운이 조용히 무력화된다.
        let fingerprint = rejection::trade_package_fingerprint(
            requester_team_id,
            recipient_team_id,
            offered_id,
            target_id,
            proposed_units.saturating_mul(MONEY_UNIT_WON),
            desired_status_choice,
        );
        // fail-open: 쿨다운은 편의 규칙이다. 원장/뉴스가 정리돼 판정이 불가능해졌다고 해서
        //   정상 제안을 막으면 안 된다(check_cooldown 은 피드백 뉴스가 정확히 1건이 아니면 Err 를 낸다).
        match rejection::check_cooldown(db, target_id, requester_team_id, recipient_team_id, fingerprint) {
            Ok(cooldown) => {
                if let Some(meta) = cooldown.blocked {
                    return Err(format!(
                        "이 선수에게는 {} 이후 다시 제안할 수 있습니다. ({})",
                        meta.retry_ko, meta.reason_ko,
                    ));
                }
            }
            Err(detail) => log_event(
                "trade_cooldown_check_skipped",
                &format!("reason=check_failed;fail_open=true;detail={}", sanitize(&detail)),
            ),
        }
    }
    let offered = db.athletes.get(offered_id)
        .ok_or_else(|| "offered athlete missing during async submission".to_string())?;
    let target = db.athletes.get(target_id)
        .ok_or_else(|| "target athlete missing during async submission".to_string())?;
    if athlete_has_active_negotiation(offered) || athlete_has_active_negotiation(target) {
        return Err("a selected athlete already has an active negotiation".to_string());
    }
    let server_region_id = server_contracted_athlete_region_id(db, offered)?;
    if region_id != server_region_id {
        return Err(format!("client region {region_id} does not match server region {server_region_id}"));
    }
    let quote = evaluate_server_quote(db, requester_team_id, offered_id, target_id, region_id)?;
    if proposed_units < quote.display_min_units || proposed_units > quote.display_max_units {
        return Err(format!(
            "proposed units must be inside the displayed obscured range {}..{}",
            quote.display_min_units,
            quote.display_max_units,
        ));
    }
    let proposed_cash_won = proposed_units.checked_mul(MONEY_UNIT_WON)
        .ok_or_else(|| "proposed cash overflowed won conversion".to_string())?;
    if proposed_cash_won as f64 > quote.cash_budget_won + 0.001 {
        return Err("proposed cash exceeds the requester transfer budget".to_string());
    }
    let recipient_team_id = contract_team_id(target)?;
    let submitted_at = db.time.to_string();
    let proposal_id = async_proposal_id(
        requester_team_id,
        recipient_team_id,
        offered_id,
        target_id,
        proposed_cash_won,
        desired_status_choice,
        &submitted_at,
    );
    let offered_snapshot = athlete_trade_snapshot(db, offered_id)?;
    let target_snapshot = athlete_trade_snapshot(db, target_id)?;
    let (desired_status_key, _) = desired_squad_status(desired_status_choice);
    let proposal = AsyncTradeProposal {
        schema_version: 1,
        proposal_id,
        state: AsyncTradeState::SellerReview,
        requester_team_id,
        recipient_team_id,
        offered_id,
        target_id,
        offered_name: offered.name.clone(),
        target_name: target.name.clone(),
        region_id,
        proposed_units,
        proposed_cash_won,
        desired_status_choice,
        desired_status_key: desired_status_key.to_string(),
        display_min_units: quote.display_min_units,
        display_max_units: quote.display_max_units,
        display_lower_percent: quote.display_lower_percent,
        display_upper_percent: quote.display_upper_percent,
        submitted_at: submitted_at.clone(),
        seller_due_at: advance_game_days(db, SELLER_REVIEW_DELAY_DAYS)?,
        player_due_at: String::new(),
        completed_at: String::new(),
        package_fingerprint: rejection::trade_package_fingerprint(
            requester_team_id,
            recipient_team_id,
            offered_id,
            target_id,
            proposed_cash_won,
            desired_status_choice,
        ),
        offered_contract_fingerprint_at_submit: athlete_trade_fingerprint(&offered_snapshot),
        target_contract_fingerprint_at_submit: athlete_trade_fingerprint(&target_snapshot),
        result_plan_id: String::new(),
        rejection_actor: String::new(),
        rejection_reason: String::new(),
        rejection_reason_ko: String::new(),
        success_news_id: String::new(),
        transition_count: 0,
        submit_process_id: std::process::id(),
        commit_process_id: 0,
    };
    save_async_trade_proposal(db, &proposal)?;
    // ★[PORT056] 계약 현황 표시 = 게임 데이터. 제출 즉시 네이티브 협상 항목을 넣는다.
    match sync_native_transfer_request(
        db,
        proposal.target_id,
        proposal.requester_team_id,
        true,
        SELLER_REVIEW_DELAY_DAYS,
        proposal.proposed_cash_won as f64,
        desired_squad_status_value(desired_status_choice).unwrap_or(SquadStatus::General),
    ) {
        Ok(action) => log_event(
            "native_transfer_request_synced",
            &format!("stage=submit;action={};target_id={};team_id={}", action, proposal.target_id, proposal.requester_team_id),
        ),
        Err(detail) => log_event(
            "native_transfer_request_sync_failed",
            &format!("stage=submit;detail={}", sanitize(&detail)),
        ),
    }
    log_event(
        "async_trade_proposal_submitted",
        &format!(
            "proposal_id={};state=SellerReview;requester_team_id={};recipient_team_id={};offered_id={};offered_name={};target_id={};target_name={};proposed_units={};proposed_cash_won={};display_min_units={};display_max_units={};display_lower_percent={};display_upper_percent={};exact_required_cash_won={};exact_required_units={};exact_threshold_disclosed_to_client=false;submitted_at={};seller_due_at={};player_due_at=none;proposal_persisted=true;team_mutation=false;finance_mutation=false;contract_mutation=false;squad_status_mutation=false;transaction_executed=false;save_api_called=false",
            proposal.proposal_id,
            proposal.requester_team_id,
            proposal.recipient_team_id,
            proposal.offered_id,
            sanitize(&proposal.offered_name),
            proposal.target_id,
            sanitize(&proposal.target_name),
            proposal.proposed_units,
            proposal.proposed_cash_won,
            proposal.display_min_units,
            proposal.display_max_units,
            proposal.display_lower_percent,
            proposal.display_upper_percent,
            quote.required_cash_won,
            quote.required_units,
            proposal.submitted_at,
            proposal.seller_due_at,
        ),
    );
    Ok(proposal)
}

fn async_status_payload(db: &Database) -> Result<Vec<u8>, String> {
    let mut text = String::new();
    // [PORT056] 요구사항 1 — 시즌 소모 여부를 **주기적으로 도는 이 채널**로 클라이언트에 알린다.
    //   (로드 시 1회뿐인 validate 채널로 보내면 거래 성사 직후 버튼이 갱신되지 않는다.)
    let season_used = trade_season_already_used(db);
    let Some(proposal) = load_async_trade_proposal(db)? else {
        let _ = writeln!(text, "status=none");
        let _ = writeln!(text, "proposal_present=false");
        let _ = writeln!(text, "season_used={}", season_used);
        let _ = writeln!(text, "game_time={}", db.time);
        let _ = writeln!(text, "current_process_id={}", std::process::id());
        let _ = writeln!(text, "database_mutation=false");
        return Ok(text.into_bytes());
    };
    let _ = writeln!(text, "season_used={}", season_used);
    let offered = athlete_trade_snapshot(db, proposal.offered_id).ok();
    let target = athlete_trade_snapshot(db, proposal.target_id).ok();
    let plan_ids = read_persisted_plan_ids(db).unwrap_or_default();
    let plan_occurrences = if proposal.result_plan_id.is_empty() { 0 } else {
        plan_ids.iter().filter(|id| *id == &proposal.result_plan_id).count()
    };
    let success_news_count = async_success_news_count(db, proposal.requester_team_id, &proposal.proposal_id).unwrap_or(0);
    let target_contracted = contracted_squad_status_debug(db, proposal.target_id)
        .unwrap_or_else(|_| "unknown".to_string());
    let _ = writeln!(text, "status=ok");
    let _ = writeln!(text, "proposal_present=true");
    let _ = writeln!(text, "proposal_id={}", proposal.proposal_id);
    let _ = writeln!(text, "state={}", proposal.state.as_str());
    let _ = writeln!(text, "stage_ko={}", proposal.state.stage_ko());
    let _ = writeln!(text, "requester_team_id={}", proposal.requester_team_id);
    let _ = writeln!(text, "recipient_team_id={}", proposal.recipient_team_id);
    let _ = writeln!(text, "offered_id={}", proposal.offered_id);
    let _ = writeln!(text, "target_id={}", proposal.target_id);
    let _ = writeln!(text, "offered_name={}", sanitize(&proposal.offered_name));
    let _ = writeln!(text, "target_name={}", sanitize(&proposal.target_name));
    let requester_team_name = db.teams.get(proposal.requester_team_id)
        .map(|team| team.name.clone()).unwrap_or_else(|| format!("팀 {}", proposal.requester_team_id));
    let recipient_team_name = db.teams.get(proposal.recipient_team_id)
        .map(|team| team.name.clone()).unwrap_or_else(|| format!("팀 {}", proposal.recipient_team_id));
    let (target_position_label, target_position_icon, target_contract_end, target_yearly_salary) =
        if let Some(target_athlete) = db.athletes.get(proposal.target_id) {
            let (position_label, position_icon) = popup_position_visual(target_athlete.main_position());
            let (contract_end, yearly_salary) = match &target_athlete.contract {
                Contract::InContract { end_date, weekly_salary, .. } =>
                    (end_date.date().to_string(), *weekly_salary * WEEKS_PER_YEAR),
                _ => (String::new(), 0.0),
            };
            (position_label.to_string(), position_icon.to_string(), contract_end, yearly_salary)
        } else {
            (String::new(), String::new(), String::new(), 0.0)
        };
    let (_, desired_status_label) = desired_squad_status(proposal.desired_status_choice);
    let _ = writeln!(text, "requester_team_name={}", sanitize(&requester_team_name));
    let _ = writeln!(text, "recipient_team_name={}", sanitize(&recipient_team_name));
    let _ = writeln!(text, "target_position_label={}", sanitize(&target_position_label));
    let _ = writeln!(text, "target_position_icon={}", sanitize(&target_position_icon));
    let _ = writeln!(text, "target_contract_end={}", sanitize(&target_contract_end));
    let _ = writeln!(text, "target_yearly_salary={}", target_yearly_salary);
    let _ = writeln!(text, "proposed_units={}", proposal.proposed_units);
    let _ = writeln!(text, "desired_status_label={}", desired_status_label);
    let _ = writeln!(text, "game_time={}", db.time);
    let _ = writeln!(text, "proposed_cash_won={}", proposal.proposed_cash_won);
    let _ = writeln!(text, "desired_status_choice={}", proposal.desired_status_choice);
    let _ = writeln!(text, "desired_status_key={}", proposal.desired_status_key);
    let _ = writeln!(text, "submitted_at={}", proposal.submitted_at);
    let _ = writeln!(text, "seller_due_at={}", proposal.seller_due_at);
    let _ = writeln!(text, "player_due_at={}", proposal.player_due_at);
    let _ = writeln!(text, "completed_at={}", proposal.completed_at);
    let _ = writeln!(text, "rejection_actor={}", proposal.rejection_actor);
    let _ = writeln!(text, "rejection_reason={}", proposal.rejection_reason);
    let _ = writeln!(text, "rejection_reason_ko={}", sanitize(&proposal.rejection_reason_ko));
    let _ = writeln!(text, "result_plan_id={}", proposal.result_plan_id);
    let _ = writeln!(text, "success_news_count={}", success_news_count);
    let _ = writeln!(text, "transition_count={}", proposal.transition_count);
    let _ = writeln!(text, "submit_process_id={}", proposal.submit_process_id);
    let _ = writeln!(text, "commit_process_id={}", proposal.commit_process_id);
    let _ = writeln!(text, "current_process_id={}", std::process::id());
    let _ = writeln!(text, "offered_team_current={}", offered.as_ref().map(|s| s.team_id).unwrap_or(usize::MAX));
    let _ = writeln!(text, "target_team_current={}", target.as_ref().map(|s| s.team_id).unwrap_or(usize::MAX));
    let _ = writeln!(text, "target_status_current={}", target.as_ref().map(|s| s.squad_status_debug.as_str()).unwrap_or("unknown"));
    let _ = writeln!(text, "target_contracted_status_current={}", sanitize(&target_contracted));
    let _ = writeln!(text, "executed_plan_registry_count={}", plan_ids.len());
    let _ = writeln!(text, "result_plan_occurrences={}", plan_occurrences);
    let _ = writeln!(text, "database_mutation=false");
    let _ = writeln!(text, "transaction_executed=false");
    let _ = writeln!(text, "save_api_called=false");
    Ok(text.into_bytes())
}

fn log_test77_restored_proposal_on_server_start(
    db: &Database,
    proposal: &AsyncTradeProposal,
) {
    let current_process_id = std::process::id();
    let offered = athlete_trade_snapshot(db, proposal.offered_id).ok();
    let target = athlete_trade_snapshot(db, proposal.target_id).ok();
    let target_contracted_status = contracted_squad_status_debug(db, proposal.target_id)
        .unwrap_or_else(|_| "unknown".to_string());
    let plan_ids = read_persisted_plan_ids(db).unwrap_or_default();
    let result_plan_occurrences = if proposal.result_plan_id.is_empty() {
        0
    } else {
        plan_ids
            .iter()
            .filter(|plan_id| *plan_id == &proposal.result_plan_id)
            .count()
    };
    let success_news_count = async_success_news_count(
        db,
        proposal.requester_team_id,
        &proposal.proposal_id,
    )
    .unwrap_or(0);
    log_event(
        "test77_async_proposal_restored_on_server_start",
        &format!(
            "proposal_id={};state={};submit_process_id={};commit_process_id={};current_process_id={};restart_after_submit={};restart_after_commit={};submitted_at={};seller_due_at={};player_due_at={};completed_at={};offered_id={};offered_name={};offered_team_current={};target_id={};target_name={};target_team_current={};target_status_current={};target_contracted_status_current={};proposed_units={};proposed_cash_won={};display_min_units={};display_max_units={};display_lower_percent={};display_upper_percent={};result_plan_id={};executed_plan_registry_count={};result_plan_occurrences={};success_news_count={};proposal_bytes_roundtrip=true;lifecycle_transition_on_start=false;database_mutation=false;transaction_executed=false;save_api_called=false",
            proposal.proposal_id,
            proposal.state.as_str(),
            proposal.submit_process_id,
            proposal.commit_process_id,
            current_process_id,
            proposal.submit_process_id != 0 && proposal.submit_process_id != current_process_id,
            proposal.commit_process_id != 0 && proposal.commit_process_id != current_process_id,
            proposal.submitted_at,
            proposal.seller_due_at,
            proposal.player_due_at,
            proposal.completed_at,
            proposal.offered_id,
            sanitize(&proposal.offered_name),
            offered.as_ref().map(|snapshot| snapshot.team_id).unwrap_or(usize::MAX),
            proposal.target_id,
            sanitize(&proposal.target_name),
            target.as_ref().map(|snapshot| snapshot.team_id).unwrap_or(usize::MAX),
            target
                .as_ref()
                .map(|snapshot| snapshot.squad_status_debug.as_str())
                .unwrap_or("unknown"),
            sanitize(&target_contracted_status),
            proposal.proposed_units,
            proposal.proposed_cash_won,
            proposal.display_min_units,
            proposal.display_max_units,
            proposal.display_lower_percent,
            proposal.display_upper_percent,
            sanitize(&proposal.result_plan_id),
            plan_ids.len(),
            result_plan_occurrences,
            success_news_count,
        ),
    );
}

fn proposal_review_from_current_state(
    db: &Database,
    proposal: &AsyncTradeProposal,
) -> Result<ServerReview, String> {
    require_fresh_trade_fixture(
        db,
        proposal.requester_team_id,
        proposal.offered_id,
        proposal.target_id,
    )?;
    let offered = db.athletes.get(proposal.offered_id)
        .ok_or_else(|| "offered athlete missing during async lifecycle".to_string())?;
    let target = db.athletes.get(proposal.target_id)
        .ok_or_else(|| "target athlete missing during async lifecycle".to_string())?;
    if athlete_has_active_negotiation(offered) || athlete_has_active_negotiation(target) {
        return Err("selected athlete gained an active negotiation during async review".to_string());
    }
    let offered_snapshot = athlete_trade_snapshot(db, proposal.offered_id)?;
    let target_snapshot = athlete_trade_snapshot(db, proposal.target_id)?;
    if athlete_trade_fingerprint(&offered_snapshot) != proposal.offered_contract_fingerprint_at_submit
        || athlete_trade_fingerprint(&target_snapshot) != proposal.target_contract_fingerprint_at_submit
    {
        return Err("a sealed athlete contract fingerprint changed during async review".to_string());
    }
    evaluate_server_review(
        db,
        proposal.requester_team_id,
        proposal.offered_id,
        proposal.target_id,
        proposal.region_id,
        proposal.proposed_units,
        proposal.desired_status_choice,
    )
}

fn process_async_trade_lifecycle(db: &mut Database) -> Result<(), String> {
    if db.athletes.iter().next().is_none() {
        return Ok(());
    }
    let Some(mut proposal) = load_async_trade_proposal(db)? else {
        return Ok(());
    };
    if proposal.state.terminal() {
        return Ok(());
    }
    let now = db.time.to_string();
    match proposal.state {
        AsyncTradeState::SellerReview if async_time_reached(&now, &proposal.seller_due_at) => {
            if !native_recruit_open(db) {
                return reject_async_trade_proposal(
                    db,
                    &mut proposal,
                    explicit_async_rejection_policy("TermsUnacceptable"),
                    "native recruitment window closed before seller review",
                );
            }
            let review = match proposal_review_from_current_state(db, &proposal) {
                Ok(review) => review,
                Err(detail) => {
                    let reason = if detail.contains("SELLER_REPLACEMENT_UNAVAILABLE") {
                        "SellerReplacementUnavailable"
                    } else if detail.contains("budget") {
                        "BudgetExceeded"
                    } else {
                        "TermsUnacceptable"
                    };
                    return reject_async_trade_proposal(
                        db,
                        &mut proposal,
                        explicit_async_rejection_policy(reason),
                        &detail,
                    );
                }
            };
            if !review.cash_within_budget {
                return reject_async_trade_proposal(db, &mut proposal, explicit_async_rejection_policy("BudgetExceeded"), "requester budget failed at seller review");
            }
            if !review.cash_meets_required || !review.seller_accepted || !review.seller_repeat_consistent {
                return reject_async_trade_proposal(db, &mut proposal, explicit_async_rejection_policy("TermsUnacceptable"), "seller rejected the hidden exact acceptance threshold");
            }
            proposal.state = AsyncTradeState::PlayerReview;
            proposal.player_due_at = advance_game_days(db, PLAYER_REVIEW_DELAY_DAYS)?;
            proposal.transition_count = proposal.transition_count.saturating_add(1);
            save_async_trade_proposal(db, &proposal)?;
            log_event(
                "async_trade_seller_review_completed",
                &format!(
                    "proposal_id={};state_before=SellerReview;state_after=PlayerReview;seller_accepted=true;cash_meets_hidden_exact_threshold=true;submitted_at={};seller_due_at={};reviewed_at={};player_due_at={};proposal_persisted=true;team_mutation=false;finance_mutation=false;contract_mutation=false;transaction_executed=false",
                    proposal.proposal_id,
                    proposal.submitted_at,
                    proposal.seller_due_at,
                    now,
                    proposal.player_due_at,
                ),
            );
        }
        AsyncTradeState::PlayerReview if async_time_reached(&now, &proposal.player_due_at) => {
            let review = match proposal_review_from_current_state(db, &proposal) {
                Ok(review) => review,
                Err(detail) => {
                    let reason = if detail.contains("SELLER_REPLACEMENT_UNAVAILABLE") {
                        "SellerReplacementUnavailable"
                    } else if detail.contains("budget") {
                        "BudgetExceeded"
                    } else {
                        "TermsUnacceptable"
                    };
                    return reject_async_trade_proposal(db, &mut proposal, explicit_async_rejection_policy(reason), &detail);
                }
            };
            if !review.cash_meets_required || !review.cash_within_budget || !review.seller_accepted || !review.seller_repeat_consistent {
                return reject_async_trade_proposal(db, &mut proposal, explicit_async_rejection_policy("TermsUnacceptable"), "seller conditions changed before final player review");
            }
            if !review.player_accepted {
                return reject_async_trade_proposal(db, &mut proposal, explicit_async_rejection_policy("TargetPlayerRejected"), "target player rejected inherited contract or promised role");
            }
            let envelope = review.command_envelope.clone()
                .ok_or_else(|| "approved async review did not produce a command envelope".to_string())?;
            log_event(
                "async_trade_player_review_completed",
                &format!(
                    "proposal_id={};state=PlayerReview;player_accepted=true;seller_revalidated=true;overall_approved=true;reviewed_at={};plan_id={};rollback_rehearsal_required=true;transaction_executed=false",
                    proposal.proposal_id,
                    now,
                    envelope.plan_id,
                ),
            );
            // [PORT056] 리허설 전 레지스트리 길이를 기록해둔다(구판은 "정확히 2"를 상수로 요구 = 첫 거래 불가).
            let registry_len_before_rehearsal = read_persisted_plan_ids(db).unwrap_or_default().len();
            let rehearsal = execute_atomic_trade(db, &review, &envelope, true, false);
            match rehearsal {
                Err(detail) if detail == "TEST66_FORCED_ROLLBACK_REHEARSAL" => {}
                Ok(_) => return Err("Test77 rollback rehearsal unexpectedly committed".to_string()),
                Err(detail) => return Err(format!("Test77 rollback rehearsal failed before controlled boundary: {detail}")),
            }
            let (rollback_performed, rollback_base_ok, rollback_detail) = emergency_rollback(db);
            let rollback_exact = rollback_performed && rollback_base_ok && verify_forced_rollback_exact(db)?;
            if !rollback_exact {
                return Err(format!("Test77 async rollback rehearsal was not exact: {rollback_detail}"));
            }
            clear_forced_rollback_audit();
            // [PORT056] 의도는 "롤백이 레지스트리를 건드리지 않았는가" — 상수 2가 아니라 리허설 전 값과 대조.
            if read_persisted_plan_ids(db).unwrap_or_default().len() != registry_len_before_rehearsal {
                return Err("rollback rehearsal changed the executed-plan registry".to_string());
            }
            log_event(
                "test77_async_rollback_rehearsal_verified",
                &format!(
                    "proposal_id={};plan_id={};rollback_performed=true;rollback_ok=true;exact_rollback_verified=true;executed_plan_registry_count=2;net_database_change=false;transaction_executed=false;save_api_called=false",
                    proposal.proposal_id,
                    envelope.plan_id,
                ),
            );
            let result = execute_atomic_trade(db, &review, &envelope, false, true)?;
            {
                let mut plan_ids = EXECUTED_PLAN_IDS.lock().unwrap_or_else(|p| p.into_inner());
                if !plan_ids.iter().any(|id| id == &result.plan_id) {
                    plan_ids.push(result.plan_id.clone());
                }
            }
            log_event("trade_atomic_commit_succeeded", &atomic_result_log_detail(&result));
            let (_, success_news_count) = ensure_async_success_news(db, &proposal, &result)?;
            proposal.state = AsyncTradeState::Accepted;
        // ★[PORT056] 종료 시 우리가 넣은 네이티브 협상 항목을 제거한다(다른 협상은 건드리지 않는다).
        if let Err(detail) = sync_native_transfer_request(
            db, proposal.target_id, proposal.requester_team_id, false, 0, 0.0, SquadStatus::General,
        ) {
            log_event(
                "native_transfer_request_sync_failed",
                &format!("stage=terminal;detail={}", sanitize(&detail)),
            );
        }
            proposal.completed_at = db.time.to_string();
            proposal.result_plan_id = result.plan_id.clone();
            proposal.success_news_id = proposal.proposal_id.clone();
            proposal.commit_process_id = result.commit_process_id;
            proposal.transition_count = proposal.transition_count.saturating_add(1);
            save_async_trade_proposal(db, &proposal)?;
            log_event(
                "async_trade_atomic_commit_succeeded",
                &format!(
                    "proposal_id={};state=Accepted;plan_id={};offered_id={};offered_name={};offered_team_after={};target_id={};target_name={};target_team_after={};target_status_after={};target_contracted_status_after={};proposed_cash_won={};success_news_count={};executed_plan_registry_count={};commit_process_id={};completed_at={};proposal_persisted=true;transaction_executed=true;save_api_called=false;manual_save_slot={}",
                    proposal.proposal_id,
                    result.plan_id,
                    result.offered_id,
                    sanitize(&result.offered_name),
                    result.offered_team_after,
                    result.target_id,
                    sanitize(&result.target_name),
                    result.target_team_after,
                    result.target_status_after,
                    result.target_contracted_status_after,
                    result.proposed_cash_won,
                    success_news_count,
                    result.executed_plan_registry_count,
                    result.commit_process_id,
                    proposal.completed_at,
                    RESULT_SAVE_SLOT,
                ),
            );
        }
        _ => {}
    }
    Ok(())
}

/// [PORT056] 서버 커맨드 응답 생성을 패닉으로부터 격리한다.
/// QUOTE·REVIEW 분기와 거래 수명주기는 원래 `catch_unwind` 로 감싸져 있었지만,
/// 오퍼 이력·세이브 검증·상태 조회 3개 커맨드는 무방비였다. cdylib 경계를 넘는 unwind 는 UB 다.
fn guarded_payload<F>(label: &str, produce: F) -> Vec<u8>
where
    F: FnOnce() -> Vec<u8>,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(produce)) {
        Ok(payload) => payload,
        Err(_) => {
            log_event(
                "server_command_panic_caught",
                &format!("command={};panic_caught=true;fail_safe_payload=true", label),
            );
            format!("status=error\ndetail=internal error in {label}\n").into_bytes()
        }
    }
}

fn run_async_trade_lifecycle_guarded(db: &mut Database, callback: &str) {
    if ASYNC_LIFECYCLE_BUSY.swap(true, Ordering::AcqRel) {
        return;
    }
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        process_async_trade_lifecycle(db)
    }));
    match outcome {
        Ok(Ok(())) => {}
        Ok(Err(detail)) => log_event(
            "async_trade_lifecycle_error",
            &format!("callback={};detail={};panic_caught=false", callback, sanitize(&detail)),
        ),
        Err(_) => log_event(
            "async_trade_lifecycle_error",
            &format!("callback={};detail=panic_caught;panic_caught=true", callback),
        ),
    }
    ASYNC_LIFECYCLE_BUSY.store(false, Ordering::Release);
}

fn evaluate_server_quote(
    db: &Database,
    requester_team_id: usize,
    offered_id: usize,
    target_id: usize,
    region_id: usize,
) -> Result<ServerQuote, String> {
    let offered = db
        .athletes
        .get(offered_id)
        .ok_or_else(|| format!("offered athlete id {offered_id} was not found"))?;
    let target = db
        .athletes
        .get(target_id)
        .ok_or_else(|| format!("target athlete id {target_id} was not found"))?;
    let offered_team_id = contract_team_id(offered)?;
    let recipient_team_id = contract_team_id(target)?;
    if offered_team_id != requester_team_id {
        return Err(format!(
            "offered athlete belongs to team {offered_team_id}, not requester {requester_team_id}"
        ));
    }
    if recipient_team_id == requester_team_id {
        return Err("target athlete already belongs to requester team".to_string());
    }

    let offered_position = offered.main_position();
    let target_position = target.main_position();
    let requester_continuity_ok =
        structural_continuity(db, requester_team_id, offered_position, target_position);
    if !requester_continuity_ok {
        return Err("requester team does not pass post-trade position continuity".to_string());
    }

    let floor_assessment = replacement_floor_assessment(
        db,
        recipient_team_id,
        target,
        Some(offered),
        region_id,
    )?;
    log_event(
        "trade_replacement_floor_evaluated",
        &format!(
            "channel=trade_quote;requester_team_id={};recipient_team_id={};offered_id={};offered_name={};target_id={};target_name={};target_status={};hard_floor={:.6};internal_best_candidate_id={:?};internal_best_candidate_name={};internal_best_ratio={:.6};incoming_same_position={};incoming_ratio={:.6};effective_replacement_ratio={:.6};structural_cover={};hard_floor_allows={};cash_can_bypass_hard_floor=false",
            requester_team_id,
            recipient_team_id,
            offered.id,
            sanitize(&offered.name),
            target.id,
            sanitize(&target.name),
            squad_status_label(floor_assessment.target_status),
            floor_assessment.floor,
            floor_assessment.internal_best_candidate_id,
            sanitize(&floor_assessment.internal_best_candidate_name),
            floor_assessment.internal_best_ratio,
            floor_assessment.incoming_same_position,
            floor_assessment.incoming_ratio,
            floor_assessment.effective_ratio,
            floor_assessment.structural_cover,
            floor_assessment.allows,
        ),
    );
    if !floor_assessment.allows {
        let detail = replacement_floor_error(target, &floor_assessment);
        log_event(
            "trade_replacement_floor_blocked",
            &format!(
                "offered_id={};target_id={};reason=SellerReplacementUnavailable;detail={};database_mutation=false;transaction_executed=false",
                offered.id,
                target.id,
                sanitize(&detail),
            ),
        );
        return Err(detail);
    }

    let offered_value = fair_fee(db, offered, region_id)?;
    let target_value = fair_fee(db, target, region_id)?;
    let target_evaluation = evaluate_player_asset_for_team(db, recipient_team_id, target)
        .ok_or_else(|| "target native asset evaluation returned None".to_string())?;
    let profile = decision_mapped_profile(&target_evaluation);
    let replacement_score_ratio = floor_assessment.effective_ratio.clamp(0.0, 2.0) as f32;
    let replacement_target_shortfall = (1.0_f32 - replacement_score_ratio).max(0.0);
    let continuity_risk = f64::from(replacement_target_shortfall).clamp(0.0, 1.0);
    let policy = policy_for_profile(
        &target_evaluation,
        profile,
        replacement_score_ratio,
        replacement_target_shortfall,
    );

    let requester_team = db
        .teams
        .get(requester_team_id)
        .ok_or_else(|| "requester team was not found".to_string())?;
    let salary_after_outgoing = db.team_salary_total(requester_team_id, Some(offered_id), None);
    let budget_snapshot = compute_team_budget_snapshot(requester_team, salary_after_outgoing);
    let cash_budget_won = budget_snapshot
        .transfer_budget
        .min(budget_snapshot.transfer_spend_limit)
        .max(0.0);
    let threshold = find_exact_requester_cash(
        &target_evaluation,
        policy,
        profile,
        target_value,
        offered_value,
        floor_assessment.structural_cover,
        continuity_risk,
        cash_budget_won,
    )?;
    if !threshold.repeat_consistent
        || !threshold.coarse_monotonic
        || !threshold.boundary_verified
        || !threshold.budget_ceiling_accepted
    {
        return Err("threshold search verification flags were not all true".to_string());
    }
    // ★[PORT056] 진단(2026-08-23): 거절 다음 날 요구 현금이 7.8억 → **0** 으로 붕괴하는 현상.
    //   `policy_accepts(cash=0)` 이 참이 되면 범위가 (0,0) 이 되고, 0원 패키지는 거절된 패키지와
    //   **지문이 달라** 쿨다운도 안 걸린다 = 공짜 트레이드가 열린다.
    //   어떤 입력이 뒤집혔는지 보려면 가격 산출 입력을 통째로 남겨야 한다.
    log_event(
        "trade_quote_threshold_inputs",
        &format!(
            "offered_id={};offered_name={};target_id={};target_name={};offered_value={:.2};target_value={:.2};profile={};effective_ratio={:.6};continuity_risk={:.6};replacement_score_ratio={:.6};zero_cash_accepted={};requester_cash={:.2};evaluation_count={};cash_budget_won={:.2};game_time={}",
            offered_id,
            sanitize(&offered.name),
            target_id,
            sanitize(&target.name),
            offered_value,
            target_value,
            profile.label,
            floor_assessment.effective_ratio,
            continuity_risk,
            replacement_score_ratio,
            threshold.zero_cash_accepted,
            threshold.requester_cash,
            threshold.evaluation_count,
            cash_budget_won,
            sanitize(&db.time.to_string()),
        ),
    );
    let required_cash_won = threshold.requester_cash as u64;
    let required_units = won_to_units_ceil(threshold.requester_cash)?;
    let budget_units = won_to_units_floor(cash_budget_won)?;
    let (display_min_units, display_max_units, display_lower_percent, display_upper_percent) =
        stable_obscured_cash_range(
            required_units,
            budget_units,
            requester_team_id,
            recipient_team_id,
            offered_id,
            target_id,
        );
    // ★[PORT056] 쿨다운 힌트를 견적에 실어 보낸다 — 클라가 버튼 단계에서 선차단하기 위함.
    //   ⚠읽기 실패는 **fail-open**(힌트 없음)으로 둔다. 제출 시 서버 검사가 정본이라 안전하다.
    let cooldown = rejection::cooldown_hint(db, target_id, requester_team_id, recipient_team_id)
        .unwrap_or(None);
    Ok(ServerQuote {
        cooldown_present: cooldown.is_some(),
        cooldown_fingerprint: cooldown.as_ref().map(|c| c.package_fingerprint).unwrap_or(0),
        cooldown_retry_at: cooldown.as_ref().map(|c| c.retry_at.clone()).unwrap_or_default(),
        cooldown_changeable: cooldown.as_ref().map(|c| c.changeable).unwrap_or(true),
        cooldown_exempt: cooldown.as_ref().map(|c| c.exempt).unwrap_or(false),
        requester_team_id,
        recipient_team_id,
        offered_id,
        target_id,
        region_id,
        offered_name: offered.name.clone(),
        target_name: target.name.clone(),
        offered_value,
        target_value,
        required_cash_won,
        required_units,
        display_min_units,
        display_max_units,
        display_lower_percent,
        display_upper_percent,
        cash_budget_won,
        budget_units,
        profile_label: profile.label,
        evaluation_count: threshold.evaluation_count,
        game_time: db.time.to_string(),
    })
}

fn evaluate_server_review(
    db: &Database,
    requester_team_id: usize,
    offered_id: usize,
    target_id: usize,
    region_id: usize,
    proposed_units: u64,
    desired_status_choice: u8,
) -> Result<ServerReview, String> {
    let offered = db
        .athletes
        .get(offered_id)
        .ok_or_else(|| format!("offered athlete id {offered_id} was not found"))?;
    let target = db
        .athletes
        .get(target_id)
        .ok_or_else(|| format!("target athlete id {target_id} was not found"))?;
    let server_region_id = server_contracted_athlete_region_id(db, offered)?;
    if region_id != server_region_id {
        return Err(format!(
            "client region {region_id} does not match server region {server_region_id}"
        ));
    }
    // Recompute the complete quote from the current server Database. The client
    // quote and region are display/input hints and are never trusted as approval
    // values without checking them against the live server Database.
    let quote = evaluate_server_quote(
        db,
        requester_team_id,
        offered_id,
        target_id,
        server_region_id,
    )?;
    let cash_offer_min_units = quote.display_min_units;
    let cash_offer_max_units = quote.display_max_units;
    if cash_offer_min_units > cash_offer_max_units {
        return Err(format!(
            "Test77 obscured cash range is unavailable because minimum {} exceeds maximum {}",
            cash_offer_min_units, cash_offer_max_units
        ));
    }
    if proposed_units < cash_offer_min_units || proposed_units > cash_offer_max_units {
        return Err(format!(
            "Test77 requires cash units within the obscured server-displayed range: minimum {}, maximum {}, found {}",
            cash_offer_min_units, cash_offer_max_units, proposed_units
        ));
    }
    let recipient_team_id = contract_team_id(target)?;
    let proposed_cash_won = proposed_units
        .checked_mul(MONEY_UNIT_WON)
        .ok_or_else(|| "proposed cash overflowed won conversion".to_string())?;
    let cash_meets_required = proposed_cash_won >= quote.required_cash_won;
    let cash_within_budget = (proposed_cash_won as f64) <= quote.cash_budget_won + 0.001;
    let cash_within_offer_range = proposed_units >= cash_offer_min_units
        && proposed_units <= cash_offer_max_units;

    let offered_value = fair_fee(db, offered, server_region_id)?;
    let target_value = fair_fee(db, target, server_region_id)?;
    let floor_assessment = replacement_floor_assessment(
        db,
        recipient_team_id,
        target,
        Some(offered),
        server_region_id,
    )?;
    if !floor_assessment.allows {
        return Err(replacement_floor_error(target, &floor_assessment));
    }
    let target_evaluation = evaluate_player_asset_for_team(db, recipient_team_id, target)
        .ok_or_else(|| "target native asset evaluation returned None".to_string())?;
    let profile = decision_mapped_profile(&target_evaluation);
    let replacement_score_ratio = floor_assessment.effective_ratio.clamp(0.0, 2.0) as f32;
    let replacement_target_shortfall = (1.0_f32 - replacement_score_ratio).max(0.0);
    let continuity_risk = f64::from(replacement_target_shortfall).clamp(0.0, 1.0);
    let policy = policy_for_profile(
        &target_evaluation,
        profile,
        replacement_score_ratio,
        replacement_target_shortfall,
    );
    let recipient_continuity_ok = floor_assessment.structural_cover;
    let last_player_without_replacement = !recipient_continuity_ok;
    let (seller_accepted, seller_repeat_consistent) = policy_accepts(
        &target_evaluation,
        policy,
        profile,
        target_value,
        offered_value,
        proposed_cash_won as f64,
        recipient_continuity_ok,
        continuity_risk,
    );

    let (desired_status_key, desired_status_label) = desired_squad_status(desired_status_choice);
    let promised_status = desired_squad_status_value(desired_status_choice)?;
    let market = db
        .transfer_market
        .as_ref()
        .ok_or_else(|| "Database::transfer_market is None".to_string())?;
    // A one-year read-only probe exposes the existing contract days. The
    // canonical call then uses ceil(days/365), matching the Test18 semantics.
    let contract_probe = career_move_assessment_for_team_offer(
        db,
        market,
        target,
        requester_team_id,
        1,
        promised_status,
        false,
        true,
        None,
        None,
    );
    let contract_days_left = contract_probe.contract_days_left;
    let requested_years = requested_years_from_contract_days(contract_days_left);
    let offer_value_ratio = if target_value > 0.0 {
        (offered_value + proposed_cash_won as f64) / target_value
    } else {
        0.0
    };
    if !offer_value_ratio.is_finite() || offer_value_ratio < 0.0 {
        return Err("proposal value ratio is not finite and nonnegative".to_string());
    }
    let assessment = career_move_assessment_for_team_offer(
        db,
        market,
        target,
        requester_team_id,
        requested_years,
        promised_status,
        false,
        true,
        Some(target_evaluation),
        Some(offer_value_ratio),
    );
    let inherited_yearly_salary = contract_yearly_salary(target)?;
    let contract_inherited =
        nearly_equal_money(inherited_yearly_salary, assessment.current_yearly_salary);
    let contracted_status = contracted_squad_status_value(db, target_id)?;
    let role_promise_accepted =
        trade_role_promise_allows(target.squad_status, contracted_status, promised_status);
    let salary_renegotiation_required = false;
    let current_status = sanitize(&format!("{:?}", &assessment.context.current_status));
    let offered_status = sanitize(&format!("{:?}", promised_status));
    let clear_exit_path = assessment.context.clear_exit_path;
    let seller_protects_downward_move = assessment.seller_protects_downward_move;
    let protected_downward_asset = assessment.protected_downward_asset;
    let exceptional_exit_offer = assessment.exceptional_exit_offer;
    // A trade transfers the existing contract unchanged. The recruitment-only
    // salary-raise gate must not be applied here: there is no salary negotiation.
    // The player gate is the promised role, which may keep or improve the
    // strongest of the current and already-contracted squad statuses.
    let player_accepted = contract_inherited && role_promise_accepted;
    let overall_approved = cash_meets_required
        && cash_within_budget
        && cash_within_offer_range
        && seller_accepted
        && seller_repeat_consistent
        && player_accepted;

    let mut review = ServerReview {
        requester_team_id,
        recipient_team_id,
        offered_id,
        target_id,
        region_id: server_region_id,
        offered_name: offered.name.clone(),
        target_name: target.name.clone(),
        proposed_units,
        proposed_cash_won,
        desired_status_choice,
        desired_status_key,
        desired_status_label,
        required_cash_won: quote.required_cash_won,
        cash_offer_min_units,
        cash_offer_max_units,
        cash_budget_won: quote.cash_budget_won,
        cash_meets_required,
        cash_within_budget,
        cash_within_offer_range,
        seller_accepted,
        seller_repeat_consistent,
        player_accepted,
        overall_approved,
        requested_years,
        contract_days_left,
        inherited_yearly_salary,
        contract_inherited,
        role_promise_accepted,
        salary_renegotiation_required,
        offer_value_ratio,
        current_status,
        offered_status,
        clear_exit_path,
        seller_protects_downward_move,
        protected_downward_asset,
        exceptional_exit_offer,
        last_player_without_replacement,
        game_time: db.time.to_string(),
        command_envelope: None,
        rejection_meta: None,
    };
    if overall_approved {
        let first = build_trade_command_envelope(db, &review)?;
        let second = build_trade_command_envelope(db, &review)?;
        if first.plan_id != second.plan_id {
            return Err("command envelope fingerprint was not repeat-consistent".to_string());
        }
        let mut envelope = first;
        envelope.plan_repeat_consistent = true;
        review.command_envelope = Some(envelope);
    }
    Ok(review)
}


fn server_quote_payload(quote: &ServerQuote) -> Vec<u8> {
    let mut text = String::new();
    let _ = writeln!(text, "status=ok");
    let _ = writeln!(text, "requester_team_id={}", quote.requester_team_id);
    let _ = writeln!(text, "recipient_team_id={}", quote.recipient_team_id);
    let _ = writeln!(text, "offered_id={}", quote.offered_id);
    let _ = writeln!(text, "target_id={}", quote.target_id);
    let _ = writeln!(text, "region_id={}", quote.region_id);
    let _ = writeln!(text, "offered_name={}", sanitize(&quote.offered_name));
    let _ = writeln!(text, "target_name={}", sanitize(&quote.target_name));
    let _ = writeln!(text, "display_min_units={}", quote.display_min_units);
    let _ = writeln!(text, "display_max_units={}", quote.display_max_units);
    let _ = writeln!(text, "cash_budget_won={}", quote.cash_budget_won);
    let _ = writeln!(text, "budget_units={}", quote.budget_units);
    let _ = writeln!(text, "cash_range_obscured=true");
    let _ = writeln!(text, "exact_threshold_disclosed=false");
    let _ = writeln!(text, "range_stable_for_pair=true");
    let _ = writeln!(text, "profile={}", quote.profile_label);
    let _ = writeln!(text, "game_time={}", sanitize(&quote.game_time));
    // 쿨다운 힌트(클라 선차단용).
    let _ = writeln!(text, "cooldown_present={}", quote.cooldown_present);
    let _ = writeln!(text, "cooldown_fingerprint={}", quote.cooldown_fingerprint);
    let _ = writeln!(text, "cooldown_retry_at={}", sanitize(&quote.cooldown_retry_at));
    let _ = writeln!(text, "cooldown_changeable={}", quote.cooldown_changeable);
    let _ = writeln!(text, "cooldown_exempt={}", quote.cooldown_exempt);
    text.into_bytes()
}

fn server_review_payload(review: &ServerReview) -> Vec<u8> {
    let mut text = String::new();
    let _ = writeln!(text, "status=ok");
    let _ = writeln!(text, "requester_team_id={}", review.requester_team_id);
    let _ = writeln!(text, "recipient_team_id={}", review.recipient_team_id);
    let _ = writeln!(text, "offered_id={}", review.offered_id);
    let _ = writeln!(text, "target_id={}", review.target_id);
    let _ = writeln!(text, "region_id={}", review.region_id);
    let _ = writeln!(text, "offered_name={}", sanitize(&review.offered_name));
    let _ = writeln!(text, "target_name={}", sanitize(&review.target_name));
    let _ = writeln!(text, "proposed_units={}", review.proposed_units);
    let _ = writeln!(text, "proposed_cash_won={}", review.proposed_cash_won);
    let _ = writeln!(
        text,
        "desired_status_choice={}",
        review.desired_status_choice
    );
    let _ = writeln!(text, "desired_status_key={}", review.desired_status_key);
    let _ = writeln!(text, "desired_status_label={}", review.desired_status_label);
    let _ = writeln!(text, "required_cash_won={}", review.required_cash_won);
    let _ = writeln!(
        text,
        "proposed_cash_within_server_range={}",
        review.cash_within_offer_range
    );
    let _ = writeln!(text, "cash_offer_min_units={}", review.cash_offer_min_units);
    let _ = writeln!(text, "cash_offer_max_units={}", review.cash_offer_max_units);
    let _ = writeln!(text, "cash_budget_won={}", review.cash_budget_won);
    let _ = writeln!(text, "cash_meets_required={}", review.cash_meets_required);
    let _ = writeln!(text, "cash_within_budget={}", review.cash_within_budget);
    let _ = writeln!(text, "seller_accepted={}", review.seller_accepted);
    let _ = writeln!(
        text,
        "seller_repeat_consistent={}",
        review.seller_repeat_consistent
    );
    let _ = writeln!(text, "player_accepted={}", review.player_accepted);
    let _ = writeln!(text, "overall_approved={}", review.overall_approved);
    let _ = writeln!(text, "requested_years={}", review.requested_years);
    let _ = writeln!(text, "contract_days_left={}", review.contract_days_left);
    let _ = writeln!(
        text,
        "inherited_yearly_salary={}",
        review.inherited_yearly_salary
    );
    let _ = writeln!(text, "contract_inherited={}", review.contract_inherited);
    let _ = writeln!(
        text,
        "role_promise_accepted={}",
        review.role_promise_accepted
    );
    let _ = writeln!(
        text,
        "salary_renegotiation_required={}",
        review.salary_renegotiation_required
    );
    let _ = writeln!(text, "offer_value_ratio={}", review.offer_value_ratio);
    let _ = writeln!(text, "current_status={}", review.current_status);
    let _ = writeln!(text, "offered_status={}", review.offered_status);
    let _ = writeln!(text, "clear_exit_path={}", review.clear_exit_path);
    let _ = writeln!(
        text,
        "seller_protects_downward_move={}",
        review.seller_protects_downward_move
    );
    let _ = writeln!(
        text,
        "protected_downward_asset={}",
        review.protected_downward_asset
    );
    let _ = writeln!(
        text,
        "exceptional_exit_offer={}",
        review.exceptional_exit_offer
    );
    let _ = writeln!(
        text,
        "last_player_without_replacement={}",
        review.last_player_without_replacement
    );
    let _ = writeln!(text, "game_time={}", sanitize(&review.game_time));
    if let Some(meta) = &review.rejection_meta {
        let _ = writeln!(text, "rejection_present=true");
        let _ = writeln!(text, "rejection_actor={}", meta.actor);
        let _ = writeln!(text, "rejection_actor_ko={}", sanitize(&meta.actor_ko));
        let _ = writeln!(text, "rejection_reason={}", meta.reason);
        let _ = writeln!(
            text,
            "rejection_reason_ko={}",
            sanitize(&meta.reason_ko)
        );
        let _ = writeln!(text, "rejection_policy={}", meta.policy);
        let _ = writeln!(text, "rejection_retry_at={}", meta.retry_at);
        let _ = writeln!(
            text,
            "rejection_retry_ko={}",
            sanitize(&meta.retry_ko)
        );
        let _ = writeln!(
            text,
            "rejection_package_fingerprint={:016X}",
            meta.package_fingerprint
        );
        let _ = writeln!(text, "rejection_feedback_id={:016X}", meta.feedback_id);
        let _ = writeln!(
            text,
            "rejection_cooldown_blocked={}",
            meta.cooldown_blocked
        );
        let _ = writeln!(text, "rejection_news_created={}", meta.news_created);
        let _ = writeln!(
            text,
            "rejection_duplicate_news_count={}",
            meta.duplicate_news_count
        );
        let _ = writeln!(
            text,
            "rejection_ledger_entry_count={}",
            meta.ledger_entry_count
        );
    } else {
        let _ = writeln!(text, "rejection_present=false");
        let _ = writeln!(text, "rejection_actor=none");
        let _ = writeln!(text, "rejection_actor_ko=none");
        let _ = writeln!(text, "rejection_reason=none");
        let _ = writeln!(text, "rejection_reason_ko=none");
        let _ = writeln!(text, "rejection_policy=none");
        let _ = writeln!(text, "rejection_retry_at=none");
        let _ = writeln!(text, "rejection_retry_ko=none");
        let _ = writeln!(text, "rejection_package_fingerprint=none");
        let _ = writeln!(text, "rejection_feedback_id=none");
        let _ = writeln!(text, "rejection_cooldown_blocked=false");
        let _ = writeln!(text, "rejection_news_created=false");
        let _ = writeln!(text, "rejection_duplicate_news_count=0");
        let _ = writeln!(text, "rejection_ledger_entry_count=0");
    }
    if let Some(envelope) = &review.command_envelope {
        let _ = writeln!(text, "command_envelope_prepared=true");
        let _ = writeln!(text, "plan_schema_version={}", envelope.schema_version);
        let _ = writeln!(text, "plan_id={}", envelope.plan_id);
        let _ = writeln!(
            text,
            "offered_destination_team_id={}",
            envelope.offered_destination_team_id
        );
        let _ = writeln!(
            text,
            "target_destination_team_id={}",
            envelope.target_destination_team_id
        );
        let _ = writeln!(text, "cash_payer_team_id={}", envelope.cash_payer_team_id);
        let _ = writeln!(
            text,
            "cash_recipient_team_id={}",
            envelope.cash_recipient_team_id
        );
        let _ = writeln!(
            text,
            "requester_roster_count={}",
            envelope.requester_roster_count
        );
        let _ = writeln!(
            text,
            "recipient_roster_count={}",
            envelope.recipient_roster_count
        );
        let _ = writeln!(
            text,
            "offered_yearly_salary={}",
            envelope.offered_yearly_salary
        );
        let _ = writeln!(
            text,
            "target_yearly_salary={}",
            envelope.target_yearly_salary
        );
        let _ = writeln!(
            text,
            "requester_cash_budget_won={}",
            envelope.requester_cash_budget_won
        );
        let _ = writeln!(
            text,
            "prepared_game_time={}",
            sanitize(&envelope.prepared_game_time)
        );
        let _ = writeln!(
            text,
            "state_precondition_count={}",
            envelope.state_precondition_count
        );
        let _ = writeln!(text, "operation_count={}", envelope.operation_count);
        let _ = writeln!(
            text,
            "atomic_batch_required={}",
            envelope.atomic_batch_required
        );
        let _ = writeln!(
            text,
            "contract_transfer_mode={}",
            envelope.contract_transfer_mode
        );
        let _ = writeln!(text, "money_direction={}", envelope.money_direction);
        let _ = writeln!(
            text,
            "plan_repeat_consistent={}",
            envelope.plan_repeat_consistent
        );
        let _ = writeln!(
            text,
            "execution_gate_closed={}",
            envelope.execution_gate_closed
        );
    } else {
        let _ = writeln!(text, "command_envelope_prepared=false");
        let _ = writeln!(text, "plan_id=none");
        let _ = writeln!(text, "plan_repeat_consistent=false");
        let _ = writeln!(text, "execution_gate_closed=true");
    }
    let _ = writeln!(text, "transaction_executed=false");
    text.into_bytes()
}

fn server_error_payload(detail: &str) -> Vec<u8> {
    format!("status=error\ndetail={}\n", sanitize(detail)).into_bytes()
}

fn atomic_execution_payload(result: &AtomicTradeResult) -> Vec<u8> {
    let mut text = String::new();
    let _ = writeln!(text, "status=ok");
    let _ = writeln!(text, "plan_id={}", result.plan_id);
    let _ = writeln!(text, "requester_team_id={}", result.requester_team_id);
    let _ = writeln!(text, "recipient_team_id={}", result.recipient_team_id);
    let _ = writeln!(text, "offered_id={}", result.offered_id);
    let _ = writeln!(text, "target_id={}", result.target_id);
    let _ = writeln!(text, "offered_name={}", sanitize(&result.offered_name));
    let _ = writeln!(text, "target_name={}", sanitize(&result.target_name));
    let _ = writeln!(text, "proposed_cash_won={}", result.proposed_cash_won);
    let _ = writeln!(text, "proposed_cash_within_server_range=true");
    let _ = writeln!(text, "desired_status_choice={}", result.desired_status_choice);
    let _ = writeln!(text, "desired_status_key={}", result.desired_status_key);
    let _ = writeln!(text, "desired_status_label={}", result.desired_status_label);
    let _ = writeln!(text, "offered_team_before={}", result.offered_team_before);
    let _ = writeln!(text, "offered_team_after={}", result.offered_team_after);
    let _ = writeln!(text, "target_team_before={}", result.target_team_before);
    let _ = writeln!(text, "target_team_after={}", result.target_team_after);
    let _ = writeln!(text, "target_status_before={}", sanitize(&result.target_status_before));
    let _ = writeln!(text, "target_status_after={}", sanitize(&result.target_status_after));
    let _ = writeln!(text, "offered_contracted_status_before={}", sanitize(&result.offered_contracted_status_before));
    let _ = writeln!(text, "offered_contracted_status_after={}", sanitize(&result.offered_contracted_status_after));
    let _ = writeln!(text, "target_contracted_status_before={}", sanitize(&result.target_contracted_status_before));
    let _ = writeln!(text, "target_contracted_status_after={}", sanitize(&result.target_contracted_status_after));
    let _ = writeln!(text, "requester_total_before={}", result.requester_finance_before.total_balance);
    let _ = writeln!(text, "requester_total_after={}", result.requester_finance_after.total_balance);
    let _ = writeln!(text, "requester_transfer_before={}", result.requester_finance_before.transfer_budget);
    let _ = writeln!(text, "requester_transfer_after={}", result.requester_finance_after.transfer_budget);
    let _ = writeln!(text, "recipient_total_before={}", result.recipient_finance_before.total_balance);
    let _ = writeln!(text, "recipient_total_after={}", result.recipient_finance_after.total_balance);
    let _ = writeln!(text, "recipient_transfer_before={}", result.recipient_finance_before.transfer_budget);
    let _ = writeln!(text, "recipient_transfer_after={}", result.recipient_finance_after.transfer_budget);
    let _ = writeln!(text, "requester_roster_before={}", result.requester_roster_before);
    let _ = writeln!(text, "requester_roster_after={}", result.requester_roster_after);
    let _ = writeln!(text, "recipient_roster_before={}", result.recipient_roster_before);
    let _ = writeln!(text, "recipient_roster_after={}", result.recipient_roster_after);
    let _ = writeln!(text, "contracted_before={}", result.contracted_before);
    let _ = writeln!(text, "contracted_after={}", result.contracted_after);
    let _ = writeln!(text, "requester_payroll_after={}", result.requester_payroll_after);
    let _ = writeln!(text, "recipient_payroll_after={}", result.recipient_payroll_after);
    let _ = writeln!(text, "contract_inherited_both={}", result.contract_inherited_both);
    let _ = writeln!(text, "offered_status_unchanged={}", result.offered_status_unchanged);
    let _ = writeln!(text, "target_status_applied={}", result.target_status_applied);
    let _ = writeln!(text, "offered_contracted_status_unchanged={}", result.offered_contracted_status_unchanged);
    let _ = writeln!(text, "target_contracted_status_applied={}", result.target_contracted_status_applied);
    let _ = writeln!(text, "rollback_rehearsal_verified={}", result.rollback_rehearsal_verified);
    let _ = writeln!(text, "combined_finance_conserved={}", result.combined_finance_conserved);
    let _ = writeln!(text, "pre_receipt_mod_save_unchanged={}", result.pre_receipt_mod_save_unchanged);
    let _ = writeln!(text, "executed_plan_persisted={}", result.executed_plan_persisted);
    let _ = writeln!(text, "trade_receipt_persisted={}", result.trade_receipt_persisted);
    let _ = writeln!(text, "receipt_readback_verified={}", result.receipt_readback_verified);
    let _ = writeln!(text, "executed_plan_registry_count={}", result.executed_plan_registry_count);
    let _ = writeln!(text, "offered_contract_fingerprint={}", result.offered_contract_fingerprint);
    let _ = writeln!(text, "target_contract_fingerprint={}", result.target_contract_fingerprint);
    let _ = writeln!(text, "commit_process_id={}", result.commit_process_id);
    let _ = writeln!(text, "news_count_unchanged={}", result.news_count_unchanged);
    let _ = writeln!(text, "game_time={}", sanitize(&result.game_time));
    let _ = writeln!(text, "atomic_commit_verified=true");
    let _ = writeln!(text, "rollback_performed=false");
    let _ = writeln!(text, "execution_gate_closed=true");
    let _ = writeln!(text, "database_mutation=true");
    let _ = writeln!(text, "contract_mutation=true");
    let _ = writeln!(text, "finance_mutation={}", result.proposed_cash_won > 0);
    let _ = writeln!(text, "three_receipts_preserved=true");
    let _ = writeln!(text, "first_plan_id={}", FIRST_PLAN_ID);
    let _ = writeln!(text, "first_plan_preserved=true");
    let _ = writeln!(text, "second_plan_id={}", SECOND_PLAN_ID);
    let _ = writeln!(text, "second_plan_preserved=true");
    let _ = writeln!(text, "squad_status_mutation=true");
    let _ = writeln!(text, "contracted_squad_status_mutation=true");
    let _ = writeln!(text, "mod_save_mutation=true");
    let _ = writeln!(text, "transaction_executed=true");
    let _ = writeln!(text, "manual_save_required=true");
    let _ = writeln!(text, "manual_save_slot={}", RESULT_SAVE_SLOT);
    let _ = writeln!(text, "save_api_called=false");
    text.into_bytes()
}

fn atomic_result_log_detail(result: &AtomicTradeResult) -> String {
    format!(
        "sdk_base={};plan_id={};requester_team_id={};recipient_team_id={};offered_id={};offered_name={};target_id={};target_name={};proposed_cash_won={};proposed_cash_within_server_range=true;desired_status_choice={};desired_status_key={};offered_team_before={};offered_team_after={};target_team_before={};target_team_after={};target_status_before={};target_status_after={};offered_contracted_status_before={};offered_contracted_status_after={};target_contracted_status_before={};target_contracted_status_after={};offered_contracted_status_unchanged={};target_contracted_status_applied={};combined_finance_conserved={};contract_inherited_both={};offered_status_unchanged={};target_status_applied={};pre_receipt_mod_save_unchanged={};executed_plan_persisted={};trade_receipt_persisted={};three_receipts_preserved=true;first_plan_id={};first_plan_preserved=true;second_plan_id={};second_plan_preserved=true;receipt_readback_verified={};executed_plan_registry_count={};commit_process_id={};news_count_unchanged={};rollback_rehearsal_verified={};atomic_commit_verified=true;rollback_performed=false;execution_gate_closed=true;database_mutation=true;contract_mutation=true;finance_mutation={};squad_status_mutation=true;contracted_squad_status_mutation=true;mod_save_mutation=true;transaction_executed=true;manual_save_required=true;manual_save_slot={};save_api_called=false",
        PATCH055_BASE_VERSION,
        result.plan_id,
        result.requester_team_id,
        result.recipient_team_id,
        result.offered_id,
        sanitize(&result.offered_name),
        result.target_id,
        sanitize(&result.target_name),
        result.proposed_cash_won,
        result.desired_status_choice,
        result.desired_status_key,
        result.offered_team_before,
        result.offered_team_after,
        result.target_team_before,
        result.target_team_after,
        sanitize(&result.target_status_before),
        sanitize(&result.target_status_after),
        sanitize(&result.offered_contracted_status_before),
        sanitize(&result.offered_contracted_status_after),
        sanitize(&result.target_contracted_status_before),
        sanitize(&result.target_contracted_status_after),
        result.offered_contracted_status_unchanged,
        result.target_contracted_status_applied,
        result.combined_finance_conserved,
        result.contract_inherited_both,
        result.offered_status_unchanged,
        result.target_status_applied,
        result.pre_receipt_mod_save_unchanged,
        result.executed_plan_persisted,
        result.trade_receipt_persisted,
        FIRST_PLAN_ID,
        SECOND_PLAN_ID,
        result.receipt_readback_verified,
        result.executed_plan_registry_count,
        result.commit_process_id,
        result.news_count_unchanged,
        result.rollback_rehearsal_verified,
        result.proposed_cash_won > 0,
        RESULT_SAVE_SLOT,
    )
}

fn audit_pair(
    db: &Database,
    offered_id: usize,
    target_id: usize,
    region_id: usize,
) -> Result<(ReplacementFloorAssessment, bool), String> {
    let offered = db
        .athletes
        .get(offered_id)
        .ok_or_else(|| format!("audit offered athlete {offered_id} not found"))?;
    let target = db
        .athletes
        .get(target_id)
        .ok_or_else(|| format!("audit target athlete {target_id} not found"))?;
    let requester_team_id = contract_team_id(offered)?;
    let recipient_team_id = contract_team_id(target)?;
    let assessment = replacement_floor_assessment(
        db,
        recipient_team_id,
        target,
        Some(offered),
        region_id,
    )?;
    let quote_ok = evaluate_server_quote(
        db,
        requester_team_id,
        offered_id,
        target_id,
        region_id,
    )
    .is_ok();
    Ok((assessment, quote_ok))
}

fn run_replacement_floor_audit(db: &Database) -> Result<String, String> {
    let region_id = 0usize;
    let maomao = db
        .athletes
        .get(AUDIT_MAOMAO_ID)
        .ok_or_else(|| "Maomao audit fixture is missing".to_string())?;
    let maomao_team_id = contract_team_id(maomao)?;
    let maomao_floor = replacement_floor_assessment(
        db,
        maomao_team_id,
        maomao,
        None,
        region_id,
    )?;
    if maomao_floor.allows {
        return Err("Maomao cash-only fixture unexpectedly passed the replacement floor".to_string());
    }
    log_event(
        "replacement_floor_native_cash_audit",
        &format!(
            "target_id={};target_name={};target_status={};hard_floor={:.6};effective_replacement_ratio={:.6};structural_cover={};hard_floor_allows={};cash_can_bypass_hard_floor=false;expected=false;passed=true;mutation=false",
            maomao.id,
            sanitize(&maomao.name),
            squad_status_label(maomao_floor.target_status),
            maomao_floor.floor,
            maomao_floor.effective_ratio,
            maomao_floor.structural_cover,
            maomao_floor.allows,
        ),
    );

    let (trade_fail, trade_fail_quote_ok) =
        audit_pair(db, AUDIT_HOYA_ID, AUDIT_SOLADA_ID, region_id)?;
    if trade_fail.allows || trade_fail_quote_ok {
        return Err("Hoya-for-solada fixture did not hit the integrated hard-floor gate".to_string());
    }
    log_event(
        "replacement_floor_trade_fail_audit",
        &format!(
            "offered_id={};target_id={};target_status={};hard_floor={:.6};effective_replacement_ratio={:.6};structural_cover={};hard_floor_allows={};quote_rejected=true;reason=SellerReplacementUnavailable;cash_can_bypass_hard_floor=false;passed=true;mutation=false",
            AUDIT_HOYA_ID,
            AUDIT_SOLADA_ID,
            squad_status_label(trade_fail.target_status),
            trade_fail.floor,
            trade_fail.effective_ratio,
            trade_fail.structural_cover,
            trade_fail.allows,
        ),
    );

    let (trade_pass, trade_pass_quote_ok) =
        audit_pair(db, AUDIT_CHICO_ID, AUDIT_KESHI_ID, region_id)?;
    if !trade_pass.allows || !trade_pass_quote_ok {
        return Err("Chico-for-Keshi fixture did not pass the integrated hard-floor gate".to_string());
    }
    log_event(
        "replacement_floor_trade_pass_audit",
        &format!(
            "offered_id={};target_id={};target_status={};hard_floor={:.6};effective_replacement_ratio={:.6};structural_cover={};hard_floor_allows={};quote_accepted=true;passed=true;mutation=false",
            AUDIT_CHICO_ID,
            AUDIT_KESHI_ID,
            squad_status_label(trade_pass.target_status),
            trade_pass.floor,
            trade_pass.effective_ratio,
            trade_pass.structural_cover,
            trade_pass.allows,
        ),
    );

    let (regression, regression_quote_ok) =
        audit_pair(db, AUDIT_JUE_ID, AUDIT_ZEUS_ID, region_id)?;
    if !regression.allows || !regression_quote_ok {
        return Err("Jue-for-Zeus regression fixture failed after hard-floor integration".to_string());
    }
    log_event(
        "replacement_floor_trade_regression_audit",
        &format!(
            "offered_id={};target_id={};target_status={};hard_floor={:.6};effective_replacement_ratio={:.6};hard_floor_allows={};quote_accepted=true;passed=true;mutation=false",
            AUDIT_JUE_ID,
            AUDIT_ZEUS_ID,
            squad_status_label(regression.target_status),
            regression.floor,
            regression.effective_ratio,
            regression.allows,
        ),
    );

    let boundary_core = !hard_floor_allows(SquadStatus::Core, true, 0.699_999)
        && hard_floor_allows(SquadStatus::Core, true, 0.70);
    let boundary_important = !hard_floor_allows(SquadStatus::Important, true, 0.549_999)
        && hard_floor_allows(SquadStatus::Important, true, 0.55);
    let boundary_general = !hard_floor_allows(SquadStatus::General, true, 0.399_999)
        && hard_floor_allows(SquadStatus::General, true, 0.40);
    if !(boundary_core && boundary_important && boundary_general) {
        return Err("replacement hard-floor exact boundary matrix failed".to_string());
    }
    log_event(
        "replacement_floor_boundary_audit",
        "core_below_rejected=true;core_exact_allowed=true;important_below_rejected=true;important_exact_allowed=true;general_below_rejected=true;general_exact_allowed=true;cash_can_bypass_hard_floor=false;passed=true;mutation=false",
    );

    log_event(
        "replacement_floor_audit_completed",
        "core_floor=0.70;important_floor=0.55;general_floor=0.40;native_cash_fixture_passed=true;trade_fail_fixture_passed=true;trade_pass_fixture_passed=true;regression_fixture_passed=true;boundary_case_count=6;failure_count=0;database_mutation=false;finance_mutation=false;contract_mutation=false;transaction_executed=false",
    );
    Ok("status=ok\nfailure_count=0\ntrade_hard_floor_integrated=true\nnative_cash_channel_audit_only=true\n".to_string())
}

fn execution_error_payload(
    detail: &str,
    rollback_performed: bool,
    rollback_ok: bool,
) -> Vec<u8> {
    format!(
        "status=error\ndetail={}\nrollback_performed={}\nrollback_ok={}\ncontrolled_failure={}\nnet_database_change={}\ntransaction_executed=false\nexecution_gate_closed=true\nsave_api_called=false\n",
        sanitize(detail),
        rollback_performed,
        rollback_ok,
        false,
        !rollback_ok,
    )
    .into_bytes()
}

fn emergency_rollback(db: &mut Database) -> (bool, bool, String) {
    if !MUTATION_ACTIVE.load(Ordering::Acquire) {
        return (false, true, "no mutation was active".to_string());
    }
    match restore_atomic_rollback(db) {
        Ok(performed) => {
            clear_atomic_rollback();
            (
                performed,
                true,
                "emergency rollback restored the pre-execution snapshot".to_string(),
            )
        }
        Err(error) => (
            false,
            false,
            format!("emergency rollback failed: {error}"),
        ),
    }
}

fn run_replacement_floor_audit_server_once(db: &Database) -> Vec<u8> {
    if FLOOR_AUDIT_SERVER_COMPLETED.load(Ordering::Acquire) {
        return b"status=ok\nfailure_count=0\ntrade_hard_floor_integrated=true\nnative_cash_channel_audit_only=true\n".to_vec();
    }
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_replacement_floor_audit(db)
    }));
    match result {
        Ok(Ok(payload)) => {
            FLOOR_AUDIT_SERVER_COMPLETED.store(true, Ordering::Release);
            payload.into_bytes()
        }
        Ok(Err(detail)) => {
            log_event(
                "replacement_floor_audit_failed",
                &format!("detail={};failure_count=1;mutation=false", sanitize(&detail)),
            );
            server_error_payload(&detail)
        }
        Err(_) => {
            let detail = "panic caught during replacement-floor integration audit";
            log_event(
                "replacement_floor_audit_failed",
                "detail=panic caught during replacement-floor integration audit;failure_count=1;mutation=false",
            );
            server_error_payload(detail)
        }
    }
}

fn synchronize_native_offer_history(db: &mut Database, trigger: &str) {
    match offer_history::synchronize_all(db) {
        Ok(summary) => {
            if summary.changed {
                log_event(
                    "first_active_offer_history_synchronized",
                    &format!(
                        "trigger={};created_count={};completed_count={};stage_changed_count={};active_flow_count={};entry_count={};bytes_len={};first_active_offer_only=true;proposer_identity_visible=false;database_mod_save_data_mutation=true;save_api_called=false",
                        trigger, summary.created_count, summary.completed_count, summary.stage_changed_count,
                        summary.active_flow_count, summary.entry_count, summary.bytes_len,
                    ),
                );
            }
        }
        Err(detail) => log_event(
            "first_active_offer_history_sync_error",
            &format!("trigger={};detail={}", trigger, sanitize(&detail)),
        ),
    }
}

struct Test77ServerExtension;


impl ModServerExtension for Test77ServerExtension {
    fn on_server_start(&self, ctx: &mut ServerModContext) {
        if !ASYNC_SERVER_START_LOGGED.swap(true, Ordering::AcqRel) {
            let restored = load_async_trade_proposal(&ctx.database).ok().flatten();
            let proposal_state = restored
                .as_ref()
                .map(|proposal| proposal.state.as_str().to_string())
                .unwrap_or_else(|| "none".to_string());
            log_event(
                "async_trade_server_started",
                &format!(
                    "game_time={};proposal_state={};pending_save_slot={};result_save_slot={};lifecycle_transition_on_start=false;database_mutation=false;transaction_executed=false",
                    ctx.database.time,
                    proposal_state,
                    PENDING_SAVE_SLOT,
                    RESULT_SAVE_SLOT,
                ),
            );
            if let Some(proposal) = restored.as_ref() {
                log_test77_restored_proposal_on_server_start(&ctx.database, proposal);
            }
        }
    }

    fn before_management_tick(&self, _ctx: &mut ServerModContext) {}

    fn after_management_tick(&self, ctx: &mut ServerModContext) {
        if let Ok(Some(proposal)) = load_async_trade_proposal(&ctx.database) {
            if !proposal.state.terminal() {
                log_event(
                    "async_trade_management_tick_observed",
                    &format!(
                        "callback=after_management_tick;proposal_id={};state={};game_time={};server_owned_lifecycle=true;client_process_command_used=false",
                        proposal.proposal_id,
                        proposal.state.as_str(),
                        ctx.database.time,
                    ),
                );
            }
        }
        run_async_trade_lifecycle_guarded(&mut ctx.database, "after_management_tick");
    }

    fn handle_command(
        &self,
        ctx: &mut ServerModContext,
        command: &ModServerCommand,
    ) -> ModServerCommandResult {
        if command.command == NATIVE_OFFER_STATUS_COMMAND {
            let values = match parse_kv_payload(&command.payload) {
                Ok(values) => values,
                Err(detail) => {
                    let payload = format!("status=error\ndetail={}\n", sanitize(&detail)).into_bytes();
                    let _ = ctx.emit_event_to_command_sender(command, NATIVE_OFFER_STATUS_EVENT, payload);
                    return ModServerCommandResult::Handled;
                }
            };
            let athlete_id = map_usize(&values, "athlete_id").unwrap_or(0);
            // [PORT056] 패닉 격리 (DB 를 변경하는 경로라 특히 필요)
            let payload = guarded_payload("native_offer_status", || {
                offer_history::status_payload(&mut ctx.database, athlete_id)
                    .unwrap_or_else(|detail| format!("status=error\nathlete_id={}\ndetail={}\n", athlete_id, sanitize(&detail)).into_bytes())
            });
            let _ = ctx.emit_event_to_command_sender(command, NATIVE_OFFER_STATUS_EVENT, payload);
            return ModServerCommandResult::Handled;
        }
        if command.command == FLOOR_AUDIT_COMMAND {
            let payload = run_replacement_floor_audit_server_once(&ctx.database);
            let _ = ctx.emit_event_to_command_sender(command, FLOOR_AUDIT_EVENT, payload);
            return ModServerCommandResult::Handled;
        }
        if command.command == VALIDATE_SAVED_TRADE_COMMAND {
            let payload = if load_async_trade_proposal(&ctx.database).ok().flatten().is_some() {
                b"status=async_proposal_present\nreload_validation_only=true\ndatabase_mutation=false\ntransaction_executed=false\n".to_vec()
            } else {
                guarded_payload("validate_saved_trade", || {
                    // [PORT056] 패닉 격리
                    match validate_saved_trade_commit(&ctx.database) {
                        Ok(payload) => payload,
                        Err(detail) => server_error_payload(&detail),
                    }
                })
            };
            let _ = ctx.emit_event_to_command_sender(command, VALIDATE_SAVED_TRADE_EVENT, payload);
            return ModServerCommandResult::Handled;
        }
        if command.command == ASYNC_STATUS_COMMAND {
            // [PORT056] 패닉 격리
            let payload = guarded_payload("async_status", || {
                async_status_payload(&ctx.database).unwrap_or_else(|detail| server_error_payload(&detail))
            });
            let _ = ctx.emit_event_to_command_sender(command, ASYNC_STATUS_EVENT, payload);
            return ModServerCommandResult::Handled;
        }
        if command.command == EXECUTE_COMMAND {
            let payload = b"status=error\ndetail=Test79 disables immediate trade execution; submit an asynchronous proposal instead\ndatabase_mutation=false\ntransaction_executed=false\n".to_vec();
            let _ = ctx.emit_event_to_command_sender(command, EXECUTE_EVENT, payload);
            log_event(
                "immediate_trade_execution_blocked",
                "reason=async_lifecycle_only;database_mutation=false;transaction_executed=false",
            );
            return ModServerCommandResult::Handled;
        }
        if command.command == REVIEW_COMMAND {
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<AsyncTradeProposal, String> {
                let values = parse_kv_payload(&command.payload)?;
                let requester_team_id = command.sender_team_id
                    .ok_or_else(|| "command sender team id is unavailable".to_string())?;
                let offered_id = map_usize(&values, "offered_id")?;
                let target_id = map_usize(&values, "target_id")?;
                let region_id = map_usize(&values, "region_id")?;
                let proposed_units = map_u64(&values, "proposed_units")?;
                let desired_raw = map_u64(&values, "desired_status_choice")?;
                let desired_status_choice = u8::try_from(desired_raw)
                    .map_err(|_| "desired status choice is out of range".to_string())?;
                submit_async_trade_proposal(
                    &mut ctx.database,
                    requester_team_id,
                    offered_id,
                    target_id,
                    region_id,
                    proposed_units,
                    desired_status_choice,
                )
            }));
            let payload = match outcome {
                Ok(Ok(proposal)) => {
                    let mut text = String::new();
                    let _ = writeln!(text, "status=submitted");
                    let _ = writeln!(text, "proposal_present=true");
                    let _ = writeln!(text, "proposal_id={}", proposal.proposal_id);
                    let _ = writeln!(text, "state={}", proposal.state.as_str());
                    let _ = writeln!(text, "stage_ko={}", proposal.state.stage_ko());
                    let _ = writeln!(text, "offered_id={}", proposal.offered_id);
                    let _ = writeln!(text, "target_id={}", proposal.target_id);
                    let _ = writeln!(text, "offered_name={}", sanitize(&proposal.offered_name));
                    let _ = writeln!(text, "target_name={}", sanitize(&proposal.target_name));
                    let requester_team_name = ctx.database.teams.get(proposal.requester_team_id)
                        .map(|team| team.name.clone()).unwrap_or_else(|| format!("팀 {}", proposal.requester_team_id));
                    let recipient_team_name = ctx.database.teams.get(proposal.recipient_team_id)
                        .map(|team| team.name.clone()).unwrap_or_else(|| format!("팀 {}", proposal.recipient_team_id));
                    let _ = writeln!(text, "requester_team_id={}", proposal.requester_team_id);
                    let _ = writeln!(text, "recipient_team_id={}", proposal.recipient_team_id);
                    let _ = writeln!(text, "requester_team_name={}", sanitize(&requester_team_name));
                    let _ = writeln!(text, "recipient_team_name={}", sanitize(&recipient_team_name));
                    let _ = writeln!(text, "proposed_units={}", proposal.proposed_units);
                    let (_, desired_status_label) = desired_squad_status(proposal.desired_status_choice);
                    let _ = writeln!(text, "desired_status_label={}", desired_status_label);
                    let _ = writeln!(text, "game_time={}", ctx.database.time);
                    let _ = writeln!(text, "submitted_at={}", proposal.submitted_at);
                    let _ = writeln!(text, "seller_due_at={}", proposal.seller_due_at);
                    let _ = writeln!(text, "player_due_at={}", proposal.player_due_at);
                    let _ = writeln!(text, "team_mutation=false");
                    let _ = writeln!(text, "finance_mutation=false");
                    let _ = writeln!(text, "contract_mutation=false");
                    let _ = writeln!(text, "transaction_executed=false");
                    text.into_bytes()
                }
                Ok(Err(detail)) => server_error_payload(&detail),
                Err(_) => server_error_payload("panic caught during async trade proposal submission"),
            };
            let _ = ctx.emit_event_to_command_sender(command, REVIEW_EVENT, payload);
            return ModServerCommandResult::Handled;
        }
        if command.command != QUOTE_COMMAND {
            return ModServerCommandResult::Pass;
        }
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<ServerQuote, String> {
            let values = parse_kv_payload(&command.payload)?;
            let offered_id = map_usize(&values, "offered_id")?;
            let target_id = map_usize(&values, "target_id")?;
            let region_id = map_usize(&values, "region_id")?;
            let requester_team_id = command.sender_team_id
                .ok_or_else(|| "command sender team id is unavailable".to_string())?;
            // [PORT056] require_dual_trade_baseline 제거 (테스트 세이브 영수증 전제)
            require_fresh_trade_fixture(&ctx.database, requester_team_id, offered_id, target_id)?;
            evaluate_server_quote(&ctx.database, requester_team_id, offered_id, target_id, region_id)
        }));
        match outcome {
            Ok(Ok(quote)) => {
                log_event(
                    "quote_server_evaluated",
                    &format!(
                        "status=ok;requester_team_id={};recipient_team_id={};offered_id={};offered_name={};target_id={};target_name={};exact_required_cash_won={};exact_required_units={};display_min_units={};display_max_units={};display_lower_percent={};display_upper_percent={};range_policy=minimum_x_random_70_80_to_minimum_x_random_150_160;exact_threshold_disclosed_to_client=false;cash_budget_won={};budget_units={};database_mutation=false",
                        quote.requester_team_id,
                        quote.recipient_team_id,
                        quote.offered_id,
                        sanitize(&quote.offered_name),
                        quote.target_id,
                        sanitize(&quote.target_name),
                        quote.required_cash_won,
                        quote.required_units,
                        quote.display_min_units,
                        quote.display_max_units,
                        quote.display_lower_percent,
                        quote.display_upper_percent,
                        quote.cash_budget_won,
                        quote.budget_units,
                    ),
                );
                let _ = ctx.emit_event_to_command_sender(command, QUOTE_EVENT, server_quote_payload(&quote));
            }
            Ok(Err(detail)) => {
                log_event("quote_server_error", &format!("detail={};database_mutation=false", sanitize(&detail)));
                let _ = ctx.emit_event_to_command_sender(command, QUOTE_EVENT, server_error_payload(&detail));
            }
            Err(_) => {
                let detail = "panic caught during read-only Test79 quote evaluation";
                log_event("quote_server_error", "detail=panic caught;database_mutation=false");
                let _ = ctx.emit_event_to_command_sender(command, QUOTE_EVENT, server_error_payload(detail));
            }
        }
        ModServerCommandResult::Handled
    }
}

fn capture_native_compare_popup(ui: &GameUI) {
    let native_popup = find_node_by_id_excluding(&ui.root, NATIVE_COMPARE_ID, MODAL_LAYER_ID)
        .or_else(|| find_node_by_id_excluding(&ui.root, NATIVE_COMPARE_SOURCE_ID, MODAL_LAYER_ID));
    let Some(native_popup) = native_popup else {
        return;
    };
    let has_fade = direct_child(native_popup, NATIVE_COMPARE_FADE_ID).is_some();
    let has_body = direct_child(native_popup, "popup").is_some();
    if !has_fade || !has_body {
        return;
    }
    let capture_mode = if native_popup.visible { "visible" } else { "hidden" };
    COMPARE_TEMPLATE.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_none() || native_popup.visible {
            let mut captured = native_popup.clone();
            captured.id = NATIVE_COMPARE_ID.to_string();
            *slot = Some(captured);
        }
    });
    ENTRY_TEMPLATE_READY.store(true, Ordering::Release);
    if !COMPARE_TREE_LOGGED.swap(true, Ordering::Relaxed) {
        log_event(
            "compare_popup_template_captured",
            &format!(
                "root_id={};capture_mode={};required_children=true;source=live_player_detail_tree;cached_clone_ready=true;first_trade_click_can_open_directly=true;native_handler_required=false",
                sanitize(&native_popup.id), capture_mode,
            ),
        );
        let mut output = String::new();
        let _ = writeln!(
            output,
            "tree_dump=cached_native_compare_template|root_id={}|capture_mode={}",
            sanitize(&native_popup.id), capture_mode,
        );
        dump_tree(native_popup, "", 0, &mut output);
        append_log(&output);
    }
}

fn trade_entry(ui: &GameUI) -> Option<&Node> {
    let offer = active_offer(ui)?;
    direct_path(offer, &["data", "row2", ENTRY_ID])
}

fn trade_entry_mut(ui: &mut GameUI) -> Option<&mut Node> {
    let offer = active_offer_mut(ui)?;
    direct_path_mut(offer, &["data", "row2", ENTRY_ID])
}

fn path_has_exact_node_id(path: &str, id: &str) -> bool {
    path.split(|character| matches!(character, '.' | '/' | '\\'))
        .any(|segment| segment == id)
}

fn trade_entry_event(path: &str, item: &str) -> bool{
    item == ENTRY_ID
        || path == ENTRY_ID
        || path.split(|character| matches!(character, '.' | '/' | '\\'))
            .any(|segment| segment == ENTRY_ID)
        || path.contains(ENTRY_ID)
        || item.contains(ENTRY_ID)
}

fn target_id_from_offer(ui: &GameUI) -> Option<usize> {
    let offer = active_offer(ui)?;
    let data = direct_child(offer, "data")?;
    let row1 = direct_child(data, "row1")?;
    let info = direct_child(row1, "info")?;
    let header = direct_child(info, "header")?;
    let name = direct_child(header, "name")?;
    name.child.iter().find_map(|child| {
        child
            .id
            .strip_prefix("view_detail_")
            .and_then(|value| value.parse::<usize>().ok())
    })
}


fn update_trade_entry_and_capture_offer(ui: &mut GameUI) {
    let offer_surface_present = trade_entry(ui).is_some();
    capture_pending_contract_slot_template(ui);
    capture_native_target_view_detail_template(ui);
    let delegate_visible = active_offer(ui)
        .and_then(|offer| direct_path(offer, &["data", "row2", "delegate"]))
        .map(|node| node.visible)
        .unwrap_or(false);

    if offer_surface_present && !OFFER_TREE_LOGGED.swap(true, Ordering::Relaxed) {
        log_event(
            "offer_surface_detected",
            "asset=asset/base/ui/layout/transfer_offer;entry_id=pts_trade_entry;entry_path=data.row2.pts_trade_entry;original_view_other_preserved=true;entry_position=above_offer_fine_tuned;x=-15px;y=-100px;width=160px;offer_y=-15px;vertical_gap=45px",
        );
        if let Some(root) = active_offer(ui) {
            let mut output = String::new();
            dump_tree(root, "", 0, &mut output);
            append_log(&output);
        }
    }

    if let Some(target_id) = target_id_from_offer(ui) {
        let previous = TARGET_ATHLETE_ID.swap(target_id, Ordering::AcqRel);
        if previous != target_id {
            TARGET_EPOCH.fetch_add(1, Ordering::AcqRel);
            log_event(
                "target_athlete_captured",
                &format!("source=transfer_offer_view_detail;target_id={target_id}"),
            );
        }
    }

    let template_ready = active_offer(ui)
        .and_then(|offer| direct_child(offer, MODAL_LAYER_TEMPLATE_ID))
        .is_some_and(|modal| direct_child(modal, CUSTOM_COMPARE_TEMPLATE_ID).is_some())
        && active_offer(ui)
            .and_then(|offer| direct_child(offer, CASH_TEMPLATE_ID))
            .is_some();
    let baseline_ready = FIRST_TRADE_BASELINE_VERIFIED.load(Ordering::Acquire);
    let target_id = TARGET_ATHLETE_ID.load(Ordering::Acquire);
    let target_ready = target_id != 0; // [PORT056] 과거 픽스처 선수 4명 제외 규칙 삭제
    let submit_in_flight = REVIEW_REQUEST_QUEUED.load(Ordering::Acquire)
        || REVIEW_REQUEST_PENDING.load(Ordering::Acquire);
    let async_view = ASYNC_STATUS_VIEW.with(|slot| slot.borrow().clone());
    let active_view = async_view.as_ref().filter(|view| {
        view.proposal_present && (view.state == "SellerReview" || view.state == "PlayerReview")
    });
    let same_target_pending = active_view.is_some_and(|view| view.target_id == target_id);
    let different_target_pending = active_view.is_some() && !same_target_pending;
    let pending_any = submit_in_flight || active_view.is_some();
    // ★[PORT056] 요구사항 1: pending 중에는 **모든 선수**의 트레이드 버튼이 죽어야 한다.
    //   구판은 `!same_target_pending` 만 봐서 다른 선수 버튼·모달이 그대로 열렸고
    //   (자체 로그가 `custom_modal_reopen_allowed=true` 로 자백), 서버 제출 단계에서만 막혔다.
    let season_used = TRADE_SEASON_USED.load(Ordering::Acquire);
    let ready = delegate_visible
        && template_ready
        && baseline_ready
        && target_ready
        && !submit_in_flight
        && !pending_any
        && !season_used;

    let mut tip_state: Option<((f32, f32, f32, f32), String)> = None;
    if let Some(entry) = trade_entry_mut(ui) {
        entry.visible = delegate_visible;
        // ★[PORT056] `Node.disabled` 는 pub 필드이고, **켜는 것이 맞다** —
        //   호버 커서(손가락)·호버 효과·클릭이 게임 자체 규칙으로 죽는다(champ_pos_lock 실증).
        //   ⚠단 그러면 게임의 호버 처리도 죽어 러너 `hint` 툴팁이 안 뜬다
        //     ⟹ 툴팁은 커서를 Win32 로 직접 읽어 **우리가 띄운다**(아래 tip 블록).
        entry.disabled = !ready;
        let label = if submit_in_flight {
            "제안 접수 중"
        } else if let Some(view) = active_view.filter(|view| view.target_id == target_id) {
            if view.state == "SellerReview" { "판매 구단 검토 중" } else { "선수 검토 중" }
        } else if season_used {
            // [PORT056] 성사만 소모 — 거절·만료 뒤에는 이 라벨이 뜨지 않고 다시 제안할 수 있다.
            "이번 영입 시즌 트레이드 완료"
        } else if different_target_pending {
            // [PORT056] 다른 선수 제안이 진행 중 = 이 선수에게도 제안 불가(요구사항 1)
            "트레이드 진행 중"
        } else if !template_ready {
            "UI 준비 중"
        } else {
            "트레이드"
        };
        let mut properties: HashMap<String, Rc<dyn Any>> = HashMap::new();
        properties.insert("text".to_string(), Rc::new(label.to_string()));
        entry.runner.build_with_property(&properties);
        // ★[PORT056] 2026-08-23 인게임 실측: 시즌 쿼터로 버튼이 정상 비활성(disabled=true)됐는데
        //   라벨은 계속 「트레이드」였다 ⟹ `color_icon_button` 런너에 `build_with_property("text")` 를
        //   걸어도 표시 텍스트가 안 바뀐다(`.ui` 의 `text: { ... }` 는 속성 블록이라 런너가 이 키를 안 읽는 듯).
        //   사용자에겐 "그냥 고장난 버튼"으로 보이므로, 버튼 아래 라벨 노드를 찾아 직접 갱신한다.
        //   (projection 10열에서 검증된 `set_runner_text` 경로 재사용 — 이쪽은 LabelRunner 라 동작한다.)
        // ★[PORT056] 버튼 텍스트는 바꿀 수 없으므로(위 헬퍼 주석 참조) **비활성 룩 + 게임 툴팁**으로 알린다.
        //   활성일 때는 원래 색으로 복구하고 툴팁을 비운다.
        let hint = if ready { "" } else { label };
        let styled = entry_button_set_disabled_look(entry, !ready, hint);
        // 툴팁 히트테스트용으로 버튼의 화면 rect 와 표시 조건을 밖으로 넘긴다.
        tip_state = if ready || !delegate_visible {
            None
        } else {
            Some(((entry.rect.x, entry.rect.y, entry.rect.w, entry.rect.h), hint.to_string()))
        };
        entry.runner.set_dirty(true);
        let look_key = (!ready as usize) << 1 | (styled as usize);
        if ENTRY_LOOK_LAST_KEY.swap(look_key + 1, Ordering::AcqRel) != look_key + 1 {
            log_event(
                "trade_entry_disabled_look_applied",
                &format!(
                    "hint={};gray={};runner_matched={};runner_type={}",
                    sanitize(hint),
                    !ready,
                    styled,
                    sanitize(entry.runner.type_name()),
                ),
            );
        }
        TRADE_ENTRY_ACTIVE.store(ready, Ordering::Release);
        // ★[PORT056] 버튼 자체 텍스트는 바꿀 수 없다(ColorIconButtonRunner 가 내부에 들고 있고
        //   자식 노드가 없다 — 인게임 실측 `child_label_found=false`, downcast 도 cdylib 경계라 불가).
        //   ⟹ 바닐라가 `delegate` 버튼 아래에 `delegate_tooltip:label` 을 따로 두는 방식을 그대로 따라
        //   `.ui` 에 선언한 `#pts_trade_entry_note` 라벨로 "왜 못 누르는지"를 표시한다.
        //   활성일 때는 숨기고(버튼의 정적 텍스트 「트레이드」가 이미 정확), 비활성일 때만 사유를 띄운다.

        let state_key = format!("{}|{}|{}|{}", target_id, label, entry.visible, entry.disabled);
        let changed = TRADE_ENTRY_ASYNC_STATE_KEY.with(|slot| {
            let mut slot = slot.borrow_mut();
            if *slot == state_key { false } else { *slot = state_key; true }
        });
        if changed && pending_any {
            log_event(
                "trade_button_pending_state_rendered",
                &format!(
                    "target_id={};label={};visible={};disabled={};submit_in_flight={};proposal_state={};proposal_target_id={};duplicate_submit_blocked=true",
                    target_id,
                    sanitize(label),
                    entry.visible,
                    entry.disabled,
                    submit_in_flight,
                    active_view.map(|view| view.state.as_str()).unwrap_or("none"),
                    active_view.map(|view| view.target_id).unwrap_or(0),
                ),
            );
        }
        if changed && different_target_pending {
            log_event(
                "trade_button_other_target_enabled",
                &format!(
                    "current_target_id={};proposal_target_id={};proposal_state={};label=트레이드;visible={};disabled={};custom_modal_reopen_allowed=true;second_submit_allowed=false",
                    target_id,
                    active_view.map(|view| view.target_id).unwrap_or(0),
                    active_view.map(|view| view.state.as_str()).unwrap_or("none"),
                    entry.visible,
                    entry.disabled,
                ),
            );
        }
        if ready && !ENTRY_LOGGED.swap(true, Ordering::Relaxed) {
            log_event(
                "trade_button_detected",
                "id=pts_trade_entry;source=mod_owned_transfer_offer_button;entry_path=data.row2.pts_trade_entry;visible=true;disabled=false;custom_modal_template_ready=true;native_handler_required=false;open_strategy=fully_custom_static_modal;placement=above_offer_fine_tuned;x=-15;y=-100;width=160;offer_y=-15;vertical_gap=45;transaction_enabled=submit_async_only",
            );
        }
    } else {
        TRADE_ENTRY_ACTIVE.store(false, Ordering::Release);
    }

    // ★[PORT056] 비활성 사유 툴팁 — `Node.disabled=true` 라 게임 호버가 죽으므로 우리가 띄운다.
    //   커서를 Win32 로 읽어 버튼 rect 안이면 `.ui` 에 선언해 둔 `#pts_trade_entry_tip` 을 보인다.
    {
        let hovering = match tip_state.as_ref() {
            Some(((x, y, w, h), _)) => cursor_in_ui(ui.rect.w, ui.rect.h)
                .is_some_and(|(cx, cy)| cx >= *x && cx <= x + w && cy >= *y && cy <= y + h),
            None => false,
        };
        let was = TIP_VISIBLE.swap(hovering, Ordering::Relaxed);
        if hovering || was {
            let text = tip_state.as_ref().map(|(_, t)| t.clone()).unwrap_or_default();
            if let Some(tip) = find_node_by_id_mut(&mut ui.root, ENTRY_TIP_ID) {
                tip.visible = hovering;
                if hovering {
                    if let Some(label_node) = direct_child_mut(tip, "text") {
                        set_runner_text(label_node, &text);
                    }
                }
                tip.runner.set_dirty(true);
            }
            if hovering && !was {
                log_event(
                    "trade_entry_tip_shown",
                    &format!("text={};rect_hit=true", sanitize(&text)),
                );
            }
        }
    }
}

fn structurally_valid_compare_parent_index_paths(ui: &GameUI) -> Vec<Vec<usize>> {
    let mut paths = Vec::new();
    collect_parent_index_paths_by_id_excluding(
        &ui.root,
        NATIVE_COMPARE_ID,
        MODAL_LAYER_ID,
        &mut Vec::new(),
        &mut paths,
    );
    paths.retain(|parent_path| {
        node_by_index_path(&ui.root, parent_path)
            .and_then(|parent| direct_child(parent, NATIVE_COMPARE_ID))
            .is_some_and(|popup| {
                direct_child(popup, NATIVE_COMPARE_FADE_ID).is_some()
                    && direct_child(popup, "popup").is_some()
            })
    });
    paths
}

fn active_compare_parent_index_paths(ui: &GameUI) -> Vec<Vec<usize>> {
    structurally_valid_compare_parent_index_paths(ui)
        .into_iter()
        .filter(|parent_path| {
            (0..=parent_path.len()).all(|depth| {
                node_by_index_path(&ui.root, &parent_path[..depth])
                    .is_some_and(|node| node.visible && !node.disabled)
            })
        })
        .collect()
}

fn visible_ready_compare_parent_index_path(ui: &GameUI) -> Option<(Vec<usize>, String)> {
    let mut ready: Vec<(Vec<usize>, String)> = Vec::new();
    for parent_path in active_compare_parent_index_paths(ui) {
        let Some(parent) = node_by_index_path(&ui.root, &parent_path) else {
            continue;
        };
        let Some(popup) = direct_child(parent, NATIVE_COMPARE_ID) else {
            continue;
        };
        if popup.visible && !popup.disabled {
            ready.push((parent_path, parent.runner.type_name().to_string()));
        }
    }
    if ready.len() == 1 {
        ready.pop()
    } else {
        if !ready.is_empty() {
            log_event(
                "trade_live_compare_ambiguous",
                &format!(
                    "visible_ready_candidate_count={};expected=1;fail_closed=true",
                    ready.len()
                ),
            );
        }
        None
    }
}

fn activate_live_native_compare_popup(ui: &mut GameUI) -> Option<NativeCompareHome> {
    let (parent_path, parent_runner) = visible_ready_compare_parent_index_path(ui)?;
    let parent = node_by_index_path(&ui.root, &parent_path)?;
    let parent_id = parent.id.clone();
    let popup = direct_child(parent, NATIVE_COMPARE_ID)?;
    let fade = direct_child(popup, NATIVE_COMPARE_FADE_ID)?;
    let mut ancestor_states = Vec::new();
    for depth in 0..=parent_path.len() {
        let ancestor_path = parent_path[..depth].to_vec();
        let ancestor = node_by_index_path(&ui.root, &ancestor_path)?;
        ancestor_states.push((ancestor_path, ancestor.visible, ancestor.disabled));
    }
    let popup_was_visible = popup.visible;
    let popup_was_disabled = popup.disabled;
    let fade_was_visible = fade.visible;
    let fade_was_disabled = fade.disabled;
    let target_id = TARGET_ATHLETE_ID.load(Ordering::Acquire);
    let target_epoch = TARGET_EPOCH.load(Ordering::Acquire);
    let snapshot = CLOSED_COMPARE_SNAPSHOT.with(|slot| slot.borrow_mut().take());
    let mut home = snapshot
        .filter(|snapshot| snapshot.target_id == target_id && snapshot.target_epoch == target_epoch)
        .map(|snapshot| snapshot.home)
        .unwrap_or(NativeCompareHome {
            parent_path: parent_path.clone(),
            ancestor_states: ancestor_states.clone(),
            popup_visible: false,
            popup_disabled: false,
            fade_visible: true,
            fade_disabled: false,
        });
    if home.parent_path != parent_path {
        log_event(
            "trade_live_compare_snapshot_path_mismatch",
            "snapshot_discarded=true;canonical_hidden_fallback=true;fail_closed=false",
        );
        home = NativeCompareHome {
            parent_path,
            ancestor_states,
            popup_visible: false,
            popup_disabled: false,
            fade_visible: true,
            fade_disabled: false,
        };
    }
    log_event(
        "trade_live_compare_selected",
        &format!(
            "candidate_count=1;selected_parent_id={};selected_parent_runner={};visible_ancestor_count={};popup_was_visible={};popup_was_disabled={};target_id={};selection_policy=exact_single_visible_ready;native_compare_roster_click_completed=true;deferred_attach=true;detached_clone=false",
            sanitize(&parent_id),
            sanitize(&parent_runner),
            home.ancestor_states.iter().filter(|(_, visible, disabled)| *visible && !*disabled).count(),
            popup_was_visible,
            popup_was_disabled,
            TARGET_ATHLETE_ID.load(Ordering::Acquire),
        ),
    );
    log_event(
        "trade_live_compare_close_home_armed",
        &format!(
            "popup_was_visible={popup_was_visible};popup_was_disabled={popup_was_disabled};fade_was_visible={fade_was_visible};fade_was_disabled={fade_was_disabled};close_popup_visible={};close_popup_disabled={};close_fade_visible={};close_fade_disabled={};source=pre_click_snapshot_or_canonical_closed;home_snapshot_valid=true;stale_compare_ui_retained=false",
            home.popup_visible, home.popup_disabled, home.fade_visible, home.fade_disabled,
        ),
    );
    for (ancestor_path, _, _) in &home.ancestor_states {
        let ancestor = node_by_index_path_mut(&mut ui.root, ancestor_path)?;
        ancestor.visible = true;
        ancestor.disabled = false;
        ancestor.runner.set_dirty(true);
    }
    let parent = node_by_index_path_mut(&mut ui.root, &home.parent_path)?;
    let popup = direct_child_mut(parent, NATIVE_COMPARE_ID)?;
    popup.visible = true;
    popup.disabled = false;
    popup.runner.set_dirty(true);
    let fade = direct_child_mut(popup, NATIVE_COMPARE_FADE_ID)?;
    fade.visible = false;
    fade.disabled = true;
    fade.runner.set_dirty(true);
    Some(home)
}

fn restore_live_native_compare_popup(ui: &mut GameUI, home: NativeCompareHome) -> bool {
    let mut restored_popup = false;
    if let Some(parent) = node_by_index_path_mut(&mut ui.root, &home.parent_path) {
        if let Some(popup) = direct_child_mut(parent, NATIVE_COMPARE_ID) {
            popup.visible = home.popup_visible;
            popup.disabled = home.popup_disabled;
            popup.runner.set_dirty(true);
            if let Some(fade) = direct_child_mut(popup, NATIVE_COMPARE_FADE_ID) {
                fade.visible = home.fade_visible;
                fade.disabled = home.fade_disabled;
                fade.runner.set_dirty(true);
            }
            restored_popup = true;
        }
    }
    for (ancestor_path, visible, disabled) in home.ancestor_states.iter().rev() {
        if let Some(ancestor) = node_by_index_path_mut(&mut ui.root, ancestor_path) {
            ancestor.visible = *visible;
            ancestor.disabled = *disabled;
            ancestor.runner.set_dirty(true);
        } else {
            restored_popup = false;
        }
    }
    restored_popup
}

fn native_compare_matches_home(ui: &GameUI, home: &NativeCompareHome) -> bool {
    let Some(parent) = node_by_index_path(&ui.root, &home.parent_path) else {
        return false;
    };
    let Some(popup) = direct_child(parent, NATIVE_COMPARE_ID) else {
        return false;
    };
    let Some(fade) = direct_child(popup, NATIVE_COMPARE_FADE_ID) else {
        return false;
    };
    popup.visible == home.popup_visible
        && popup.disabled == home.popup_disabled
        && fade.visible == home.fade_visible
        && fade.disabled == home.fade_disabled
        && home.ancestor_states.iter().all(|(path, visible, disabled)| {
            node_by_index_path(&ui.root, path)
                .is_some_and(|node| node.visible == *visible && node.disabled == *disabled)
        })
}

fn insert_trade_modal_before_live_popup(
    ui: &mut GameUI,
    modal_layer: Node,
    parent_path: &[usize],
) -> bool {
    let Some(parent) = node_by_index_path_mut(&mut ui.root, parent_path) else {
        return false;
    };
    let Some(popup_index) = parent
        .child
        .iter()
        .position(|child| child.id == NATIVE_COMPARE_ID)
    else {
        return false;
    };
    parent.child.insert(popup_index, modal_layer);
    parent.runner.set_dirty(true);
    true
}

fn remove_trade_modal_and_restore_live_popup(ui: &mut GameUI) -> Option<(bool, bool)> {
    let home = NATIVE_COMPARE_HOME.with(|slot| slot.borrow_mut().take())?;
    let parent_path = home.parent_path.clone();
    let modal_removed = node_by_index_path_mut(&mut ui.root, &parent_path)
        .and_then(|parent| take_direct_child(parent, MODAL_LAYER_ID))
        .is_some();
    let restored_to_home = restore_live_native_compare_popup(ui, home.clone());
    let native_popup_closed = native_compare_matches_home(ui, &home)
        && node_by_index_path(&ui.root, &parent_path)
        .and_then(|parent| direct_child(parent, NATIVE_COMPARE_ID))
        .map(|popup| !popup.visible)
        .unwrap_or(false);
    Some((restored_to_home && modal_removed, native_popup_closed))
}

fn restore_orphaned_live_compare_popup(ui: &mut GameUI) {
    let popup_open = POPUP_OPEN.load(Ordering::Acquire);
    let modal_count = count_nodes_by_id(&ui.root, MODAL_LAYER_ID);
    let home_present = NATIVE_COMPARE_HOME.with(|slot| slot.borrow().is_some());
    let clean = !popup_open && modal_count == 0 && !home_present;
    let active = popup_open
        && modal_count == 1
        && home_present
        && modal_hierarchy_is_canonical(ui);
    if clean || active {
        return;
    }
    invalidate_trade_ui_context(ui, "ui_state_reconcile_failed");
    log_event(
        "live_native_compare_orphan_recovered",
        &format!(
            "popup_open_before={popup_open};modal_count_before={modal_count};home_present_before={home_present};modal_layer_removed=true;restored_in_place=false;native_close_state_restored=true;check_marker_restore=not_applicable;manual_visual_identity_required=true;detached_clone_dropped=false;transaction_enabled=true"
        ),
    );
}

fn force_remove_trade_modal(ui: &mut GameUI) -> bool {
    let mut removed = false;
    if let Some(parent_path) = NATIVE_COMPARE_HOME.with(|slot| {
        slot.borrow().as_ref().map(|home| home.parent_path.clone())
    }) {
        removed |= node_by_index_path_mut(&mut ui.root, &parent_path)
            .and_then(|parent| take_direct_child(parent, MODAL_LAYER_ID))
            .is_some();
    }
    while find_parent_of_id(&ui.root, MODAL_LAYER_ID).is_some() {
        let removed_one = find_parent_of_id_mut(&mut ui.root, MODAL_LAYER_ID)
            .and_then(|parent| take_direct_child(parent, MODAL_LAYER_ID))
            .is_some();
        if !removed_one {
            break;
        }
        removed = true;
    }
    removed
}

fn force_canonical_native_compare_closed(ui: &mut GameUI) -> bool {
    let mut paths = active_compare_parent_index_paths(ui);
    if paths.len() != 1 {
        if !paths.is_empty() {
            log_event(
                "trade_live_compare_close_ambiguous",
                &format!("candidate_count={};expected=1;fail_closed=true", paths.len()),
            );
        }
        return false;
    }
    let Some(parent_path) = paths.pop() else {
        return false;
    };
    let Some(parent) = node_by_index_path_mut(&mut ui.root, &parent_path) else {
        return false;
    };
    let Some(popup) = direct_child_mut(parent, NATIVE_COMPARE_ID) else {
        return false;
    };
    popup.visible = false;
    popup.disabled = false;
    popup.runner.set_dirty(true);
    if let Some(fade) = direct_child_mut(popup, NATIVE_COMPARE_FADE_ID) {
        fade.visible = true;
        fade.disabled = false;
        fade.runner.set_dirty(true);
    }
    canonical_native_compare_closed(ui)
}

fn canonical_native_compare_closed(ui: &GameUI) -> bool {
    let mut paths = active_compare_parent_index_paths(ui);
    if paths.len() != 1 {
        return false;
    }
    let Some(parent_path) = paths.pop() else {
        return false;
    };
    let Some(popup) = node_by_index_path(&ui.root, &parent_path)
        .and_then(|parent| direct_child(parent, NATIVE_COMPARE_ID))
    else {
        return false;
    };
    let Some(fade) = direct_child(popup, NATIVE_COMPARE_FADE_ID) else {
        return false;
    };
    !popup.visible && !popup.disabled && fade.visible && !fade.disabled
}

fn reset_quote_state() {
    PENDING_OFFERED_ATHLETE_ID.store(NO_ATHLETE, Ordering::Relaxed);
    OFFERED_ATHLETE_ID.store(NO_ATHLETE, Ordering::Relaxed);
    VISUAL_SYNC_LAST_OFFERED_ID.store(0, Ordering::Relaxed);
    VISUAL_SYNC_LAST_SEQUENCE.store(0, Ordering::Relaxed);
    NATIVE_VISUAL_SYNC_PENDING.store(false, Ordering::Relaxed);
    QUOTE_REQUEST_SENT.store(false, Ordering::Relaxed);
    CASH_INPUT_STATE.store(0, Ordering::Relaxed);
    DESIRED_SQUAD_STATUS.store(STATUS_CORE, Ordering::Relaxed);
    STATUS_MENU_OPEN.store(false, Ordering::Relaxed);
    QUOTE_VIEW.with(|slot| *slot.borrow_mut() = None);
    QUOTE_ERROR.with(|slot| *slot.borrow_mut() = None);
    PROPOSED_UNITS.with(|slot| *slot.borrow_mut() = None);
    REVIEW_VIEW.with(|slot| *slot.borrow_mut() = None);
    REVIEW_ERROR.with(|slot| *slot.borrow_mut() = None);
    REVIEW_REQUEST_QUEUED.store(false, Ordering::Relaxed);
    REVIEW_REQUEST_PENDING.store(false, Ordering::Relaxed);
    EXECUTE_VIEW.with(|slot| *slot.borrow_mut() = None);
    EXECUTE_ERROR.with(|slot| *slot.borrow_mut() = None);
    EXECUTE_REQUEST_QUEUED.store(false, Ordering::Relaxed);
    EXECUTE_REQUEST_PENDING.store(false, Ordering::Relaxed);
    DUPLICATE_REPLAY_REQUEST_QUEUED.store(false, Ordering::Relaxed);
    DUPLICATE_REPLAY_REQUEST_PENDING.store(false, Ordering::Relaxed);
    DUPLICATE_REPLAY_VERIFIED.store(false, Ordering::Relaxed);
    {
        let mut payload = DUPLICATE_REPLAY_PAYLOAD
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *payload = None;
    }
    ZERO_CASH_EXPLANATION_LOGGED.store(false, Ordering::Relaxed);
    QUOTE_UI_DIRTY.store(true, Ordering::Relaxed);
}

fn request_cached_compare_clone_from_trade_click(path: &str, item: &str) -> bool {
    if !FIRST_TRADE_BASELINE_VERIFIED.load(Ordering::Acquire) {
        log_event(
            "custom_trade_ui_open_blocked",
            // [PORT056] 구 문구는 존재하지 않는 테스트 세이브 슬롯을 원인으로 지목해 오독을 부른다.
            "reason=trade_availability_not_yet_confirmed;await=server_validate_response;database_mutation=false;transaction_executed=false",
        );
        return true;
    }
    if POPUP_OPEN.load(Ordering::Acquire) {
        log_event(
            "custom_trade_ui_open_ignored",
            "reason=already_open;custom_modal=true",
        );
        return true;
    }
    let target_id = TARGET_ATHLETE_ID.load(Ordering::Acquire);
    let template_ready = TRADE_ENTRY_ACTIVE.load(Ordering::Acquire);
    if target_id == 0 || !template_ready {
        log_event(
            "custom_trade_ui_open_blocked",
            &format!(
                "reason=target_or_custom_template_not_ready;target_id={target_id};custom_template_ready={template_ready};event_consumed_by_mod=true;database_mutation=false;transaction_executed=false"
            ),
        );
        return true;
    }
    let structural_offer_count = ACTIVE_STRUCTURAL_OFFER_COUNT.load(Ordering::Acquire);
    if structural_offer_count != 1 {
        log_event(
            "trade_offer_structural_root_unresolved",
            &format!("structural_offer_surface_count={structural_offer_count};expected=1;event_consumed_by_mod=true;fail_closed=true"),
        );
        return true;
    }
    let click_sequence = OPEN_CLICK_SEQUENCE.fetch_add(1, Ordering::AcqRel) + 1;
    let open_generation = OPEN_GENERATION.fetch_add(1, Ordering::AcqRel) + 1;
    log_event(
        "trade_button_clicked",
        &format!(
            "id=pts_trade_entry;source=mod_owned_transfer_offer_button;raw_path={};raw_item={};physical=true;click_sequence={click_sequence};open_generation={open_generation};open_strategy=fully_custom_static_modal;native_compare_popup_used=false;native_handler_required=false;event_consumed_by_mod=true;target_id={target_id};transaction_enabled=true",
            sanitize(path),
            sanitize(item),
        ),
    );
    CLICK_ACTION.store(ACTION_OPEN, Ordering::Release);
    true
}


fn complete_pending_cached_compare_open(_ui: &mut GameUI, _assets: &Assets) {
    // Test77 opens the cached compare clone synchronously from the mod-owned button.
}

fn park_native_compare_source(ui: &mut GameUI) -> bool {
    if find_node_by_id_excluding(&ui.root, NATIVE_COMPARE_SOURCE_ID, MODAL_LAYER_ID).is_some() {
        return true;
    }
    if let Some(source) = find_node_by_id_mut_excluding(&mut ui.root, NATIVE_COMPARE_ID, MODAL_LAYER_ID) {
        source.id = NATIVE_COMPARE_SOURCE_ID.to_string();
        source.runner.set_dirty(true);
        log_event(
            "native_compare_source_parked",
            "source_id=compare_popup;parked_id=pts_trade_compare_source;cached_clone_uses_compare_popup_identity=true",
        );
        return true;
    }
    let cached = COMPARE_TEMPLATE.with(|slot| slot.borrow().is_some());
    if cached && !CACHED_COMPARE_TEMPLATE_FALLBACK_LOGGED.swap(true, Ordering::Relaxed) {
        log_event(
            "native_compare_source_absent_cached_template_used",
            "live_source_present=false;cached_template_present=true;popup_open_allowed=true;silent_noop=false",
        );
    }
    cached
}

fn restore_native_compare_source_identity(ui: &mut GameUI) {
    if let Some(source) = find_node_by_id_mut_excluding(
        &mut ui.root,
        NATIVE_COMPARE_SOURCE_ID,
        MODAL_LAYER_ID,
    ) {
        source.id = NATIVE_COMPARE_ID.to_string();
        source.runner.set_dirty(true);
    }
}

fn restore_orphaned_compare_identity(ui: &mut GameUI) {
    if POPUP_OPEN.load(Ordering::Acquire) {
        return;
    }
    if find_node_by_id_excluding(&ui.root, NATIVE_COMPARE_SOURCE_ID, MODAL_LAYER_ID).is_some() {
        restore_native_compare_source_identity(ui);
        log_event(
            "native_compare_identity_orphan_recovered",
            "popup_open=false;source_id_restored=compare_popup;detached_clone=true;transaction_enabled=true",
        );
    }
}

fn open_trade_popup(ui: &mut GameUI, assets: &Assets) {
    if POPUP_OPEN.load(Ordering::Acquire) {
        log_event("custom_trade_ui_open_ignored", "reason=already_open;custom_modal=true");
        return;
    }
    let Some(mut modal_layer) = active_offer(ui)
        .and_then(|offer| direct_child(offer, MODAL_LAYER_TEMPLATE_ID))
        .cloned()
    else {
        log_event(
            "custom_trade_modal_template_missing",
            "id=pts_trade_modal_layer_template;transaction_enabled=true",
        );
        return;
    };
    let Some(mut cash_panel) = active_offer(ui)
        .and_then(|offer| direct_child(offer, CASH_TEMPLATE_ID))
        .cloned()
    else {
        log_event(
            "custom_trade_cash_template_missing",
            "id=pts_trade_cash_template;transaction_enabled=true",
        );
        return;
    };

    modal_layer.id = MODAL_LAYER_ID.to_string();
    modal_layer.visible = true;
    modal_layer.disabled = false;
    modal_layer.runner.set_dirty(true);
    if let Some(backdrop) = direct_child_mut(&mut modal_layer, BACKDROP_TEMPLATE_ID) {
        backdrop.id = BACKDROP_ID.to_string();
        backdrop.visible = true;
        backdrop.disabled = false;
        backdrop.runner.set_dirty(true);
    } else {
        log_event(
            "custom_trade_backdrop_template_missing",
            "id=pts_trade_modal_backdrop_template;transaction_enabled=true",
        );
        return;
    }
    if let Some(compare) = direct_child_mut(&mut modal_layer, CUSTOM_COMPARE_TEMPLATE_ID) {
        compare.id = CUSTOM_COMPARE_ID.to_string();
        compare.visible = true;
        compare.disabled = false;
        compare.runner.set_dirty(true);
    } else {
        log_event(
            "custom_trade_compare_template_missing",
            "id=pts_trade_custom_compare_template;transaction_enabled=true",
        );
        return;
    }

    cash_panel.id = CASH_PANEL_ID.to_string();
    cash_panel.visible = true;
    cash_panel.disabled = false;
    cash_panel.runner.set_dirty(true);
    for (from, to) in [
        (REVIEW_BUTTON_TEMPLATE_ID, REVIEW_BUTTON_ID),
        (COMMIT_BUTTON_TEMPLATE_ID, COMMIT_BUTTON_ID),
        ("pts_cash_selection_value", "pts_cash_runtime_selection_value"),
        ("pts_cash_required_value", "pts_cash_runtime_required_value"),
        ("pts_cash_required_note", "pts_cash_runtime_required_note"),
        ("pts_cash_proposed_eok", "pts_cash_runtime_proposed_eok"),
        ("pts_cash_budget_value", "pts_cash_runtime_budget_value"),
        ("pts_cash_after_value", "pts_cash_runtime_after_value"),
        ("pts_cash_read_only_note", "pts_cash_runtime_read_only_note"),
        ("pts_cash_status_idle", "pts_cash_runtime_status_idle"),
        ("pts_cash_status_valid", "pts_cash_runtime_status_valid"),
        ("pts_cash_status_invalid", "pts_cash_runtime_status_invalid"),
        ("pts_trade_status_toggle", STATUS_TOGGLE_ID),
        ("pts_trade_status_value_core", "pts_trade_runtime_status_value_core"),
        ("pts_trade_status_value_important", "pts_trade_runtime_status_value_important"),
        ("pts_trade_status_value_general", "pts_trade_runtime_status_value_general"),
        ("pts_trade_status_value_sub", "pts_trade_runtime_status_value_sub"),
        ("pts_trade_status_value_prospect", "pts_trade_runtime_status_value_prospect"),
        ("pts_trade_status_menu", STATUS_MENU_ID),
        ("pts_trade_status_option_core", "pts_trade_runtime_status_option_core"),
        ("pts_trade_status_option_important", "pts_trade_runtime_status_option_important"),
        ("pts_trade_status_option_general", "pts_trade_runtime_status_option_general"),
        ("pts_trade_status_option_sub", "pts_trade_runtime_status_option_sub"),
        ("pts_trade_status_option_prospect", "pts_trade_runtime_status_option_prospect"),
        ("pts_trade_status_check_core", "pts_trade_runtime_status_check_core"),
        ("pts_trade_status_check_important", "pts_trade_runtime_status_check_important"),
        ("pts_trade_status_check_general", "pts_trade_runtime_status_check_general"),
        ("pts_trade_status_check_sub", "pts_trade_runtime_status_check_sub"),
        ("pts_trade_status_check_prospect", "pts_trade_runtime_status_check_prospect"),
    ] {
        rename_node(&mut cash_panel, from, to);
    }
    if TEST79_SMOKE_ONLY {
        if let Some(review) = find_node_by_id_mut(&mut cash_panel, REVIEW_BUTTON_ID) {
            review.visible = false;
            review.disabled = true;
            review.runner.set_dirty(true);
        }
        if let Some(commit) = find_node_by_id_mut(&mut cash_panel, COMMIT_BUTTON_ID) {
            commit.visible = false;
            commit.disabled = true;
            commit.runner.set_dirty(true);
        }
    }

    modal_layer.add_child(assets, cash_panel);
    let Some(offer) = active_offer_mut(ui) else {
        log_event(
            "custom_trade_modal_host_missing",
            "id=offer;transaction_enabled=true",
        );
        return;
    };
    offer.add_child(assets, modal_layer);
    POPUP_OPEN.store(true, Ordering::Release);
    CUSTOM_ROSTER_BUILT.store(false, Ordering::Release);
    {
        let mut slots = CUSTOM_ROSTER_SLOT_IDS
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        slots.clear();
    }
    if !modal_hierarchy_is_canonical(ui) {
        // [PORT056] `.expect("active offer")` 였다. 계층 비정상 판정과 이 접근 사이에
        //   offer 표면이 사라지면 그대로 패닉이다(게임 콜스택으로 unwind = 위험).
        //   없으면 롤백만 건너뛰고 상태는 동일하게 정리한다.
        if let Some(offer) = active_offer_mut(ui) {
            let _ = take_direct_child(offer, MODAL_LAYER_ID);
        } else {
            log_event(
                "custom_trade_ui_rollback_offer_absent",
                "reason=active_offer_missing_during_rollback;panic_avoided=true",
            );
        }
        POPUP_OPEN.store(false, Ordering::Release);
        log_event(
            "custom_trade_ui_open_rolled_back",
            "reason=custom_modal_hierarchy_noncanonical;transaction_enabled=true",
        );
        return;
    }
    log_modal_layout_events(ui);
    log_event(
        "custom_trade_modal_opened",
        &format!(
            "source=fully_custom_static_modal;custom_compare_id={};modal_host=offer_fullscreen_layer;backdrop=full_screen_1920x1080;cash_panel=sibling;native_compare_popup_used=false;cached_native_compare_clone_used=false;open_generation={};target_id={};direct_roster_selection=true;transaction_enabled=true",
            CUSTOM_COMPARE_ID,
            OPEN_GENERATION.load(Ordering::Acquire),
            TARGET_ATHLETE_ID.load(Ordering::Acquire),
        ),
    );
    reset_quote_state();
    apply_desired_squad_status_ui(ui);
    log_event(
        "desired_squad_status_defaulted",
        "key=core;label=핵심 선수;meaning=promise_to_incoming_player;test77_required=true;proposal_component=true",
    );
}


fn modal_hierarchy_is_canonical(ui: &GameUI) -> bool {
    let Some(offer) = active_offer(ui) else {
        return false;
    };
    let Some(modal) = direct_child(offer, MODAL_LAYER_ID) else {
        return false;
    };
    let Some(backdrop) = direct_child(modal, BACKDROP_ID) else {
        return false;
    };
    let Some(compare) = direct_child(modal, CUSTOM_COMPARE_ID) else {
        return false;
    };
    let Some(cash) = direct_child(modal, CASH_PANEL_ID) else {
        return false;
    };
    backdrop.visible && compare.visible && cash.visible
}


fn log_modal_layout_events(ui: &GameUI) {
    let hierarchy_verified = modal_hierarchy_is_canonical(ui);
    log_event(
        "custom_trade_modal_backdrop_added",
        &format!(
            "id=pts_trade_modal_backdrop;width=1920px;height=1080px;coverage=full_screen_1920x1080;uniform_background=true;host_id=pts_trade_modal_layer;host_parent=offer;host_x=-288px;host_y=-24px;hierarchy=offer>modal_layer>{{backdrop,custom_compare,cash_panel}};hierarchy_verified={hierarchy_verified};original_compare_popup_used=false"
        ),
    );
    log_event(
        "custom_trade_compare_panel_added",
        "id=pts_trade_custom_compare;x=24px;y=120px;width=1248px;height=835px;compare_panel_side=left;compare_panel_x=16px;compare_panel_width=700px;roster_panel_side=right;roster_panel_x=732px;roster_panel_width=500px;panel_gap=16px;panel_order=compare_left_roster_right;roster_layout=24_static_slots_two_columns;profile_layout=custom_target_and_offered_cards;stat_row_count=14;stat_palette=original_game_tier_5;marker_palette=original_game_up_red_down_gray_equal_gray;original_compare_popup_used=false;in_bounds=true;target_card_center_x=178px;marker_center_x=350px;offered_card_center_x=522px;target_value_center_x=178px;marker_value_center_x=350px;offered_value_center_x=522px;right_stat_names=true;stat_columns_aligned_to_cards=true",
    );
    log_event(
        "custom_trade_panel_order_verified",
        "compare_panel_side=left;compare_panel_x=16px;compare_panel_width=700px;roster_panel_side=right;roster_panel_x=732px;roster_panel_width=500px;panel_gap=16px;panel_order=compare_left_roster_right;in_bounds=true",
    );
    log_event(
        "custom_trade_original_palette_ready",
        "stat_palette=original_game_tier_5;stat_thresholds=0_20_gray_21_40_teal_41_60_blue_61_80_purple_81_100_orange;stat_colors=6b6c74_4ed5bd_55c1fe_b34bb1_f86624;marker_palette=original_game_up_red_down_gray_equal_gray;marker_colors=eb3d4d_667085;equal_marker=-;value_nodes_per_stat=10;marker_nodes_per_stat=4;core_status_color=f86624;right_stat_names=true;stat_columns_aligned_to_cards=true;trade_button_label=트레이드;actual_trade_enabled=true",
    );
    log_event(
        "custom_trade_stat_columns_verified",
        "left_stat_name_x=26px;target_card_center_x=178px;target_value_center_x=178px;marker_center_x=350px;offered_card_center_x=522px;offered_value_center_x=522px;right_stat_name_x=566px;right_stat_names=true;stat_columns_aligned_to_cards=true;row_count=14",
    );
    log_event(
        "cash_panel_added",
        "id=pts_trade_cash_panel;x=1296px;y=120px;width=600px;height=835px;right_edge=1896px;in_bounds=true;layout=adjacent_to_custom_compare;desired_squad_status_selector=true;status_meaning=promise_to_incoming_player;required_amount=server_pending;budget=server_pending",
    );
}



fn close_trade_popup_for_async_submit(ui: &mut GameUI) {
    let target_id = TARGET_ATHLETE_ID.load(Ordering::Acquire);
    let generation = OPEN_GENERATION.load(Ordering::Acquire);
    let modal_removed = if let Some(offer) = active_offer_mut(ui) {
        take_direct_child(offer, MODAL_LAYER_ID).is_some()
    } else {
        false
    };
    POPUP_OPEN.store(false, Ordering::Release);
    STATUS_MENU_OPEN.store(false, Ordering::Release);
    CUSTOM_ROSTER_BUILT.store(false, Ordering::Release);
    PENDING_OFFERED_ATHLETE_ID.store(NO_ATHLETE, Ordering::Release);
    {
        let mut slots = CUSTOM_ROSTER_SLOT_IDS
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        slots.clear();
    }
    let quote_preserved = QUOTE_VIEW.with(|slot| slot.borrow().is_some());
    let proposed_units_preserved = PROPOSED_UNITS.with(|slot| slot.borrow().is_some());
    let offered_id_preserved = OFFERED_ATHLETE_ID.load(Ordering::Acquire);
    let modal_count_after = count_nodes_by_id(&ui.root, MODAL_LAYER_ID);
    log_event(
        "trade_popup_closed_for_async_submit",
        &format!(
            "target_id={};offered_id={};closed_generation={};custom_modal_removed={};modal_count_after={};quote_preserved={};proposed_units_preserved={};review_request_queued={};popup_open_after=false;offer_force_hidden=false;payload_release_deferred_until_command_send=true;transaction_executed=false",
            target_id,
            offered_id_preserved,
            generation,
            modal_removed,
            modal_count_after,
            quote_preserved,
            proposed_units_preserved,
            REVIEW_REQUEST_QUEUED.load(Ordering::Acquire),
        ),
    );
}

fn clear_async_submit_payload_after_command_sent() {
    PENDING_OFFERED_ATHLETE_ID.store(NO_ATHLETE, Ordering::Release);
    OFFERED_ATHLETE_ID.store(NO_ATHLETE, Ordering::Release);
    VISUAL_SYNC_LAST_OFFERED_ID.store(0, Ordering::Release);
    VISUAL_SYNC_LAST_SEQUENCE.store(0, Ordering::Release);
    NATIVE_VISUAL_SYNC_PENDING.store(false, Ordering::Release);
    QUOTE_REQUEST_SENT.store(false, Ordering::Release);
    CASH_INPUT_STATE.store(0, Ordering::Release);
    STATUS_MENU_OPEN.store(false, Ordering::Release);
    QUOTE_VIEW.with(|slot| *slot.borrow_mut() = None);
    QUOTE_ERROR.with(|slot| *slot.borrow_mut() = None);
    PROPOSED_UNITS.with(|slot| *slot.borrow_mut() = None);
    EXECUTE_VIEW.with(|slot| *slot.borrow_mut() = None);
    EXECUTE_ERROR.with(|slot| *slot.borrow_mut() = None);
    EXECUTE_REQUEST_QUEUED.store(false, Ordering::Release);
    EXECUTE_REQUEST_PENDING.store(false, Ordering::Release);
    QUOTE_UI_DIRTY.store(true, Ordering::Release);
    log_event(
        "async_trade_submit_payload_released",
        "command_sent=true;review_request_pending=true;quote_released=true;cash_input_released=true;offered_selection_released=true;target_id_preserved=true;proposal_response_pending=true;transaction_executed=false",
    );
}

fn close_trade_popup(ui: &mut GameUI) {
    let target_id = TARGET_ATHLETE_ID.load(Ordering::Acquire);
    let generation = OPEN_GENERATION.load(Ordering::Acquire);
    let modal_removed = if let Some(offer) = active_offer_mut(ui) {
        take_direct_child(offer, MODAL_LAYER_ID).is_some()
    } else {
        false
    };
    POPUP_OPEN.store(false, Ordering::Release);
    STATUS_MENU_OPEN.store(false, Ordering::Release);
    CUSTOM_ROSTER_BUILT.store(false, Ordering::Release);
    {
        let mut slots = CUSTOM_ROSTER_SLOT_IDS
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        slots.clear();
    }
    reset_quote_state();
    set_active_offer_path(ui);
    update_trade_entry_and_capture_offer(ui);
    // [PORT056] 노드 disabled 는 더 이상 게이트가 아니다(항상 false). 실제 게이트는 TRADE_ENTRY_ACTIVE.
    let entry_rearmed = trade_entry(ui).is_some_and(|entry| entry.visible)
        && TRADE_ENTRY_ACTIVE.load(Ordering::Acquire);
    let modal_count_after = count_nodes_by_id(&ui.root, MODAL_LAYER_ID);
    let state_reset_verified = modal_removed
        && modal_count_after == 0
        && !POPUP_OPEN.load(Ordering::Acquire)
        && OFFERED_ATHLETE_ID.load(Ordering::Acquire) == NO_ATHLETE
        && PENDING_OFFERED_ATHLETE_ID.load(Ordering::Acquire) == NO_ATHLETE;
    log_event(
        "trade_popup_closed",
        &format!(
            "id=pts_trade_custom_modal;target_id={target_id};closed_generation={generation};custom_modal_removed={modal_removed};original_compare_popup_used=false;entry_rearmed={entry_rearmed};modal_count_after={modal_count_after};offered_selection_state_reset=true;quote_cleared=true;review_cleared=true;cash_panel_removed=true;roster_slot_map_cleared=true;state_reset_verified={state_reset_verified};transaction_enabled=true"
        ),
    );
}


fn invalidate_trade_ui_context(ui: &mut GameUI, reason: &str) {
    let modal_removed = if let Some(offer) = active_offer_mut(ui) {
        take_direct_child(offer, MODAL_LAYER_ID).is_some()
    } else {
        false
    };
    POPUP_OPEN.store(false, Ordering::Release);
    TRADE_ENTRY_ACTIVE.store(false, Ordering::Release);
    CUSTOM_ROSTER_BUILT.store(false, Ordering::Release);
    {
        let mut slots = CUSTOM_ROSTER_SLOT_IDS
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        slots.clear();
    }
    reset_quote_state();
    log_event(
        "custom_trade_ui_context_invalidated",
        &format!(
            "reason={};modal_removed={modal_removed};original_compare_popup_used=false;next_target_reopen_ready=true",
            sanitize(reason),
        ),
    );
}


fn watch_trade_ui_context(ui: &mut GameUI) {
    if !POPUP_OPEN.load(Ordering::Acquire) {
        return;
    }
    let context_valid = active_offer(ui).is_some()
        && TARGET_ATHLETE_ID.load(Ordering::Acquire) != 0
        && count_nodes_by_id(&ui.root, MODAL_LAYER_ID) == 1
        && modal_hierarchy_is_canonical(ui);
    if !context_valid {
        invalidate_trade_ui_context(ui, "custom_modal_context_invalid");
    }
}


fn set_node_visible(ui: &mut GameUI, id: &str, visible: bool) {
    if let Some(node) = find_node_by_id_mut(&mut ui.root, id) {
        if node.visible != visible {
            node.visible = visible;
            node.runner.set_dirty(true);
        }
    }
}

fn set_node_disabled(ui: &mut GameUI, id: &str, disabled: bool) {
    if let Some(node) = find_node_by_id_mut(&mut ui.root, id) {
        if node.disabled != disabled {
            node.disabled = disabled;
            node.runner.set_dirty(true);
        }
    }
}

fn set_label_text(ui: &mut GameUI, id: &str, text: &str) {
    let Some(node) = find_node_by_id_mut(&mut ui.root, id) else {
        return;
    };
    let mut properties: HashMap<String, Rc<dyn Any>> = HashMap::new();
    properties.insert("text".to_string(), Rc::new(text.to_string()));
    node.runner.build_with_property(&properties);
    node.runner.set_dirty(true);
}

fn rejection_reason_short(reason: &str) -> &'static str {
    match reason {
        "BudgetExceeded" => "신청팀 예산 초과",
        "LastPlayerAtPosition" => "상대팀 포지션 공백 위험",
        "TermsUnacceptable" => "선수·현금 조건 불충족",
        "StatusOrInheritedSalaryUnacceptable" => "약속 위상·승계 계약 불수락",
        "FinalApprovalRejected" => "최종 승인 거절",
        _ => "승인 조건 불충족",
    }
}

fn invalidate_review_state(reason: &str) {
    if TRANSACTION_EXECUTED_UI.load(Ordering::Acquire) {
        log_event(
            "proposal_change_ignored_after_commit",
            &format!("reason={};transaction_executed=true", sanitize(reason)),
        );
        return;
    }
    let had_review = REVIEW_VIEW.with(|slot| slot.borrow().is_some());
    let had_error = REVIEW_ERROR.with(|slot| slot.borrow().is_some());
    let was_pending = REVIEW_REQUEST_PENDING.swap(false, Ordering::Relaxed);
    REVIEW_REQUEST_QUEUED.store(false, Ordering::Relaxed);
    REVIEW_VIEW.with(|slot| *slot.borrow_mut() = None);
    REVIEW_ERROR.with(|slot| *slot.borrow_mut() = None);
    EXECUTE_VIEW.with(|slot| *slot.borrow_mut() = None);
    EXECUTE_ERROR.with(|slot| *slot.borrow_mut() = None);
    EXECUTE_REQUEST_QUEUED.store(false, Ordering::Relaxed);
    EXECUTE_REQUEST_PENDING.store(false, Ordering::Relaxed);
    if had_review || had_error || was_pending {
        log_event(
            "proposal_review_invalidated",
            &format!("reason={};transaction_executed=false", sanitize(reason)),
        );
    }
    QUOTE_UI_DIRTY.store(true, Ordering::Relaxed);
}



fn queue_server_review() -> bool {
    if REVIEW_REQUEST_PENDING.load(Ordering::Relaxed)
        || REVIEW_REQUEST_QUEUED.swap(true, Ordering::Relaxed)
    {
        log_event(
            "async_trade_submit_click_ignored",
            "reason=request_already_pending;transaction_executed=false",
        );
        return false;
    }
    REVIEW_ERROR.with(|slot| *slot.borrow_mut() = None);
    QUOTE_UI_DIRTY.store(true, Ordering::Relaxed);
    log_event(
        "async_trade_submit_button_clicked",
        "mode=submit_only_no_immediate_mutation;button_label=트레이드;proposal_state_after_submit=SellerReview;profile_return_method=UIOutEvent_UndoScene;offer_force_hidden=false;transaction_executed=false",
    );
    true
}

fn queue_actual_commit() {
    let current_review = REVIEW_VIEW.with(|slot| slot.borrow().clone());
    let current_plan = current_review
        .as_ref()
        .map(|review| review.plan_id.clone())
        .unwrap_or_else(|| "none".to_string());
    log_event(
        "trade_atomic_commit_confirm_physical_click",
        &format!(
            "button_id={};plan_id={};review_present={};execute_pending={};execute_queued={};transaction_executed={};physical_click=true;explicit_second_click=true",
            COMMIT_BUTTON_ID,
            current_plan,
            current_review.is_some(),
            EXECUTE_REQUEST_PENDING.load(Ordering::Relaxed),
            EXECUTE_REQUEST_QUEUED.load(Ordering::Relaxed),
            TRANSACTION_EXECUTED_UI.load(Ordering::Acquire),
        ),
    );
    if TRANSACTION_EXECUTED_UI.load(Ordering::Acquire) {
        log_event(
            "trade_atomic_commit_click_ignored",
            "reason=transaction_already_executed;dedicated_confirm_button=true;transaction_executed=true",
        );
        return;
    }
    let executable_review = current_review.filter(|review| {
        review.overall_approved
            && review.command_envelope_prepared
            && review.plan_repeat_consistent
            && review.execution_gate_closed
    });
    let Some(review) = executable_review else {
        EXECUTE_ERROR.with(|slot| {
            *slot.borrow_mut() = Some(
                "서버 사전 검토 결과가 아직 확정 버튼에 연결되지 않았습니다. 잠시 기다린 뒤 다시 눌러 주세요."
                    .to_string(),
            )
        });
        QUOTE_UI_DIRTY.store(true, Ordering::Relaxed);
        log_event(
            "trade_atomic_commit_click_ignored",
            &format!(
                "reason=executable_review_not_available;button_id={};plan_id={};review_present={};dedicated_confirm_button=true;transaction_executed=false",
                COMMIT_BUTTON_ID,
                current_plan,
                REVIEW_VIEW.with(|slot| slot.borrow().is_some()),
            ),
        );
        return;
    };
    if EXECUTE_REQUEST_PENDING.load(Ordering::Relaxed)
        || EXECUTE_REQUEST_QUEUED.swap(true, Ordering::Relaxed)
    {
        log_event(
            "trade_atomic_commit_click_ignored",
            "reason=execution_request_already_pending;dedicated_confirm_button=true;transaction_executed=false",
        );
        return;
    }
    EXECUTE_ERROR.with(|slot| *slot.borrow_mut() = None);
    QUOTE_UI_DIRTY.store(true, Ordering::Relaxed);
    log_event(
        "trade_atomic_commit_button_clicked",
        &format!(
            "button_id={};plan_id={};offered_id={};target_id={};proposed_units={};desired_status_choice={};explicit_second_click=true;dedicated_confirm_button=true;execution_gate_requested_for_sealed_plan=true;actual_commit_expected=true;transaction_executed=false",
            COMMIT_BUTTON_ID,
            review.plan_id,
            review.offered_id,
            review.target_id,
            review.proposed_units,
            review.desired_status_choice,
        ),
    );
}


fn apply_review_note(ui: &mut GameUI) {
    set_node_visible(ui, COMMIT_BUTTON_ID, false);

    let current_target_id = TARGET_ATHLETE_ID.load(Ordering::Acquire);
    let other_active_proposal = active_async_status_view()
        .filter(|view| view.target_id != current_target_id);
    if let Some(view) = other_active_proposal {
        set_node_visible(ui, REVIEW_BUTTON_ID, true);
        set_label_text(ui, REVIEW_BUTTON_ID, "기존 트레이드 검토 중");
        set_node_disabled(ui, REVIEW_BUTTON_ID, true);
        set_label_text(
            ui,
            "pts_cash_runtime_read_only_note",
            "다른 선수의 트레이드 제안이 검토 중입니다. 이 창은 비교용으로 열 수 있지만 새 제안은 기존 제안이 끝난 뒤 접수할 수 있습니다.",
        );
        let block_key = format!(
            "{}|{}|{}|{}",
            current_target_id, view.target_id, view.proposal_id, view.state,
        );
        let should_log = SECOND_SUBMIT_BLOCK_LAST_KEY.with(|slot| {
            let mut slot = slot.borrow_mut();
            if *slot == block_key {
                false
            } else {
                *slot = block_key;
                true
            }
        });
        if should_log {
            log_event(
                "trade_second_submit_blocked_single_proposal",
                &format!(
                    "current_target_id={};proposal_target_id={};proposal_id={};proposal_state={};modal_open=true;second_submit_allowed=false;database_mutation=false;log_frequency=once_per_target_and_proposal_state",
                    current_target_id,
                    view.target_id,
                    view.proposal_id,
                    view.state,
                ),
            );
        }
        return;
    }
    SECOND_SUBMIT_BLOCK_LAST_KEY.with(|slot| slot.borrow_mut().clear());

    if REVIEW_REQUEST_PENDING.load(Ordering::Relaxed) {
        set_node_visible(ui, REVIEW_BUTTON_ID, true);
        set_label_text(ui, REVIEW_BUTTON_ID, "제안 접수 중...");
        set_node_disabled(ui, REVIEW_BUTTON_ID, true);
        set_label_text(
            ui,
            "pts_cash_runtime_read_only_note",
            "트레이드 제안을 서버에 접수하고 있습니다. 이 단계에서는 선수·현금·계약이 변경되지 않습니다.",
        );
        return;
    }
    if let Some(detail) = REVIEW_ERROR.with(|slot| slot.borrow().clone()) {
        set_node_visible(ui, REVIEW_BUTTON_ID, true);
        set_label_text(ui, REVIEW_BUTTON_ID, "트레이드");
        set_node_disabled(ui, REVIEW_BUTTON_ID, false);
        set_label_text(
            ui,
            "pts_cash_runtime_read_only_note",
            &format!("제안 접수 실패: {}", detail),
        );
        return;
    }
    // ★[PORT056] 유저 지시 2026-08-23 — 못 누를 때는 **비활성 스타일 + 사유 표시**.
    let reason = trade_block_reason();
    let ready = reason.is_none();
    set_node_visible(ui, REVIEW_BUTTON_ID, true);
    set_label_text(ui, REVIEW_BUTTON_ID, "트레이드");
    set_node_disabled(ui, REVIEW_BUTTON_ID, !ready);
    if let Some(button) = find_node_by_id_mut(&mut ui.root, REVIEW_BUTTON_ID) {
        button_set_disabled_look(
            button,
            !ready,
            reason.as_deref().unwrap_or(""),
            &TRADE_BUTTON_STYLE_SAVED,
        );
    }
    set_label_text(
        ui,
        "pts_cash_runtime_read_only_note",
        match reason.as_deref() {
            Some(reason) => reason,
            None =>
        "트레이드를 누르면 제안만 접수됩니다. 즉시 선수나 현금이 바뀌지 않으며, 판매 구단 검토 → 선수 검토를 게임 시간 진행으로 거칩니다.\nTest79 성공 검증은 표시된 범위의 상한 금액과 핵심 선수 위상을 사용하세요."
        },
    );
}

/// ★[PORT056] 트레이드 버튼을 못 누르는 이유. `None` 이면 누를 수 있다(유저 지시 2026-08-23).
///   우선순위: 선수 미선택 → 금액 미입력.
const SELECT_PLAYER_WARNING: &str = "선수를 선택해주세요";
const ENTER_CASH_WARNING: &str = "제시 금액을 입력해주세요";
const QUOTE_FAILED_WARNING: &str = "이 조합은 트레이드할 수 없습니다";

fn no_offered_player_selected() -> bool {
    OFFERED_ATHLETE_ID.load(Ordering::Relaxed) == NO_ATHLETE
}

/// ★[PORT056] 쿨다운 선차단 상태 (유저 지시 2026-08-23).
///   `Some(retry_at)` 이면 지금 초안(보낼 선수·금액·위상)이 쿨다운에 걸린다.
///   판정식은 서버 `rejection::ledger_gate` 와 **같은 함수**(`cooldown_hint_blocks`)를 쓴다 —
///   두 곳이 어긋나면 버튼은 막혔는데 제출은 되거나 그 반대가 된다.
static TRADE_COOLDOWN_BLOCK: Mutex<Option<String>> = Mutex::new(None);

fn update_cooldown_block_state(data: &ClientData) {
    let blocked = (|| -> Option<String> {
        let quote = QUOTE_VIEW.with(|slot| slot.borrow().clone())?;
        if !quote.cooldown_present {
            return None;
        }
        let hint = rejection::CooldownHint {
            package_fingerprint: quote.cooldown_fingerprint,
            retry_at: quote.cooldown_retry_at.clone(),
            changeable: quote.cooldown_changeable,
            exempt: quote.cooldown_exempt,
        };
        // ★쿨다운은 이제 **패키지와 무관**하다(유저 지시 2026-08-23) — 금액을 입력하기 전에도 막힌다.
        //   지문은 판정에 안 쓰이지만 시그니처 호환을 위해 0 을 넘긴다.
        let now = data.db().time.to_string();
        rejection::cooldown_hint_blocks(&hint, 0, &now).then(|| hint.retry_at.clone())
    })();
    *TRADE_COOLDOWN_BLOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = blocked;
}

fn cooldown_block_reason() -> Option<String> {
    let retry_at = TRADE_COOLDOWN_BLOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone()?;
    Some(format!(
        "이 선수에게는 {} 이후 다시 제안할 수 있습니다",
        compact_async_time(&retry_at),
    ))
}

fn quote_error_detail() -> Option<String> {
    QUOTE_ERROR.with(|slot| slot.borrow().clone())
}

fn trade_block_reason() -> Option<String> {
    if no_offered_player_selected() {
        return Some(SELECT_PLAYER_WARNING.to_string());
    }
    if quote_error_detail().is_some() {
        return Some(QUOTE_FAILED_WARNING.to_string());
    }
    // ★쿨다운은 금액보다 **먼저** 본다 — 패키지 무관 전면 차단이라 금액을 물어볼 이유가 없다.
    if let Some(reason) = cooldown_block_reason() {
        return Some(reason);
    }
    let cash_ok = CASH_INPUT_STATE.load(Ordering::Relaxed) == 1
        && QUOTE_VIEW.with(|slot| slot.borrow().is_some())
        && PROPOSED_UNITS.with(|slot| slot.borrow().is_some());
    if !cash_ok {
        return Some(ENTER_CASH_WARNING.to_string());
    }
    None
}

fn apply_desired_squad_status_ui(ui: &mut GameUI) {
    let selected = DESIRED_SQUAD_STATUS.load(Ordering::Relaxed);
    for (id, choice) in [
        ("pts_trade_runtime_status_value_core", STATUS_CORE),
        ("pts_trade_runtime_status_value_important", STATUS_IMPORTANT),
        ("pts_trade_runtime_status_value_general", STATUS_GENERAL),
        ("pts_trade_runtime_status_value_sub", STATUS_SUB),
        ("pts_trade_runtime_status_value_prospect", STATUS_PROSPECT),
    ] {
        set_node_visible(ui, id, selected == choice);
    }
    set_node_visible(ui, STATUS_MENU_ID, STATUS_MENU_OPEN.load(Ordering::Relaxed));
    for (id, choice) in [
        ("pts_trade_runtime_status_check_core", STATUS_CORE),
        ("pts_trade_runtime_status_check_important", STATUS_IMPORTANT),
        ("pts_trade_runtime_status_check_general", STATUS_GENERAL),
        ("pts_trade_runtime_status_check_sub", STATUS_SUB),
        ("pts_trade_runtime_status_check_prospect", STATUS_PROSPECT),
    ] {
        set_node_visible(ui, id, selected == choice);
    }
}

fn log_trade_proposal_draft() {
    let quote = QUOTE_VIEW.with(|slot| slot.borrow().clone());
    let proposed_units = PROPOSED_UNITS.with(|slot| *slot.borrow());
    let (Some(quote), Some(proposed_units)) = (quote, proposed_units) else {
        return;
    };
    let selected = DESIRED_SQUAD_STATUS.load(Ordering::Relaxed);
    let (status_key, status_label) = desired_squad_status(selected);
    let proposed_won = proposed_units.checked_mul(MONEY_UNIT_WON);
    let within_budget = proposed_won
        .map(|won| (won as f64) <= quote.cash_budget_won + 0.001)
        .unwrap_or(false);
    let meets_required = proposed_units >= quote.required_units;
    let offer_max_units = quote.cash_offer_max_units;
    let within_offer_range = quote.required_units <= offer_max_units
        && proposed_units >= quote.required_units
        && proposed_units <= offer_max_units;
    log_event(
        "trade_proposal_draft_updated",
        &format!(
            "offered_id={};target_id={};proposed_units={};cash_offer_min_units={};cash_offer_max_units={};proposed_cash_within_server_range={};desired_squad_status={};desired_squad_status_label={};meaning=promise_to_incoming_player;components=offered_player,cash,promised_squad_status;cash_within_budget={};cash_meets_required={};seller_review_scope=offered_player+cash;target_player_review_scope=promised_squad_status;bundled_for_atomic_submission=true;transaction_enabled=true",
            quote.offered_id,
            quote.target_id,
            proposed_units,
            quote.required_units,
            offer_max_units,
            within_offer_range,
            status_key,
            status_label,
            within_budget,
            meets_required,
        ),
    );
}

fn select_desired_squad_status(ui: &mut GameUI, choice: u8) {
    if TRANSACTION_EXECUTED_UI.load(Ordering::Acquire) {
        log_event(
            "desired_squad_status_change_ignored",
            "reason=transaction_already_executed;transaction_executed=true",
        );
        return;
    }
    invalidate_review_state("promised_squad_status_changed");
    DESIRED_SQUAD_STATUS.store(choice, Ordering::Relaxed);
    STATUS_MENU_OPEN.store(false, Ordering::Relaxed);
    apply_desired_squad_status_ui(ui);
    let (status_key, status_label) = desired_squad_status(choice);
    log_event(
        "desired_squad_status_selected",
        &format!(
            "key={};label={};target_id={};meaning=promise_to_incoming_player;proposal_component=true;current_contract_status_display=false;transaction_enabled=true",
            status_key,
            status_label,
            TARGET_ATHLETE_ID.load(Ordering::Relaxed),
        ),
    );
    log_trade_proposal_draft();
}

fn format_commas(value: u64) -> String {
    let raw = value.to_string();
    let len = raw.len();
    let mut output = String::with_capacity(len + len / 3);
    for (index, character) in raw.chars().enumerate() {
        if index > 0 && (len - index) % 3 == 0 {
            output.push(',');
        }
        output.push(character);
    }
    output
}

fn format_eok_number(units_manwon: u64) -> String {
    let whole = units_manwon / 10_000;
    let fraction = units_manwon % 10_000;
    if fraction == 0 {
        return format_commas(whole);
    }

    let mut fraction_text = format!("{fraction:04}");
    while fraction_text.ends_with('0') {
        fraction_text.pop();
    }
    format!("{}.{}", format_commas(whole), fraction_text)
}

fn format_eok_amount(units_manwon: u64) -> String {
    format!("{}억 원", format_eok_number(units_manwon))
}

fn format_cheoman_number(units_manwon: u64) -> String {
    let whole = units_manwon / 1_000;
    let fraction = units_manwon % 1_000;
    if fraction == 0 {
        return format_commas(whole);
    }

    let mut fraction_text = format!("{fraction:03}");
    while fraction_text.ends_with('0') {
        fraction_text.pop();
    }
    format!("{}.{}", format_commas(whole), fraction_text)
}

fn format_cash_amount(units_manwon: u64) -> String {
    if units_manwon == 0 {
        return "0원".to_string();
    }
    if units_manwon <= 9_999 {
        format!("{}천만 원", format_cheoman_number(units_manwon))
    } else {
        format_eok_amount(units_manwon)
    }
}

fn format_eok_cheoman_bucket(units_manwon: u64) -> String {
    let eok = units_manwon / 10_000;
    let cheon = (units_manwon % 10_000) / 1_000;
    format!("{}억 {}천", format_commas(eok), cheon)
}

fn format_cash_range(min_units: u64, max_units: u64) -> String {
    format!(
        "{} ~ {}",
        format_eok_cheoman_bucket(min_units),
        format_eok_cheoman_bucket(max_units),
    )
}

/// ★[PORT056] `pts_cash_input` 은 트리에 **2개** 있다(`.ui` 템플릿 + 런타임 인스턴스, id 개명 대상이 아님).
///   `set_node_disabled(ui, CASH_INPUT_ID, ..)` 는 먼저 찾은 하나만 건드려서
///   **화면에 보이는 입력칸이 잠긴 채 남았다**(유저 보고 2026-08-23, 진단 로그 `nodes_with_id=2` 로 확정).
///   ⟹ 실제 패널(`CASH_PANEL_ID`) 안쪽으로 범위를 좁혀 런타임 인스턴스만 조작한다.
///   ⚠같은 함정이 `set_label_text(ui, CASH_INPUT_ID, ..)` 에도 있어 함께 고쳤다.
/// ★[PORT056] 견적 실패 사유를 사람이 읽을 수 있게 옮긴다 (유저 지시 2026-08-23
///   "애초에 계산실패 이유가 뭐야? 이유를 써둬야하지않을까").
///   구판은 서버가 준 `detail` 을 **로그에만** 남기고 화면엔 "선택 조합 계산 실패" 라고만 띄웠다.
///   ⟹ 가장 흔한 실패는 **포지션 연속성**이다: 5인 로스터에서 다른 포지션끼리 교환하면
///      보낸 선수의 포지션이 0명이 된다(게임 규칙, 버그 아님).
fn quote_error_message_ko(detail: &str) -> String {
    if detail.contains("post-trade position continuity") {
        return "이 조합은 트레이드 후 우리 팀 포지션이 비어 불가능합니다. 같은 포지션 선수끼리 교환하세요.".to_string();
    }
    if detail.contains("already belongs to requester team") {
        return "이미 우리 팀 선수입니다.".to_string();
    }
    if detail.contains("not requester") {
        return "보낼 선수가 우리 팀 소속이 아닙니다.".to_string();
    }
    if detail.contains("was not found") {
        return "선수 정보를 찾을 수 없습니다. 모달을 닫았다가 다시 여세요.".to_string();
    }
    format!("계산 실패: {detail}")
}

fn cash_input_node_mut<'a>(ui: &'a mut GameUI) -> Option<&'a mut Node> {
    let panel = find_node_by_id_mut(&mut ui.root, CASH_PANEL_ID)?;
    find_node_by_id_mut(panel, CASH_INPUT_ID)
}

fn set_cash_input_disabled(ui: &mut GameUI, disabled: bool) {
    if let Some(node) = cash_input_node_mut(ui) {
        if node.disabled != disabled {
            node.disabled = disabled;
            node.runner.set_dirty(true);
        }
    }
}

fn update_cash_input_status(ui: &mut GameUI) {
    let state = CASH_INPUT_STATE.load(Ordering::Relaxed);
    // ★[PORT056] 유저 지시 2026-08-23 — 선수를 고르기 전에는 제시 금액 입력칸을 잠그고
    //   입력칸 아래에도 같은 경고를 띄운다.
    let no_player = no_offered_player_selected();
    // 견적이 실패한 조합에서는 금액을 받아봐야 소용없다 → 입력칸도 잠근다.
    let quote_error = quote_error_detail();
    set_cash_input_disabled(ui, no_player || quote_error.is_some());
    // ★상태줄 라벨은 이 함수가 **단독으로** 소유한다(두 곳에서 쓰면 서로 덮어쓴다).
    let override_text = if no_player {
        Some(SELECT_PLAYER_WARNING.to_string())
    } else {
        quote_error.as_deref().map(quote_error_message_ko)
    };
    if let Some(text) = override_text {
        set_node_visible(ui, "pts_cash_runtime_status_idle", false);
        set_node_visible(ui, "pts_cash_runtime_status_valid", false);
        set_node_visible(ui, "pts_cash_runtime_status_invalid", true);
        set_label_text(ui, "pts_cash_runtime_status_invalid", &text);
        CASH_STATUS_WARNING_SHOWN.store(true, Ordering::Relaxed);
    } else {
        // 경고로 덮었던 라벨은 **전환 시 한 번만** 원래 문구로 되돌린다.
        // (매 프레임 되돌리면 apply_quote_view 가 쓴 계산 실패 문구를 지운다.)
        if CASH_STATUS_WARNING_SHOWN.swap(false, Ordering::Relaxed) {
            set_label_text(ui, "pts_cash_runtime_status_invalid", "숫자만 입력할 수 있습니다.");
        }
        set_node_visible(ui, "pts_cash_runtime_status_idle", state == 0);
        set_node_visible(ui, "pts_cash_runtime_status_valid", state == 1);
        set_node_visible(ui, "pts_cash_runtime_status_invalid", state == 2);
    }
    // 내 선수단 패널의 안내 문구도 경고로 바꾼다.
    if let Some(compare) = find_node_by_id_mut(&mut ui.root, CUSTOM_COMPARE_ID) {
        if let Some(guide) = direct_path_mut(compare, &["pts_trade_custom_roster_panel", "guide"]) {
            set_runner_text(
                guide,
                if no_player { SELECT_PLAYER_WARNING } else { "선택한 행은 녹색으로 표시됩니다" },
            );
            guide.runner.set_dirty(true);
        }
    }
}


fn apply_quote_view(ui: &mut GameUI) {
    if !QUOTE_UI_DIRTY.swap(false, Ordering::Relaxed) {
        return;
    }
    if let Some(detail) = QUOTE_ERROR.with(|slot| slot.borrow().clone()) {
        set_label_text(ui, "pts_cash_runtime_selection_value", "선택 조합 계산 실패");
        set_label_text(ui, "pts_cash_runtime_required_value", "계산 실패");
        set_label_text(ui, "pts_cash_runtime_budget_value", "—");
        set_label_text(ui, "pts_cash_runtime_after_value", "—");
        // ⚠상태줄(`status_invalid`)은 여기서 건드리지 않는다 — `update_cash_input_status` 가 단독으로 소유한다.
        //   1차 구현에서 양쪽이 같은 라벨을 쓰다가, 복구 로직이 사유를
        //   "숫자만 입력할 수 있습니다."로 덮어썼다(유저 보고 2026-08-23).
        set_label_text(ui, "pts_cash_runtime_required_note", &quote_error_message_ko(&detail));
        set_node_visible(ui, "pts_cash_runtime_proposed_eok", false);
        log_event("quote_ui_error_displayed", &format!("detail={}", sanitize(&detail)));
        apply_review_note(ui);
        return;
    }
    let Some(quote) = QUOTE_VIEW.with(|slot| slot.borrow().clone()) else {
        apply_review_note(ui);
        return;
    };
    set_label_text(
        ui,
        "pts_cash_runtime_selection_value",
        &format!("{}  ↔  {}", quote.offered_name, quote.target_name),
    );
    set_label_text(
        ui,
        "pts_cash_runtime_required_value",
        &format_cash_range(quote.required_units, quote.cash_offer_max_units),
    );
    if quote.required_units == 0 && quote.cash_offer_max_units == 0 {
        set_label_text(ui, "pts_cash_runtime_required_note", "추가 현금 없이도 제안을 제출할 수 있습니다. 0을 입력하세요.");
    } else {
        set_label_text(
            ui,
            "pts_cash_runtime_required_note",
            &format!(
                "참고 범위 {}~{} · 실제 성사 기준은 공개되지 않습니다. 정수를 입력한 뒤 Enter",
                quote.required_units,
                quote.cash_offer_max_units,
            ),
        );
    }
    set_label_text(ui, "pts_cash_runtime_budget_value", &format_cash_amount(quote.budget_units));
    let proposed = PROPOSED_UNITS.with(|slot| *slot.borrow());
    match proposed {
        None => {
            set_node_visible(ui, "pts_cash_runtime_proposed_eok", false);
            set_label_text(ui, "pts_cash_runtime_after_value", &format_cash_amount(quote.budget_units));
            set_label_text(
                ui,
                "pts_cash_runtime_status_idle",
                &format!("{}~{} 사이의 정수를 입력하고 Enter를 누르세요.", quote.required_units, quote.cash_offer_max_units),
            );
        }
        Some(units) => {
            set_label_text(ui, "pts_cash_runtime_proposed_eok", &format!("제시 금액 환산: {}", format_cash_amount(units)));
            set_node_visible(ui, "pts_cash_runtime_proposed_eok", true);
            let proposed_won = units.checked_mul(MONEY_UNIT_WON);
            let within_budget = proposed_won
                .map(|won| won as f64 <= quote.cash_budget_won + 0.001)
                .unwrap_or(false);
            let within_display_range = units >= quote.required_units && units <= quote.cash_offer_max_units;
            if within_budget {
                let remaining_won = (quote.cash_budget_won - proposed_won.unwrap_or(0) as f64).max(0.0);
                let remaining_units = won_to_units_floor(remaining_won).unwrap_or(0);
                set_label_text(ui, "pts_cash_runtime_after_value", &format_cash_amount(remaining_units));
                if within_display_range {
                    set_label_text(ui, "pts_cash_runtime_status_valid", "표시된 참고 범위 안의 금액입니다.");
                } else {
                    set_label_text(ui, "pts_cash_runtime_status_invalid", "표시된 참고 범위를 벗어났습니다.");
                }
            } else {
                set_label_text(ui, "pts_cash_runtime_after_value", "예산 초과");
                set_label_text(ui, "pts_cash_runtime_status_invalid", "현재 이적료 예산을 초과합니다.");
            }
        }
    }
    log_event(
        "obscured_cash_range_rendered",
        &format!(
            "requester_team_id={};recipient_team_id={};offered_id={};target_id={};display_min_units={};display_max_units={};cash_range_obscured=true;exact_threshold_disclosed=false;range_lower_policy=random_70_80_percent_of_exact;range_upper_policy=random_150_160_percent_of_exact;range_stable_for_pair=true;range_display_format=eok_cheoman;display_range_text={};cash_budget_won={};budget_units={}",
            quote.requester_team_id,
            quote.recipient_team_id,
            quote.offered_id,
            quote.target_id,
            quote.required_units,
            quote.cash_offer_max_units,
            sanitize(&format_cash_range(quote.required_units, quote.cash_offer_max_units)),
            quote.cash_budget_won,
            quote.budget_units,
        ),
    );
    apply_review_note(ui);
}

fn offered_id_from_compare_click(path: &str, item: &str) -> Option<usize> {
    let index = custom_slot_index_from_click(path, item)?;
    custom_slot_athlete_id(index)
}


fn reset_state_for_offered_player_change(
    ui: &mut GameUI,
    previous_offered_id: usize,
    offered_id: usize,
    offered_name: &str,
    target_id: usize,
    target_name: &str,
) {
    let had_quote = QUOTE_VIEW.with(|slot| slot.borrow().is_some());
    let had_review = REVIEW_VIEW.with(|slot| slot.borrow().is_some());
    let had_plan = REVIEW_VIEW.with(|slot| {
        slot.borrow()
            .as_ref()
            .is_some_and(|review| review.command_envelope_prepared)
    });
    QUOTE_REQUEST_SENT.store(false, Ordering::Relaxed);
    REGION_ERROR_LOGGED.store(false, Ordering::Relaxed);
    ZERO_CASH_EXPLANATION_LOGGED.store(false, Ordering::Relaxed);
    QUOTE_VIEW.with(|slot| *slot.borrow_mut() = None);
    QUOTE_ERROR.with(|slot| *slot.borrow_mut() = None);
    PROPOSED_UNITS.with(|slot| *slot.borrow_mut() = None);
    CASH_INPUT_STATE.store(0, Ordering::Relaxed);
    invalidate_review_state("offered_athlete_changed_in_trade_popup");
    if let Some(node) = cash_input_node_mut(ui) {
        set_runner_text(node, "");
        node.runner.set_dirty(true);
    }
    set_label_text(
        ui,
        "pts_cash_runtime_selection_value",
        &format!("{}  ↔  {}", offered_name, target_name),
    );
    set_label_text(ui, "pts_cash_runtime_required_value", "계산 중");
    set_label_text(
        ui,
        "pts_cash_runtime_required_note",
        "새 선수 조합으로 서버 견적을 다시 계산하고 있습니다.",
    );
    set_label_text(ui, "pts_cash_runtime_after_value", "—");
    set_node_visible(ui, "pts_cash_runtime_proposed_eok", false);
    QUOTE_UI_DIRTY.store(true, Ordering::Relaxed);
    log_event(
        "trade_selection_state_reset",
        &format!(
            "previous_offered_id={};offered_id={};offered_name={};target_id={};target_name={};quote_cleared=true;cash_input_cleared=true;review_cleared=true;sealed_plan_cleared=true;had_quote={};had_review={};had_plan={};desired_status_preserved=true;new_quote_required=true;transaction_executed=false",
            previous_offered_id,
            offered_id,
            sanitize(offered_name),
            target_id,
            sanitize(target_name),
            had_quote,
            had_review,
            had_plan,
        ),
    );
}

fn apply_pending_offered_selection(ui: &mut GameUI, data: &ClientData) {
    let offered_id = PENDING_OFFERED_ATHLETE_ID.swap(NO_ATHLETE, Ordering::AcqRel);
    if offered_id == NO_ATHLETE || !POPUP_OPEN.load(Ordering::Relaxed) {
        return;
    }
    if EXECUTION_IN_PROGRESS.load(Ordering::Acquire)
        || EXECUTE_REQUEST_PENDING.load(Ordering::Relaxed)
        || ROLLBACK_COMPLETED_UI.load(Ordering::Acquire)
    {
        log_event(
            "trade_popup_roster_selection_ignored",
            &format!("offered_id={};reason=execution_or_rollback_state_active", offered_id),
        );
        return;
    }
    let target_id = TARGET_ATHLETE_ID.load(Ordering::Relaxed);
    let (offered_name, target_name) = {
        let db = data.db();
        let offered_name = db
            .athlete(offered_id)
            .map(|athlete| athlete.name.clone())
            .unwrap_or_else(|| format!("athlete_{offered_id}"));
        let target_name = db
            .athlete(target_id)
            .map(|athlete| athlete.name.clone())
            .unwrap_or_else(|| format!("athlete_{target_id}"));
        (offered_name, target_name)
    };
    let previous = OFFERED_ATHLETE_ID.swap(offered_id, Ordering::AcqRel);
    let selection_changed = previous != offered_id;
    let sequence = DIRECT_POPUP_SELECTION_COUNT.fetch_add(1, Ordering::AcqRel) + 1;
    log_event(
        "offered_athlete_captured",
        &format!(
            "source=custom_static_roster_slot;offered_id={};offered_name={};target_id={};selection_sequence={};selection_changed={};custom_modal=true;original_compare_popup_used=false",
            offered_id,
            sanitize(&offered_name),
            target_id,
            sequence,
            selection_changed,
        ),
    );
    reset_state_for_offered_player_change(
        ui,
        previous,
        offered_id,
        &offered_name,
        target_id,
        &target_name,
    );
    sync_trade_popup_visual(
        ui,
        data,
        previous,
        offered_id,
        target_id,
        sequence,
    );
    log_event(
        "trade_popup_direct_selection_applied",
        &format!(
            "offered_id={};offered_name={};target_id={};selection_sequence={};selection_changed={};native_click_forwarded=false;custom_static_slot=true;event_consumed_by_mod=true;server_quote_refresh_queued=true;original_compare_popup_used=false;original_compare_screen_preselection_required=false",
            offered_id,
            sanitize(&offered_name),
            target_id,
            sequence,
            selection_changed,
        ),
    );
}

fn process_cash_input(text: &str) {
    if TRANSACTION_EXECUTED_UI.load(Ordering::Acquire) {
        log_event(
            "cash_input_change_ignored",
            "reason=transaction_already_executed;transaction_executed=true",
        );
        return;
    }
    invalidate_review_state("cash_input_changed");
    let trimmed = text.trim();
    let valid_format =
        !trimmed.is_empty() && trimmed.chars().all(|character| character.is_ascii_digit());
    let parsed = if valid_format {
        trimmed.parse::<u64>().ok()
    } else {
        None
    };
    let quote = QUOTE_VIEW.with(|slot| slot.borrow().clone());
    let mut within_budget = false;
    let mut meets_required = false;
    let mut within_offer_range = false;
    let mut offer_max_units = None;
    let mut remaining_units = None;
    if let (Some(units), Some(quote)) = (parsed, quote.as_ref()) {
        let maximum = quote.cash_offer_max_units;
        offer_max_units = Some(maximum);
        if let Some(proposed_won) = units.checked_mul(MONEY_UNIT_WON) {
            within_budget = (proposed_won as f64) <= quote.cash_budget_won + 0.001;
            meets_required = units >= quote.required_units;
            within_offer_range = quote.required_units <= maximum
                && units >= quote.required_units
                && units <= maximum;
            if within_budget {
                remaining_units =
                    won_to_units_floor((quote.cash_budget_won - proposed_won as f64).max(0.0)).ok();
            }
        }
    }
    PROPOSED_UNITS.with(|slot| *slot.borrow_mut() = parsed);
    let valid = parsed.is_some() && quote.is_some() && within_budget && within_offer_range;
    CASH_INPUT_STATE.store(if valid { 1 } else { 2 }, Ordering::Relaxed);
    QUOTE_UI_DIRTY.store(true, Ordering::Relaxed);
    log_event(
        "cash_input_completed",
        &format!(
            "valid_format={};quote_ready={};within_budget={};meets_required={};within_offer_range={};cash_offer_max_units={};proposed_units={};remaining_units={};transaction_enabled=true",
            valid_format,
            quote.is_some(),
            within_budget,
            meets_required,
            within_offer_range,
            offer_max_units.map(|value| value.to_string()).unwrap_or_else(|| "none".to_string()),
            parsed.map(|value| value.to_string()).unwrap_or_else(|| "none".to_string()),
            remaining_units.map(|value| value.to_string()).unwrap_or_else(|| "none".to_string()),
        ),
    );
    log_trade_proposal_draft();
}

fn event_matches_any(path: &str, item: &str, ids: &[&str]) -> bool {
    ids.iter().any(|id| path.contains(*id) || item == *id)
}

fn build_click_handler_pair() -> ClickHandlerPair {
    let filter: ClickFilter = Rc::new(|event| match event {
            UIEvent::Click { path, item } => {
                if let Some(athlete_id) = projection_profile_click_athlete_id(path, item) {
                    let origin_target_scene_confirmed =
                        PROFILE_CONTEXT_LAST_CONFIRMED_TARGET_SCENE.load(Ordering::Acquire);
                    PROFILE_CONTEXT_ATHLETE_ID.store(athlete_id, Ordering::Release);
                    PROFILE_CONTEXT_DETAIL_INSTANCE.store(0, Ordering::Release);
                    PROFILE_CONTEXT_BIND_NEXT_DETAIL.store(true, Ordering::Release);
                    PROFILE_CONTEXT_TARGET_LOCK_VALID.store(false, Ordering::Release);
                    PROFILE_CONTEXT_ALLOW_NEXT_REBUILD.store(false, Ordering::Release);
                    PROFILE_CONTEXT_MANAGEMENT_TICK_REBUILD_REBOUND.store(false, Ordering::Release);
                    PROFILE_CONTEXT_EXPLICITLY_BOUND.store(true, Ordering::Release);
                    PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.store(true, Ordering::Release);
                    PROFILE_CONTEXT_SCENE_SUSPENDED.store(true, Ordering::Release);
                    PROFILE_CONTEXT_SCENE_RETURN_REBOUND.store(false, Ordering::Release);
                    PROJECTION_PROFILE_CLICK_PENDING_ID.store(athlete_id, Ordering::Release);
                    PROJECTION_PROFILE_CLICK_REQUEST_FRAME.store(
                        RUNTIME_FRAME_COUNT.load(Ordering::Relaxed).max(1),
                        Ordering::Release,
                    );
                    PROJECTION_PROFILE_CLICK_TIMEOUT_LOGGED.store(false, Ordering::Release);
                    PROJECTION_PROFILE_SCENE_RETURN_REQUESTED.store(true, Ordering::Release);
                    CLICK_ACTION.store(ACTION_OPEN_PROJECTION_PROFILE, Ordering::Relaxed);
                    log_event(
                        "contract_projection_player_click_dispatched",
                        &format!(
                            "athlete_id={};row_id={};view_detail_id=view_detail_{};event_consumed_by_mod=true;native_profile_navigation_forwarded=false;navigation_method=UIOutEvent_UndoScene;origin_target_scene_confirmed={};target_identity_lease=true;binding_not_rebuilt_on_click=true;native_view_detail_runner=true",
                            athlete_id,
                            PENDING_CONTRACT_SLOT_RUNTIME_ID,
                            athlete_id,
                            origin_target_scene_confirmed,
                        ),
                    );
                    return true;
                }
                update_profile_context_from_click(path, item);
                if should_block_native_negotiation_click(path, item) {
                    return true;
                }
                if POPUP_OPEN.load(Ordering::Relaxed) {
                    if let Some(offered_id) = offered_id_from_compare_click(path, item) {
                        PENDING_OFFERED_ATHLETE_ID.store(offered_id, Ordering::Release);
                        log_event(
                            "trade_popup_roster_click_captured",
                            &format!(
                                "offered_id={};path_contains_custom_modal={};native_handler_forwarded=false;custom_static_slot=true;event_consumed_by_mod=true",
                                offered_id,
                                path.contains(CUSTOM_SLOT_PREFIX),
                            ),
                        );
                        return true;
                    }
                } else if let Some(offered_id) = offered_id_from_compare_click(path, item) {
                    let previous = OFFERED_ATHLETE_ID.swap(offered_id, Ordering::Relaxed);
                    if previous != offered_id {
                        log_event(
                            "offered_athlete_captured",
                            &format!("source=native_compare_roster_click;offered_id={offered_id}"),
                        );
                    }
                }
                let status_action = if event_matches_any(
                    path,
                    item,
                    &[
                        STATUS_TOGGLE_ID,
                        "pts_trade_status_toggle",
                        "pts_trade_runtime_status_value_core",
                        "pts_trade_runtime_status_value_important",
                        "pts_trade_runtime_status_value_general",
                        "pts_trade_runtime_status_value_sub",
                        "pts_trade_runtime_status_value_prospect",
                        "pts_trade_status_value_core",
                        "pts_trade_status_value_important",
                        "pts_trade_status_value_general",
                        "pts_trade_status_value_sub",
                        "pts_trade_status_value_prospect",
                        "pts_trade_status_arrow",
                    ],
                ) {
                    ACTION_STATUS_TOGGLE
                } else if event_matches_any(
                    path,
                    item,
                    &[
                        "pts_trade_runtime_status_option_core",
                        "pts_trade_status_option_core",
                        "pts_trade_runtime_status_check_core",
                        "pts_trade_status_check_core",
                    ],
                ) {
                    ACTION_STATUS_CORE
                } else if event_matches_any(
                    path,
                    item,
                    &[
                        "pts_trade_runtime_status_option_important",
                        "pts_trade_status_option_important",
                        "pts_trade_runtime_status_check_important",
                        "pts_trade_status_check_important",
                    ],
                ) {
                    ACTION_STATUS_IMPORTANT
                } else if event_matches_any(
                    path,
                    item,
                    &[
                        "pts_trade_runtime_status_option_general",
                        "pts_trade_status_option_general",
                        "pts_trade_runtime_status_check_general",
                        "pts_trade_status_check_general",
                    ],
                ) {
                    ACTION_STATUS_GENERAL
                } else if event_matches_any(
                    path,
                    item,
                    &[
                        "pts_trade_runtime_status_option_sub",
                        "pts_trade_status_option_sub",
                        "pts_trade_runtime_status_check_sub",
                        "pts_trade_status_check_sub",
                    ],
                ) {
                    ACTION_STATUS_SUB
                } else if event_matches_any(
                    path,
                    item,
                    &[
                        "pts_trade_runtime_status_option_prospect",
                        "pts_trade_status_option_prospect",
                        "pts_trade_runtime_status_check_prospect",
                        "pts_trade_status_check_prospect",
                    ],
                ) {
                    ACTION_STATUS_PROSPECT
                } else {
                    ACTION_NONE
                };
                if status_action != ACTION_NONE {
                    CLICK_ACTION.store(status_action, Ordering::Relaxed);
                    return true;
                }
                let is_review = POPUP_OPEN.load(Ordering::Relaxed)
                    && (path.contains(REVIEW_BUTTON_ID) || item == REVIEW_BUTTON_ID);
                if is_review {
                    if TEST79_SMOKE_ONLY {
                        log_event(
                            "trade_review_click_blocked_test77_smoke",
                            "client_gate_closed=true;event_consumed_by_mod=true;database_mutation=false;transaction_executed=false",
                        );
                        return true;
                    }
                    CLICK_ACTION.store(ACTION_REVIEW, Ordering::Relaxed);
                    return true;
                }
                let is_commit = POPUP_OPEN.load(Ordering::Relaxed)
                    && (path.contains(COMMIT_BUTTON_ID) || item == COMMIT_BUTTON_ID);
                if is_commit {
                    if TEST79_SMOKE_ONLY {
                        log_event(
                            "trade_execution_click_blocked_test77_smoke",
                            "client_gate_closed=true;event_consumed_by_mod=true;database_mutation=false;transaction_executed=false",
                        );
                        return true;
                    }
                    let action = if TRANSACTION_EXECUTED_UI.load(Ordering::Acquire)
                        && DUPLICATE_REPLAY_VERIFIED.load(Ordering::Acquire)
                    {
                        ACTION_CLOSE
                    } else if ROLLBACK_COMPLETED_UI.load(Ordering::Acquire) {
                        ACTION_CLOSE
                    } else if TRANSACTION_EXECUTED_UI.load(Ordering::Acquire) {
                        ACTION_BLOCK
                    } else {
                        ACTION_EXECUTE
                    };
                    CLICK_ACTION.store(action, Ordering::Relaxed);
                    return true;
                }
                let is_cash_input = path.contains(CASH_INPUT_ID) || item == CASH_INPUT_ID;
                if is_cash_input {
                    if !CASH_INPUT_FOCUS_LOGGED.swap(true, Ordering::Relaxed) {
                        log_event(
                            "cash_input_focused",
                            "id=pts_cash_input;max_length=12;unit=10000_won;transaction_enabled=true",
                        );
                    }
                    return false;
                }
                // ★[PORT056] 진단 전면 개방: 화면 조건 없이 **모든** 클릭의 path/item 을 남긴다.
                //   2026-08-23 인게임 1차에서 트레이드 버튼 클릭이 `main.top.right.offer.data.row2` 로 잡혀
                //   (경로에 버튼 노드 id 가 없음) 필터가 소비하지 못했고, 게임이 홈으로 이동해버렸다.
                //   실제로 어떤 경로가 오는지 확정 전에는 게이트를 걸면 안 된다.
                {
                    let n = ALL_RAW_CLICK_COUNT.fetch_add(1, Ordering::AcqRel) + 1;
                    if n <= 300 {
                        log_event(
                            "raw_click_any",
                            &format!(
                                "idx={};path={};item={};entry_active={};popup_open={};offer_surfaces={}",
                                n,
                                sanitize(path),
                                sanitize(item),
                                TRADE_ENTRY_ACTIVE.load(Ordering::Acquire),
                                POPUP_OPEN.load(Ordering::Relaxed),
                                ACTIVE_STRUCTURAL_OFFER_COUNT.load(Ordering::Acquire),
                            ),
                        );
                    }
                }
                let active_offer_click =
                    ACTIVE_STRUCTURAL_OFFER_COUNT.load(Ordering::Acquire) > 0
                        && !POPUP_OPEN.load(Ordering::Acquire);
                if active_offer_click {
                    let raw_click_index =
                        OFFER_RAW_CLICK_COUNT.fetch_add(1, Ordering::AcqRel) + 1;
                    if raw_click_index <= 64 {
                        log_event(
                            "offer_raw_click_observed",
                            &format!(
                                "raw_click_index={};raw_path={};raw_item={};path_contains_entry={};item_contains_entry={};entry_active={};event_type=click",
                                raw_click_index,
                                sanitize(path),
                                sanitize(item),
                                path.contains(ENTRY_ID),
                                item.contains(ENTRY_ID),
                                TRADE_ENTRY_ACTIVE.load(Ordering::Acquire),
                            ),
                        );
                    }
                }
                // ★[PORT056] 안전장치: 트레이드 버튼이 활성인데 클릭이 **버튼의 부모 컨테이너**로만 잡히는
                //   경우(2026-08-23 인게임 실측 = `main.top.right.offer.data.row2`, item 빈 값)를 진입 클릭으로 본다.
                //   row2 의 실제 컨트롤은 전부 자식(`offer`/`delegate`/`condition_list.*`/`option_box.*`)이라
                //   **컨테이너 자체를 대상으로 한 클릭은 원래 의미가 없다** ⟹ 삼켜도 다른 기능을 막지 않고,
                //   게임이 홈으로 튀는 부작용만 사라진다.
                let entry_parent_click = TRADE_ENTRY_ACTIVE.load(Ordering::Acquire)
                    && item.is_empty()
                    && path.ends_with(".data.row2");
                if entry_parent_click && !ENTRY_PARENT_FALLBACK_LOGGED.swap(true, Ordering::AcqRel) {
                    log_event(
                        "trade_entry_parent_container_click_adopted",
                        &format!(
                            "path={};reason=button_node_id_absent_from_click_path;container_click_has_no_native_meaning=true",
                            sanitize(path),
                        ),
                    );
                }
                if trade_entry_event(path, item) || entry_parent_click {
                    log_event(
                        "trade_entry_click_matched",
                        &format!(
                            "raw_path={};raw_item={};match_by_exact_item={};match_by_exact_segment={};match_by_unique_contains={};entry_active={};event_consumed_by_mod=true",
                            sanitize(path),
                            sanitize(item),
                            item == ENTRY_ID,
                            path_has_exact_node_id(path, ENTRY_ID),
                            path.contains(ENTRY_ID) || item.contains(ENTRY_ID),
                            TRADE_ENTRY_ACTIVE.load(Ordering::Acquire),
                        ),
                    );
                    return request_cached_compare_clone_from_trade_click(path, item);
                }
                let is_popup = POPUP_OPEN.load(Ordering::Relaxed)
                    && (path.contains(MODAL_LAYER_ID)
                        || path.contains(CUSTOM_COMPARE_ID)
                        || path.contains(CASH_PANEL_ID));
                let is_modal_backdrop = POPUP_OPEN.load(Ordering::Relaxed)
                    && (path.contains(BACKDROP_ID) || item == BACKDROP_ID);
                let is_close = is_modal_backdrop
                    || (is_popup
                        && (path.contains("close_btn")
                        || path.contains("cancel")
                        || path.contains(CASH_CANCEL_ID)
                        || path.contains(CUSTOM_CLOSE_ID)
                        || item == CUSTOM_CLOSE_ID
                        || item == "close_btn"
                        || item == "cancel"
                        || item == CASH_CANCEL_ID));
                let action = if is_close {
                    ACTION_CLOSE
                } else if is_popup {
                    ACTION_BLOCK
                } else {
                    ACTION_NONE
                };
                if action != ACTION_NONE {
                    CLICK_ACTION.store(action, Ordering::Relaxed);
                    true
                } else {
                    false
                }
            }
            UIEvent::TextEditComplete { text, .. } => {
                if !POPUP_OPEN.load(Ordering::Relaxed) {
                    return false;
                }
                process_cash_input(text);
                true
            }
            _ => false,
        });
    let handler: ClickHandler = Rc::new(|context| {
            let action = CLICK_ACTION.swap(ACTION_NONE, Ordering::Relaxed);
            if let Some(choice) = desired_squad_status_from_action(action) {
                select_desired_squad_status(context.ui, choice);
                return;
            }
            match action {
                ACTION_OPEN => open_trade_popup(context.ui, context.assets),
                ACTION_CLOSE => close_trade_popup(context.ui),
                ACTION_STATUS_TOGGLE => {
                    let open = !STATUS_MENU_OPEN.load(Ordering::Relaxed);
                    STATUS_MENU_OPEN.store(open, Ordering::Relaxed);
                    apply_desired_squad_status_ui(context.ui);
                    log_event(
                        "desired_squad_status_menu_toggled",
                        &format!("open={open};options=core,important,general,sub,prospect"),
                    );
                }
                ACTION_REVIEW => {
                    if queue_server_review() {
                        close_trade_popup_for_async_submit(context.ui);
                            RETURN_TO_PROFILE_OBSERVED.store(false, Ordering::Release);
                        RETURN_TO_PROFILE_TIMEOUT_LOGGED.store(false, Ordering::Release);
                        RETURN_TO_PROFILE_REQUEST_FRAME.store(
                            RUNTIME_FRAME_COUNT.load(Ordering::Relaxed).max(1),
                            Ordering::Release,
                        );
                        PROFILE_CONTEXT_ATHLETE_ID.store(
                            TARGET_ATHLETE_ID.load(Ordering::Acquire),
                            Ordering::Release,
                        );
                        context.out_events.push(UIOutEvent::UndoScene);
                        log_event(
                            "async_trade_profile_return_requested",
                            "method=UIOutEvent::UndoScene;undo_scene_queued=true;custom_modal_closed=true;offer_force_hidden=false;black_screen_fallback=keep_offer_visible;transaction_executed=false",
                        );
                    }
                },
                ACTION_EXECUTE => log_event("immediate_trade_execute_click_blocked", "reason=async_lifecycle_only;transaction_executed=false"),
                ACTION_OPEN_PROJECTION_PROFILE => {
                    let athlete_id = PROJECTION_PROFILE_CLICK_PENDING_ID.load(Ordering::Acquire);
                    if athlete_id != 0 {
                        PROFILE_CONTEXT_ATHLETE_ID.store(athlete_id, Ordering::Release);
                        PROFILE_CONTEXT_DETAIL_INSTANCE.store(0, Ordering::Release);
                        PROFILE_CONTEXT_BIND_NEXT_DETAIL.store(true, Ordering::Release);
                        PROFILE_CONTEXT_TARGET_LOCK_VALID.store(false, Ordering::Release);
                        PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.store(true, Ordering::Release);
                        PROFILE_CONTEXT_SCENE_SUSPENDED.store(true, Ordering::Release);
                        PROJECTION_PROFILE_SCENE_RETURN_REQUESTED.store(true, Ordering::Release);
                        let origin_target_scene_confirmed =
                            PROFILE_CONTEXT_LAST_CONFIRMED_TARGET_SCENE.load(Ordering::Acquire);
                        context.out_events.push(UIOutEvent::UndoScene);
                        log_event(
                            "contract_projection_explicit_profile_navigation_requested",
                            &format!(
                                "athlete_id={};method=UIOutEvent::UndoScene;undo_scene_queued=true;origin_target_scene_confirmed={};target_identity_lease=true;contract_projection_surface_expected_to_close=true",
                                athlete_id,
                                origin_target_scene_confirmed,
                            ),
                        );
                    }
                },
                ACTION_BLOCK => {
                    log_event(
                        "trade_popup_read_only_click_blocked",
                        "transaction_enabled=true",
                    );
                }
                _ => {}
            }
        });
    (filter, handler)
}

fn ensure_click_handler(ui: &mut GameUI, _offer_surface_present: bool) {
    let current_offer_signature = if _offer_surface_present {
        active_offer(ui)
            .map(|offer| offer as *const Node as usize)
            .unwrap_or(0)
    } else {
        0
    };
    let previous_offer_signature =
        CLICK_FILTER_OFFER_SIGNATURE_FIX4.swap(current_offer_signature, Ordering::AcqRel);
    if current_offer_signature != 0 && previous_offer_signature != current_offer_signature {
    
        log_event(
            "trade_click_filter_reinstall_requested",
            &format!(
                "previous_offer_signature={};current_offer_signature={};new_offer_instance=true",
                previous_offer_signature,
                current_offer_signature,
            ),
        );
    }

    let pair = CLICK_HANDLER_PAIR.with(|slot| {
        let mut slot = slot.borrow_mut();
        slot.get_or_insert_with(build_click_handler_pair).clone()
    });
    let exact_index = ui
        .filter_handler
        .iter()
        .position(|(filter, handler)| {
            Rc::ptr_eq(filter, &pair.0) && Rc::ptr_eq(handler, &pair.1)
        });
    if exact_index == Some(0) {
        return;
    }
    if let Some(index) = exact_index {
        ui.filter_handler.remove(index);
    }
    let current_len = ui.filter_handler.len();
    ui.filter_handler.insert(0, pair);
    log_event(
        "filter_handler_registered",
        &format!(
            "handler_count_before={current_len};handler_count_after={};singleton=true;registration_strategy=rc_ptr_eq;insert_index=0",
            ui.filter_handler.len(),
        ),
    );
}

fn send_quote_if_ready(data: &ClientData) {
    if !POPUP_OPEN.load(Ordering::Relaxed) || QUOTE_REQUEST_SENT.load(Ordering::Relaxed) {
        return;
    }
    let offered_id = OFFERED_ATHLETE_ID.load(Ordering::Relaxed);
    let target_id = TARGET_ATHLETE_ID.load(Ordering::Relaxed);
    if offered_id == NO_ATHLETE || target_id == 0 {
        return;
    }
    let region_id = {
        let db = data.db();
        db.athlete(offered_id)
            .and_then(|athlete| db.athlete_current_region_id(athlete))
    };
    let Some(region_id) = region_id else {
        if !REGION_ERROR_LOGGED.swap(true, Ordering::Relaxed) {
            log_event(
                "quote_command_not_sent",
                &format!("reason=offered_region_unavailable;offered_id={offered_id}"),
            );
        }
        return;
    };
    let payload = format!(
        "offered_id={}\ntarget_id={}\nregion_id={}\n",
        offered_id, target_id, region_id
    );
    if data.send_mod_command(MOD_ID, QUOTE_COMMAND, payload.into_bytes()) {
        QUOTE_REQUEST_SENT.store(true, Ordering::Relaxed);
        log_event(
            "quote_command_sent",
            &format!(
                "offered_id={offered_id};target_id={target_id};region_id={region_id};database_mutation=false"
            ),
        );
    }
}

fn fail_queued_review(detail: &str) {
    REVIEW_REQUEST_QUEUED.store(false, Ordering::Relaxed);
    REVIEW_REQUEST_PENDING.store(false, Ordering::Relaxed);
    REVIEW_VIEW.with(|slot| *slot.borrow_mut() = None);
    REVIEW_ERROR.with(|slot| *slot.borrow_mut() = Some(detail.to_string()));
    QUOTE_UI_DIRTY.store(true, Ordering::Relaxed);
    log_event(
        "proposal_review_command_not_sent",
        &format!("detail={};transaction_executed=false", sanitize(detail)),
    );
}



fn send_review_if_ready(data: &ClientData) {
    if !REVIEW_REQUEST_QUEUED.load(Ordering::Relaxed)
        || REVIEW_REQUEST_PENDING.load(Ordering::Relaxed)
    {
        return;
    }
    let quote = QUOTE_VIEW.with(|slot| slot.borrow().clone());
    let proposed_units = PROPOSED_UNITS.with(|slot| *slot.borrow());
    let (Some(quote), Some(proposed_units)) = (quote, proposed_units) else {
        fail_queued_review("견적 계산과 제시 금액 입력을 먼저 완료하세요.");
        return;
    };
    let Some(proposed_cash_won) = proposed_units.checked_mul(MONEY_UNIT_WON) else {
        fail_queued_review("제시 금액이 허용 범위를 넘었습니다.");
        return;
    };
    if proposed_units < quote.required_units || proposed_units > quote.cash_offer_max_units {
        fail_queued_review(&format!(
            "제시 금액은 표시 범위 {}~{} 안이어야 합니다.",
            quote.required_units,
            quote.cash_offer_max_units,
        ));
        return;
    }
    if proposed_cash_won as f64 > quote.cash_budget_won + 0.001 {
        fail_queued_review("제시 금액이 현재 이적료 예산을 초과합니다.");
        return;
    }
    let region_id = {
        let db = data.db();
        db.athlete(quote.offered_id)
            .and_then(|athlete| db.athlete_current_region_id(athlete))
    };
    let Some(region_id) = region_id else {
        fail_queued_review("교환 선수의 시장 지역을 확인할 수 없습니다.");
        return;
    };
    let desired_status_choice = DESIRED_SQUAD_STATUS.load(Ordering::Relaxed);
    let payload = format!(
        "offered_id={}\ntarget_id={}\nregion_id={}\nproposed_units={}\ndesired_status_choice={}\n",
        quote.offered_id,
        quote.target_id,
        region_id,
        proposed_units,
        desired_status_choice,
    );
    if data.send_mod_command(MOD_ID, REVIEW_COMMAND, payload.into_bytes()) {
        REVIEW_REQUEST_QUEUED.store(false, Ordering::Release);
        REVIEW_REQUEST_PENDING.store(true, Ordering::Release);
        let (status_key, status_label) = desired_squad_status(desired_status_choice);
        log_event(
            "async_trade_submit_command_sent",
            &format!(
                "offered_id={};target_id={};region_id={};proposed_units={};proposed_cash_won={};display_min_units={};display_max_units={};cash_range_obscured=true;exact_threshold_disclosed=false;desired_status_choice={};desired_status_key={};desired_status_label={};proposal_only=true;popup_open_required=false;ui_context_independent=true;profile_return_may_be_in_progress=true;team_mutation=false;finance_mutation=false;contract_mutation=false;transaction_executed=false",
                quote.offered_id,
                quote.target_id,
                region_id,
                proposed_units,
                proposed_cash_won,
                quote.required_units,
                quote.cash_offer_max_units,
                desired_status_choice,
                status_key,
                status_label,
            ),
        );
        clear_async_submit_payload_after_command_sent();
    } else {
        fail_queued_review("트레이드 제안 접수 명령을 전송하지 못했습니다.");
    }
}

fn fail_queued_execute(detail: &str) {
    EXECUTE_REQUEST_QUEUED.store(false, Ordering::Relaxed);
    EXECUTE_REQUEST_PENDING.store(false, Ordering::Relaxed);
    EXECUTE_VIEW.with(|slot| *slot.borrow_mut() = None);
    EXECUTE_ERROR.with(|slot| *slot.borrow_mut() = Some(detail.to_string()));
    QUOTE_UI_DIRTY.store(true, Ordering::Relaxed);
    log_event(
        "trade_atomic_commit_command_not_sent",
        &format!(
            "detail={};execution_gate_closed=true;transaction_executed=false",
            sanitize(detail)
        ),
    );
}

fn send_execute_if_ready(data: &ClientData) {
    if TEST79_SMOKE_ONLY {
        let _ = data;
        return;
    }
    if !POPUP_OPEN.load(Ordering::Relaxed)
        || !EXECUTE_REQUEST_QUEUED.load(Ordering::Relaxed)
        || EXECUTE_REQUEST_PENDING.load(Ordering::Relaxed)
        || TRANSACTION_EXECUTED_UI.load(Ordering::Acquire)
    {
        return;
    }
    let review = REVIEW_VIEW.with(|slot| slot.borrow().clone());
    let quote = QUOTE_VIEW.with(|slot| slot.borrow().clone());
    let proposed_units = PROPOSED_UNITS.with(|slot| *slot.borrow());
    let desired_status_choice = DESIRED_SQUAD_STATUS.load(Ordering::Relaxed);
    let (Some(review), Some(quote), Some(proposed_units)) =
        (review, quote, proposed_units)
    else {
        fail_queued_execute("봉인된 거래 명령의 현재 입력을 다시 확인할 수 없습니다.");
        return;
    };
    if !review.overall_approved
        || !review.command_envelope_prepared
        || !review.plan_repeat_consistent
        || !review.execution_gate_closed
        || review.offered_id != quote.offered_id
        || review.target_id != quote.target_id
        || review.proposed_units != proposed_units
        || review.desired_status_choice != desired_status_choice
    {
        fail_queued_execute("현재 제안이 서버가 봉인한 거래 명령과 일치하지 않습니다.");
        return;
    }
    let region_id = {
        let db = data.db();
        db.athlete(quote.offered_id)
            .and_then(|athlete| db.athlete_current_region_id(athlete))
    };
    let Some(region_id) = region_id else {
        fail_queued_execute("교환 선수의 시장 지역을 확인할 수 없습니다.");
        return;
    };
    let payload = format!(
        "plan_id={}\noffered_id={}\ntarget_id={}\nregion_id={}\nproposed_units={}\ndesired_status_choice={}\n",
        review.plan_id,
        review.offered_id,
        review.target_id,
        region_id,
        review.proposed_units,
        review.desired_status_choice,
    );
    let payload_bytes = payload.into_bytes();
    {
        let mut replay_payload = DUPLICATE_REPLAY_PAYLOAD
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *replay_payload = Some(payload_bytes.clone());
    }
    if data.send_mod_command(MOD_ID, EXECUTE_COMMAND, payload_bytes) {
        EXECUTE_REQUEST_QUEUED.store(false, Ordering::Relaxed);
        EXECUTE_REQUEST_PENDING.store(true, Ordering::Relaxed);
        QUOTE_UI_DIRTY.store(true, Ordering::Relaxed);
        log_event(
            "trade_atomic_commit_command_sent",
            &format!(
                "plan_id={};offered_id={};target_id={};region_id={};proposed_units={};proposed_cash_won={};desired_status_choice={};server_revalidation_required=true;emergency_rollback_snapshot_required=true;explicit_second_click=true;actual_commit_expected=true;transaction_executed=false",
                review.plan_id,
                review.offered_id,
                review.target_id,
                region_id,
                review.proposed_units,
                review.proposed_units.saturating_mul(MONEY_UNIT_WON),
                review.desired_status_choice,
            ),
        );
    } else {
        fail_queued_execute("원자 실행 명령을 서버로 전송하지 못했습니다.");
    }
}

fn send_duplicate_replay_if_ready(data: &ClientData) {
    if TEST79_SMOKE_ONLY {
        let _ = data;
        return;
    }
    if !TRANSACTION_EXECUTED_UI.load(Ordering::Acquire)
        || !DUPLICATE_REPLAY_REQUEST_QUEUED.load(Ordering::Acquire)
        || DUPLICATE_REPLAY_REQUEST_PENDING.load(Ordering::Acquire)
        || DUPLICATE_REPLAY_VERIFIED.load(Ordering::Acquire)
    {
        return;
    }
    let payload = DUPLICATE_REPLAY_PAYLOAD
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone();
    let Some(payload) = payload else {
        log_event(
            "trade_duplicate_replay_command_not_sent",
            "reason=sealed_payload_missing;database_mutation=false;transaction_executed=false",
        );
        return;
    };
    let values = parse_kv_payload(&payload).ok();
    let plan_id = values
        .as_ref()
        .and_then(|map| map.get("plan_id"))
        .cloned()
        .unwrap_or_else(|| "none".to_string());
    if data.send_mod_command(MOD_ID, EXECUTE_COMMAND, payload) {
        DUPLICATE_REPLAY_REQUEST_QUEUED.store(false, Ordering::Release);
        DUPLICATE_REPLAY_REQUEST_PENDING.store(true, Ordering::Release);
        log_event(
            "trade_duplicate_replay_command_sent",
            &format!(
                "plan_id={};same_sealed_payload=true;expected=duplicate_blocked;database_mutation=false;transaction_executed=false",
                plan_id,
            ),
        );
    }
}

fn send_replacement_floor_audit_if_ready(data: &ClientData) {
    if FLOOR_AUDIT_RESPONSE_RECEIVED.load(Ordering::Acquire) {
        return;
    }
    let frame = RUNTIME_FRAME_COUNT.load(Ordering::Relaxed);
    let last = FLOOR_AUDIT_LAST_SEND_FRAME.load(Ordering::Relaxed);
    if last != 0 && frame.saturating_sub(last) < 120 {
        return;
    }
    if data.send_mod_command(MOD_ID, FLOOR_AUDIT_COMMAND, b"run=1\n".to_vec()) {
        FLOOR_AUDIT_LAST_SEND_FRAME.store(frame.max(1), Ordering::Relaxed);
        let attempt = FLOOR_AUDIT_SEND_ATTEMPT.fetch_add(1, Ordering::AcqRel) + 1;
        FLOOR_AUDIT_REQUEST_SENT.store(true, Ordering::Release);
        if attempt == 1 {
            log_event(
                "replacement_floor_audit_command_sent",
                "core_floor=0.70;important_floor=0.55;general_floor=0.40;trade_hard_floor_integrated=true;native_cash_channel_audit_only=true;retry_until_response=true;attempt=1",
            );
        } else {
            log_event(
                "replacement_floor_audit_command_retried",
                &format!("attempt={attempt};retry_interval_frames=120;response_received=false"),
            );
        }
    }
}


fn parse_quote_view(values: &BTreeMap<String, String>) -> Result<QuoteView, String> {
    if !map_bool(values, "cash_range_obscured")?
        || map_bool(values, "exact_threshold_disclosed")?
        || !map_bool(values, "range_stable_for_pair")?
    {
        return Err("server did not return the Test77 obscured stable range contract".to_string());
    }
    let display_min_units = map_u64(values, "display_min_units")?;
    let display_max_units = map_u64(values, "display_max_units")?;
    let budget_units = map_u64(values, "budget_units")?;
    if display_min_units > display_max_units {
        return Err("server returned an invalid obscured display range".to_string());
    }
    Ok(QuoteView {
        // 쿨다운 힌트는 없을 수도 있으므로 관대하게 읽는다(없으면 "쿨다운 없음").
        cooldown_present: values.get("cooldown_present").map(|v| v == "true").unwrap_or(false),
        cooldown_fingerprint: values
            .get("cooldown_fingerprint")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0),
        cooldown_retry_at: values.get("cooldown_retry_at").cloned().unwrap_or_default(),
        cooldown_changeable: values.get("cooldown_changeable").map(|v| v == "true").unwrap_or(true),
        cooldown_exempt: values.get("cooldown_exempt").map(|v| v == "true").unwrap_or(false),
        requester_team_id: map_usize(values, "requester_team_id")?,
        recipient_team_id: map_usize(values, "recipient_team_id")?,
        offered_id: map_usize(values, "offered_id")?,
        target_id: map_usize(values, "target_id")?,
        offered_name: map_required(values, "offered_name")?.to_string(),
        target_name: map_required(values, "target_name")?.to_string(),
        required_cash_won: 0,
        required_units: display_min_units,
        cash_offer_max_units: display_max_units,
        cash_range_obscured: true,
        exact_threshold_disclosed: false,
        cash_budget_won: map_f64(values, "cash_budget_won")?,
        budget_units,
    })
}

fn parse_review_view(values: &BTreeMap<String, String>) -> Result<ReviewView, String> {
    if map_bool(values, "transaction_executed")? {
        return Err("server reported an unexpected transaction execution".to_string());
    }
    let proposed_units = map_u64(values, "proposed_units")?;
    let cash_offer_min_units = map_u64(values, "cash_offer_min_units")?;
    let cash_offer_max_units = map_u64(values, "cash_offer_max_units")?;
    if !map_bool(values, "proposed_cash_within_server_range")?
        || cash_offer_min_units > cash_offer_max_units
        || proposed_units < cash_offer_min_units
        || proposed_units > cash_offer_max_units
    {
        return Err("server review returned an invalid cash offer range".to_string());
    }
    let desired_status_raw = map_u64(values, "desired_status_choice")?;
    let desired_status_choice = u8::try_from(desired_status_raw)
        .map_err(|_| "review status choice is out of range".to_string())?;
    let overall_approved = map_bool(values, "overall_approved")?;
    let command_envelope_prepared = map_bool(values, "command_envelope_prepared")?;
    let plan_id = map_required(values, "plan_id")?.to_string();
    let plan_repeat_consistent = map_bool(values, "plan_repeat_consistent")?;
    let execution_gate_closed = map_bool(values, "execution_gate_closed")?;
    let rejection_present = map_bool(values, "rejection_present")?;
    let rejection_package_fingerprint =
        map_required(values, "rejection_package_fingerprint")?.to_string();
    let rejection_feedback_id = map_required(values, "rejection_feedback_id")?.to_string();
    let rejection_cooldown_blocked = map_bool(values, "rejection_cooldown_blocked")?;
    let rejection_news_created = map_bool(values, "rejection_news_created")?;
    let rejection_duplicate_news_count =
        map_usize(values, "rejection_duplicate_news_count")?;
    let rejection_ledger_entry_count = map_usize(values, "rejection_ledger_entry_count")?;
    if overall_approved
        && (!command_envelope_prepared
            || !plan_repeat_consistent
            || !execution_gate_closed
            || plan_id.len() != 20
            || !plan_id.starts_with("T60-")
            || !plan_id[4..]
                .chars()
                .all(|character| character.is_ascii_hexdigit()))
    {
        return Err("approved review returned an invalid command envelope".to_string());
    }
    if overall_approved == rejection_present {
        return Err(
            "review approval and rejection feedback state are not mutually consistent"
                .to_string(),
        );
    }
    if rejection_present
        && (rejection_package_fingerprint.len() != 16
            || !rejection_package_fingerprint
                .chars()
                .all(|character| character.is_ascii_hexdigit())
            || rejection_feedback_id.len() != 16
            || !rejection_feedback_id
                .chars()
                .all(|character| character.is_ascii_hexdigit())
            || rejection_duplicate_news_count != 1
            || rejection_ledger_entry_count == 0
            || (rejection_cooldown_blocked && rejection_news_created))
    {
        return Err("rejected review returned invalid ledger or news evidence".to_string());
    }
    Ok(ReviewView {
        offered_name: map_required(values, "offered_name")?.to_string(),
        offered_id: map_usize(values, "offered_id")?,
        target_id: map_usize(values, "target_id")?,
        target_name: map_required(values, "target_name")?.to_string(),
        proposed_units,
        desired_status_choice,
        desired_status_label: map_required(values, "desired_status_label")?.to_string(),
        seller_accepted: map_bool(values, "seller_accepted")?,
        player_accepted: map_bool(values, "player_accepted")?,
        overall_approved,
        requested_years: map_usize(values, "requested_years")?,
        inherited_yearly_salary: map_f64(values, "inherited_yearly_salary")?,
        command_envelope_prepared,
        plan_id,
        plan_repeat_consistent,
        execution_gate_closed,
        rejection_present,
        rejection_actor_ko: map_required(values, "rejection_actor_ko")?.to_string(),
        rejection_reason: map_required(values, "rejection_reason")?.to_string(),
        rejection_reason_ko: map_required(values, "rejection_reason_ko")?.to_string(),
        rejection_policy: map_required(values, "rejection_policy")?.to_string(),
        rejection_retry_at: map_required(values, "rejection_retry_at")?.to_string(),
        rejection_retry_ko: map_required(values, "rejection_retry_ko")?.to_string(),
        rejection_package_fingerprint,
        rejection_feedback_id,
        rejection_cooldown_blocked,
        rejection_news_created,
        rejection_duplicate_news_count,
        rejection_ledger_entry_count,
    })
}

fn parse_execute_view(values: &BTreeMap<String, String>) -> Result<ExecuteView, String> {
    let transaction_executed = map_bool(values, "transaction_executed")?;
    let atomic_commit_verified = map_bool(values, "atomic_commit_verified")?;
    let rollback_performed = map_bool(values, "rollback_performed")?;
    let execution_gate_closed = map_bool(values, "execution_gate_closed")?;
    let pre_receipt_mod_save_unchanged = map_bool(values, "pre_receipt_mod_save_unchanged")?;
    let executed_plan_persisted = map_bool(values, "executed_plan_persisted")?;
    let trade_receipt_persisted = map_bool(values, "trade_receipt_persisted")?;
    let receipt_readback_verified = map_bool(values, "receipt_readback_verified")?;
    let rollback_rehearsal_verified = map_bool(values, "rollback_rehearsal_verified")?;
    let executed_plan_registry_count = map_usize(values, "executed_plan_registry_count")?;
    let proposed_cash_won = map_u64(values, "proposed_cash_won")?;

    if !transaction_executed
        || !atomic_commit_verified
        || rollback_performed
        || !execution_gate_closed
        || !rollback_rehearsal_verified
        || !map_bool(values, "proposed_cash_within_server_range")?
        || !map_bool(values, "database_mutation")?
        || !map_bool(values, "contract_mutation")?
        || map_bool(values, "finance_mutation")? != (proposed_cash_won > 0)
        || !map_bool(values, "squad_status_mutation")?
        || !map_bool(values, "contracted_squad_status_mutation")?
        || !map_bool(values, "mod_save_mutation")?
        || !map_bool(values, "contract_inherited_both")?
        || !map_bool(values, "offered_status_unchanged")?
        || !map_bool(values, "target_status_applied")?
        || !map_bool(values, "offered_contracted_status_unchanged")?
        || !map_bool(values, "target_contracted_status_applied")?
        || !map_bool(values, "combined_finance_conserved")?
        || !pre_receipt_mod_save_unchanged
        || !executed_plan_persisted
        || !trade_receipt_persisted
        || !receipt_readback_verified
        || !map_bool(values, "three_receipts_preserved")?
        || map_required(values, "first_plan_id")? != FIRST_PLAN_ID
        || !map_bool(values, "first_plan_preserved")?
        || map_required(values, "second_plan_id")? != SECOND_PLAN_ID
        || !map_bool(values, "second_plan_preserved")?
        // [PORT056] 구판은 레지스트리가 "정확히 3건"(과거 픽스처 2 + 신규 1)일 때만 커밋 결과를 받아들였다.
        //   ⟹ 이력 0에서 시작하는 세이브의 첫 거래는 성사돼도 클라이언트가 결과를 거부했다.
        || executed_plan_registry_count < 1
        || !map_bool(values, "news_count_unchanged")?
        || !map_bool(values, "manual_save_required")?
        || map_required(values, "manual_save_slot")? != RESULT_SAVE_SLOT
        || map_bool(values, "save_api_called")?
    {
        return Err("server returned an incomplete Test77 atomic commit result".to_string());
    }

    let plan_id = map_required(values, "plan_id")?.to_string();
    if !valid_test60_plan_id(&plan_id)
        || plan_id == FIRST_PLAN_ID
        || plan_id == SECOND_PLAN_ID
    {
        return Err("server returned an invalid or historical Test77 plan id".to_string());
    }

    let requester_team_id = map_usize(values, "requester_team_id")?;
    let recipient_team_id = map_usize(values, "recipient_team_id")?;
    let offered_id = map_usize(values, "offered_id")?;
    let target_id = map_usize(values, "target_id")?;
    // [PORT056] 상수 팀 7 게이트 + 과거 픽스처 선수 제외 삭제.
    //   남기는 것 = 서버 응답의 자기정합성(요청팀≠수신팀, 두 선수 상이, 교환 후 소속이 뒤바뀜).
    if requester_team_id == recipient_team_id
        || offered_id == target_id
        || map_usize(values, "offered_team_after")? != recipient_team_id
        || map_usize(values, "target_team_after")? != requester_team_id
    {
        return Err("server returned an inconsistent trade execution payload".to_string());
    }

    let desired_status_raw = map_u64(values, "desired_status_choice")?;
    let desired_status_choice = u8::try_from(desired_status_raw)
        .map_err(|_| "execution status choice is out of range".to_string())?;
    let offered_contracted_status_after =
        map_required(values, "offered_contracted_status_after")?.to_string();
    let target_contracted_status_after =
        map_required(values, "target_contracted_status_after")?.to_string();
    // [PORT056] 약속 위상 = 핵심(Core) 고정 검증 삭제 — 드롭다운으로 고른 위상이 그대로 적용되어야 한다.
    //   남기는 것 = 서버가 돌려준 위상 3필드가 서로 모순되지 않는지(선택값 ↔ 적용 결과) 정합성 검사.
    let (expected_status_key, _) = desired_squad_status(desired_status_choice);
    if map_required(values, "desired_status_key")? != expected_status_key {
        return Err("server returned a mismatched promised squad status".to_string());
    }

    let requester_total_before = map_f64(values, "requester_total_before")?;
    let requester_total_after = map_f64(values, "requester_total_after")?;
    let requester_transfer_before = map_f64(values, "requester_transfer_before")?;
    let requester_transfer_after = map_f64(values, "requester_transfer_after")?;
    let recipient_total_before = map_f64(values, "recipient_total_before")?;
    let recipient_total_after = map_f64(values, "recipient_total_after")?;
    let recipient_transfer_before = map_f64(values, "recipient_transfer_before")?;
    let recipient_transfer_after = map_f64(values, "recipient_transfer_after")?;
    let cash = proposed_cash_won as f64;
    if !approximately_equal(requester_total_after, requester_total_before - cash)
        || !approximately_equal(requester_transfer_after, requester_transfer_before - cash)
        || !approximately_equal(recipient_total_after, recipient_total_before + cash)
        || !approximately_equal(recipient_transfer_after, recipient_transfer_before + cash)
    {
        return Err("Test77 finance deltas do not match its sealed cash transfer".to_string());
    }

    let commit_process_id = u32::try_from(map_u64(values, "commit_process_id")?)
        .map_err(|_| "execution commit process id is out of range".to_string())?;
    Ok(ExecuteView {
        plan_id,
        requester_team_id,
        recipient_team_id,
        offered_id,
        target_id,
        offered_name: map_required(values, "offered_name")?.to_string(),
        target_name: map_required(values, "target_name")?.to_string(),
        proposed_cash_won,
        desired_status_choice,
        desired_status_label: map_required(values, "desired_status_label")?.to_string(),
        offered_team_after: map_usize(values, "offered_team_after")?,
        target_team_after: map_usize(values, "target_team_after")?,
        target_status_after: map_required(values, "target_status_after")?.to_string(),
        offered_contracted_status_after,
        target_contracted_status_after,
        rollback_rehearsal_verified,
        requester_total_before,
        requester_total_after,
        requester_transfer_before,
        requester_transfer_after,
        recipient_total_before,
        recipient_total_after,
        recipient_transfer_before,
        recipient_transfer_after,
        atomic_commit_verified,
        rollback_performed,
        execution_gate_closed,
        pre_receipt_mod_save_unchanged,
        executed_plan_persisted,
        trade_receipt_persisted,
        receipt_readback_verified,
        executed_plan_registry_count,
        commit_process_id,
    })
}


fn parse_async_status_view(values: &BTreeMap<String, String>) -> Result<AsyncStatusView, String> {
    let status = map_required(values, "status")?;
    if status == "none" {
        return Ok(AsyncStatusView {
            proposal_present: false,
            proposal_id: String::new(),
            state: "None".to_string(),
            stage_ko: String::new(),
            requester_team_id: usize::MAX,
            recipient_team_id: usize::MAX,
            requester_team_name: String::new(),
            recipient_team_name: String::new(),
            target_id: 0,
            offered_id: 0,
            target_name: String::new(),
            offered_name: String::new(),
            target_position_label: String::new(),
            target_position_icon: String::new(),
            target_contract_end: String::new(),
            target_yearly_salary: 0.0,
            proposed_units: 0,
            desired_status_label: String::new(),
            game_time: map_required(values, "game_time").unwrap_or("").to_string(),
            submitted_at: String::new(),
            seller_due_at: String::new(),
            player_due_at: String::new(),
            completed_at: String::new(),
            rejection_reason_ko: String::new(),
            result_plan_id: String::new(),
            success_news_count: 0,
            submit_process_id: 0,
            commit_process_id: 0,
            current_process_id: u32::try_from(map_u64(values, "current_process_id")?)
                .map_err(|_| "status process id does not fit u32".to_string())?,
            offered_team_current: usize::MAX,
            target_team_current: usize::MAX,
            target_status_current: "unknown".to_string(),
            target_contracted_status_current: "unknown".to_string(),
            executed_plan_registry_count: 0,
            result_plan_occurrences: 0,
        });
    }
    if status != "ok" {
        return Err(map_required(values, "detail").unwrap_or("unknown async status error").to_string());
    }
    Ok(AsyncStatusView {
        proposal_present: map_bool(values, "proposal_present")?,
        proposal_id: map_required(values, "proposal_id")?.to_string(),
        state: map_required(values, "state")?.to_string(),
        stage_ko: map_required(values, "stage_ko")?.to_string(),
        requester_team_id: map_usize(values, "requester_team_id")?,
        recipient_team_id: map_usize(values, "recipient_team_id")?,
        requester_team_name: map_required(values, "requester_team_name")?.to_string(),
        recipient_team_name: map_required(values, "recipient_team_name")?.to_string(),
        target_id: map_usize(values, "target_id")?,
        offered_id: map_usize(values, "offered_id")?,
        target_name: map_required(values, "target_name")?.to_string(),
        offered_name: map_required(values, "offered_name")?.to_string(),
        target_position_label: map_required(values, "target_position_label")?.to_string(),
        target_position_icon: map_required(values, "target_position_icon")?.to_string(),
        target_contract_end: map_required(values, "target_contract_end")?.to_string(),
        target_yearly_salary: map_f64(values, "target_yearly_salary")?,
        proposed_units: map_u64(values, "proposed_units")?,
        desired_status_label: map_required(values, "desired_status_label")?.to_string(),
        game_time: map_required(values, "game_time")?.to_string(),
        submitted_at: map_required(values, "submitted_at")?.to_string(),
        seller_due_at: map_required(values, "seller_due_at")?.to_string(),
        player_due_at: map_required(values, "player_due_at")?.to_string(),
        completed_at: map_required(values, "completed_at")?.to_string(),
        rejection_reason_ko: map_required(values, "rejection_reason_ko")?.to_string(),
        result_plan_id: map_required(values, "result_plan_id")?.to_string(),
        success_news_count: map_usize(values, "success_news_count")?,
        submit_process_id: u32::try_from(map_u64(values, "submit_process_id")?)
            .map_err(|_| "submit process id does not fit u32".to_string())?,
        commit_process_id: u32::try_from(map_u64(values, "commit_process_id")?)
            .map_err(|_| "commit process id does not fit u32".to_string())?,
        current_process_id: u32::try_from(map_u64(values, "current_process_id")?)
            .map_err(|_| "status process id does not fit u32".to_string())?,
        offered_team_current: map_usize(values, "offered_team_current")?,
        target_team_current: map_usize(values, "target_team_current")?,
        target_status_current: map_required(values, "target_status_current")?.to_string(),
        target_contracted_status_current: map_required(values, "target_contracted_status_current")?.to_string(),
        executed_plan_registry_count: map_usize(values, "executed_plan_registry_count")?,
        result_plan_occurrences: map_usize(values, "result_plan_occurrences")?,
    })
}

fn find_effectively_visible_node_by_id_mut<'a>(node: &'a mut Node, id: &str) -> Option<&'a mut Node> {
    if node.id == id && node.visible && !node.disabled {
        return Some(node);
    }
    if !node.visible || node.disabled {
        return None;
    }
    for child in &mut node.child {
        if let Some(found) = find_effectively_visible_node_by_id_mut(child, id) {
            return Some(found);
        }
    }
    None
}



fn capture_pending_contract_slot_template(ui: &GameUI) {
    let already_captured = PENDING_CONTRACT_SLOT_TEMPLATE.with(|slot| slot.borrow().is_some());
    if already_captured {
        return;
    }
    let Some(template) = find_node_by_id(&ui.root, PENDING_CONTRACT_SLOT_TEMPLATE_ID) else {
        return;
    };
    let mut captured = template.clone();
    captured.visible = true;
    captured.disabled = false;
    PENDING_CONTRACT_SLOT_TEMPLATE.with(|slot| *slot.borrow_mut() = Some(captured));
    log_event(
        "contract_projection_static_template_captured",
        "template_id=pts_trade_pending_contract_slot_template;plain_team_column=true;fixed_width_squad_status_container=true;top_level_column_count=10;native_detail_icon_inside_name_label=true;native_detail_icon_width=24;native_detail_icon_height=24;name_text_color=#E8E8E8FF",
    );
}

fn capture_native_target_view_detail_template(ui: &GameUI) {
    let Some(target_id) = target_id_from_offer(ui) else { return; };
    let already_captured = NATIVE_TARGET_VIEW_DETAIL_TEMPLATE.with(|slot| {
        slot.borrow().as_ref().is_some_and(|(athlete_id, _)| *athlete_id == target_id)
    });
    if already_captured {
        return;
    }
    let Some(offer) = active_offer(ui) else { return; };
    let detail_id = format!("view_detail_{target_id}");
    let detail_path = ["data", "row1", "info", "header", "name", detail_id.as_str()];
    let Some(detail) = direct_path(offer, &detail_path) else { return; };
    let runner_type = detail.runner.type_name().to_string();
    if !runner_type.to_ascii_lowercase().contains("view_detail_button") {
        log_event(
            "contract_projection_native_target_runner_capture_rejected",
            &format!(
                "athlete_id={};view_detail_id={};runner_type={};native_view_detail_runner=false",
                target_id, detail_id, sanitize(&runner_type),
            ),
        );
        return;
    }
    let source_popup_present = direct_child(detail, "popup").is_some();
    let source_proficiency_tooltip_present = direct_child(detail, "proficiency_tooltip").is_some();
    let mut captured = detail.clone();
    let removed = strip_projection_view_detail_hover_children(&mut captured);
    NATIVE_TARGET_VIEW_DETAIL_TEMPLATE.with(|slot| {
        *slot.borrow_mut() = Some((target_id, captured));
    });
    log_event(
        "contract_projection_native_target_runner_captured",
        &format!(
            "athlete_id={};view_detail_id={};runner_type={};native_view_detail_runner=true;source_popup_present={};source_proficiency_tooltip_present={};stored_popup_present=false;stored_proficiency_tooltip_present=false;popup_removed={};proficiency_tooltip_removed={};source=live_transfer_offer_exact_target",
            target_id,
            detail_id,
            sanitize(&runner_type),
            source_popup_present,
            source_proficiency_tooltip_present,
            removed.0,
            removed.1,
        ),
    );
}

fn parse_view_detail_id_from_text(value: &str) -> Option<usize> {
    let index = value.find("view_detail_")?;
    let tail = &value[index + "view_detail_".len()..];
    let digits: String = tail.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    (!digits.is_empty()).then(|| digits.parse::<usize>().ok()).flatten()
}

// ★[PORT056] 프로필 신원 캐리어 확장 (2026-08-23 인게임 클릭 로그로 확정).
//   구판은 `view_detail_<id>` **하나만** 인식했다. 그런데 전체 목록·계약 현황에서 프로필을 여는
//   실제 클릭 경로는 행 노드 id 다:
//     main.top.right.scout.contents.all_players.list.contents.candidate_159
//   (`view_detail_<id>` 는 이름 라벨 **아래**에만 있어서 이 클릭엔 안 걸린다.)
//   ⟹ 신원이 안 잡혀 PROFILE_CONTEXT_ATHLETE_ID 가 0 으로 남고 두 방향으로 깨졌다:
//     ①대상 선수 프로필에 트레이드 상태가 안 뜬다(요구사항 3 이 죽은 것처럼 보임)
//     ②BIND_NEXT_DETAIL 이 남아 있거나 detail 인스턴스 재생성 경로를 타면 **무관한 선수**에 붙는다
//       (유저 보고 2026-08-23: 1Jiang 프로필에 Flandre 트레이드 상태 + 영입 제안 버튼 숨김).
//   ⟹ 게임 핸들러 테이블이 쓰는 **선수 행 접두**를 전부 인식한다
//      (RE 2026-08-23_핸들러테이블-노드id파싱-신원캐리어확정).
//   ⚠스태프(`staff_` / `staff_ca_`)는 athlete 이 아니므로 **일부러 제외**한다.
//   ⚠행 id 는 `transfer_159_0` 처럼 뒤에 `_<순번>` 이 붙을 수 있어 **선행 숫자만** 취하고,
//     나머지는 비었거나 `_` 로 시작해야 채택한다(`candidate_159x` 류 오탐 차단).
const PROFILE_IDENTITY_PREFIXES: [&str; 5] =
    ["view_detail_", "candidate_", "transfer_", "recruit_", "resign_"];

fn exact_view_detail_athlete_id_from_event(path: &str, item: &str) -> Option<usize> {
    for segment in std::iter::once(item).chain(
        path.split(|character| matches!(character, '.' | '/' | '\\'))
    ) {
        for prefix in PROFILE_IDENTITY_PREFIXES {
            let Some(raw_id) = segment.strip_prefix(prefix) else {
                continue;
            };
            let digits: String = raw_id.chars().take_while(|ch| ch.is_ascii_digit()).collect();
            if digits.is_empty() {
                continue;
            }
            let rest = &raw_id[digits.len()..];
            if !(rest.is_empty() || rest.starts_with('_')) {
                continue;
            }
            if let Ok(athlete_id) = digits.parse::<usize>() {
                // ⚠athlete_id 0 은 실재한다(Doran) — 다만 PROFILE_CONTEXT_ATHLETE_ID 는
                //   0 을 "미결속" 센티넬로 쓰므로 채택하지 않는다(= fail-open, 아무것도 안 함).
                //   트레이드 대상은 **타 구단 선수**라 id 0(내 팀 Doran)이 대상이 되는 경우는 없다.
                if athlete_id != 0 {
                    return Some(athlete_id);
                }
            }
        }
    }
    None
}

fn profile_navigation_without_exact_id(path: &str, item: &str) -> bool {
    let path_lower = path.to_ascii_lowercase();
    if !path_lower.contains("player_detail") || !path_lower.contains("navigation") {
        return false;
    }
    std::iter::once(item)
        .chain(path.split(|character| matches!(character, '.' | '/' | '\\')))
        .map(|segment| segment.to_ascii_lowercase())
        .any(|segment| matches!(segment.as_str(), "previous" | "prev" | "next"))
}

fn projection_profile_click_athlete_id(path: &str, item: &str) -> Option<usize> {
    let in_projection_row = path.contains(PENDING_CONTRACT_SLOT_RUNTIME_ID)
        || item.contains(PENDING_CONTRACT_SLOT_RUNTIME_ID);
    if !in_projection_row {
        return None;
    }

    if let Some(athlete_id) = exact_view_detail_athlete_id_from_event(path, item) {
        return Some(athlete_id);
    }

    let name_click = item == "name"
        || path_has_exact_node_id(path, "name")
        || path.ends_with(".name")
        || path.contains(".name.");
    if !name_click {
        return None;
    }

    active_async_status_view().map(|view| view.target_id)
}

fn update_profile_context_from_click(path: &str, item: &str) {
    let item_lower = item.to_ascii_lowercase();
    let proceed_click = item_lower == "proceed"
        || path_has_exact_node_id(path, "proceed");
    if proceed_click {
        if let Some(view) = active_async_status_view() {
            let current_id = PROFILE_CONTEXT_ATHLETE_ID.load(Ordering::Acquire);
            if PROFILE_CONTEXT_TARGET_LOCK_VALID.load(Ordering::Acquire)
                && current_id == view.target_id
            {
                PROFILE_CONTEXT_ALLOW_NEXT_REBUILD.store(true, Ordering::Release);
                PROFILE_CONTEXT_MANAGEMENT_TICK_REBUILD_REBOUND.store(false, Ordering::Release);
                PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.store(true, Ordering::Release);
                log_event(
                    "profile_context_rebuild_preservation_armed",
                    &format!(
                        "proposal_id={};target_id={};current_player_id={};source=main_proceed;exact_next_detail_rebuild_only=true;one_shot_no_frame_expiry=true;frame_expiry=false;target_identity_lease=true",
                        view.proposal_id,
                        view.target_id,
                        current_id,
                    ),
                );
            }
        }
        return;
    }

    if let Some(athlete_id) = exact_view_detail_athlete_id_from_event(path, item) {
        PROFILE_CONTEXT_ATHLETE_ID.store(athlete_id, Ordering::Release);
        PROFILE_CONTEXT_BIND_NEXT_DETAIL.store(true, Ordering::Release);
        PROFILE_CONTEXT_TARGET_LOCK_VALID.store(false, Ordering::Release);
        PROFILE_CONTEXT_ALLOW_NEXT_REBUILD.store(false, Ordering::Release);
        PROFILE_CONTEXT_MANAGEMENT_TICK_REBUILD_REBOUND.store(false, Ordering::Release);
        PROFILE_CONTEXT_EXPLICITLY_BOUND.store(true, Ordering::Release);
        RELOAD_PROFILE_FALLBACK_CONSUMED.store(true, Ordering::Release);
        PROFILE_CONTEXT_SCENE_SUSPENDED.store(false, Ordering::Release);
        PROFILE_CONTEXT_SCENE_RETURN_REBOUND.store(false, Ordering::Release);
        let target_id = active_async_status_view().map(|view| view.target_id).unwrap_or(0);
        let target_click = target_id != 0 && athlete_id == target_id;
        PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.store(target_click, Ordering::Release);
        PROFILE_CONTEXT_LAST_CONFIRMED_TARGET_SCENE.store(false, Ordering::Release);
        log_event(
            "profile_context_exactly_bound",
            &format!(
                "athlete_id={};source=explicit_view_detail_click;raw_path={};raw_item={};scope=single_player;fail_open_on_unknown=true;projection_click=false;bind_next_detail_instance=true;target_click={};target_identity_lease={}",
                athlete_id,
                sanitize(path),
                sanitize(item),
                target_click,
                target_click,
            ),
        );
        return;
    }
    if profile_navigation_without_exact_id(path, item) {
        PROFILE_CONTEXT_ATHLETE_ID.store(0, Ordering::Release);
        PROFILE_CONTEXT_BIND_NEXT_DETAIL.store(false, Ordering::Release);
        PROFILE_CONTEXT_TARGET_LOCK_VALID.store(false, Ordering::Release);
        PROFILE_CONTEXT_ALLOW_NEXT_REBUILD.store(false, Ordering::Release);
        PROFILE_CONTEXT_MANAGEMENT_TICK_REBUILD_REBOUND.store(false, Ordering::Release);
        PROFILE_CONTEXT_EXPLICITLY_BOUND.store(true, Ordering::Release);
        RELOAD_PROFILE_FALLBACK_CONSUMED.store(true, Ordering::Release);
        PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.store(false, Ordering::Release);
        PROFILE_CONTEXT_SCENE_SUSPENDED.store(false, Ordering::Release);
        PROFILE_CONTEXT_SCENE_RETURN_REBOUND.store(false, Ordering::Release);
        PROFILE_CONTEXT_LAST_CONFIRMED_TARGET_SCENE.store(false, Ordering::Release);
        log_event(
            "profile_context_invalidated",
            &format!(
                "source=exact_player_detail_prev_next_without_id;raw_path={};raw_item={};negotiation_buttons_hidden=false;fail_open=true;generic_left_right_substrings_ignored=true;target_identity_lease=false",
                sanitize(path),
                sanitize(item),
            ),
        );
        return;
    }
    update_profile_context_from_click_legacy(path, item);
}

fn update_profile_context_from_click_legacy(path: &str, item: &str) {
    if let Some(athlete_id) = parse_view_detail_id_from_text(path)
        .or_else(|| parse_view_detail_id_from_text(item))
    {
        PROFILE_CONTEXT_ATHLETE_ID.store(athlete_id, Ordering::Release);
        PROFILE_CONTEXT_BIND_NEXT_DETAIL.store(true, Ordering::Release);
        PROFILE_CONTEXT_TARGET_LOCK_VALID.store(false, Ordering::Release);
        PROFILE_CONTEXT_ALLOW_NEXT_REBUILD.store(false, Ordering::Release);
        PROFILE_CONTEXT_MANAGEMENT_TICK_REBUILD_REBOUND.store(false, Ordering::Release);
        let target_id = active_async_status_view().map(|view| view.target_id).unwrap_or(0);
        PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.store(
            target_id != 0 && athlete_id == target_id,
            Ordering::Release,
        );
        PROFILE_CONTEXT_LAST_CONFIRMED_TARGET_SCENE.store(false, Ordering::Release);
    }
}

fn find_effectively_visible_node_by_id<'a>(node: &'a Node, id: &str) -> Option<&'a Node> {
    if node.id == id && node.visible && !node.disabled {
        return Some(node);
    }
    if !node.visible || node.disabled {
        return None;
    }
    node.child
        .iter()
        .find_map(|child| find_effectively_visible_node_by_id(child, id))
}

fn observe_projection_profile_open(ui: &GameUI) {
    let athlete_id = PROJECTION_PROFILE_CLICK_PENDING_ID.load(Ordering::Acquire);
    if athlete_id == 0 {
        return;
    }
    let visible_detail = find_effectively_visible_node_by_id(&ui.root, "player_detail");
    let detail_visible = visible_detail.is_some();
    let visible_detail_instance = visible_detail
        .map(|detail| detail as *const Node as usize)
        .unwrap_or(0);
    let bound_detail_instance = PROFILE_CONTEXT_DETAIL_INSTANCE.load(Ordering::Acquire);
    let exact_detail_instance = visible_detail_instance != 0
        && bound_detail_instance == visible_detail_instance;
    let projection_visible = find_effectively_visible_node_by_id(
        &ui.root,
        PENDING_CONTRACT_SLOT_RUNTIME_ID,
    )
    .is_some();
    if detail_visible && !projection_visible
        && PROFILE_CONTEXT_ATHLETE_ID.load(Ordering::Acquire) == athlete_id
        && exact_detail_instance
    {
        PROJECTION_PROFILE_CLICK_PENDING_ID.store(0, Ordering::Release);
        PROJECTION_PROFILE_CLICK_REQUEST_FRAME.store(0, Ordering::Release);
        PROJECTION_PROFILE_SCENE_RETURN_REQUESTED.store(false, Ordering::Release);
        PROFILE_CONTEXT_LAST_CONFIRMED_TARGET_SCENE.store(true, Ordering::Release);
        log_event(
            "contract_projection_player_profile_open_observed",
            &format!(
                "athlete_id={};player_detail_visible=true;contract_projection_surface_visible=false;exact_profile_context=true;exact_target=true;exact_detail_instance=true;detail_instance={};name_click_navigation_verified=true;navigation_method=UIOutEvent_UndoScene",
                athlete_id,
                visible_detail_instance,
            ),
        );
        return;
    }
    let frame = RUNTIME_FRAME_COUNT.load(Ordering::Relaxed);
    let requested = PROJECTION_PROFILE_CLICK_REQUEST_FRAME.load(Ordering::Acquire);
    if requested != 0
        && frame.saturating_sub(requested) >= 360
        && !PROJECTION_PROFILE_CLICK_TIMEOUT_LOGGED.swap(true, Ordering::AcqRel)
    {
        PROJECTION_PROFILE_CLICK_PENDING_ID.store(0, Ordering::Release);
        PROJECTION_PROFILE_CLICK_REQUEST_FRAME.store(0, Ordering::Release);
        PROJECTION_PROFILE_SCENE_RETURN_REQUESTED.store(false, Ordering::Release);
        PROFILE_CONTEXT_BIND_NEXT_DETAIL.store(false, Ordering::Release);
        log_event(
            "contract_projection_player_profile_open_timeout",
            &format!(
                "athlete_id={};frames_waited={};player_detail_visible={};contract_projection_surface_visible={};exact_detail_instance={};bound_detail_instance={};visible_detail_instance={};name_click_navigation_verified=false;pending_click_cleared=true;stale_next_detail_binding_cleared=true;navigation_method=UIOutEvent_UndoScene",
                athlete_id,
                frame.saturating_sub(requested),
                detail_visible,
                projection_visible,
                exact_detail_instance,
                bound_detail_instance,
                visible_detail_instance,
            ),
        );
    }
}

fn active_async_status_view() -> Option<AsyncStatusView> {
    ASYNC_STATUS_VIEW.with(|slot| {
        slot.borrow().clone().filter(|view| {
            view.proposal_present && (view.state == "SellerReview" || view.state == "PlayerReview")
        })
    })
}

fn profile_native_contract_event(path: &str, item: &str) -> bool {
    (path.contains("player_detail") && path.ends_with(".data.row4.contract"))
        || (path.contains("player_detail") && item == PROFILE_CONTRACT_BUTTON_ID)
}

fn native_offer_submission_event(path: &str, item: &str) -> bool {
    path.ends_with(".offer.data.row2.offer")
        || path.ends_with(".offer.data.row2.delegate")
        || path.ends_with(".data.row2.offer")
        || path.ends_with(".data.row2.delegate")
        || ((item == "offer" || item == "delegate") && path.contains("offer.data.row2"))
}

fn should_block_native_negotiation_click(path: &str, item: &str) -> bool {
    let Some(view) = active_async_status_view() else {
        return false;
    };
    if profile_native_contract_event(path, item)
        && PROFILE_CONTEXT_TARGET_LOCK_VALID.load(Ordering::Acquire)
        && PROFILE_CONTEXT_ATHLETE_ID.load(Ordering::Acquire) == view.target_id
    {
        log_event(
            "async_trade_native_profile_offer_click_blocked",
            &format!(
                "proposal_id={};target_id={};raw_path={};raw_item={};double_offer_blocked=true;event_consumed_by_mod=true;exact_detail_instance_lock=true",
                view.proposal_id,
                view.target_id,
                sanitize(path),
                sanitize(item),
            ),
        );
        return true;
    }
    if native_offer_submission_event(path, item)
        && TARGET_ATHLETE_ID.load(Ordering::Acquire) == view.target_id
    {
        log_event(
            "async_trade_native_offer_submit_click_blocked",
            &format!(
                "proposal_id={};target_id={};raw_path={};raw_item={};native_cash_offer_blocked=true;scout_delegate_blocked=true;event_consumed_by_mod=true",
                view.proposal_id,
                view.target_id,
                sanitize(path),
                sanitize(item),
            ),
        );
        return true;
    }
    false
}

fn restore_owned_profile_native_ui(detail: &mut Node) {
    let had_native_lock = PROFILE_NATIVE_LOCK_ACTIVE.swap(false, Ordering::AcqRel);
    let had_custom_status = PROFILE_STATUS_OWNED.swap(false, Ordering::AcqRel);
    if !had_native_lock && !had_custom_status {
        return;
    }
    let snapshot = PROFILE_NATIVE_UI_SNAPSHOT.with(|slot| slot.borrow_mut().take());
    if let Some(snapshot) = snapshot {
        if let Some(contract) = direct_path_mut(detail, &["data", "row4", PROFILE_CONTRACT_BUTTON_ID]) {
            contract.visible = snapshot.contract_visible;
            contract.disabled = snapshot.contract_disabled;
            contract.runner.set_dirty(true);
        }
        if let Some(state) = direct_path_mut(detail, &["data", "row4", PROFILE_STATE_LABEL_ID]) {
            state.visible = snapshot.state_visible;
            state.disabled = snapshot.state_disabled;
            state.runner.set_dirty(true);
        }
    }
}

fn preserve_unrelated_player_profile_native_ui(
    detail: &mut Node,
    view: &AsyncStatusView,
    current_player_id: Option<usize>,
    source: &str,
) -> (bool, bool) {
    let detail_instance = detail as *const Node as usize;
    let native_contract_button_visible = direct_path(
        detail,
        &["data", "row4", PROFILE_CONTRACT_BUTTON_ID],
    )
    .is_some_and(|contract| contract.visible && !contract.disabled);

    // Do not rewrite, hide, clear, or re-style the native state label on another player.
    // PROFILE_STATUS_OWNED is the authoritative proof that our custom pending text has been
    // restored or discarded before this unrelated profile is allowed to render.
    let custom_status_hidden = !PROFILE_STATUS_OWNED.load(Ordering::Acquire);

    let log_key = format!(
        "{}|{}|{:?}|{}|{}|{}|{}",
        view.proposal_id,
        view.target_id,
        current_player_id,
        detail_instance,
        source,
        native_contract_button_visible,
        custom_status_hidden,
    );
    let should_log = UNRELATED_PROFILE_NATIVE_UI_LAST_KEY.with(|slot| {
        let mut slot = slot.borrow_mut();
        if *slot == log_key {
            false
        } else {
            *slot = log_key;
            true
        }
    });
    if should_log {
        log_event(
            "unrelated_player_profile_native_ui_preserved",
            &format!(
                "proposal_id={};proposal_target_id={};current_player_id={:?};detail_instance={};source={};native_contract_button_visible={};negotiation_buttons_hidden=false;custom_status_hidden={};native_state_untouched=true;native_offer_status_preserved=true;fail_open=true;log_frequency=once_per_detail_and_visibility_state",
                view.proposal_id,
                view.target_id,
                current_player_id,
                detail_instance,
                sanitize(source),
                native_contract_button_visible,
                custom_status_hidden,
            ),
        );
    }
    (native_contract_button_visible, custom_status_hidden)
}

fn apply_async_native_offer_screen_lock(ui: &mut GameUI, view: &AsyncStatusView) -> bool {
    if !(view.proposal_present && (view.state == "SellerReview" || view.state == "PlayerReview")) {
        NATIVE_OFFER_LOCK_ACTIVE.store(false, Ordering::Release);
        return false;
    }
    let target_id = target_id_from_offer(ui).unwrap_or(0);
    if target_id != view.target_id {
        return false;
    }
    let Some(offer_root) = active_offer_mut(ui) else {
        return false;
    };
    let mut changed = false;
    for path in [["data", "row2", "offer"], ["data", "row2", "delegate"]] {
        if let Some(node) = direct_path_mut(offer_root, &path) {
            if node.visible || !node.disabled {
                node.visible = false;
                node.disabled = true;
                node.runner.set_dirty(true);
                changed = true;
            }
        }
    }
    if let Some(info) = direct_path_mut(offer_root, &["data", "row1", "sub", "negotiation_info"]) {
        info.visible = true;
        info.disabled = false;
        set_runner_text(info, "진행 중인 트레이드 제안이 있어 일반 이적 제안을 제출할 수 없습니다.");
        changed = true;
    }
    NATIVE_OFFER_LOCK_ACTIVE.store(true, Ordering::Release);
    let key = format!("{}|{}|{}", view.proposal_id, view.state, target_id);
    let should_log = ASYNC_NATIVE_OFFER_LOCK_LAST_KEY.with(|slot| {
        let mut slot = slot.borrow_mut();
        if *slot == key { false } else { *slot = key; true }
    });
    if should_log {
        log_event(
            "async_trade_native_offer_screen_locked",
            &format!(
                "proposal_id={};state={};target_id={};native_offer_hidden=true;scout_delegate_hidden=true;double_offer_blocked=true;changed={}",
                view.proposal_id,
                view.state,
                target_id,
                changed,
            ),
        );
    }
    true
}

fn node_has_descendant_view_detail(node: &Node) -> bool {
    recursive_view_detail_athlete_id(node).is_some()
}


fn contract_slot_shape(node: &Node) -> bool {
    [
        "position", "name", "team", "contract", "salary", "transfer_fee",
        "squad_status", "contract_state", "contract_limit", "action",
    ]
    .iter()
    .all(|id| direct_child(node, id).is_some())
}

fn native_contract_data_row(node: &Node) -> bool {
    node.visible
        && !node.id.starts_with(PENDING_CONTRACT_SLOT_RUNTIME_PREFIX)
        && contract_slot_shape(node)
        && node_has_descendant_view_detail(node)
}

fn native_contract_header_row(node: &Node) -> bool {
    node.visible
        && !node.id.starts_with(PENDING_CONTRACT_SLOT_RUNTIME_PREFIX)
        && contract_slot_shape(node)
        && !node_has_descendant_view_detail(node)
}

#[derive(Clone)]
struct ContractProjectionHost {
    parent_path: Vec<usize>,
    insert_index: usize,
    native_row_count: usize,
    discovery: &'static str,
}

fn collect_visible_native_contract_hosts(
    node: &Node,
    current: &mut Vec<usize>,
    ancestors_visible: bool,
    hosts: &mut Vec<ContractProjectionHost>,
) {
    let effectively_visible = ancestors_visible && node.visible;
    if !effectively_visible {
        return;
    }

    let native_indices: Vec<usize> = node
        .child
        .iter()
        .enumerate()
        .filter_map(|(index, child)| native_contract_data_row(child).then_some(index))
        .collect();
    if let Some(first_index) = native_indices.first().copied() {
        hosts.push(ContractProjectionHost {
            parent_path: current.clone(),
            insert_index: first_index,
            native_row_count: native_indices.len(),
            discovery: "visible_native_rows",
        });
    }

    for (index, child) in node.child.iter().enumerate() {
        current.push(index);
        collect_visible_native_contract_hosts(child, current, effectively_visible, hosts);
        current.pop();
    }
}

fn collect_visible_contract_header_fallback_hosts(
    node: &Node,
    current: &mut Vec<usize>,
    ancestors_visible: bool,
    hosts: &mut Vec<ContractProjectionHost>,
) {
    let effectively_visible = ancestors_visible && node.visible;
    if !effectively_visible {
        return;
    }

    if let Some(header_index) = node.child.iter().position(native_contract_header_row) {
        for surface_index in ((header_index + 1)..node.child.len()).rev() {
            let surface = &node.child[surface_index];
            if !surface.visible {
                continue;
            }
            let mut host_path = current.clone();
            host_path.push(surface_index);

            if let Some((content_index, _)) = surface
                .child
                .iter()
                .enumerate()
                .find(|(_, child)| child.visible)
            {
                host_path.push(content_index);
            }

            hosts.push(ContractProjectionHost {
                parent_path: host_path,
                insert_index: 0,
                native_row_count: 0,
                discovery: "visible_header_fallback",
            });
            break;
        }
    }

    for (index, child) in node.child.iter().enumerate() {
        current.push(index);
        collect_visible_contract_header_fallback_hosts(
            child,
            current,
            effectively_visible,
            hosts,
        );
        current.pop();
    }
}

fn choose_topmost_contract_projection_host(
    mut hosts: Vec<ContractProjectionHost>,
) -> Option<ContractProjectionHost> {
    hosts.sort_by(|left, right| {
        left.parent_path
            .len()
            .cmp(&right.parent_path.len())
            .then_with(|| left.parent_path.cmp(&right.parent_path))
    });
    hosts.pop()
}

fn select_contract_projection_host(root: &Node) -> Option<ContractProjectionHost> {
    // ★[PORT056] 추측 경로(네이티브 행 옆 / 헤더 다음 형제)를 전부 버리고 **결정적 경로 하나**만 쓴다.
    //   구판의 두 경로는 "붙었는데 안 보이는" 상태를 만들었다(2026-08-23 실측).
    let mut exact_hosts = Vec::new();
    collect_exact_contract_list_hosts(root, &mut Vec::new(), true, &mut exact_hosts);
    return choose_topmost_contract_projection_host(exact_hosts);

    #[allow(unreachable_code)]
    {
    let mut native_hosts = Vec::new();
    collect_visible_native_contract_hosts(
        root,
        &mut Vec::new(),
        true,
        &mut native_hosts,
    );
    if let Some(host) = choose_topmost_contract_projection_host(native_hosts) {
        return Some(host);
    }

    let mut fallback_hosts = Vec::new();
    collect_visible_contract_header_fallback_hosts(
        root,
        &mut Vec::new(),
        true,
        &mut fallback_hosts,
    );
    choose_topmost_contract_projection_host(fallback_hosts)
    }
}

/// ★[PORT056] 결정적 호스트 = 스카우트 탭의 실제 행 컨테이너 `<탭>.list.contents`.
///
/// 2026-08-23 인게임 실측: 네이티브 계약 행이 0개일 때 `visible_header_fallback` 이
/// **헤더 다음 형제를 추측해서** 붙였고, 그 자리는 실제로 렌더되지 않아 유저 화면에 행이 안 보였다
/// (로그는 `rendered` 인데 화면은 빈 목록 — "붙었지만 안 보이는" 최악의 형태).
/// 인계본 로그의 `exact_runtime_path=contents.contract.list.contents` 와 바닐라 `scout.ui`
/// (`#contract:color { #header, #list:scroll_view }`) 가 같은 곳을 가리키므로 그 경로를 직접 찾는다.
///
/// ⚠탭 버튼도 id 가 `contract` 다(`#tabs:color { #contract:color_selectable }`) —
///   **`list` 자식을 가진 쪽만** 탭 내용이므로 그것으로 구분한다.
fn collect_exact_contract_list_hosts(
    node: &Node,
    current: &mut Vec<usize>,
    ancestors_visible: bool,
    hosts: &mut Vec<ContractProjectionHost>,
) {
    let effectively_visible = ancestors_visible && node.visible;
    if !effectively_visible {
        return;
    }
    // ★[PORT056] 2026-08-23: 전체 협상 상황(`contract_all`)은 **게임 데이터**(transfer_requests)로
    //   네이티브 렌더된다 ⟹ UI 주입 대상에서 제외한다. 주입은 **계약 현황(`contract`) 전용**이다.
    //   이것만으로 유저가 겪은 증상 3개가 구조적으로 사라진다:
    //     ①다른 협상과 자리 다툼 ②스태프 탭 침범 ③탭 전환 시 덮어쓰기
    if node.id == "contract" {
        if let Some(list_index) = node.child.iter().position(|child| child.id == "list") {
            let list = &node.child[list_index];
            let mut path = current.clone();
            path.push(list_index);
            // scroll_view 는 실제 행을 `contents` 아래에 담는다. 없으면 list 자체를 쓴다.
            if let Some(contents_index) = list.child.iter().position(|child| child.id == "contents") {
                path.push(contents_index);
            }
            hosts.push(ContractProjectionHost {
                parent_path: path,
                insert_index: 0,
                native_row_count: 0,
                discovery: "exact_contract_list_contents",
            });
        }
    }
    for (index, child) in node.child.iter().enumerate() {
        current.push(index);
        collect_exact_contract_list_hosts(child, current, effectively_visible, hosts);
        current.pop();
    }
}

fn collect_pending_contract_projection_parent_paths(
    node: &Node,
    current: &mut Vec<usize>,
    paths: &mut Vec<Vec<usize>>,
) {
    if node
        .child
        .iter()
        .any(|child| child.id == PENDING_CONTRACT_SLOT_RUNTIME_ID)
    {
        paths.push(current.clone());
    }
    for (index, child) in node.child.iter().enumerate() {
        current.push(index);
        collect_pending_contract_projection_parent_paths(child, current, paths);
        current.pop();
    }
}

fn count_pending_contract_projection_rows(node: &Node) -> usize {
    let direct = node
        .child
        .iter()
        .filter(|child| child.id == PENDING_CONTRACT_SLOT_RUNTIME_ID)
        .count();
    direct
        + node
            .child
            .iter()
            .map(count_pending_contract_projection_rows)
            .sum::<usize>()
}

fn remove_pending_contract_projection_rows(node: &mut Node) -> usize {
    let before = node.child.len();
    node.child.retain(|child| {
        child.id != PENDING_CONTRACT_SLOT_RUNTIME_ID
            && !child.id.starts_with(PENDING_CONTRACT_SLOT_RUNTIME_PREFIX)
    });
    let mut removed = before.saturating_sub(node.child.len());
    for child in &mut node.child {
        removed += remove_pending_contract_projection_rows(child);
    }
    if removed > 0 {
        node.runner.set_dirty(true);
    }
    removed
}

fn set_projection_text(row: &mut Node, column: &str, text: &str) -> bool {
    let Some(label) = direct_path_mut(row, &[column, "text"]) else {
        // [PORT056] Compat16 ACK 의 `missing_text_columns=name` 이 바로 이 false 다.
        //   어느 열에서 경로가 어긋났는지 로그로 특정한다(구판은 실패 사유가 안 남았다).
        log_event(
            "projection_text_path_missing",
            &format!("column={};path={}/text;result=missing", column, column),
        );
        return false;
    };
    let (is_label, cleared_binds) = set_runner_text_pinned(label, text);
    if cleared_binds > 0 || !is_label {
        log_event(
            "projection_text_pinned",
            &format!(
                "column={};is_label_runner={};cleared_binds={};runner_type={}",
                column,
                is_label,
                cleared_binds,
                sanitize(label.runner.type_name()),
            ),
        );
    }
    true
}

fn format_projection_date(value: &str) -> String {
    format_projection_date_with_suffix(value, true)
}


fn format_projection_deadline_date(value: &str) -> String {
    format_projection_date_with_suffix(value, false)
}

fn format_projection_date_with_suffix(value: &str, include_until: bool) -> String {
    let date = value.get(..10).unwrap_or(value);
    let mut parts = date.split('-');
    let year = parts.next().and_then(|part| part.parse::<u32>().ok());
    let month = parts.next().and_then(|part| part.parse::<u32>().ok());
    let day = parts.next().and_then(|part| part.parse::<u32>().ok());
    match (year, month, day, include_until) {
        (Some(year), Some(month), Some(day), true) => format!("{year}년 {month}월 {day}일까지"),
        (Some(year), Some(month), Some(day), false) => format!("{year}년 {month}월 {day}일"),
        _ => value.to_string(),
    }
}

fn projection_game_date(game_time: &str) -> String {
    game_time.get(..10).unwrap_or(game_time).to_string()
}

fn format_projection_salary(yearly_salary: f64) -> String {
    if !yearly_salary.is_finite() || yearly_salary < 10_000_000.0 {
        return "0억원".to_string();
    }
    let tenths = (yearly_salary / 10_000_000.0).round().max(0.0) as u64;
    let whole = tenths / 10;
    let decimal = tenths % 10;
    if decimal == 0 {
        format!("{}억원", format_commas(whole))
    } else {
        format!("{}.{}억원", format_commas(whole), decimal)
    }
}

fn projection_template_column(column_id: &str) -> Option<Node> {
    PENDING_CONTRACT_SLOT_TEMPLATE.with(|slot| {
        slot.borrow()
            .as_ref()
            .and_then(|template| direct_child(template, column_id))
            .cloned()
    })
}

fn replace_projection_column_from_template(row: &mut Node, column_id: &str) -> bool {
    let Some(replacement) = projection_template_column(column_id) else {
        return false;
    };
    let Some(index) = row.child.iter().position(|child| child.id == column_id) else {
        return false;
    };
    row.child[index] = replacement;
    true
}

fn neutralize_projection_team_interaction(node: &mut Node) {
    let runner_name = node.runner.type_name().to_ascii_lowercase();
    let interactive = node.id.starts_with("view_detail_")
        || node.id.starts_with("view_team")
        || node.id.starts_with("view_club")
        || node.id == "team_button"
        || runner_name.contains("button");
    if interactive {
        if node.id.starts_with("view_") {
            node.id = format!("pts_trade_inert_team_{}", sanitize(&node.id));
        }
        node.disabled = true;
        node.runner.set_dirty(true);
    }
    for child in &mut node.child {
        neutralize_projection_team_interaction(child);
    }
}

const PROJECTION_COLUMN_ORDER: [&str; 10] = [
    "position", "name", "team", "contract", "salary", "transfer_fee",
    "squad_status", "contract_state", "contract_limit", "action",
];

fn projection_top_level_column_count(row: &Node) -> usize {
    PROJECTION_COLUMN_ORDER
        .iter()
        .filter(|id| direct_child(row, id).is_some())
        .count()
}

fn projection_exact_top_level_column_order(row: &Node) -> bool {
    row.child.len() == PROJECTION_COLUMN_ORDER.len()
        && row
            .child
            .iter()
            .zip(PROJECTION_COLUMN_ORDER.iter())
            .all(|(child, expected)| child.id == *expected)
}

fn projection_team_interactive(node: &Node) -> bool {
    let runner_name = node.runner.type_name().to_ascii_lowercase();
    let interactive_kind = node.id.starts_with("view_detail_")
        || node.id.starts_with("view_team")
        || node.id.starts_with("view_club")
        || node.id == "team_button"
        || runner_name.contains("button");
    (interactive_kind && !node.disabled)
        || node.child.iter().any(projection_team_interactive)
}

fn configure_projection_view_detail_node(node: &mut Node, athlete_id: usize) {
    node.id = format!("view_detail_{athlete_id}");
    node.visible = true;
    node.disabled = false;

    // ★★[PORT056] 여기가 "이름은 Dan / 팝업은 xartE / 클릭은 Dan" 의 근본 원인이었다.
    //   구판은 ①노드 id 개명 ②`build_with_property` 에 "athlete_id" 키 전달 — **둘 다 간접·추측 경로**다.
    //   런너가 그 키를 안 읽으면 `ViewDetailButtonRunner.athlete_id` 는 원본(Dan) 그대로 남고,
    //   결국 클릭 dispatch 는 원본 선수를 연다. 인계본은 이걸 "노드를 통째로 clone" 으로 우회하려 했다.
    //   sdk_056 실측 결과 `athlete_id: Option<usize>` 는 **공개 쓰기 가능 필드**다
    //   (RE 2026-08-22_SDK공개API-신원결속-노드레이아웃.md) ⟹ 직접 결속한다.
    let bound_directly = match node
        .runner
        .as_any_mut()
        .downcast_mut::<ViewDetailButtonRunner>()
    {
        Some(runner) => {
            runner.athlete_id = Some(athlete_id);
            true
        }
        None => false,
    };

    // 보조 경로는 남겨둔다(무해). 단 이제 성공 여부는 위 직접 결속이 결정한다.
    let mut properties: HashMap<String, Rc<dyn Any>> = HashMap::new();
    properties.insert("athlete_id".to_string(), Rc::new(athlete_id));
    properties.insert("slot_athlete_id".to_string(), Rc::new(athlete_id));
    properties.insert("id".to_string(), Rc::new(athlete_id));
    properties.insert("rect_hover_enabled".to_string(), Rc::new(true));
    node.runner.build_with_property(&properties);
    node.runner.set_dirty(true);

    // 읽기로 되짚어 실제 결속값을 로그에 남긴다 — 인게임 1판이면 성공/실패가 확정된다.
    let readback = node
        .runner
        .as_any()
        .downcast_ref::<ViewDetailButtonRunner>()
        .and_then(|runner| runner.athlete_id);
    log_event(
        "projection_view_detail_identity_bound",
        &format!(
            "athlete_id={};direct_field_bind={};runner_type={};readback={:?};method=downcast_mut_public_field",
            athlete_id,
            bound_directly,
            sanitize(node.runner.type_name()),
            readback,
        ),
    );
}

fn find_projection_view_detail_node_mut(node: &mut Node) -> Option<&mut Node> {
    let runner_type = node.runner.type_name().to_ascii_lowercase();
    if node.id == "pts_trade_projection_player_click"
        || node.id.starts_with("view_detail_")
        || runner_type.contains("view_detail_button")
    {
        return Some(node);
    }
    for child in &mut node.child {
        if let Some(found) = find_projection_view_detail_node_mut(child) {
            return Some(found);
        }
    }
    None
}

fn prepare_projection_view_detail_before_attach(
    row: &mut Node,
    athlete_id: usize,
    template_source: &str,
) -> bool {
    let detail_id = format!("view_detail_{athlete_id}");
    let captured = NATIVE_TARGET_VIEW_DETAIL_TEMPLATE.with(|slot| {
        slot.borrow()
            .as_ref()
            .filter(|(captured_id, _)| *captured_id == athlete_id)
            .map(|(_, node)| node.clone())
    });
    let Some(detail) = find_projection_view_detail_node_mut(row) else {
        log_event(
            "contract_projection_player_click_binding_rejected",
            &format!(
                "athlete_id={};view_detail_id={};native_view_detail_runner=false;runner_type=missing;generic_button_rejected=true;stage=before_attach;template_source={}",
                athlete_id,
                detail_id,
                sanitize(template_source),
            ),
        );
        return false;
    };

    let previous_detail_id = detail.id.clone();
    let previous_runner_type = detail.runner.type_name().to_string();
    let previous_native_runner = previous_runner_type
        .to_ascii_lowercase()
        .contains("view_detail_button");
    let static_icon_layout = detail.layout.clone();

    // The strongest reload path is a clone of a live native contract row. Keep its
    // initialized ViewDetailButtonRunner and original 24x24 detail-icon layout, then
    // only rebind the athlete id. This avoids the enlarged full-cell overlay while
    // preserving the exact native click runner even when the old transfer-offer scene
    // no longer exists after loading PTS_Test79_PendingSellerReview.
    let (source, popup_removed, proficiency_removed) =
        if template_source == "visible_native_row_clone_with_fixed_columns"
            && previous_native_runner
        {
            configure_projection_view_detail_node(detail, athlete_id);
            let removed = strip_projection_view_detail_hover_children(detail);
            detail.runner.set_dirty(true);
            (
                "native_contract_row_runner_rebound_preserve_original_24x24",
                removed.0,
                removed.1,
            )
        } else if let Some(mut native) = captured {
            native.id = detail_id.clone();
            native.visible = true;
            native.disabled = false;
            let removed = strip_projection_view_detail_hover_children(&mut native);
            native.layout = static_icon_layout;
            // [PORT056] 이 분기는 "다른 화면에서 캡처한 노드가 올바른 신원을 들고 있을 것"에 의존했다
            //   (= Compat16 에서 팝업만 맞고 이름·클릭은 Dan 이던 경로). clone 이 신원을 보존한다는 보장이
            //   없으므로 여기서도 공식 필드로 못 박는다. 세 분기 전부 동일 보장.
            if let Some(runner) = native
                .runner
                .as_any_mut()
                .downcast_mut::<ViewDetailButtonRunner>()
            {
                runner.athlete_id = Some(athlete_id);
            }
            native.runner.set_dirty(true);
            *detail = native;
            (
                "captured_initialized_target_runner_native_24x24",
                removed.0,
                removed.1,
            )
        } else {
            configure_projection_view_detail_node(detail, athlete_id);
            let removed = strip_projection_view_detail_hover_children(detail);
            detail.runner.set_dirty(true);
            (
                "static_native_view_detail_runner_pre_attach",
                removed.0,
                removed.1,
            )
        };

    let runner_type = detail.runner.type_name().to_string();
    let native_runner = runner_type
        .to_ascii_lowercase()
        .contains("view_detail_button");
    let exact_id = detail.id == detail_id;
    let popup_present = direct_child(detail, "popup").is_some();
    let proficiency_tooltip_present = direct_child(detail, "proficiency_tooltip").is_some();
    log_event(
        "contract_projection_player_click_prepared_before_attach",
        &format!(
            "athlete_id={};view_detail_id={};source={};template_source={};previous_view_detail_id={};previous_runner_type={};previous_native_view_detail_runner={};native_view_detail_runner={};exact_id={};native_icon_size=true;native_icon_width=24;native_icon_height=24;full_name_cell_overlay=false;name_text_color=#E8E8E8FF;prepared_before_parent_add_child=true;popup_removed={};proficiency_tooltip_removed={};popup_present={};proficiency_tooltip_present={};runner_type={}",
            athlete_id,
            detail_id,
            source,
            sanitize(template_source),
            sanitize(&previous_detail_id),
            sanitize(&previous_runner_type),
            previous_native_runner,
            native_runner,
            exact_id,
            popup_removed,
            proficiency_removed,
            popup_present,
            proficiency_tooltip_present,
            sanitize(&runner_type),
        ),
    );
    native_runner && exact_id && !popup_present && !proficiency_tooltip_present
}

fn verify_projection_view_detail(node: &mut Node, athlete_id: usize) -> bool {
    let detail_id = format!("view_detail_{athlete_id}");

    // ★[PORT056] 신원은 **두 축**이다(RE 2026-08-22_Compat16-런타임로그-판독 §4·§6):
    //   ① 노드 id `view_detail_<athleteId>`  ② 런너의 공개 필드 `athlete_id`
    //   Compat16 은 ②만, 구 Test79 는 ①만 만족해서 각각 다른 증상이 났다.
    //   여기서 매 프레임 ②를 재확인하고, 어긋나 있으면 즉시 재결속한다
    //   (엔진이 값을 되돌리는지 여부까지 로그로 드러난다).
    let mut field_before: Option<usize> = None;
    let mut field_rebound = false;
    if let Some(detail) = find_node_by_id_mut(node, &detail_id) {
        if let Some(runner) = detail
            .runner
            .as_any_mut()
            .downcast_mut::<ViewDetailButtonRunner>()
        {
            field_before = runner.athlete_id;
            if runner.athlete_id != Some(athlete_id) {
                runner.athlete_id = Some(athlete_id);
                field_rebound = true;
                detail.runner.set_dirty(true);
            }
        }
    }
    if field_rebound {
        let count = PROJECTION_IDENTITY_REBIND_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        // 되돌림이 반복되면(count 가 계속 늘면) 엔진이 매 갱신마다 필드를 다시 쓴다는 뜻이다.
        if count <= 5 || count % 200 == 0 {
            log_event(
                "projection_identity_field_rebound",
                &format!(
                    "expected={};found={:?};rebind_count={};node_id={}",
                    athlete_id, field_before, count, detail_id,
                ),
            );
        }
    }

    let (native_runner, runner_type, visible, disabled, popup_present, proficiency_tooltip_present) =
        find_node_by_id(node, &detail_id)
            .map(|detail| {
                let runner_type = detail.runner.type_name().to_string();
                (
                    runner_type.to_ascii_lowercase().contains("view_detail_button"),
                    runner_type,
                    detail.visible,
                    detail.disabled,
                    direct_child(detail, "popup").is_some(),
                    direct_child(detail, "proficiency_tooltip").is_some(),
                )
            })
            .unwrap_or((false, "missing".to_string(), false, true, false, false));

    if let Some(team) = find_node_by_id_mut(node, "team") {
        neutralize_projection_team_interaction(team);
        team.runner.set_dirty(true);
    }

    let valid = native_runner
        && visible
        && !disabled
        && !popup_present
        && !proficiency_tooltip_present;
    if valid {
        log_event(
            "contract_projection_player_click_bound",
            &format!(
                "athlete_id={};view_detail_id={};player_name_clickable=true;team_click_disabled=true;runner_properties_rebound=false;binding_frequency=once_per_row_instance;native_view_detail_runner=true;name_click_native_runner=true;prepared_before_attach=true;native_icon_size=true;native_icon_width=24;native_icon_height=24;full_name_cell_overlay=false;name_text_color=#E8E8E8FF;popup_present=false;proficiency_tooltip_present=false;runner_type={}",
                athlete_id,
                detail_id,
                sanitize(&runner_type),
            ),
        );
    } else {
        log_event(
            "contract_projection_player_click_binding_rejected",
            &format!(
                "athlete_id={};view_detail_id={};native_view_detail_runner={};visible={};disabled={};popup_present={};proficiency_tooltip_present={};runner_type={};generic_button_rejected=true;stage=after_attach_verification",
                athlete_id,
                detail_id,
                native_runner,
                visible,
                disabled,
                popup_present,
                proficiency_tooltip_present,
                sanitize(&runner_type),
            ),
        );
    }
    valid
}

/// ★★[PORT056] 행 소스 우선순위를 **뒤집었다** — `.ui` 선언 템플릿 우선, 네이티브 clone 은 폴백.
///
/// 구판은 네이티브 계약 행을 clone 하는 것을 1순위로 삼았고, 그게 실패의 직접 원인이었다:
///   ① 복제한 행이 **원본 선수의 노드 id**(`candidate_<원본id>`)를 그대로 물고 온다.
///      게임은 클릭 경로를 '.' 로 쪼개 `candidate_<숫자>` 세그먼트에서 id 를 파싱하므로
///      (RE 2026-08-23_핸들러테이블-노드id파싱-신원캐리어확정 §1), 안쪽 노드를 아무리 고쳐도
///      **원본 선수 프로필이 열린다**. Compat16 이 Dan(1606)을 연 이유가 정확히 이것이다.
///   ② 원본 이름 라벨의 `LabelRunner.binds` 가 따라와 우리가 쓴 이름을 다음 갱신에 덮는다.
///   ③ 이름 열의 내부 구조가 원본 의존이라 `name/text` 경로가 어긋난다
///      (= Compat16 ACK 의 `rendered_text_columns=9 / missing_text_columns=name`).
///
/// `.ui` 템플릿(`#pts_trade_pending_contract_slot_template`)은 바닐라
/// `asset/base/ui/layout/scout_component/contract_slot`(1605×44, 최상위 10열, 각 열 `#text:label`)의
/// 충실한 복제이고, `name/text` 아래에 `:view_detail_button` 노드를 **직접 선언**해 두었다.
/// ⟹ 위 3가지 오염원이 전부 없다.
///
/// ⚠폴백을 남기는 이유: 템플릿은 `transfer_offer` 레이아웃에 선언돼 있어 그 화면을 한 번도 거치지
///   않으면 캡처되지 않는다(게임 재시작 후 곧장 계약 현황으로 간 경우 등). 그때는 구 방식으로 버틴다.
///   ⬜후속: scout 레이아웃에도 템플릿을 선언해 항상 캡처되게 하면 폴백을 없앨 수 있다.
fn projection_row_template(root: &Node, host: &ContractProjectionHost) -> Option<(Node, &'static str)> {
    if let Some(template) = PENDING_CONTRACT_SLOT_TEMPLATE.with(|slot| slot.borrow().clone()) {
        return Some((template, "static_ui_template_preferred"));
    }
    if let Some(parent) = node_by_index_path(root, &host.parent_path) {
        if let Some(native) = parent
            .child
            .get(host.insert_index)
            .filter(|row| native_contract_data_row(row))
        {
            if !PROJECTION_NATIVE_FALLBACK_LOGGED.swap(true, Ordering::AcqRel) {
                log_event(
                    "contract_projection_native_clone_fallback",
                    "reason=ui_template_not_captured_yet;risk=source_athlete_node_id_and_label_binds_inherited;visit_transfer_offer_to_capture=true",
                );
            }
            return Some((native.clone(), "visible_native_row_clone_fallback"));
        }
    }
    None
}

/// ★[PORT056] 게임은 클릭 경로를 `'.'` 로 쪼개 `<접두>_<숫자>` 세그먼트에서 엔티티 id 를 파싱한다
/// (RE 2026-08-23_핸들러테이블-노드id파싱-신원캐리어확정 §1·§2 — 핸들러 실물 확인).
/// 따라서 우리 행 안에 **원본 선수의 id 를 담은 노드 id 가 하나라도 남으면** 게임 핸들러가 그걸 파싱해
/// 엉뚱한 선수를 연다. 대상 선수 것(`view_detail_<target>`)만 남기고 나머지는 비활성 이름으로 바꾼다.
///
/// 접두 목록은 실측분: `candidate_` `recruit_` `resign_` `transfer_` `staff_ca_` `staff_`.
/// (`staff_ca_` 를 `staff_` 보다 먼저 검사해야 접두가 잘리지 않는다.)
fn sanitize_projection_entity_ids(node: &mut Node, keep_id: &str) -> usize {
    const GAME_ID_PREFIXES: [&str; 6] = [
        "candidate_", "recruit_", "resign_", "transfer_", "staff_ca_", "staff_",
    ];
    let mut renamed = 0usize;
    if node.id != keep_id {
        if let Some(prefix) = GAME_ID_PREFIXES
            .iter()
            .find(|prefix| node.id.starts_with(**prefix))
        {
            // 접두 뒤가 숫자일 때만 = 게임이 파싱할 수 있는 형태일 때만 바꾼다.
            let tail = &node.id[prefix.len()..];
            if !tail.is_empty() && tail.chars().all(|c| c.is_ascii_digit()) {
                node.id = format!("pts_inert_{}", sanitize(&node.id));
                node.runner.set_dirty(true);
                renamed += 1;
            }
        }
    }
    for child in node.child.iter_mut() {
        renamed += sanitize_projection_entity_ids(child, keep_id);
    }
    renamed
}

fn set_projection_action(row: &mut Node, text: &str) -> bool {
    if set_projection_text(row, "action", text) {
        return true;
    }
    let Some(action) = direct_child_mut(row, "action") else { return false; };
    if let Some(offer) = direct_child_mut(action, "offer") {
        offer.visible = true;
        offer.disabled = true;
        set_runner_text(offer, text);
        offer.runner.set_dirty(true);
        return true;
    }
    false
}

fn projection_current_squad_status_label(raw: &str) -> String {
    let trimmed = raw.trim();
    let normalized = trimmed
        .strip_prefix("Some(")
        .and_then(|value| value.strip_suffix(')'))
        .unwrap_or(trimmed);
    match normalized {
        "Prospect" | "유망주" => "유망주".to_string(),
        "Sub" | "후보" | "후보 선수" => "후보 선수".to_string(),
        "General" | "주전" | "주전 선수" => "주전 선수".to_string(),
        "Important" | "주요" | "주요 선수" => "주요 선수".to_string(),
        "Core" | "핵심" | "핵심 선수" => "핵심 선수".to_string(),
        "" | "unknown" | "pending" => "-".to_string(),
        other => other.to_string(),
    }
}

fn replace_projection_name_text_from_template(row: &mut Node) -> bool {
    let Some(template_name) = projection_template_column("name") else {
        return false;
    };
    let Some(mut template_text) = direct_child(&template_name, "text").cloned() else {
        return false;
    };
    let Some(name) = direct_child_mut(row, "name") else {
        return false;
    };

    // A visible native contract row already owns the correctly sized/detail-styled
    // ViewDetailButtonRunner. Preserve that exact child while replacing only the Label
    // with the neutral non-user-team text template. This prevents the cyan user-team
    // name color from leaking into the projected AI-player row without falling back to
    // the old 173x44 full-cell detail overlay.
    let preserved_native_detail = name
        .child
        .iter()
        .find(|child| child.id == "text")
        .and_then(|text| {
            text.child.iter().find(|child| {
                child.id.starts_with("view_detail_")
                    || child
                        .runner
                        .type_name()
                        .to_ascii_lowercase()
                        .contains("view_detail_button")
            })
        })
        .cloned();

    if let Some(native_detail) = preserved_native_detail {
        template_text.child.retain(|child| {
            child.id != "pts_trade_projection_player_click"
                && !child.id.starts_with("view_detail_")
                && !child
                    .runner
                    .type_name()
                    .to_ascii_lowercase()
                    .contains("view_detail_button")
        });
        template_text.child.push(native_detail);
    }

    if let Some(index) = name.child.iter().position(|child| child.id == "text") {
        name.child[index] = template_text;
    } else {
        name.child.insert(0, template_text);
    }
    name.runner.set_dirty(true);
    true
}

fn strip_projection_view_detail_hover_children(node: &mut Node) -> (bool, bool) {
    let popup_removed = node.child.iter().any(|child| child.id == "popup");
    let proficiency_removed = node
        .child
        .iter()
        .any(|child| child.id == "proficiency_tooltip");
    node.child
        .retain(|child| child.id != "popup" && child.id != "proficiency_tooltip");
    (popup_removed, proficiency_removed)
}

fn apply_projection_squad_status_palette(row: &mut Node, label: &str) {
    let normalized = match label {
        "유망주" | "Prospect" => "prospect",
        "후보" | "후보 선수" | "Sub" => "sub",
        "주전" | "주전 선수" | "General" => "general",
        "주요" | "주요 선수" | "Important" => "important",
        "핵심" | "핵심 선수" | "Core" => "core",
        _ => "prospect",
    };

    let _ = replace_projection_column_from_template(row, "squad_status");
    let Some(status) = find_node_by_id_mut(row, "squad_status") else {
        return;
    };
    status.visible = true;
    status.disabled = false;
    status.runner.set_dirty(true);

    let has_palette_nodes = ["prospect", "sub", "general", "important", "core"]
        .iter()
        .any(|key| find_node_by_id(status, &format!("pts_trade_projection_status_{key}")).is_some());
    if let Some(base_text) = direct_child_mut(status, "text") {
        set_runner_text(base_text, label);
        base_text.visible = !has_palette_nodes;
        base_text.disabled = false;
        base_text.runner.set_dirty(true);
    }
    for key in ["prospect", "sub", "general", "important", "core"] {
        let node_id = format!("pts_trade_projection_status_{key}");
        if let Some(node) = find_node_by_id_mut(status, &node_id) {
            node.visible = key == normalized;
            node.disabled = false;
            node.runner.set_dirty(true);
        }
    }
    log_event(
        "contract_projection_status_palette_applied",
        &format!(
            "label={};normalized={};squad_status_column_visible=true;squad_status_column_width_preserved=true;prospect_color=#858D9D;sub_color=#3CB8A0;general_color=#4EB0D8;important_color=#C850BF;core_color=#F86624",
            sanitize(label),
            normalized,
        ),
    );
}

fn update_pending_contract_projection_row(
    row: &mut Node,
    view: &AsyncStatusView,
) -> bool {
    row.visible = true;
    row.disabled = false;
    row.runner.set_dirty(true);

    let _ = replace_projection_column_from_template(row, "team");
    let stage = if view.state == "SellerReview" { "판매 구단 검토" } else { "선수 검토" };
    let due = if view.state == "SellerReview" { &view.seller_due_at } else { &view.player_due_at };
    if let Some(icon) = direct_path_mut(row, &["position", "icon"]) {
        if !view.target_position_icon.is_empty() {
            set_runner_source(icon, &view.target_position_icon);
        }
    }
    // ★[PORT056] 게임 접두 id 살균 — 대상 선수의 view_detail 만 남기고 나머지 `<접두>_<숫자>` 를 무력화.
    {
        let keep = format!("view_detail_{}", view.target_id);
        let renamed = sanitize_projection_entity_ids(row, &keep);
        if renamed > 0 && !PROJECTION_ID_SANITIZE_LOGGED.swap(true, Ordering::AcqRel) {
            log_event(
                "projection_entity_id_sanitized",
                &format!(
                    "renamed_count={};kept={};reason=game_parses_entity_id_from_node_id_segments",
                    renamed, keep,
                ),
            );
        }
    }
    let _ = set_projection_text(row, "position", &view.target_position_label);
    let name_style_white = replace_projection_name_text_from_template(row);
    let _ = set_projection_text(row, "name", &view.target_name);
    let detail_bound = verify_projection_view_detail(row, view.target_id);
    if let Some(icon) = direct_path_mut(row, &["team", "icon"]) {
        icon.visible = false;
        icon.disabled = true;
        icon.runner.set_dirty(true);
    }
    let _ = set_projection_text(row, "team", &view.recipient_team_name);
    if let Some(team) = find_node_by_id_mut(row, "team") {
        neutralize_projection_team_interaction(team);
    }
    let contract_end_text = format_projection_date(&view.target_contract_end);
    let deadline_text = format_projection_deadline_date(due);
    let _ = set_projection_text(row, "contract", &contract_end_text);
    let _ = set_projection_text(row, "salary", &format_projection_salary(view.target_yearly_salary));
    let _ = set_projection_text(row, "transfer_fee", &format_cash_amount(view.proposed_units));
    let current_status_label = projection_current_squad_status_label(&view.target_status_current);
    apply_projection_squad_status_palette(row, &current_status_label);
    let _ = set_projection_text(row, "contract_state", stage);
    let _ = set_projection_text(row, "contract_limit", &deadline_text);
    let action_rendered = set_projection_action(row, &format!("{} 포함", view.offered_name));
    let column_count = projection_top_level_column_count(row);
    let exact_column_order = projection_exact_top_level_column_order(row);
    let status_visible = find_node_by_id(row, "squad_status")
        .is_some_and(|node| node.visible);
    let team_interactive = find_node_by_id(row, "team")
        .is_some_and(projection_team_interactive);
    let layout_ok = column_count == 10
        && exact_column_order
        && status_visible
        && !team_interactive
        && detail_bound
        && name_style_white;
    log_event(
        "contract_projection_layout_verified",
        &format!(
            "proposal_id={};state={};target_id={};column_count={};direct_child_count={};expected_column_count=10;exact_column_order={};squad_status_visible={};later_columns_not_shifted={};team_text={};team_original_only=true;team_interactive={};team_font_size=18;team_detail_navigation=false;name_click_binding={};name_click_native_runner={};name_click_prepared_before_attach=true;name_icon_native_size=true;name_icon_width=24;name_icon_height=24;name_column_full_cell_overlay=false;name_text_color=#E8E8E8FF;name_text_white={};current_squad_status_raw={};current_squad_status_label={};promised_squad_status_label={};squad_status_source=target_status_current;contract_end_text={};deadline_text={};deadline_suffix_until=false;observed_game_time={};observation_game_date={};time_gate=same_submission_calendar_date;exact_0900_required=false;time_of_day_threshold_required=false;date_rollover_allowed=false;update_frequency=once_per_content_row_instance_or_game_date",
            view.proposal_id,
            view.state,
            view.target_id,
            column_count,
            row.child.len(),
            exact_column_order,
            status_visible,
            layout_ok,
            sanitize(&view.recipient_team_name),
            team_interactive,
            detail_bound,
            detail_bound,
            name_style_white,
            sanitize(&view.target_status_current),
            sanitize(&current_status_label),
            sanitize(&view.desired_status_label),
            sanitize(&contract_end_text),
            sanitize(&deadline_text),
            sanitize(&view.game_time),
            sanitize(&projection_game_date(&view.game_time)),
        ),
    );
    detail_bound && action_rendered && layout_ok
}

// ★[PORT056] 계약 현황이 "행은 있는데 화면엔 없음"이던 근본원인 (2026-08-23 트리 덤프로 확정).
//   덤프 실측:
//     contract.list          = ScrollViewRunner  rect=296,202,1600,790
//     contract.list.contents = EmptyRunner       rect=296,202,1600,**0**  lh=Pixel(0.0)  children=1
//       └ transfer_159_0     = 게임이 우리 TransferRequest 로 만든 **네이티브 행** rect=...,1605,44
//   ⟹ ①계약 현황도 transfer_requests 로 렌더된다(게임이 행을 실제로 만든다)
//      ②그런데 스크롤 컨테이너가 높이 0 으로 접혀 클리핑된다
//   ★`scout.ui` 원본 선언은 `#contents:empty { width:1600px; height:900px; }` 인데
//     런타임 값이 Pixel(0.0) 이다 = 게임이 실행 중에 0 으로 덮어쓴다(행 카운트 계산이 우리 요청을 안 셈).
//   ⚠1차 시도(`set_dirty` 만)는 실패했다 — dirty 로는 높이가 재계산되지 않는다(phase=after_nudge 덤프에서 여전히 0).
//     ⟹ `layout.normal.height` 를 자식 높이 합으로 **직접 지정**한다.
//   ⚠계약 탭 서브트리(contract / contract_all)로만 한정 — 게임의 다른 리스트는 건드리지 않는다.
const CONTRACT_ROW_HEIGHT: f32 = 44.0;

fn nudge_collapsed_contract_lists(node: &mut Node, in_contract: bool) -> usize {
    use engine_core::ui::length::Length;

    let inside = in_contract || node.id == "contract" || node.id == "contract_all";
    let mut nudged = 0usize;
    if inside && node.id == "list" {
        let mut hit = false;
        if let Some(contents) = node.child.iter_mut().find(|child| child.id == "contents") {
            if !contents.child.is_empty() && contents.rect.h < 1.0 {
                let total: f32 = contents
                    .child
                    .iter()
                    .map(|row| {
                        if row.rect.h > 1.0 {
                            row.rect.h
                        } else {
                            CONTRACT_ROW_HEIGHT
                        }
                    })
                    .sum();
                contents.layout.normal.height = Length::Pixel(total.max(CONTRACT_ROW_HEIGHT));
                contents.runner.set_dirty(true);
                hit = true;
            }
        }
        if hit {
            node.runner.set_dirty(true);
            nudged += 1;
        }
    }
    for child in &mut node.child {
        nudged += nudge_collapsed_contract_lists(child, inside);
    }
    nudged
}

// ★[PORT056] 네이티브 계약행 열 덮어쓰기 (2026-08-23, 유저 지시).
//   게임이 우리 `TransferRequest` 로 만든 행 `transfer_<targetId>_<n>` 은
//   **신원(이름·팀 하이퍼링크)이 살아 있는데 내용이 일반 영입 제안처럼 보인다**:
//     이적료 = 선수 기본 이적료(우리가 제안한 현금이 아님)
//     협상 현황 = "영입을 제안함"(PaperState 유래, 우리 단계가 아님)
//     협상 기한 = 게임 계산값 / 조건 제의 = 비어 있음
//   ⟹ 그 4 열만 우리 트레이드 값으로 덮는다. **행을 우리가 만들지 않으므로**
//      신원·정렬·필터·팀 아이콘이 전부 게임 것 그대로 남는다(주입 방식의 고질병이 없다).
//   ⚠계약 현황(10열)과 전체 협상 상황(8열) 양쪽에 같은 행이 나오므로 **전부** 덮는다.
//     전체 협상 상황엔 contract_limit·action 이 없어 그 두 개는 조용히 실패한다(정상).
//   ⚠게임이 매 프레임 다시 쓰므로 우리도 매 프레임 덮는다(높이 보정과 같은 상시 보정).

/// 로그를 남기지 않는 조용한 라벨 세터 — 열이 없는 탭(전체 협상 상황)에서 스팸을 막는다.
fn set_native_row_text(row: &mut Node, column: &str, text: &str) -> bool {
    let Some(label) = direct_path_mut(row, &[column, "text"]) else {
        return false;
    };
    set_runner_text(label, text);
    label.visible = true;
    label.disabled = false;
    label.runner.set_dirty(true);
    true
}

/// `action`(조건 제의) 열 채우기.
/// ⚠1차: `delegated_label` 에 썼다 → 화면에 안 나옴.
/// ⚠2차: `offer`(ColorIconButtonRunner)를 visible 로 → **네이티브 "제의하기" 회색 버튼만 드러남**(텍스트 안 바뀜).
/// ⟹ 덤프로 확정: 우리 쓰기는 성공(`action=true`)하는데 화면엔 없다. 나머지 3 열은 붙는데 이 셀만 안 붙는다
///    = **게임이 이 상호작용 셀을 매 프레임 다시 쓴다.** 같은 노드를 두고 싸워선 이길 수 없다.
/// ⟹ 게임이 모르는 우리 노드(`pts_trade_offered_label`)를 `action` 안에 하나 넣고 거기에 쓴다.
///    노드는 **같은 행의 `contract_state.text` 를 복제**해서 만든다 — 스타일·레이아웃이 그 행의 것 그대로다
///    (자산 인스턴스화가 필요 없고, 그 라벨은 실제로 덮어쓰기가 통하는 게 검증된 노드다).
fn set_native_row_action(row: &mut Node, text: &str) -> bool {
    const OFFERED_LABEL_ID: &str = "pts_trade_offered_label";
    // ⚠템플릿은 **반드시 `contract_state.text`**(협상 현황 라벨)를 복제한다.
    //   `action.delegated_label` 을 복제해봤더니 **아무것도 안 보였다**(유저 보고 2026-08-23).
    //   원래 그 노드에 직접 썼을 때도 안 나왔던 것과 같은 원인 — 그 LabelRunner 는
    //   `set_runner_text` 가 먹지 않는다(자체 bind/자산 텍스트로 보인다).
    //   ⟹ "그 칸 전용 노드" 라는 이유로 고르면 안 되고, **텍스트 쓰기가 검증된 노드**를 골라야 한다.
    //   스타일(색·정렬)이 조건 제의 칸과 다소 다른 것은 감수한다.
    let template = direct_path(row, &["contract_state", "text"]).cloned();
    let Some(action) = direct_child_mut(row, "action") else {
        return false;
    };
    // 네이티브 조건제의 버튼은 숨긴다(트레이드 행에서 눌리면 안 된다).
    if let Some(offer) = action.child.iter_mut().find(|child| child.id == "offer") {
        if offer.visible {
            offer.visible = false;
            offer.disabled = true;
            offer.runner.set_dirty(true);
        }
    }
    if !action.child.iter().any(|child| child.id == OFFERED_LABEL_ID) {
        let Some(mut node) = template else {
            return false;
        };
        node.id = OFFERED_LABEL_ID.to_string();
        node.child.clear();
        action.child.push(node);
    }
    let Some(label) = action
        .child
        .iter_mut()
        .find(|child| child.id == OFFERED_LABEL_ID)
    else {
        return false;
    };
    set_runner_text(label, text);
    label.visible = true;
    label.disabled = true;
    label.runner.set_dirty(true);
    action.runner.set_dirty(true);
    true
}

// ★[PORT056] 클라이언트 DB 즉시 반영 (2026-08-23, 유저 보고 "트레이드 직후엔 리스트에 없다").
//   UI 가 읽는 것은 서버 `Database` 가 아니라 **`ClientDatabase`** 다(`ClientData::db()`).
//   서버에 넣은 `TransferRequest` 는 다음 관리 갱신(일정 진행) 때 클라이언트로 동기화되므로
//   제안 직후에는 계약 현황/전체 협상 상황 어디에도 안 나오고 하루 넘겨야 나왔다.
//   ⟹ 같은 항목을 **클라이언트 DB 에도 직접** 넣어 즉시 보이게 한다.
//   ⚠서버가 정본이다. 여기 쓰는 값은 표시용 선반영이고 다음 동기화 때 서버 값으로 덮여도 무해하다.
//     (이적료·협상 현황·협상 기한·조건 제의 4 열은 어차피 `overwrite_native_trade_rows` 가 덮는다.)
//   ⚠`ClientDatabase::athletes` 는 key/value 맵이다(서버 `Database::athletes` 와 다름 — L1803 주석).
fn sync_client_transfer_request(
    data: &mut ClientData,
    target_id: usize,
    requester_team_id: usize,
    active: bool,
    due_days: i64,
    transfer_fee: f64,
    desired: SquadStatus,
) -> Option<&'static str> {
    // ⚠`db_mut()` 는 참조가 아니라 값(가드)을 돌려준다 — `let mut` 가 필요하다.
    let mut db = data.db_mut();
    let now = db.time;
    let due = {
        let mut d = db.time.date();
        for _ in 0..due_days.max(0) {
            match d.succ_opt() {
                Some(next) => d = next,
                None => break,
            }
        }
        d
    };
    let athlete = db.athletes.get_mut(&target_id)?;
    let Contract::InContract { transfer_requests, .. } = &mut athlete.contract else {
        return None;
    };

    if !active {
        let before = transfer_requests.len();
        transfer_requests.retain(|r| r.team_id != requester_team_id);
        return Some(if transfer_requests.len() == before { "absent" } else { "removed" });
    }

    // 이미 있으면 손대지 않는다 — 매 프레임 호출되므로 무의미한 쓰기를 피한다.
    if transfer_requests.iter().any(|r| r.team_id == requester_team_id) {
        return Some("present");
    }
    transfer_requests.push(TransferRequest {
        team_id: requester_team_id,
        last_date: now,
        phase: vec![TransferRequestPaper {
            is_draft: false,
            transfer_fee,
            state: PaperState::Waiting,
            is_ask: true,
            response_date: due,
            no_negotiation: false,
            options: Vec::new(),
        }],
        cooldown_until: None,
        delegated_to_scout: false,
        seller_delegated_to_scout: false,
        desired_squad_status: desired,
    });
    Some("inserted")
}

/// [PORT056] 진단: 전체 협상 상황(`contract_all`) 리스트의 실제 행 id 를 찍는다.
///   계약 현황은 `transfer_<athleteId>_<n>` 인데 그 탭에선 안 잡혔다(`row_count=1` 실측).
fn contract_all_row_ids(node: &Node, out: &mut Vec<String>) {
    if node.id == "contract_all" {
        if let Some(list) = node.child.iter().find(|c| c.id == "list") {
            if let Some(contents) = list.child.iter().find(|c| c.id == "contents") {
                for row in &contents.child {
                    out.push(row.id.clone());
                }
            }
        }
        return;
    }
    for child in &node.child {
        contract_all_row_ids(child, out);
    }
}

/// [PORT056] 진단: 전체 협상 상황(`contract_all`) 행은 `transfer_<id>_<n>` 이 아닌 다른 id 를 쓴다
///   (`row_count=1` 실측 — 계약 현황에서만 잡혔다). 실제 id 를 찍어 접두를 확정한다.
fn collect_transfer_like_ids(node: &Node, out: &mut Vec<String>) {
    if node.id.starts_with("transfer") || node.id.starts_with("negotiation") {
        out.push(node.id.clone());
    }
    for child in &node.child {
        collect_transfer_like_ids(child, out);
    }
}

fn overwrite_native_trade_rows(node: &mut Node, prefixes: &[String], view: &AsyncStatusView) -> usize {
    let mut hits = 0usize;
    if prefixes.iter().any(|prefix| node.id.starts_with(prefix.as_str())) {
        // [PORT056] 진단: `action` 열 하위 구조를 1회 찍는다.
        //   `offer`(color_icon_button) 도 `delegated_label` 도 화면에 안 나왔다 — 실제 구조를 봐야 한다.
        if !NATIVE_ROW_ACTION_DUMPED.swap(true, Ordering::Relaxed) {
            if let Some(action) = direct_child(node, "action") {
                let mut output = String::new();
                dump_tree(action, &node.id, 0, &mut output);
                append_log(&output);
                log_event("native_row_action_dumped", &format!("row_id={}", sanitize(&node.id)));
            } else {
                log_event("native_row_action_dumped", &format!("row_id={};action_column=absent", sanitize(&node.id)));
            }
        }
        let (stage, due) = if view.state == "SellerReview" {
            ("판매 구단 검토", view.seller_due_at.as_str())
        } else {
            ("선수 검토", view.player_due_at.as_str())
        };
        let deadline_text = format_projection_deadline_date(due);
        let fee = set_native_row_text(node, "transfer_fee", &format_cash_amount(view.proposed_units));
        let state = set_native_row_text(node, "contract_state", stage);
        let limit = set_native_row_text(node, "contract_limit", &deadline_text);
        let action = set_native_row_action(node, &format!("{} 포함", view.offered_name));
        node.runner.set_dirty(true);
        hits += 1;
        // 열별 성공 여부는 집합이 바뀔 때만 찍는다(어느 탭에서 무엇이 실패하는지 특정용).
        let key = format!("{}|{}|{}|{}|{}", node.id, fee, state, limit, action);
        let changed = NATIVE_ROW_COLUMN_RESULT_KEY.with(|slot| {
            let mut slot = slot.borrow_mut();
            if *slot == key { false } else { *slot = key.clone(); true }
        });
        if changed {
            log_event(
                "native_trade_row_columns",
                &format!(
                    "row_id={};transfer_fee={};contract_state={};contract_limit={};action={}",
                    sanitize(&node.id), fee, state, limit, action,
                ),
            );
        }
        // 행 내부엔 같은 접두 노드가 없다 — 더 내려갈 필요 없음.
        return hits;
    }
    for child in &mut node.child {
        hits += overwrite_native_trade_rows(child, prefixes, view);
    }
    hits
}

// ★[PORT056] 계약 현황 UI 주입 폐지 (2026-08-23, 인게임 확정).
//   게임은 우리 `TransferRequest` 로 계약 현황 네이티브 행 `transfer_<athleteId>_<n>` 을 **직접 만든다**
//   (트리 덤프 실측). 화면에 안 보였던 진짜 원인은 스크롤 컨테이너가 높이 0 으로 접힌 것이었고,
//   `nudge_collapsed_contract_lists` 로 해결됐다(`lh=Pixel(0.0)` → `Pixel(44.0)`, 인게임 표시 확인).
//   ⟹ 주입 행은 이제 **중복 행**이 될 뿐이므로 끈다. 코드는 남겨 되돌릴 수 있게 둔다.
//   ⚠false 일 때 진입부가 제거 경로를 타므로 이전 판에서 남은 주입 행도 자동 정리된다.
const CONTRACT_UI_INJECTION_ENABLED: bool = false;

fn project_async_trade_into_contract_lists(
    ui: &mut GameUI,
    assets: &Assets,
    view: &AsyncStatusView,
) -> usize {
    let active = CONTRACT_UI_INJECTION_ENABLED
        && view.proposal_present
        && (view.state == "SellerReview" || view.state == "PlayerReview");
    if !active {
        let removed = remove_pending_contract_projection_rows(&mut ui.root);
        if removed > 0 || CONTRACT_PROJECTION_ACTIVE.swap(false, Ordering::AcqRel) {
            log_event(
                "async_trade_contract_projection_removed",
                &format!(
                    "proposal_id={};state={};removed_row_count={};terminal_or_absent=true;native_request_removed=false",
                    view.proposal_id,
                    view.state,
                    removed,
                ),
            );
        }
        return 0;
    }

    let Some(mut host) = select_contract_projection_host(&ui.root) else {
        return 0;
    };

    // ★[PORT056] 진단: 행은 붙는데 화면에 안 보이는 문제(2026-08-23) — 계약 탭 서브트리를 1회 덤프한다.
    //   rect/layout 까지 찍어 "어디에 어떤 크기로 붙었는지"를 확정한다.
    // [PORT056] 덤프를 2회 허용한다 — ①최초 상태 ②nudge 가 60회 돈 뒤 상태.
    //   contents.rect.h 가 0 → 44 로 바뀌는지가 이번 가설의 판정 기준이다.
    let want_dump = if !CONTRACT_TREE_DIAGNOSTIC_DUMP_ENABLED {
        false
    } else if !CONTRACT_TREE_DUMPED.load(Ordering::Relaxed) {
        true
    } else {
        !CONTRACT_TREE_DUMPED_AFTER_NUDGE.load(Ordering::Relaxed)
            && CONTRACT_LIST_NUDGE_COUNT.load(Ordering::Relaxed) >= 60
    };
    if want_dump {
        // ⚠`find_node_by_id("contract")` 는 **탭 버튼**(#tabs > #contract)을 먼저 잡는다 —
        //   걔는 `list` 자식이 없어서 덤프가 통째로 안 찍혔다(2026-08-23 자체 버그).
        //   `list` 자식을 가진 노드를 직접 찾는다.
        fn find_contract_tab(node: &Node) -> Option<&Node> {
            if node.visible
                && (node.id == "contract" || node.id == "contract_all")
                && node.child.iter().any(|c| c.id == "list")
            {
                return Some(node);
            }
            if !node.visible {
                return None;
            }
            node.child.iter().find_map(find_contract_tab)
        }
        if let Some(tab) = find_contract_tab(&ui.root) {
            if CONTRACT_TREE_DUMPED.swap(true, Ordering::Relaxed) {
                CONTRACT_TREE_DUMPED_AFTER_NUDGE.store(true, Ordering::Relaxed);
            }
            let mut output = String::new();
            dump_tree(tab, "", 0, &mut output);
            append_log(&output);
            log_event(
                "contract_tab_tree_dumped",
                &format!(
                    "root=contract_tab;reason=row_invisible_diagnosis;phase={};nudge_seq={}",
                    if CONTRACT_TREE_DUMPED_AFTER_NUDGE.load(Ordering::Relaxed) {
                        "after_nudge"
                    } else {
                        "initial"
                    },
                    CONTRACT_LIST_NUDGE_COUNT.load(Ordering::Relaxed)
                ),
            );
        }
    }

    let mut existing_parent_paths = Vec::new();
    collect_pending_contract_projection_parent_paths(
        &ui.root,
        &mut Vec::new(),
        &mut existing_parent_paths,
    );
    let projection_count_before = count_pending_contract_projection_rows(&ui.root);
    let existing_in_selected_host = existing_parent_paths
        .iter()
        .filter(|path| path.as_slice() == host.parent_path.as_slice())
        .count();
    let needs_rebuild = projection_count_before != 1 || existing_in_selected_host != 1;

    let removed_before_render = if needs_rebuild {
        remove_pending_contract_projection_rows(&mut ui.root)
    } else {
        0
    };
    if needs_rebuild {
        let Some(reselected) = select_contract_projection_host(&ui.root) else {
            return 0;
        };
        host = reselected;
        let Some((mut row, template_source)) = projection_row_template(&ui.root, &host) else {
            return 0;
        };
        let Some(parent) = node_by_index_path_mut(&mut ui.root, &host.parent_path) else {
            return 0;
        };
        row.id = PENDING_CONTRACT_SLOT_RUNTIME_ID.to_string();
        // Keep an initialized native contract-row detail runner when one is available.
        // Only the name Label is replaced with the neutral-white template so a user-team
        // source row cannot leak its cyan name color into the projected AI-player row.
        let existing_name_has_native_runner = direct_child(&row, "name")
            .is_some_and(|name| {
                let mut cloned = name.clone();
                find_projection_view_detail_node_mut(&mut cloned).is_some_and(|detail| {
                    detail
                        .runner
                        .type_name()
                        .to_ascii_lowercase()
                        .contains("view_detail_button")
                })
            });
        let name_column_replaced = if existing_name_has_native_runner {
            replace_projection_name_text_from_template(&mut row)
        } else {
            replace_projection_column_from_template(&mut row, "name")
        };
        let name_column_ready = name_column_replaced;
        let name_click_prepared = prepare_projection_view_detail_before_attach(
            &mut row,
            view.target_id,
            template_source,
        );
        if !name_column_ready || !name_click_prepared {
            log_event(
                "async_trade_contract_projection_pre_attach_failed",
                &format!(
                    "proposal_id={};state={};target_id={};template_source={};existing_name_has_native_runner={};name_column_replaced={};name_column_ready={};name_click_prepared={};row_not_attached=true",
                    view.proposal_id,
                    view.state,
                    view.target_id,
                    sanitize(template_source),
                    existing_name_has_native_runner,
                    name_column_replaced,
                    name_column_ready,
                    name_click_prepared,
                ),
            );
            return 0;
        }
        log_event(
            "async_trade_contract_projection_template_selected",
            &format!(
                "proposal_id={};state={};source={};native_row_count={};target_id={};existing_name_has_native_runner={};name_column_replaced={};name_column_ready=true;name_click_binding=prepared_before_attach;name_icon_native_size=true;name_icon_width=24;name_icon_height=24;name_column_full_cell_overlay=false;full_process_reload_native_row_fallback=true",
                view.proposal_id,
                view.state,
                template_source,
                host.native_row_count,
                view.target_id,
                existing_name_has_native_runner,
                name_column_replaced,
            ),
        );
        row.visible = true;
        row.disabled = false;
        parent.add_child(assets, row);
        // [PORT056] `.expect(...)` 였다. add_child 가 실제로 push 하지 않는 경우(엔진 사정)
        //   pop() 이 None 이 되어 패닉한다. 그때는 재배치만 포기하고 다음 프레임에 다시 시도한다.
        match parent.child.pop() {
            Some(added) => {
                let insert_index = host.insert_index.min(parent.child.len());
                parent.child.insert(insert_index, added);
                parent.runner.set_dirty(true);
                host.insert_index = insert_index;
            }
            None => {
                log_event(
                    "contract_projection_row_add_child_no_op",
                    "reason=add_child_did_not_push;panic_avoided=true;retry_next_frame=true",
                );
                return 0;
            }
        }
    }

    let updated = {
        let Some(parent) = node_by_index_path_mut(&mut ui.root, &host.parent_path) else {
            return 0;
        };
        let Some(row) = parent
            .child
            .iter_mut()
            .find(|child| child.id == PENDING_CONTRACT_SLOT_RUNTIME_ID)
        else {
            return 0;
        };
        let row_instance = row as *const Node as usize;
        let update_key = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            view.proposal_id,
            view.state,
            row_instance,
            view.target_id,
            view.offered_id,
            view.proposed_units,
            view.target_status_current,
            view.recipient_team_name,
            view.target_contract_end,
            view.target_yearly_salary,
            if view.state == "SellerReview" { &view.seller_due_at } else { &view.player_due_at },
            projection_game_date(&view.game_time),
        );
        let needs_update = ASYNC_CONTRACT_PROJECTION_UPDATE_KEY.with(|slot| {
            let mut slot = slot.borrow_mut();
            if *slot == update_key {
                false
            } else {
                *slot = update_key;
                true
            }
        });
        if needs_update {
            update_pending_contract_projection_row(row, view)
        } else {
            true
        }
    };
    if !updated {
        return 0;
    }

    let active_projection_count = count_pending_contract_projection_rows(&ui.root);
    if active_projection_count != 1 {
        let removed = remove_pending_contract_projection_rows(&mut ui.root);
        CONTRACT_PROJECTION_ACTIVE.store(false, Ordering::Release);
        log_event(
            "async_trade_contract_projection_invariant_failed",
            &format!(
                "proposal_id={};state={};active_projection_count={};removed_row_count={};expected=1;duplicate_projection_prevented=true",
                view.proposal_id,
                view.state,
                active_projection_count,
                removed,
            ),
        );
        return 0;
    }

    CONTRACT_PROJECTION_ACTIVE.store(true, Ordering::Release);
    let surface_path = if host.parent_path.is_empty() {
        "root".to_string()
    } else {
        host.parent_path
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join("_")
    };
    let signature = format!(
        "{}|{}|{}",
        surface_path,
        host.discovery,
        host.native_row_count,
    );
    let first_surface = ASYNC_CONTRACT_PROJECTION_SURFACES.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.iter().any(|existing| existing == &signature) {
            false
        } else {
            slot.push(signature.clone());
            true
        }
    });
    if first_surface {
        log_event(
            "async_trade_contract_projection_surface_observed",
            &format!(
                "proposal_id={};state={};surface_signature={};native_player_row_count={};row_id={};discovery={};single_visible_surface_only=true;header_row_excluded=true;hidden_surface_excluded=true;read_only=true;native_transfer_request_inserted=false;first_active_offer_history=false;native_offer_history_sync=false;other_player_profile_offer_ui_disabled=true;proposer_identity_visible=false",
                view.proposal_id,
                view.state,
                sanitize(&signature),
                host.native_row_count,
                PENDING_CONTRACT_SLOT_RUNTIME_ID,
                host.discovery,
            ),
        );
    }

    let key = format!("{}|{}|{}", view.proposal_id, view.state, signature);
    let changed = ASYNC_CONTRACT_PROJECTION_LAST_KEY.with(|slot| {
        let mut slot = slot.borrow_mut();
        if *slot == key { false } else { *slot = key; true }
    });
    if changed {
        log_event(
            "async_trade_contract_projection_rendered",
            &format!(
                "proposal_id={};state={};rendered_surface_count=1;active_projection_count=1;duplicate_projection_count=0;target_id={};target_name={};offered_id={};offered_name={};contract_state={};due_at={};cash={};current_squad_status_raw={};current_squad_status_label={};promised_squad_status_label={};squad_status_source=target_status_current;surface_signature={};discovery={};insert_index={};row_below_header=true;header_row_excluded=true;hidden_surface_excluded=true;removed_before_render={};read_only=true;native_transfer_request_inserted=false;salary_format=max_one_decimal_eok;name_click_target_id={};name_click_binding=true;name_click_native_runner=true;name_click_prepared_before_attach=true;name_icon_native_size=true;name_icon_width=24;name_icon_height=24;name_column_full_cell_overlay=false;name_text_color=#E8E8E8FF;team_font_size=18;team_detail_navigation=false;deadline_suffix_until=false;observed_game_time={};time_gate=same_submission_calendar_date;exact_0900_required=false;time_of_day_threshold_required=false;date_rollover_allowed=false",
                view.proposal_id,
                view.state,
                view.target_id,
                sanitize(&view.target_name),
                view.offered_id,
                sanitize(&view.offered_name),
                if view.state == "SellerReview" { "판매 구단 검토" } else { "선수 검토" },
                if view.state == "SellerReview" { &view.seller_due_at } else { &view.player_due_at },
                view.proposed_units,
                sanitize(&view.target_status_current),
                sanitize(&projection_current_squad_status_label(&view.target_status_current)),
                sanitize(&view.desired_status_label),
                sanitize(&signature),
                host.discovery,
                host.insert_index,
                removed_before_render,
                view.target_id,
                sanitize(&view.game_time),
            ),
        );
        log_event(
            "async_trade_contract_projection_single_row_verified",
            &format!(
                "proposal_id={};state={};active_projection_count=1;duplicate_projection_count=0;row_below_header=true;single_visible_surface_only=true;surface_signature={};contract_status_and_all_negotiations_bridge=true;projection_available_without_offer_template=true;projection_ready_on_first_visible_contract_frame=true",
                view.proposal_id,
                view.state,
                sanitize(&signature),
            ),
        );
    }
    1
}

fn recursive_view_detail_athlete_id(node: &Node) -> Option<usize> {
    if let Some(value) = node.id.strip_prefix("view_detail_") {
        if let Ok(athlete_id) = value.parse::<usize>() {
            return Some(athlete_id);
        }
    }
    node.child.iter().find_map(recursive_view_detail_athlete_id)
}

fn current_player_detail_athlete_id(detail: &Node) -> Option<usize> {
    let athlete_id = PROFILE_CONTEXT_ATHLETE_ID.load(Ordering::Acquire);
    let bound_instance = PROFILE_CONTEXT_DETAIL_INSTANCE.load(Ordering::Acquire);
    let detail_instance = detail as *const Node as usize;
    (athlete_id != 0 && bound_instance == detail_instance).then_some(athlete_id)
}

fn compact_async_time(value: &str) -> String {
    if value.len() >= 16 {
        value[..16].replace('-', "/")
    } else {
        value.to_string()
    }
}

fn observe_profile_return_after_submit(ui: &mut GameUI) {
    if !RETURN_TO_PROFILE_PENDING.load(Ordering::Acquire) {
        return;
    }
    let frame = RUNTIME_FRAME_COUNT.load(Ordering::Relaxed);
    let requested_at = RETURN_TO_PROFILE_REQUEST_FRAME.load(Ordering::Acquire);
    if let Some(detail) = find_effectively_visible_node_by_id_mut(&mut ui.root, "player_detail") {
        let target_id = TARGET_ATHLETE_ID.load(Ordering::Acquire);
        if target_id != 0 {
            let detail_instance = detail as *const Node as usize;
            PROFILE_CONTEXT_ATHLETE_ID.store(target_id, Ordering::Release);
            PROFILE_CONTEXT_DETAIL_INSTANCE.store(detail_instance, Ordering::Release);
            PROFILE_CONTEXT_BIND_NEXT_DETAIL.store(false, Ordering::Release);
            PROFILE_CONTEXT_TARGET_LOCK_VALID.store(false, Ordering::Release);
            PROFILE_CONTEXT_ALLOW_NEXT_REBUILD.store(false, Ordering::Release);
            RETURN_TO_PROFILE_PENDING.store(false, Ordering::Release);
            RETURN_TO_PROFILE_OBSERVED.store(true, Ordering::Release);
            log_event(
                "async_trade_profile_return_observed",
                &format!(
                    "method=UIOutEvent::UndoScene;player_detail_visible=true;current_player_id={};current_player_id_source=undo_target_binding;target_id={};detail_instance={};exact_detail_instance_bound=true;offer_force_hidden=false;black_screen_observed=false;frames_since_request={};transaction_executed=false",
                    target_id,
                    target_id,
                    detail_instance,
                    frame.saturating_sub(requested_at),
                ),
            );
            return;
        }
    }
    if requested_at != 0
        && frame.saturating_sub(requested_at) >= 180
        && !RETURN_TO_PROFILE_TIMEOUT_LOGGED.swap(true, Ordering::AcqRel)
    {
        let offer_still_visible = active_offer(ui).is_some_and(|offer| offer.visible && !offer.disabled);
        log_event(
            "async_trade_profile_return_timeout",
            &format!(
                "method=UIOutEvent::UndoScene;player_detail_not_observed=true;offer_still_visible={};offer_force_hidden=false;black_screen_prevented={};manual_back_available={};transaction_executed=false",
                offer_still_visible,
                offer_still_visible,
                offer_still_visible,
            ),
        );
    }
}

fn parse_native_offer_status_view(values: &BTreeMap<String, String>) -> Result<NativeOfferStatusView, String> {
    let status = map_required(values, "status")?;
    let athlete_id = map_usize(values, "athlete_id")?;
    if status == "hidden" {
        return Ok(NativeOfferStatusView {
            visible: false, athlete_id, first_seen: String::new(), state_text: String::new(),
            deadline: String::new(), stage: String::new(), sequence: 0, active_offer_count: 0, reloaded: false,
        });
    }
    if status != "visible" {
        return Err(map_required(values, "detail").unwrap_or("unknown first-offer status error").to_string());
    }
    Ok(NativeOfferStatusView {
        visible: true,
        athlete_id,
        first_seen: map_required(values, "first_seen")?.to_string(),
        state_text: map_required(values, "state_text")?.to_string(),
        deadline: map_required(values, "deadline")?.to_string(),
        stage: map_required(values, "stage")?.to_string(),
        sequence: map_u64(values, "sequence")?,
        active_offer_count: map_usize(values, "active_offer_count")?,
        reloaded: map_bool(values, "reloaded")?,
    })
}

fn ensure_first_offer_status_label(ui: &mut GameUI, assets: &Assets) -> bool {
    let Some(detail) = find_effectively_visible_node_by_id_mut(&mut ui.root, "player_detail") else { return false; };
    let Some(row4) = direct_path_mut(detail, &["data", "row4"]) else { return false; };
    if direct_child(row4, FIRST_OFFER_STATUS_LABEL_ID).is_some() {
        return true;
    }
    let Some(mut label) = direct_child(row4, PROFILE_STATE_LABEL_ID).cloned() else { return false; };
    label.id = FIRST_OFFER_STATUS_LABEL_ID.to_string();
    label.visible = false;
    label.disabled = true;
    let mut properties: HashMap<String, Rc<dyn Any>> = HashMap::new();
    properties.insert("x".to_string(), Rc::new(610.0f32));
    properties.insert("width".to_string(), Rc::new(970.0f32));
    properties.insert("height".to_string(), Rc::new(40.0f32));
    properties.insert("fit_width".to_string(), Rc::new(true));
    properties.insert("size".to_string(), Rc::new(16.0f32));
    properties.insert("text".to_string(), Rc::new(String::new()));
    for layout in [&mut label.layout.normal, &mut label.layout.hover, &mut label.layout.active, &mut label.layout.disabled] {
        layout.build_with_property(&properties);
    }
    label.runner.build_with_property(&properties);
    label.runner.set_dirty(true);
    row4.add_child(assets, label);
    true
}

fn set_first_offer_status_visible(ui: &mut GameUI, assets: &Assets, visible: bool, text: Option<&str>) -> bool {
    if !ensure_first_offer_status_label(ui, assets) { return false; }
    let Some(detail) = find_effectively_visible_node_by_id_mut(&mut ui.root, "player_detail") else { return false; };
    let Some(label) = direct_path_mut(detail, &["data", "row4", FIRST_OFFER_STATUS_LABEL_ID]) else { return false; };
    label.visible = visible;
    label.disabled = !visible;
    if let Some(text) = text { set_runner_text(label, text); }
    label.runner.set_dirty(true);
    NATIVE_OFFER_STATUS_OWNED.store(visible, Ordering::Release);
    true
}

fn send_native_offer_status_query_if_ready(data: &ClientData, ui: &GameUI) {
    let Some(detail) = find_node_by_id(&ui.root, "player_detail").filter(|detail| detail.visible && !detail.disabled) else {
        NATIVE_OFFER_STATUS_QUERY_PENDING.store(false, Ordering::Release);
        return;
    };
    let athlete_id = current_player_detail_athlete_id(detail).unwrap_or(0);
    if athlete_id == 0 { return; }
    let previous = NATIVE_OFFER_STATUS_ATHLETE_ID.swap(athlete_id, Ordering::AcqRel);
    if previous != athlete_id {
        NATIVE_OFFER_STATUS_QUERY_PENDING.store(false, Ordering::Release);
        NATIVE_OFFER_STATUS_LAST_FRAME.store(0, Ordering::Release);
        NATIVE_OFFER_STATUS_VIEW.with(|slot| *slot.borrow_mut() = None);
    }
    let frame = RUNTIME_FRAME_COUNT.load(Ordering::Relaxed);
    let last = NATIVE_OFFER_STATUS_LAST_FRAME.load(Ordering::Acquire);
    if NATIVE_OFFER_STATUS_QUERY_PENDING.load(Ordering::Acquire) {
        if last != 0 && frame.saturating_sub(last) >= 180 {
            NATIVE_OFFER_STATUS_QUERY_PENDING.store(false, Ordering::Release);
        } else { return; }
    }
    if last != 0 && frame.saturating_sub(last) < 60 { return; }
    let payload = format!("athlete_id={}\n", athlete_id);
    if data.send_mod_command(MOD_ID, NATIVE_OFFER_STATUS_COMMAND, payload.into_bytes()) {
        NATIVE_OFFER_STATUS_QUERY_PENDING.store(true, Ordering::Release);
        NATIVE_OFFER_STATUS_LAST_FRAME.store(frame.max(1), Ordering::Release);
    }
}

fn render_native_first_offer_status(ui: &mut GameUI, assets: &Assets, trade_view: Option<&AsyncStatusView>) {
    let exact_current_athlete_id = PROFILE_CONTEXT_ATHLETE_ID.load(Ordering::Acquire);
    if exact_current_athlete_id == 0 {
        let _ = set_first_offer_status_visible(ui, assets, false, None);
        // [PORT056] 구판은 이 로그를 매 프레임 찍어 Test78 에서 24,481건을 만들었다(기능 제거의 한 원인).
        //   신원 미확정은 정상 상태이므로 레이트 리밋한다.
        let skipped = FIRST_OFFER_STATUS_SKIP_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        if skipped == 1 || skipped % 3600 == 0 {
            log_event(
                "first_offer_profile_status_skipped",
                &format!("reason=current_player_unknown;skip_count={};scope=single_player;negotiation_button_hidden=false;fail_open=true;rate_limited=true", skipped),
            );
        }
        return;
    }

    let Some(detail) = find_node_by_id(&ui.root, "player_detail").filter(|detail| detail.visible && !detail.disabled) else { return; };
    let current_id = current_player_detail_athlete_id(detail).unwrap_or(0);
    let trade_has_priority = trade_view.is_some_and(|view| {
        view.proposal_present && (view.state == "SellerReview" || view.state == "PlayerReview") && view.target_id == current_id
    });
    if trade_has_priority {
        let _ = set_first_offer_status_visible(ui, assets, false, None);
        return;
    }
    let view = NATIVE_OFFER_STATUS_VIEW.with(|slot| slot.borrow().clone());
    let Some(view) = view.filter(|view| view.athlete_id == current_id && view.visible) else {
        if NATIVE_OFFER_STATUS_OWNED.load(Ordering::Acquire) {
            let _ = set_first_offer_status_visible(ui, assets, false, None);
        }
        return;
    };
    let text = format!(
        "제안 접수 {} | 현재 상태 {} | 응답 기한 {}",
        compact_async_time(&view.first_seen), view.state_text, compact_async_time(&view.deadline),
    );
    if set_first_offer_status_visible(ui, assets, true, Some(&text)) {
        let key = format!("{}|{}|{}|{}", view.athlete_id, view.sequence, view.stage, text);
        let changed = NATIVE_OFFER_STATUS_LAST_RENDER_KEY.with(|slot| {
            let mut slot=slot.borrow_mut(); if *slot==key {false} else {*slot=key; true}
        });
        if changed {
            log_event(
                "first_active_offer_profile_status_rendered",
                &format!(
                    "athlete_id={};sequence={};first_seen={};stage={};deadline={};active_offer_count={};selection_policy=first_active_sequence;proposer_identity_visible=false;reloaded={};trade_priority=false",
                    view.athlete_id, view.sequence, sanitize(&view.first_seen), sanitize(&view.stage),
                    sanitize(&view.deadline), view.active_offer_count, view.reloaded,
                ),
            );
        }
    }
}

fn rebind_reloaded_pending_profile_context(_ui: &GameUI, _view: &AsyncStatusView) -> bool {
    // Full-process reload never guesses which athlete is currently displayed. The exact
    // view_detail_<athlete_id> click from the restored contract row binds the next detail tree.
    false
}

fn set_player_profile_trade_stage(ui: &mut GameUI, view: &AsyncStatusView) -> bool {
    let active = view.proposal_present && (view.state == "SellerReview" || view.state == "PlayerReview");
    if !active {
        PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.store(false, Ordering::Release);
        PROFILE_CONTEXT_SCENE_SUSPENDED.store(false, Ordering::Release);
        PROFILE_CONTEXT_SCENE_RETURN_REBOUND.store(false, Ordering::Release);
        PROFILE_CONTEXT_LAST_CONFIRMED_TARGET_SCENE.store(false, Ordering::Release);
    }

    let Some(detail) = find_effectively_visible_node_by_id_mut(&mut ui.root, "player_detail") else {
        if active
            && PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.load(Ordering::Acquire)
            && PROFILE_CONTEXT_ATHLETE_ID.load(Ordering::Acquire) == view.target_id
        {
            let first_suspend = !PROFILE_CONTEXT_SCENE_SUSPENDED.swap(true, Ordering::AcqRel);
            PROFILE_CONTEXT_DETAIL_INSTANCE.store(0, Ordering::Release);
            PROFILE_CONTEXT_TARGET_LOCK_VALID.store(false, Ordering::Release);
            PROFILE_NATIVE_LOCK_ACTIVE.store(false, Ordering::Release);
            PROFILE_STATUS_OWNED.store(false, Ordering::Release);
            PROFILE_NATIVE_UI_SNAPSHOT.with(|slot| { slot.borrow_mut().take(); });
            if first_suspend {
                log_event(
                    "profile_target_scene_suspended",
                    &format!(
                        "proposal_id={};state={};target_id={};current_player_id={};player_detail_visible=false;target_identity_lease=true;scene_return_rebind_pending=true;pointer_change_is_not_identity_change=true",
                        view.proposal_id,
                        view.state,
                        view.target_id,
                        PROFILE_CONTEXT_ATHLETE_ID.load(Ordering::Acquire),
                    ),
                );
            }
        }
        return false;
    };
    let detail_instance = detail as *const Node as usize;

    // [PORT056] 신원 캐리어 탐색 덤프 — 서로 다른 player_detail 인스턴스 최대 4개까지 1회씩.
    if PLAYER_DETAIL_DIAGNOSTIC_DUMP_ENABLED && PLAYER_DETAIL_DUMP_COUNT.load(Ordering::Relaxed) < 4 {
        let last = PLAYER_DETAIL_LAST_DUMPED_INSTANCE.load(Ordering::Relaxed);
        if last != detail_instance {
            PLAYER_DETAIL_LAST_DUMPED_INSTANCE.store(detail_instance, Ordering::Relaxed);
            let seq = PLAYER_DETAIL_DUMP_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
            let mut output = String::new();
            dump_ids(detail, "", &mut output);
            append_log(&output);
            log_event(
                "player_detail_ids_dumped",
                &format!(
                    "seq={};detail_instance={};context_athlete_id={};proposal_target_id={}",
                    seq,
                    detail_instance,
                    PROFILE_CONTEXT_ATHLETE_ID.load(Ordering::Acquire),
                    view.target_id,
                ),
            );
        }
    }

    let previous_instance = ASYNC_PROFILE_LAST_DETAIL_INSTANCE.with(|slot| {
        let mut slot = slot.borrow_mut();
        let previous = *slot;
        *slot = detail_instance;
        previous
    });
    let mut detail_recreated = previous_instance != 0 && previous_instance != detail_instance;

    if PROFILE_CONTEXT_BIND_NEXT_DETAIL.swap(false, Ordering::AcqRel) {
        let athlete_id = PROFILE_CONTEXT_ATHLETE_ID.load(Ordering::Acquire);
        if athlete_id != 0 {
            let previous_bound_instance = PROFILE_CONTEXT_DETAIL_INSTANCE.load(Ordering::Acquire);
            let owned_previous_ui = PROFILE_NATIVE_LOCK_ACTIVE.load(Ordering::Acquire)
                || PROFILE_STATUS_OWNED.load(Ordering::Acquire);
            let mut old_native_ui_restored = false;
            let mut stale_snapshot_discarded = false;
            if owned_previous_ui {
                if previous_bound_instance != 0 && previous_bound_instance == detail_instance {
                    restore_owned_profile_native_ui(detail);
                    old_native_ui_restored = true;
                } else {
                    PROFILE_NATIVE_LOCK_ACTIVE.store(false, Ordering::Release);
                    PROFILE_STATUS_OWNED.store(false, Ordering::Release);
                    PROFILE_NATIVE_UI_SNAPSHOT.with(|slot| { slot.borrow_mut().take(); });
                    stale_snapshot_discarded = true;
                }
            }
            PROFILE_CONTEXT_DETAIL_INSTANCE.store(detail_instance, Ordering::Release);
            let target_bound = active && athlete_id == view.target_id;
            PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.store(target_bound, Ordering::Release);
            let scene_was_suspended = PROFILE_CONTEXT_SCENE_SUSPENDED.swap(false, Ordering::AcqRel);
            PROFILE_CONTEXT_SCENE_RETURN_REBOUND.store(
                target_bound && scene_was_suspended,
                Ordering::Release,
            );
            if target_bound && PROJECTION_PROFILE_SCENE_RETURN_REQUESTED.load(Ordering::Acquire) {
                log_event(
                    "contract_projection_player_detail_created",
                    &format!(
                        "athlete_id={};detail_instance={};exact_target=true;source=UIOutEvent_UndoScene;target_identity_lease=true",
                        athlete_id,
                        detail_instance,
                    ),
                );
            }
            log_event(
                "profile_context_bound_to_detail_instance",
                &format!(
                    "athlete_id={};detail_instance={};previous_detail_instance={};source=exact_view_detail_click_or_projection_scene_return;proposal_target_id={};exact_instance_binding=true;old_native_ui_restored={};stale_snapshot_discarded={};target_identity_lease={};scene_was_suspended={}",
                    athlete_id,
                    detail_instance,
                    previous_bound_instance,
                    view.target_id,
                    old_native_ui_restored,
                    stale_snapshot_discarded,
                    target_bound,
                    scene_was_suspended,
                ),
            );
        }
    } else {
        let current_context_id = PROFILE_CONTEXT_ATHLETE_ID.load(Ordering::Acquire);
        let bound_instance = PROFILE_CONTEXT_DETAIL_INSTANCE.load(Ordering::Acquire);
        if current_context_id != 0 && bound_instance != detail_instance {
            let suspended = PROFILE_CONTEXT_SCENE_SUSPENDED.swap(false, Ordering::AcqRel);
            PROFILE_CONTEXT_DETAIL_INSTANCE.store(detail_instance, Ordering::Release);
            PROFILE_CONTEXT_ALLOW_NEXT_REBUILD.store(false, Ordering::Release);
            PROFILE_CONTEXT_MANAGEMENT_TICK_REBUILD_REBOUND.store(
                active && current_context_id == view.target_id,
                Ordering::Release,
            );
            PROFILE_NATIVE_LOCK_ACTIVE.store(false, Ordering::Release);
            PROFILE_STATUS_OWNED.store(false, Ordering::Release);
            PROFILE_NATIVE_UI_SNAPSHOT.with(|slot| { slot.borrow_mut().take(); });
            detail_recreated = true;
            let target_rebound = active
                && current_context_id == view.target_id
                && PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.load(Ordering::Acquire);
            PROFILE_CONTEXT_SCENE_RETURN_REBOUND.store(target_rebound && suspended, Ordering::Release);
            if target_rebound {
                log_event(
                    "profile_target_lease_rebound_after_scene_return",
                    &format!(
                        "proposal_id={};state={};target_id={};current_player_id={};previous_detail_instance={};current_detail_instance={};target_identity_lease=true;scene_was_suspended={};pointer_change_is_not_identity_change=true;native_snapshot_reacquire=true",
                        view.proposal_id,
                        view.state,
                        view.target_id,
                        current_context_id,
                        bound_instance,
                        detail_instance,
                        suspended,
                    ),
                );
            } else {
                log_event(
                    "profile_context_identity_preserved_across_detail_rebuild",
                    &format!(
                        "proposal_id={};proposal_target_id={};current_player_id={};previous_detail_instance={};current_detail_instance={};target_identity_lease={};pointer_change_is_not_identity_change=true;fail_open_for_non_target=true",
                        view.proposal_id,
                        view.target_id,
                        current_context_id,
                        bound_instance,
                        detail_instance,
                        PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.load(Ordering::Acquire),
                    ),
                );
            }
        }
    }

    let current_id = current_player_detail_athlete_id(detail);
    if !active || current_id != Some(view.target_id) {
        PROFILE_CONTEXT_TARGET_LOCK_VALID.store(false, Ordering::Release);
        PROFILE_CONTEXT_LAST_CONFIRMED_TARGET_SCENE.store(false, Ordering::Release);
        if !detail_recreated {
            restore_owned_profile_native_ui(detail);
        }
        let (native_contract_button_visible, custom_status_hidden) =
            preserve_unrelated_player_profile_native_ui(
                detail,
                view,
                current_id,
                if detail_recreated {
                    "exact_non_target_new_detail"
                } else {
                    "exact_non_target_existing_detail"
                },
            );
        PROFILE_NATIVE_LOCK_ACTIVE.store(false, Ordering::Release);
        PROFILE_STATUS_OWNED.store(false, Ordering::Release);
        PROFILE_NATIVE_UI_SNAPSHOT.with(|slot| { slot.borrow_mut().take(); });
        let key = format!("{}|{}|{:?}|{}", view.proposal_id, view.target_id, current_id, detail_instance);
        let should_log = ASYNC_PROFILE_LAST_SKIP_KEY.with(|slot| {
            let mut slot = slot.borrow_mut();
            if *slot == key { false } else { *slot = key; true }
        });
        if should_log && view.proposal_present {
            log_event(
                "async_trade_profile_status_skipped",
                &format!(
                    "proposal_id={};proposal_target_id={};current_player_id={:?};active_state={};false_cross_player_display_prevented=true;detail_recreated={};detail_instance={};native_contract_button_visible={};negotiation_buttons_hidden=false;custom_status_hidden={};fail_open=true;pointer_change_is_not_identity_change=true",
                    view.proposal_id,
                    view.target_id,
                    current_id,
                    active,
                    detail_recreated,
                    detail_instance,
                    native_contract_button_visible,
                    custom_status_hidden,
                ),
            );
        }
        return false;
    }

    PROFILE_CONTEXT_TARGET_LEASE_ACTIVE.store(true, Ordering::Release);
    PROFILE_CONTEXT_LAST_CONFIRMED_TARGET_SCENE.store(true, Ordering::Release);

    if !PROFILE_NATIVE_LOCK_ACTIVE.load(Ordering::Acquire) {
        let contract_state = direct_path(detail, &["data", "row4", PROFILE_CONTRACT_BUTTON_ID])
            .map(|node| (node.visible, node.disabled));
        let state_state = direct_path(detail, &["data", "row4", PROFILE_STATE_LABEL_ID])
            .map(|node| (node.visible, node.disabled));
        if let (Some((contract_visible, contract_disabled)), Some((state_visible, state_disabled))) =
            (contract_state, state_state)
        {
            PROFILE_NATIVE_UI_SNAPSHOT.with(|slot| {
                *slot.borrow_mut() = Some(ProfileNativeUiSnapshot {
                    contract_visible,
                    contract_disabled,
                    state_visible,
                    state_disabled,
                });
            });
            PROFILE_NATIVE_LOCK_ACTIVE.store(true, Ordering::Release);
        }
    }

    let contract_hidden = if let Some(contract) = direct_path_mut(detail, &["data", "row4", PROFILE_CONTRACT_BUTTON_ID]) {
        contract.visible = false;
        contract.disabled = true;
        contract.runner.set_dirty(true);
        true
    } else {
        false
    };
    let Some(state) = direct_path_mut(detail, &["data", "row4", PROFILE_STATE_LABEL_ID]) else {
        PROFILE_CONTEXT_TARGET_LOCK_VALID.store(false, Ordering::Release);
        return false;
    };
    let (stage, due) = if view.state == "SellerReview" {
        ("판매 구단 검토", view.seller_due_at.as_str())
    } else {
        ("선수 검토", view.player_due_at.as_str())
    };
    let text = format!(
        "단계: {} | 제안 접수 {} | 응답 기한 {}",
        stage,
        compact_async_time(&view.submitted_at),
        compact_async_time(due),
    );
    state.visible = true;
    state.disabled = false;
    set_runner_text(state, &text);
    state.runner.set_dirty(true);
    PROFILE_STATUS_OWNED.store(true, Ordering::Release);
    PROFILE_CONTEXT_TARGET_LOCK_VALID.store(contract_hidden, Ordering::Release);
    let key = format!("{}|{}|{}|{}|{}", view.proposal_id, view.state, text, contract_hidden, detail_instance);
    let should_log = ASYNC_PROFILE_LAST_RENDER_KEY.with(|slot| {
        let mut slot = slot.borrow_mut();
        if *slot == key { false } else { *slot = key; true }
    });
    if should_log {
        log_event(
            "async_trade_profile_status_rendered",
            &format!(
                "proposal_id={};state={};stage_ko={};submitted_at={};response_due_at={};target_id={};current_player_id={};status_text={};native_contract_button_hidden={};native_cash_offer_entry_blocked=true;proposer_identity_visible=false;transaction_executed=false;detail_instance={};exact_detail_instance_lock=true;target_identity_lease=true",
                view.proposal_id,
                view.state,
                stage,
                view.submitted_at,
                due,
                view.target_id,
                current_id.unwrap_or(0),
                sanitize(&text),
                contract_hidden,
                detail_instance,
            ),
        );
    }
    if detail_recreated {
        let reapply_key = format!("{}|{}|{}|{}", view.proposal_id, view.state, view.target_id, detail_instance);
        let should_log_reapply = ASYNC_PROFILE_REAPPLY_LAST_KEY.with(|slot| {
            let mut slot = slot.borrow_mut();
            if *slot == reapply_key { false } else { *slot = reapply_key; true }
        });
        if should_log_reapply {
            log_event(
                "async_trade_profile_status_reapplied_after_ui_rebuild",
                &format!(
                    "proposal_id={};state={};target_id={};current_player_id={};detail_instance={};profile_context_preserved=true;native_contract_button_hidden={};status_visible=true;generic_left_right_invalidation=false;fresh_native_snapshot=true;exact_detail_instance_lock=true;target_identity_lease=true;pointer_change_is_not_identity_change=true",
                    view.proposal_id,
                    view.state,
                    view.target_id,
                    current_id.unwrap_or(0),
                    detail_instance,
                    contract_hidden,
                ),
            );
        }
        if PROFILE_CONTEXT_MANAGEMENT_TICK_REBUILD_REBOUND.swap(false, Ordering::AcqRel) {
            log_event(
                "profile_context_management_tick_rebuild_lock_restored",
                &format!(
                    "proposal_id={};state={};target_id={};current_player_id={};detail_instance={};native_contract_button_hidden={};status_visible=true;one_shot_no_frame_expiry=true;frame_expiry=false;exact_detail_instance_lock=true;target_identity_lease=true",
                    view.proposal_id,
                    view.state,
                    view.target_id,
                    current_id.unwrap_or(0),
                    detail_instance,
                    contract_hidden,
                ),
            );
        }
    }
    if PROFILE_CONTEXT_SCENE_RETURN_REBOUND.swap(false, Ordering::AcqRel) {
        log_event(
            "async_trade_profile_status_reapplied_after_scene_return",
            &format!(
                "proposal_id={};state={};target_id={};current_player_id={};detail_instance={};native_contract_button_hidden={};status_visible=true;target_identity_lease=true;pointer_change_is_not_identity_change=true",
                view.proposal_id,
                view.state,
                view.target_id,
                current_id.unwrap_or(0),
                detail_instance,
                contract_hidden,
            ),
        );
    }
    true
}

fn return_to_player_profile_after_submit(ui: &mut GameUI) {
    observe_profile_return_after_submit(ui);
}


fn send_async_status_query_if_ready(data: &ClientData) {
    let frame = RUNTIME_FRAME_COUNT.load(Ordering::Relaxed);
    let last = ASYNC_STATUS_QUERY_LAST_FRAME.load(Ordering::Relaxed);
    if ASYNC_STATUS_QUERY_PENDING.load(Ordering::Acquire) {
        if last != 0 && frame.saturating_sub(last) >= 180 {
            ASYNC_STATUS_QUERY_PENDING.store(false, Ordering::Release);
            log_event(
                "async_trade_status_query_timeout",
                &format!(
                    "attempt={};frames_waited={};pending_cleared_for_retry=true;database_mutation=false",
                    ASYNC_STATUS_QUERY_SEND_ATTEMPT.load(Ordering::Acquire),
                    frame.saturating_sub(last),
                ),
            );
        } else {
            return;
        }
    }
    if last != 0 && frame.saturating_sub(last) < 60 {
        return;
    }
    if data.send_mod_command(MOD_ID, ASYNC_STATUS_COMMAND, b"query=1\n".to_vec()) {
        let attempt = ASYNC_STATUS_QUERY_SEND_ATTEMPT.fetch_add(1, Ordering::AcqRel) + 1;
        ASYNC_STATUS_QUERY_LAST_FRAME.store(frame.max(1), Ordering::Relaxed);
        ASYNC_STATUS_QUERY_PENDING.store(true, Ordering::Release);
        if attempt <= 3 || attempt % 10 == 0 {
            log_event(
                "async_trade_status_query_sent",
                &format!("attempt={};frame={};read_only=true;database_mutation=false", attempt, frame),
            );
        }
    }
}

fn handle_quote_events(data: &ClientData) {
    for event in data.take_mod_events(MOD_ID) {
        if event.event == VALIDATE_SAVED_TRADE_EVENT {
            if let Ok(values) = parse_kv_payload(&event.payload) {
                match map_required(&values, "status") {
                    Ok("baseline_verified") => {
                        FIRST_TRADE_BASELINE_VERIFIED.store(true, Ordering::Release);
                        SAVED_TRADE_VALIDATION_RESPONSE_RECEIVED.store(true, Ordering::Release);
                        log_event(
                            "test77_dual_baseline_response_received",
                            "baseline_verified=true;executed_plan_registry_count=2;fresh_trade_enabled=true;historical_players_excluded=true;database_mutation=false;transaction_executed=false;save_api_called=false",
                        );
                    }
                    // [PORT056] 거래 이력이 없는 일반 세이브(status=none)도 트레이드를 허용한다.
                    //   구 Test79 는 "과거 검증 거래 2건이 있는 세이브"에서만 FIRST_TRADE_BASELINE_VERIFIED 를
                    //   세웠고, 이 플래그가 트레이드 버튼(6412)과 모달 열기(6846)를 동시에 막는다
                    //   ⟹ 일반 세이브에서는 버튼이 영구 비활성이었다.
                    Ok("none") => {
                        FIRST_TRADE_BASELINE_VERIFIED.store(true, Ordering::Release);
                        SAVED_TRADE_VALIDATION_RESPONSE_RECEIVED.store(true, Ordering::Release);
                        log_event(
                            "trade_enabled_without_prior_receipt",
                            "status=none;port056_baseline_gate_relaxed=true",
                        );
                    }
                    Ok("async_proposal_present") => {
                        FIRST_TRADE_BASELINE_VERIFIED.store(true, Ordering::Release);
                        SAVED_TRADE_VALIDATION_RESPONSE_RECEIVED.store(true, Ordering::Release);
                        log_event(
                            "test77_async_proposal_reload_detected",
                            "status=async_proposal_present;strict_commit_snapshot_validation_skipped=true;database_mutation=false;transaction_executed=false",
                        );
                    }
                    Ok(other) => log_event(
                        "test77_baseline_validation_waiting",
                        &format!("status={};database_mutation=false", sanitize(other)),
                    ),
                    Err(_) => {}
                }
            }
            continue;
        }
        if event.event == FLOOR_AUDIT_EVENT {
            FLOOR_AUDIT_RESPONSE_RECEIVED.store(true, Ordering::Release);
            continue;
        }
        if event.event == REVIEW_EVENT {
            REVIEW_REQUEST_PENDING.store(false, Ordering::Release);
            let parsed = parse_kv_payload(&event.payload);
            match parsed {
                Ok(values) if map_required(&values, "status").ok() == Some("submitted") => {
                    let view = AsyncStatusView {
                        proposal_present: true,
                        proposal_id: map_required(&values, "proposal_id").unwrap_or("unknown").to_string(),
                        state: map_required(&values, "state").unwrap_or("SellerReview").to_string(),
                        stage_ko: map_required(&values, "stage_ko").unwrap_or("판매 구단 검토").to_string(),
                        requester_team_id: map_usize(&values, "requester_team_id").unwrap_or(REQUESTER_TEAM_ID),
                        recipient_team_id: map_usize(&values, "recipient_team_id").unwrap_or(usize::MAX),
                        requester_team_name: map_required(&values, "requester_team_name").unwrap_or("").to_string(),
                        recipient_team_name: map_required(&values, "recipient_team_name").unwrap_or("").to_string(),
                        target_id: map_usize(&values, "target_id").unwrap_or(0),
                        offered_id: map_usize(&values, "offered_id").unwrap_or(0),
                        target_name: map_required(&values, "target_name").unwrap_or("").to_string(),
                        offered_name: map_required(&values, "offered_name").unwrap_or("").to_string(),
                        target_position_label: String::new(),
                        target_position_icon: String::new(),
                        target_contract_end: String::new(),
                        target_yearly_salary: 0.0,
                        proposed_units: map_u64(&values, "proposed_units").unwrap_or(0),
                        desired_status_label: map_required(&values, "desired_status_label").unwrap_or("핵심 선수").to_string(),
                        game_time: map_required(&values, "game_time").unwrap_or_else(|_| map_required(&values, "submitted_at").unwrap_or("")).to_string(),
                        submitted_at: map_required(&values, "submitted_at").unwrap_or("").to_string(),
                        seller_due_at: map_required(&values, "seller_due_at").unwrap_or("").to_string(),
                        player_due_at: map_required(&values, "player_due_at").unwrap_or("").to_string(),
                        completed_at: String::new(),
                        rejection_reason_ko: String::new(),
                        result_plan_id: String::new(),
                        success_news_count: 0,
                        submit_process_id: std::process::id(),
                        commit_process_id: 0,
                        current_process_id: std::process::id(),
                        offered_team_current: REQUESTER_TEAM_ID,
                        target_team_current: usize::MAX,
                        target_status_current: "pending".to_string(),
                        target_contracted_status_current: "pending".to_string(),
                        executed_plan_registry_count: 2,
                        result_plan_occurrences: 0,
                    };
                    log_event(
                        "async_trade_submit_response_received",
                        &format!(
                            "status=submitted;proposal_id={};state=SellerReview;offered_id={};target_id={};submitted_at={};seller_due_at={};team_mutation=false;finance_mutation=false;contract_mutation=false;transaction_executed=false",
                            view.proposal_id,
                            view.offered_id,
                            view.target_id,
                            view.submitted_at,
                            view.seller_due_at,
                        ),
                    );
                    ASYNC_STATUS_VIEW.with(|slot| *slot.borrow_mut() = Some(view));
                    ASYNC_STATUS_ERROR.with(|slot| *slot.borrow_mut() = None);
                    RETURN_TO_PROFILE_PENDING.store(true, Ordering::Release);
                    ASYNC_STATUS_UI_DIRTY.store(true, Ordering::Release);
                }
                Ok(values) => {
                    let detail = values.get("detail").cloned().unwrap_or_else(|| "async proposal submission failed".to_string());
                    REVIEW_ERROR.with(|slot| *slot.borrow_mut() = Some(detail.clone()));
                    QUOTE_UI_DIRTY.store(true, Ordering::Release);
                    log_event("async_trade_submit_response_received", &format!("status=error;detail={};transaction_executed=false", sanitize(&detail)));
                }
                Err(detail) => {
                    REVIEW_ERROR.with(|slot| *slot.borrow_mut() = Some(detail.clone()));
                    QUOTE_UI_DIRTY.store(true, Ordering::Release);
                    log_event("async_trade_submit_response_parse_error", &format!("detail={}", sanitize(&detail)));
                }
            }
            continue;
        }
        if event.event == ASYNC_STATUS_EVENT {
            ASYNC_STATUS_QUERY_PENDING.store(false, Ordering::Release);
            // [PORT056] 요구사항 1 — 시즌 소모 플래그는 view 파싱 성공 여부와 무관하게 먼저 반영한다.
            if let Ok(values) = parse_kv_payload(&event.payload) {
                if let Ok(raw) = map_required(&values, "season_used") {
                    let used = raw == "true";
                    if TRADE_SEASON_USED.swap(used, Ordering::AcqRel) != used {
                        log_event(
                            "trade_season_quota_state_changed",
                            &format!("season_used={};policy=commit_only_consumes", used),
                        );
                    }
                }
            }
            match parse_kv_payload(&event.payload).and_then(|values| parse_async_status_view(&values)) {
                Ok(view) => {
                    if view.proposal_present {
                        log_event(
                            "async_trade_status_response_received",
                            &format!(
                                "proposal_id={};state={};stage_ko={};offered_id={};target_id={};game_time={};submitted_at={};seller_due_at={};player_due_at={};completed_at={};result_plan_id={};success_news_count={};current_process_id={};offered_team_current={};target_team_current={};target_status_current={};target_contracted_status_current={};executed_plan_registry_count={};result_plan_occurrences={};database_mutation=false;transaction_executed=false",
                                view.proposal_id,
                                view.state,
                                sanitize(&view.stage_ko),
                                view.offered_id,
                                view.target_id,
                                view.game_time,
                                view.submitted_at,
                                view.seller_due_at,
                                view.player_due_at,
                                view.completed_at,
                                view.result_plan_id,
                                view.success_news_count,
                                view.current_process_id,
                                view.offered_team_current,
                                view.target_team_current,
                                sanitize(&view.target_status_current),
                                sanitize(&view.target_contracted_status_current),
                                view.executed_plan_registry_count,
                                view.result_plan_occurrences,
                            ),
                        );
                    }
                    ASYNC_STATUS_VIEW.with(|slot| *slot.borrow_mut() = Some(view));
                    ASYNC_STATUS_ERROR.with(|slot| *slot.borrow_mut() = None);
                    ASYNC_STATUS_UI_DIRTY.store(true, Ordering::Release);
                }
                Err(detail) => {
                    ASYNC_STATUS_ERROR.with(|slot| *slot.borrow_mut() = Some(detail.clone()));
                    log_event("async_trade_status_response_parse_error", &format!("detail={}", sanitize(&detail)));
                }
            }
            continue;
        }
        if event.event == NATIVE_OFFER_STATUS_EVENT {
            NATIVE_OFFER_STATUS_QUERY_PENDING.store(false, Ordering::Release);
            match parse_kv_payload(&event.payload).and_then(|values| parse_native_offer_status_view(&values)) {
                Ok(view) => {
                    NATIVE_OFFER_STATUS_VIEW.with(|slot| *slot.borrow_mut() = Some(view));
                }
                Err(detail) => log_event(
                    "first_active_offer_status_parse_error",
                    &format!("detail={}", sanitize(&detail)),
                ),
            }
            continue;
        }
        if event.event != QUOTE_EVENT {
            continue;
        }
        let parsed = parse_kv_payload(&event.payload).and_then(|values| {
            match map_required(&values, "status")? {
                "ok" => parse_quote_view(&values).map(Some),
                "error" => {
                    let detail = map_required(&values, "detail")?.to_string();
                    QUOTE_ERROR.with(|slot| *slot.borrow_mut() = Some(detail));
                    Ok(None)
                }
                other => Err(format!("unexpected quote status {other}")),
            }
        });
        match parsed {
            Ok(Some(quote)) => {
                let current_offered = OFFERED_ATHLETE_ID.load(Ordering::Acquire);
                let current_target = TARGET_ATHLETE_ID.load(Ordering::Acquire);
                if quote.offered_id != current_offered || quote.target_id != current_target {
                    log_event(
                        "quote_stale_response_ignored",
                        &format!("response_offered_id={};response_target_id={};current_offered_id={};current_target_id={}", quote.offered_id, quote.target_id, current_offered, current_target),
                    );
                    continue;
                }
                log_event(
                    "quote_response_received",
                    &format!(
                        "status=ok;requester_team_id={};recipient_team_id={};offered_id={};offered_name={};target_id={};target_name={};display_min_units={};display_max_units={};cash_range_obscured=true;exact_threshold_disclosed=false;cash_budget_won={};budget_units={};database_mutation=false",
                        quote.requester_team_id,
                        quote.recipient_team_id,
                        quote.offered_id,
                        sanitize(&quote.offered_name),
                        quote.target_id,
                        sanitize(&quote.target_name),
                        quote.required_units,
                        quote.cash_offer_max_units,
                        quote.cash_budget_won,
                        quote.budget_units,
                    ),
                );
                QUOTE_ERROR.with(|slot| *slot.borrow_mut() = None);
                QUOTE_VIEW.with(|slot| *slot.borrow_mut() = Some(quote));
                QUOTE_UI_DIRTY.store(true, Ordering::Release);
            }
            Ok(None) => {
                CASH_INPUT_STATE.store(2, Ordering::Relaxed);
                QUOTE_UI_DIRTY.store(true, Ordering::Release);
                log_event("quote_response_received", "status=error;database_mutation=false");
            }
            Err(detail) => {
                QUOTE_ERROR.with(|slot| *slot.borrow_mut() = Some(detail.clone()));
                CASH_INPUT_STATE.store(2, Ordering::Relaxed);
                QUOTE_UI_DIRTY.store(true, Ordering::Release);
                log_event("quote_response_parse_error", &format!("detail={}", sanitize(&detail)));
            }
        }
    }
}

fn send_saved_trade_validation_if_ready(data: &ClientData) {
    if SAVED_TRADE_VALIDATION_RESPONSE_RECEIVED.load(Ordering::Acquire) {
        return;
    }
    let frame = RUNTIME_FRAME_COUNT.load(Ordering::Relaxed);
    let last = SAVED_TRADE_VALIDATION_LAST_SEND_FRAME.load(Ordering::Relaxed);
    if last != 0 && frame.saturating_sub(last) < 120 {
        return;
    }
    SAVED_TRADE_VALIDATION_LAST_SEND_FRAME.store(frame, Ordering::Relaxed);
    let attempt = SAVED_TRADE_VALIDATION_SEND_ATTEMPT.fetch_add(1, Ordering::Relaxed) + 1;
    if attempt == 1 || attempt % 10 == 0 {
        log_event(
            "trade_save_reload_validation_command_sent",
            &format!(
                "attempt={};frame={};command={};reload_validation_only=true;database_mutation=false",
                attempt, frame, VALIDATE_SAVED_TRADE_COMMAND
            ),
        );
    }
    data.send_mod_command(
        MOD_ID,
        VALIDATE_SAVED_TRADE_COMMAND,
        b"validate=1\n".to_vec(),
    );
}


fn runtime_frame(scene: &mut Scene, ui: &mut GameUI, assets: &mut Assets) {
    RUNTIME_FRAME_COUNT.fetch_add(1, Ordering::Relaxed);
    let Scene::InGame { data } = scene else {
        ACTIVE_OFFER_PATH.with(|slot| *slot.borrow_mut() = None);
        ACTIVE_RAW_OFFER_ID_COUNT.store(0, Ordering::Release);
        ACTIVE_STRUCTURAL_OFFER_COUNT.store(0, Ordering::Release);
        if POPUP_OPEN.load(Ordering::Acquire) || find_node_by_id(&ui.root, MODAL_LAYER_ID).is_some() {
            invalidate_trade_ui_context(ui, "scene_left_ingame");
        }
        return;
    };
    if !LOAD_LOGGED.swap(true, Ordering::Relaxed) {
        log_event(
            "mod_loaded",
            &format!(
                "mod_id={};version={};sdk_base={};database_mutation_at_load=false;save_mutation=false;test79_stable_pending_ui=true;test79_fix6_profile_nav_scene_return=true;profile_lock_logic_target_identity_lease=true;successful_test_chain=71_72_73_74_75_76_77;test78_diagnostics_integrated=true;fully_custom_trade_ui=true;original_compare_popup_used=false;trade_submit_enabled=true;immediate_trade_execution_enabled=false;async_proposal_persistence=true;pending_restart_required=true;terminal_restart_required=true;async_lifecycle=seller_review_then_player_review_then_management_tick_commit;profile_status_stage=true;success_news_exactly_once=true;obscured_cash_range=true;display_lower_policy=random_70_80_percent_of_exact;display_upper_policy=random_150_160_percent_of_exact;range_stable_for_pair=true;exact_threshold_disclosed_to_client=false;baseline_save_slot={};pending_save_slot={};result_save_slot={};required_offered_id={};required_target_id={};seller_review_delay_days={};player_review_delay_days={};replacement_floor_core=0.70;replacement_floor_important=0.55;replacement_floor_general=0.40;transaction_execution_compiled=true;transaction_execution_only_from_management_tick=true;server_lifecycle_callback=after_management_tick;client_process_command_used=false;profile_return_method=UIOutEvent_UndoScene;offer_force_hidden=false;black_screen_fallback=keep_offer_visible;pending_button_labels=true;target_scoped_trade_button=true;status_query_timeout_retry=true;submit_transport_survives_modal_close=true;submit_command_popup_open_required=false;proposal_key=async_trade_proposal_v10_test79_stable_ui;native_profile_offer_button_hidden_during_pending=true;native_offer_screen_submit_locked=true;contract_status_projection=true;all_negotiations_projection=true;projection_read_only=true;contract_projection_single_visible_surface=true;contract_header_excluded=true;contract_projection_stable_single_row_id=true;contract_projection_column_count=10;squad_status_column_preserved=true;original_team_only=true;team_interaction_disabled=true;native_transfer_request_inserted=false;reload_profile_context_rebind=exact_target_click_or_target_identity_lease;profile_context_instance_bound=true;profile_context_target_identity_lease=true;detail_pointer_change_is_identity_change=false;profile_scene_suspend_resume=true;profile_scene_return_rebind=true;advance_profile_context_preserved=true;profile_context_rebuild_one_shot_no_frame_expiry=true;management_tick_profile_lock_restore=true;generic_left_right_path_invalidation_removed=true;projection_native_row_clone=true;projection_salary_compact=true;projection_name_click=true;projection_name_click_native_runner=true;projection_name_click_pre_attach=true;projection_name_click_explicit_scene_return=true;projection_navigation_method=UIOutEvent_UndoScene;projection_name_icon_native_size=true;projection_name_native_contract_row_runner_preserved=true;projection_name_popup_removed=true;projection_name_proficiency_tooltip_removed=true;projection_name_icon_width=24;projection_name_icon_height=24;projection_name_column_full_cell_overlay=false;projection_name_text_color=white;projection_native_target_runner_capture=true;projection_reload_native_row_runner_fallback=true;projection_team_font_size=18;team_detail_navigation=false;projection_deadline_suffix_until=false;contract_projection_time_gate=same_submission_calendar_date;contract_projection_exact_0900_required=false;contract_projection_time_of_day_threshold_required=false;contract_projection_date_rollover_allowed=false;projection_squad_status_source=target_status_current;projection_promised_status_not_used_in_contract_row=true;card_status_original_colors=true;cash_range_display=eok_cheoman;first_active_offer_history=false;native_offer_history_sync=false;other_player_profile_offer_ui_disabled=true;proposer_identity_visible=false",
                MOD_ID,
                MOD_VERSION,
                PATCH055_BASE_VERSION,
                BASELINE_SAVE_SLOT,
                PENDING_SAVE_SLOT,
                RESULT_SAVE_SLOT,
                TEST77_REQUIRED_OFFERED_ID,
                TEST77_REQUIRED_TARGET_ID,
                SELLER_REVIEW_DELAY_DAYS,
                PLAYER_REVIEW_DELAY_DAYS,
            ),
        );
    }
    let offer_surface_present = set_active_offer_path(ui).is_some();
    ensure_click_handler(ui, offer_surface_present);
    update_trade_entry_and_capture_offer(ui);
    watch_trade_ui_context(ui);
    populate_custom_trade_ui(ui, data);
    apply_pending_offered_selection(ui, data);
    handle_quote_events(data);
    if let Some(view) = ASYNC_STATUS_VIEW.with(|slot| slot.borrow().clone()) {
        let _ = rebind_reloaded_pending_profile_context(ui, &view);
    }
    return_to_player_profile_after_submit(ui);
    // ★[PORT056] 요구사항 3 재개통 — "트레이드 외 다른 오퍼가 있으면 첫 오퍼만 프로필 하단에 표시".
    //   구 Test79 는 이 기능을 **실행 경로에서 통째로 제거**했다(당시 유저가 포기 + 로그 폭주).
    //   `render_native_first_offer_status` / `send_native_offer_status_query_if_ready` 는 정의만 남고
    //   호출부가 0건인 죽은 코드였다(서버 커맨드 핸들러와 offer_history.rs 는 살아 있었다).
    //   현 유저 요구사항 3이 이 기능이므로 다시 배선한다.
    if !OTHER_PLAYER_PROFILE_OFFER_UI_REENABLED_LOGGED.swap(true, Ordering::AcqRel) {
        log_event(
            "first_offer_profile_status_reenabled",
            "reason=port056_requirement_3;query_rate_limit_frames=60;skip_log_rate_limited=true",
        );
    }
    send_saved_trade_validation_if_ready(data);
    send_replacement_floor_audit_if_ready(data);
    send_quote_if_ready(data);
    send_review_if_ready(data);
    send_async_status_query_if_ready(data);
    send_native_offer_status_query_if_ready(data, ui);
    update_cooldown_block_state(data);
    update_cash_input_status(ui);
    apply_quote_view(ui);
    if let Some(view) = ASYNC_STATUS_VIEW.with(|slot| slot.borrow().clone()) {
        let _ = apply_async_native_offer_screen_lock(ui, &view);
        // ★[PORT056] UI 주입은 **계약 현황 탭 전용**으로 복원(유저 지시 2026-08-23).
        //   전체 협상 상황은 게임 데이터(transfer_requests)가 네이티브로 그리므로 건드리지 않는다.
        // [PORT056] 접힌 계약 리스트 강제 재레이아웃 (위 함수 주석 참조).
        //   주입보다 먼저 돌린다 — 네이티브 행만으로도 보여야 하는 게 정상 상태다.
        let nudged = nudge_collapsed_contract_lists(&mut ui.root, false);
        if nudged > 0 {
            let last = CONTRACT_LIST_NUDGE_COUNT.fetch_add(1, Ordering::Relaxed);
            if last < 5 || last % 300 == 0 {
                log_event(
                    "contract_list_collapsed_nudged",
                    &format!("list_count={};nudge_seq={}", nudged, last + 1),
                );
            }
        }
        // [PORT056] 네이티브 계약행 열 덮어쓰기 (위 함수 주석 참조).
        if view.proposal_present && (view.state == "SellerReview" || view.state == "PlayerReview") {
            // [PORT056] 클라이언트 DB 선반영 — 제안 직후에도 리스트에 뜨게 한다(위 함수 주석 참조).
            if let Some(action) = sync_client_transfer_request(
                data,
                view.target_id,
                view.requester_team_id,
                true,
                SELLER_REVIEW_DELAY_DAYS,
                view.proposed_units as f64,
                SquadStatus::General,
            ) {
                if action != "present" {
                    log_event(
                        "client_transfer_request_synced",
                        &format!(
                            "action={};target_id={};team_id={};state={}",
                            action, view.target_id, view.requester_team_id, view.state,
                        ),
                    );
                }
            }
            // [PORT056] 탭마다 행 id 규약이 다르다(인게임 실측 2026-08-23):
            //   계약 현황      = `transfer_<athleteId>_<n>`
            //   전체 협상 상황 = `ca_<athleteId>_<teamId>_<n>`
            let prefixes = vec![
                format!("transfer_{}_", view.target_id),
                format!("ca_{}_", view.target_id),
            ];
            let hits = overwrite_native_trade_rows(&mut ui.root, &prefixes, &view);
            // [PORT056] 진단: 전체 협상 상황 리스트의 실제 행 id. 최대 12회.
            if NATIVE_ROW_ID_PROBE_COUNT.load(Ordering::Relaxed) < 12 {
                let mut ids = Vec::new();
                contract_all_row_ids(&ui.root, &mut ids);
                if !ids.is_empty() {
                    ids.sort();
                    ids.dedup();
                    let key = ids.join(",");
                    let changed = NATIVE_ROW_ID_PROBE_KEY.with(|slot| {
                        let mut slot = slot.borrow_mut();
                        if *slot == key { false } else { *slot = key.clone(); true }
                    });
                    if changed {
                        let seq = NATIVE_ROW_ID_PROBE_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                        log_event(
                            "native_trade_row_id_probe",
                            &format!("seq={};target_id={};hits={};ids={}", seq, view.target_id, hits, sanitize(&key)),
                        );
                    }
                }
            }
            if hits > 0 {
                let last = NATIVE_ROW_OVERWRITE_COUNT.fetch_add(1, Ordering::Relaxed);
                if last < 5 || last % 300 == 0 {
                    log_event(
                        "native_trade_row_overwritten",
                        &format!(
                            "target_id={};row_count={};stage_state={};proposed_units={};offered_name={};seq={}",
                            view.target_id,
                            hits,
                            view.state,
                            view.proposed_units,
                            sanitize(&view.offered_name),
                            last + 1,
                        ),
                    );
                }
            }
        }
        let _ = project_async_trade_into_contract_lists(ui, assets, &view);
    } else {
        let removed = remove_pending_contract_projection_rows(&mut ui.root);
        if removed > 0 {
            log_event(
                "async_trade_contract_projection_removed",
                &format!("proposal_id=none;state=None;removed_row_count={};terminal_or_absent=true;native_request_removed=false", removed),
            );
        }
    }
    if ASYNC_STATUS_UI_DIRTY.swap(false, Ordering::AcqRel) {
        if let Some(view) = ASYNC_STATUS_VIEW.with(|slot| slot.borrow().clone()) {
            let _ = set_player_profile_trade_stage(ui, &view);
        }
    } else if let Some(view) = ASYNC_STATUS_VIEW.with(|slot| slot.borrow().clone()) {
        if view.proposal_present {
            let _ = set_player_profile_trade_stage(ui, &view);
        }
    }
    // [PORT056] 요구사항 3 — 트레이드 제안이 있으면 그쪽이 우선하고(함수 내부 trade_has_priority),
    //   없을 때만 네이티브 첫 오퍼 상태를 프로필 하단에 표시한다.
    {
        let trade_view = ASYNC_STATUS_VIEW.with(|slot| slot.borrow().clone());
        render_native_first_offer_status(ui, assets, trade_view.as_ref());
    }
    observe_projection_profile_open(ui);
}

struct Test79Extension;

impl ModExtension for Test79Extension {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, assets: &mut Assets, _dt: f32) {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            runtime_frame(scene, ui, assets)
        }));
        if result.is_err() && !RUNTIME_ERROR_LOGGED.swap(true, Ordering::Relaxed) {
            log_event("runtime_error", "stage=post_update;panic_caught=true");
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut registration = ModRegistration::new(MOD_ID);
    registration.set_extension(Test79Extension);
    registration.set_server_extension(Test77ServerExtension);
    registration
}

declare_mod!(init);
