use game_core::{Database, News, NewsType};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process;

const NAMESPACE: &str = "player_trade_system";
const NAMESPACE_VERSION: usize = 1;
const LEDGER_KEY: &str = "unified_acquisition_ledger_v1";
const LEDGER_MAGIC: &[u8; 8] = b"PTSULG1\0";
const FEEDBACK_BIND_KEY: &str = "PtsTradeFeedbackId";
const FEEDBACK_AUTHOR: &str = "Player Trade System";
const FNV_OFFSET: u64 = 14_695_981_039_346_656_037;
const FNV_PRIME: u64 = 1_099_511_628_211;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AcquisitionKey {
    target_athlete_id: usize,
    destination_team_id: usize,
    source_team_id: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LedgerEntry {
    key: AcquisitionKey,
    rejection_actor: String,
    rejection_reason: String,
    retry_at: String,
    package_fingerprint: u64,
    changeable: bool,
    origin: String,
    created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LedgerDocument {
    schema_version: u32,
    request_nonce: String,
    apply_process_id: u32,
    apply_game_time: String,
    baseline_save_slot: String,
    manual_save_slot: String,
    native_retry_at: String,
    world_contract_hash_after_apply: u64,
    entries: Vec<LedgerEntry>,
}

struct Reader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        if count > self.remaining() {
            return Err(format!(
                "unified ledger decode overflow: need {count}, remaining {}",
                self.remaining()
            ));
        }
        let start = self.position;
        self.position += count;
        Ok(&self.bytes[start..self.position])
    }

    fn read_u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        let mut raw = [0u8; 4];
        raw.copy_from_slice(self.take(4)?);
        Ok(u32::from_le_bytes(raw))
    }

    fn read_u64(&mut self) -> Result<u64, String> {
        let mut raw = [0u8; 8];
        raw.copy_from_slice(self.take(8)?);
        Ok(u64::from_le_bytes(raw))
    }

    fn read_string(&mut self, label: &str, max_len: usize) -> Result<String, String> {
        let len = self.read_u32()? as usize;
        if len > max_len {
            return Err(format!(
                "unified ledger string {label} length {len} exceeds {max_len}"
            ));
        }
        let bytes = self.take(len)?;
        std::str::from_utf8(bytes)
            .map(str::to_string)
            .map_err(|error| format!("unified ledger string {label} is not UTF-8: {error}"))
    }
}

#[derive(Clone, Copy)]
pub struct RejectionPolicy {
    pub actor: &'static str,
    pub reason: &'static str,
    pub reason_ko: &'static str,
    pub policy: &'static str,
    pub retry_days: i64,
    pub changeable: bool,
}

#[derive(Clone)]
pub struct RejectionMeta {
    pub actor: String,
    pub actor_ko: String,
    pub reason: String,
    pub reason_ko: String,
    pub policy: String,
    pub retry_at: String,
    pub retry_ko: String,
    pub package_fingerprint: u64,
    pub feedback_id: u64,
    pub cooldown_blocked: bool,
    pub news_created: bool,
    pub duplicate_news_count: usize,
    pub ledger_entry_count: usize,
}

pub struct CooldownCheck {
    pub blocked: Option<RejectionMeta>,
    pub changed_package_bypass: bool,
    pub legacy_trade_consent_bypass: bool,
    pub previous_package_fingerprint: Option<u64>,
    pub ledger_entry_count: usize,
}

pub fn classify_review_rejection(
    cash_meets_required: bool,
    cash_within_budget: bool,
    seller_accepted: bool,
    seller_repeat_consistent: bool,
    player_accepted: bool,
    last_player_without_replacement: bool,
) -> RejectionPolicy {
    if !cash_within_budget {
        return RejectionPolicy {
            actor: "REQUESTER_TEAM_BUDGET",
            reason: "BudgetExceeded",
            reason_ko: "신청팀이 현재 제안 조건을 감당할 예산이 부족합니다.",
            policy: "changed_package_immediate_same_offer_1d",
            retry_days: 1,
            changeable: true,
        };
    }
    if last_player_without_replacement {
        return RejectionPolicy {
            actor: "SELLER_TEAM",
            reason: "LastPlayerAtPosition",
            reason_ko: "이 거래가 성립하면 상대 구단의 해당 주 포지션 선수가 남지 않습니다.",
            policy: "changed_package_immediate_same_offer_1d",
            retry_days: 1,
            changeable: true,
        };
    }
    if !cash_meets_required || !seller_accepted || !seller_repeat_consistent {
        return RejectionPolicy {
            actor: "SELLER_TEAM",
            reason: "TermsUnacceptable",
            reason_ko: "상대 구단이 현재 선수와 현금 조건을 받아들이지 않았습니다.",
            policy: "changed_package_immediate_same_offer_1d",
            retry_days: 1,
            changeable: true,
        };
    }
    if !player_accepted {
        return RejectionPolicy {
            actor: "TARGET_PLAYER",
            reason: "InheritedContractOrPromisedRoleUnacceptable",
            reason_ko: "기존 계약 승계가 확인되지 않았거나 현재보다 낮은 팀 내 위상을 약속해 선수가 동의하지 않았습니다.",
            policy: "changed_package_immediate_same_offer_1d",
            retry_days: 1,
            changeable: true,
        };
    }
    RejectionPolicy {
        actor: "SELLER_TEAM",
        reason: "FinalApprovalRejected",
        reason_ko: "최종 승인 단계에서 제안이 거절되었습니다.",
        policy: "fixed_3d",
        retry_days: 3,
        changeable: false,
    }
}

pub fn trade_package_fingerprint(
    requester_team_id: usize,
    recipient_team_id: usize,
    offered_id: usize,
    target_id: usize,
    proposed_cash_won: u64,
    desired_status_choice: u8,
) -> u64 {
    let canonical = format!(
        "PlayerTrade|requester={requester_team_id}|recipient={recipient_team_id}|offered={offered_id}|target={target_id}|cash={proposed_cash_won}|status={desired_status_choice}|cash_direction=requester_to_recipient|contract=inherit_both"
    );
    hash_bytes(FNV_OFFSET, canonical.as_bytes())
}

/// ★[PORT056] 클라이언트 선차단용 쿨다운 힌트 (유저 지시 2026-08-23).
///   구판은 쿨다운을 **제출 시점에만** 검사해서, 같은 조건이어도 버튼이 멀쩡히 활성이고
///   눌러야 "같은 조건의 제안은 …" 에러가 떴다. 버튼 단계에서 막으려면 클라가
///   지문·해제시각·changeable 세 값을 알아야 한다(판정식은 `ledger_gate` 와 동일).
///   ⚠서버가 여전히 정본이다. 이건 표시·선차단용이고 제출 시 검사는 그대로 남는다.
#[derive(Clone, Debug)]
pub struct CooldownHint {
    pub package_fingerprint: u64,
    pub retry_at: String,
    pub changeable: bool,
    /// 레거시 동의 예외(`StatusOrInheritedSalaryUnacceptable`) — 이때는 절대 막지 않는다.
    pub exempt: bool,
}

/// 해당 (대상 선수 · 우리 팀 · 상대 팀) 조합의 최신 원장 항목을 힌트로 뽑는다.
pub fn cooldown_hint(
    db: &Database,
    target_athlete_id: usize,
    destination_team_id: usize,
    source_team_id: usize,
) -> Result<Option<CooldownHint>, String> {
    let Some(bytes) = db.mod_save_data.get_bytes(NAMESPACE, LEDGER_KEY) else {
        return Ok(None);
    };
    let document = decode_ledger(&bytes)?;
    let key = AcquisitionKey {
        target_athlete_id,
        destination_team_id,
        source_team_id,
    };
    let Some(entry) = document.entries.iter().rev().find(|entry| entry.key == key) else {
        return Ok(None);
    };
    Ok(Some(CooldownHint {
        package_fingerprint: entry.package_fingerprint,
        retry_at: entry.retry_at.clone(),
        changeable: entry.changeable,
        exempt: entry.rejection_reason == "StatusOrInheritedSalaryUnacceptable",
    }))
}

/// `ledger_gate` 와 **같은 식**이다. 두 곳이 어긋나면 버튼은 막혔는데 제출은 되거나 그 반대가 된다.
pub fn cooldown_hint_blocks(hint: &CooldownHint, _candidate_fingerprint: u64, now: &str) -> bool {
    // ⚠`ledger_gate` 와 **반드시 같은 식**이어야 한다 — 어긋나면 버튼은 막혔는데 제출은 되거나 그 반대가 된다.
    //   `exempt`(레거시 동의 예외)도 더 이상 통과시키지 않는다: 유저 지시는
    //   "해제시각 전까지 그 선수 트레이드 제안 비활성" 이다.
    now < hint.retry_at.as_str()
}

pub fn check_cooldown(
    db: &Database,
    target_athlete_id: usize,
    destination_team_id: usize,
    source_team_id: usize,
    package_fingerprint: u64,
) -> Result<CooldownCheck, String> {
    let Some(bytes) = db.mod_save_data.get_bytes(NAMESPACE, LEDGER_KEY) else {
        return Ok(CooldownCheck {
            blocked: None,
            changed_package_bypass: false,
            legacy_trade_consent_bypass: false,
            previous_package_fingerprint: None,
            ledger_entry_count: 0,
        });
    };
    let document = decode_ledger(&bytes)?;
    let key = AcquisitionKey {
        target_athlete_id,
        destination_team_id,
        source_team_id,
    };
    let now = db.time.to_string();
    let Some(entry) = document.entries.iter().rev().find(|entry| entry.key == key) else {
        return Ok(CooldownCheck {
            blocked: None,
            changed_package_bypass: false,
            legacy_trade_consent_bypass: false,
            previous_package_fingerprint: None,
            ledger_entry_count: document.entries.len(),
        });
    };
    if entry.rejection_reason == "StatusOrInheritedSalaryUnacceptable" {
        return Ok(CooldownCheck {
            blocked: None,
            changed_package_bypass: false,
            legacy_trade_consent_bypass: true,
            previous_package_fingerprint: Some(entry.package_fingerprint),
            ledger_entry_count: document.entries.len(),
        });
    }
    let still_active = now.as_str() < entry.retry_at.as_str();
    let changed_package_bypass = still_active
        && entry.changeable
        && package_fingerprint != entry.package_fingerprint;
    if !ledger_gate(entry, package_fingerprint, &now) {
        return Ok(CooldownCheck {
            blocked: None,
            changed_package_bypass,
            legacy_trade_consent_bypass: false,
            previous_package_fingerprint: Some(entry.package_fingerprint),
            ledger_entry_count: document.entries.len(),
        });
    }
    let duplicate_news_count = count_feedback_news(db, destination_team_id, feedback_id(entry))?;
    if duplicate_news_count != 1 {
        return Err(format!(
            "active trade rejection must have exactly one feedback news item, found {duplicate_news_count}"
        ));
    }
    Ok(CooldownCheck {
        blocked: Some(meta_from_entry(
            entry,
            true,
            false,
            duplicate_news_count,
            document.entries.len(),
        )),
        changed_package_bypass: false,
        legacy_trade_consent_bypass: false,
        previous_package_fingerprint: Some(entry.package_fingerprint),
        ledger_entry_count: document.entries.len(),
    })
}

pub fn record_trade_rejection(
    db: &mut Database,
    offered_athlete_name: &str,
    target_athlete_id: usize,
    target_athlete_name: &str,
    destination_team_id: usize,
    destination_team_name: &str,
    source_team_id: usize,
    source_team_name: &str,
    package_fingerprint: u64,
    policy: RejectionPolicy,
) -> Result<RejectionMeta, String> {
    if db.teams.get(destination_team_id).is_none() {
        return Err("requester team missing before trade rejection persistence".to_string());
    }
    let mod_save_before = db.mod_save_data.clone();
    let news_len_before = db
        .teams
        .get(destination_team_id)
        .map(|team| team.news.len())
        .unwrap_or(0);
    let result = catch_unwind(AssertUnwindSafe(|| -> Result<RejectionMeta, String> {
        let now = db.time.to_string();
        let retry_at = advance_calendar_days(db, policy.retry_days)?;
        let key = AcquisitionKey {
            target_athlete_id,
            destination_team_id,
            source_team_id,
        };
        let mut document = load_or_new_ledger(db, &now)?;
        document.entries.retain(|entry| entry.key != key);
        let entry = LedgerEntry {
            key,
            rejection_actor: policy.actor.to_string(),
            rejection_reason: policy.reason.to_string(),
            retry_at,
            package_fingerprint,
            changeable: policy.changeable,
            origin: "PlayerTradeServerReviewRejection".to_string(),
            created_at: now.clone(),
        };
        document.entries.push(entry.clone());
        document.entries.sort_by_key(|entry| entry.key);
        let bytes = encode_ledger(&document)?;
        if decode_ledger(&bytes)? != document {
            return Err("unified ledger codec round trip mismatch".to_string());
        }
        let _ = db.mod_save_data.set_version(NAMESPACE, NAMESPACE_VERSION);
        let _ = db.mod_save_data.set_bytes(NAMESPACE, LEDGER_KEY, bytes.clone());
        if db.mod_save_data.save_version(NAMESPACE) != NAMESPACE_VERSION {
            return Err("unified ledger namespace version mismatch after write".to_string());
        }
        let readback = db
            .mod_save_data
            .get_bytes(NAMESPACE, LEDGER_KEY)
            .ok_or_else(|| "unified ledger disappeared after write".to_string())?;
        if readback != bytes {
            return Err("unified ledger readback mismatch".to_string());
        }

        let feedback_id = feedback_id(&entry);
        let existing_count = count_feedback_news(db, destination_team_id, feedback_id)?;
        let mut news_created = false;
        if existing_count == 0 {
            let actor_ko = actor_ko(policy.actor);
            let retry_ko = retry_ko(&entry);
            let content = format!(
                "{} 선수 트레이드 제안이 거절되었습니다.\n\n결정 주체: {}\n거절 사유: {}\n재협상: {}\n제안 선수: {}\n신청 구단: {}\n상대 구단: {}",
                target_athlete_name,
                actor_ko,
                policy.reason_ko,
                retry_ko,
                offered_athlete_name,
                destination_team_name,
                source_team_name,
            );
            let news = News {
                ty: NewsType::Simple {
                    content,
                    content_bind: Vec::new(),
                },
                title: "선수 트레이드 협상 결렬".to_string(),
                title_bind: vec![(
                    FEEDBACK_BIND_KEY.to_string(),
                    format!("{feedback_id:016X}"),
                )],
                author: FEEDBACK_AUTHOR.to_string(),
                date: db.time,
                is_read: false,
                is_sent: false,
                is_favorite: false,
            };
            db.teams
                .get_mut(destination_team_id)
                .ok_or_else(|| "requester team missing during news insertion".to_string())?
                .news
                .push(news);
            news_created = true;
        }
        let duplicate_news_count = count_feedback_news(db, destination_team_id, feedback_id)?;
        if duplicate_news_count != 1 {
            return Err(format!(
                "trade rejection feedback news count must be one, found {duplicate_news_count}"
            ));
        }
        let mut meta = meta_from_entry(
            &entry,
            false,
            news_created,
            duplicate_news_count,
            document.entries.len(),
        );
        meta.policy = policy.policy.to_string();
        Ok(meta)
    }));

    match result {
        Ok(Ok(meta)) => Ok(meta),
        Ok(Err(detail)) => {
            db.mod_save_data = mod_save_before;
            if let Some(team) = db.teams.get_mut(destination_team_id) {
                team.news.truncate(news_len_before);
            }
            Err(format!(
                "trade rejection persistence failed and rolled back: {detail}"
            ))
        }
        Err(_) => {
            db.mod_save_data = mod_save_before;
            if let Some(team) = db.teams.get_mut(destination_team_id) {
                team.news.truncate(news_len_before);
            }
            Err("panic caught during trade rejection persistence; ModSave and news rolled back"
                .to_string())
        }
    }
}

fn load_or_new_ledger(db: &Database, now: &str) -> Result<LedgerDocument, String> {
    if let Some(bytes) = db.mod_save_data.get_bytes(NAMESPACE, LEDGER_KEY) {
        return decode_ledger(&bytes);
    }
    Ok(LedgerDocument {
        schema_version: NAMESPACE_VERSION as u32,
        request_nonce: "PLAYER_TRADE_SYSTEM_RUNTIME".to_string(),
        apply_process_id: process::id(),
        apply_game_time: now.to_string(),
        baseline_save_slot: "<runtime>".to_string(),
        manual_save_slot: "<runtime>".to_string(),
        native_retry_at: "<per-entry>".to_string(),
        world_contract_hash_after_apply: 0,
        entries: Vec::new(),
    })
}

/// ★[PORT056] 쿨다운 = **해제시각까지 그 대상에게 전면 차단** (유저 지시 2026-08-23).
///   ~~구: `changeable` 이면 지문이 다른 패키지는 통과~~ → 폐기.
///   폐기 이유(실측): 평가액이 게임 이적시장·날짜에 따라 흔들려 `incoming_ratio` 가 1.0 을 넘는 순간
///   요구 현금이 0 이 된다. 그러면 12억을 거절당한 뒤 **0원 패키지**를 넣어도 지문이 달라
///   쿨다운을 그대로 통과했다(= 판매자에게 더 나쁜 조건이 재제안으로 인정됨).
///   ⚠`candidate_fingerprint` 는 더 이상 판정에 쓰이지 않지만 로그·원장 호환을 위해 인자로 남긴다.
fn ledger_gate(entry: &LedgerEntry, _candidate_fingerprint: u64, now: &str) -> bool {
    now < entry.retry_at.as_str()
}

fn advance_calendar_days(db: &Database, days: i64) -> Result<String, String> {
    if days < 0 {
        return Err("retry days must be nonnegative".to_string());
    }
    let mut retry_at = db.time;
    for _ in 0..days {
        let next_date = retry_at
            .date()
            .succ_opt()
            .ok_or_else(|| "retry date overflow".to_string())?;
        retry_at = next_date.and_time(retry_at.time());
    }
    Ok(retry_at.to_string())
}

fn meta_from_entry(
    entry: &LedgerEntry,
    cooldown_blocked: bool,
    news_created: bool,
    duplicate_news_count: usize,
    ledger_entry_count: usize,
) -> RejectionMeta {
    RejectionMeta {
        actor: entry.rejection_actor.clone(),
        actor_ko: actor_ko(&entry.rejection_actor).to_string(),
        reason: entry.rejection_reason.clone(),
        reason_ko: reason_ko(&entry.rejection_reason).to_string(),
        policy: if entry.changeable {
            "changed_package_immediate_same_offer_1d".to_string()
        } else if entry.rejection_reason == "FinalApprovalRejected" {
            "fixed_3d".to_string()
        } else {
            "fixed_cooldown".to_string()
        },
        retry_at: entry.retry_at.clone(),
        retry_ko: retry_ko(entry),
        package_fingerprint: entry.package_fingerprint,
        feedback_id: feedback_id(entry),
        cooldown_blocked,
        news_created,
        duplicate_news_count,
        ledger_entry_count,
    }
}

fn actor_ko(actor: &str) -> &'static str {
    match actor {
        "SELLER_TEAM" => "상대 구단",
        "TARGET_PLAYER" => "대상 선수",
        "REQUESTER_TEAM_BUDGET" => "신청팀 예산",
        _ => "확인 불가",
    }
}

fn reason_ko(reason: &str) -> &'static str {
    match reason {
        "BudgetExceeded" => "신청팀이 현재 제안 조건을 감당할 예산이 부족합니다.",
        "LastPlayerAtPosition" => {
            "이 거래가 성립하면 상대 구단의 해당 주 포지션 선수가 남지 않습니다."
        }
        "TermsUnacceptable" => "상대 구단이 현재 선수와 현금 조건을 받아들이지 않았습니다.",
        "StatusOrInheritedSalaryUnacceptable" => {
            "선수가 제시된 위상 또는 승계 계약 조건으로 이적하는 데 동의하지 않았습니다."
        }
        "FinalApprovalRejected" => "최종 승인 단계에서 제안이 거절되었습니다.",
        _ => "제안이 승인 조건을 충족하지 못했습니다.",
    }
}

fn retry_ko(entry: &LedgerEntry) -> String {
    // ★[PORT056] 문구도 규칙에 맞춘다 — ~~"조건을 바꾸면 즉시 다시 제안할 수 있습니다"~~ 는
    //   전면 차단으로 바꾼 뒤로는 거짓말이 된다(2026-08-23).
    format!("이 선수에게는 {}부터 다시 제안할 수 있습니다.", entry.retry_at)
}

fn feedback_id(entry: &LedgerEntry) -> u64 {
    let canonical = format!(
        "PlayerTradeFeedback|target={}|destination={}|source={}|reason={}|retry={}|fingerprint={:016X}|created={}",
        entry.key.target_athlete_id,
        entry.key.destination_team_id,
        entry.key.source_team_id,
        entry.rejection_reason,
        entry.retry_at,
        entry.package_fingerprint,
        entry.created_at,
    );
    hash_bytes(FNV_OFFSET, canonical.as_bytes())
}

fn count_feedback_news(
    db: &Database,
    destination_team_id: usize,
    feedback_id: u64,
) -> Result<usize, String> {
    let team = db
        .teams
        .get(destination_team_id)
        .ok_or_else(|| "requester team missing while counting feedback news".to_string())?;
    let expected = format!("{feedback_id:016X}");
    Ok(team
        .news
        .iter()
        .filter(|news| {
            news.title_bind
                .iter()
                .any(|(key, value)| key == FEEDBACK_BIND_KEY && value == &expected)
        })
        .count())
}

fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn put_u8(bytes: &mut Vec<u8>, value: u8) {
    bytes.push(value);
}

fn put_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn put_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn put_string(
    bytes: &mut Vec<u8>,
    label: &str,
    value: &str,
    max_len: usize,
) -> Result<(), String> {
    let raw = value.as_bytes();
    if raw.len() > max_len {
        return Err(format!(
            "unified ledger string {label} length {} exceeds {max_len}",
            raw.len()
        ));
    }
    let len = u32::try_from(raw.len())
        .map_err(|_| format!("unified ledger string {label} length does not fit u32"))?;
    put_u32(bytes, len);
    bytes.extend_from_slice(raw);
    Ok(())
}

fn usize_from_u64(value: u64, label: &str) -> Result<usize, String> {
    usize::try_from(value).map_err(|_| format!("unified ledger {label} does not fit usize"))
}

fn encode_ledger(document: &LedgerDocument) -> Result<Vec<u8>, String> {
    if document.entries.len() > 64 {
        return Err("unified ledger contains more than 64 entries".to_string());
    }
    let mut bytes = Vec::with_capacity(1024);
    bytes.extend_from_slice(LEDGER_MAGIC);
    put_u32(&mut bytes, document.schema_version);
    put_string(&mut bytes, "request_nonce", &document.request_nonce, 160)?;
    put_u32(&mut bytes, document.apply_process_id);
    put_string(&mut bytes, "apply_game_time", &document.apply_game_time, 32)?;
    put_string(&mut bytes, "baseline_save_slot", &document.baseline_save_slot, 96)?;
    put_string(&mut bytes, "manual_save_slot", &document.manual_save_slot, 96)?;
    put_string(&mut bytes, "native_retry_at", &document.native_retry_at, 32)?;
    put_u64(&mut bytes, document.world_contract_hash_after_apply);
    let count = u32::try_from(document.entries.len())
        .map_err(|_| "unified ledger entry count does not fit u32".to_string())?;
    put_u32(&mut bytes, count);
    for entry in &document.entries {
        put_u64(
            &mut bytes,
            u64::try_from(entry.key.target_athlete_id)
                .map_err(|_| "target athlete id does not fit u64".to_string())?,
        );
        put_u64(
            &mut bytes,
            u64::try_from(entry.key.destination_team_id)
                .map_err(|_| "destination team id does not fit u64".to_string())?,
        );
        put_u64(
            &mut bytes,
            u64::try_from(entry.key.source_team_id)
                .map_err(|_| "source team id does not fit u64".to_string())?,
        );
        put_string(&mut bytes, "rejection_actor", &entry.rejection_actor, 64)?;
        put_string(&mut bytes, "rejection_reason", &entry.rejection_reason, 96)?;
        put_string(&mut bytes, "retry_at", &entry.retry_at, 32)?;
        put_u64(&mut bytes, entry.package_fingerprint);
        put_u8(&mut bytes, u8::from(entry.changeable));
        put_string(&mut bytes, "origin", &entry.origin, 64)?;
        put_string(&mut bytes, "created_at", &entry.created_at, 32)?;
    }
    if bytes.len() > 65_536 {
        return Err(format!(
            "unified ledger payload is unexpectedly large: {} bytes",
            bytes.len()
        ));
    }
    Ok(bytes)
}

fn decode_ledger(bytes: &[u8]) -> Result<LedgerDocument, String> {
    let mut reader = Reader::new(bytes);
    if reader.take(LEDGER_MAGIC.len())? != &LEDGER_MAGIC[..] {
        return Err("unified ledger magic mismatch".to_string());
    }
    let schema_version = reader.read_u32()?;
    if schema_version != 1 {
        return Err(format!("unsupported unified ledger schema {schema_version}"));
    }
    let request_nonce = reader.read_string("request_nonce", 160)?;
    let apply_process_id = reader.read_u32()?;
    let apply_game_time = reader.read_string("apply_game_time", 32)?;
    let baseline_save_slot = reader.read_string("baseline_save_slot", 96)?;
    let manual_save_slot = reader.read_string("manual_save_slot", 96)?;
    let native_retry_at = reader.read_string("native_retry_at", 32)?;
    let world_contract_hash_after_apply = reader.read_u64()?;
    let count = reader.read_u32()? as usize;
    if count > 64 {
        return Err(format!("unified ledger entry count {count} exceeds limit 64"));
    }
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let key = AcquisitionKey {
            target_athlete_id: usize_from_u64(reader.read_u64()?, "target athlete id")?,
            destination_team_id: usize_from_u64(reader.read_u64()?, "destination team id")?,
            source_team_id: usize_from_u64(reader.read_u64()?, "source team id")?,
        };
        let rejection_actor = reader.read_string("rejection_actor", 64)?;
        let rejection_reason = reader.read_string("rejection_reason", 96)?;
        let retry_at = reader.read_string("retry_at", 32)?;
        let package_fingerprint = reader.read_u64()?;
        let changeable_raw = reader.read_u8()?;
        if changeable_raw > 1 {
            return Err("unified ledger changeable flag is invalid".to_string());
        }
        let origin = reader.read_string("origin", 64)?;
        let created_at = reader.read_string("created_at", 32)?;
        entries.push(LedgerEntry {
            key,
            rejection_actor,
            rejection_reason,
            retry_at,
            package_fingerprint,
            changeable: changeable_raw == 1,
            origin,
            created_at,
        });
    }
    if reader.remaining() != 0 {
        return Err(format!(
            "unified ledger has {} trailing bytes",
            reader.remaining()
        ));
    }
    Ok(LedgerDocument {
        schema_version,
        request_nonce,
        apply_process_id,
        apply_game_time,
        baseline_save_slot,
        manual_save_slot,
        native_retry_at,
        world_contract_hash_after_apply,
        entries,
    })
}
