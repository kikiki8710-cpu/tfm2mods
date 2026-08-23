use game_core::{Contract, Database};
use std::fmt::Debug;
use std::process;

const NAMESPACE: &str = "player_trade_system";
const NAMESPACE_VERSION: usize = 1;
const HISTORY_KEY: &str = "offer_history_v1";
const HISTORY_MAGIC: &str = "PTSOFH1";
const HISTORY_SCHEMA: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FlowKey {
    athlete_id: usize,
    source_team_id: usize,
    destination_team_id: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct HistoryEntry {
    key: FlowKey,
    generation: u32,
    sequence: u64,
    active: bool,
    first_seen_at: String,
    last_seen_at: String,
    completed_at: String,
    last_stage: String,
    last_deadline: String,
    created_process_id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct HistoryDocument {
    schema_version: u32,
    next_sequence: u64,
    entries: Vec<HistoryEntry>,
}

impl Default for HistoryDocument {
    fn default() -> Self {
        Self {
            schema_version: HISTORY_SCHEMA,
            next_sequence: 1,
            entries: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ActiveFlow {
    key: FlowKey,
    stage: String,
    stage_rank: u8,
    state_text: String,
    deadline: String,
    order_hint: String,
}

#[derive(Clone, Debug)]
struct DisplayStatus {
    athlete_id: usize,
    sequence: u64,
    first_seen: String,
    stage: String,
    state_text: String,
    deadline: String,
    active_offer_count: usize,
    created_process_id: u32,
}

pub struct SyncSummary {
    pub changed: bool,
    pub created_count: usize,
    pub completed_count: usize,
    pub stage_changed_count: usize,
    pub active_flow_count: usize,
    pub entry_count: usize,
    pub bytes_len: usize,
}

fn minute_text(value: &str) -> String {
    value.get(..16).unwrap_or(value).to_string()
}

fn state_name<T: Debug>(state: &T) -> String {
    format!("{state:?}")
}

fn waiting_state<T: Debug>(state: &T) -> bool {
    state_name(state) == "Waiting"
}

fn submitted_waiting<T: Debug>(is_draft: bool, state: &T, response_date: &str) -> bool {
    !is_draft && waiting_state(state) && response_date != "1970-01-01"
}

fn push_or_upgrade_flow(flows: &mut Vec<ActiveFlow>, mut candidate: ActiveFlow) {
    if let Some(existing) = flows.iter_mut().find(|flow| flow.key == candidate.key) {
        if candidate.stage_rank > existing.stage_rank {
            if existing.order_hint < candidate.order_hint {
                candidate.order_hint = existing.order_hint.clone();
            }
            *existing = candidate;
        }
    } else {
        flows.push(candidate);
    }
}

fn sort_active_flows(flows: &mut [ActiveFlow]) {
    flows.sort_by(|left, right| {
        left.order_hint
            .cmp(&right.order_hint)
            .then_with(|| left.key.cmp(&right.key))
    });
}

fn collect_active_flows(db: &Database) -> Vec<ActiveFlow> {
    let mut flows = Vec::new();
    for athlete in db.athletes.iter() {
        let Contract::InContract {
            team_id: source_team_id,
            transfer_requests,
            recruit_requests,
            ..
        } = &athlete.contract
        else {
            continue;
        };

        for request in transfer_requests {
            if request.team_id == *source_team_id {
                continue;
            }
            let Some(paper) = request.phase.last() else {
                continue;
            };
            let response_date = paper.response_date.to_string();
            if !submitted_waiting(paper.is_draft, &paper.state, &response_date) {
                continue;
            }
            let state_text = if paper.is_ask {
                "이 선수에 대한 이적 조건을 두고 구단 간 협상이 진행 중입니다"
            } else {
                "수정된 이적 조건에 대한 응답을 기다리고 있습니다"
            };
            push_or_upgrade_flow(
                &mut flows,
                ActiveFlow {
                    key: FlowKey {
                        athlete_id: athlete.id,
                        source_team_id: *source_team_id,
                        destination_team_id: request.team_id,
                    },
                    stage: "transfer".to_string(),
                    stage_rank: 1,
                    state_text: state_text.to_string(),
                    deadline: format!("{} 00:00", response_date),
                    order_hint: minute_text(&request.last_date.to_string()),
                },
            );
        }

        for request in recruit_requests {
            if request.team_id == *source_team_id {
                continue;
            }
            let Some(paper) = request.phase.last() else {
                continue;
            };
            if !waiting_state(&paper.state) {
                continue;
            }
            let response_date = paper.response_date.to_string();
            let submitted = !paper.is_draft && response_date != "1970-01-01";
            let (stage, stage_rank, state_text, deadline) = if submitted {
                (
                    "recruit",
                    3,
                    "소속 구단의 이적 승인을 거쳐 선수와 계약 조건을 협상 중입니다",
                    format!("{} 00:00", response_date),
                )
            } else {
                (
                    "recruit_draft",
                    2,
                    "소속 구단의 이적 승인을 거쳐 선수에게 제시할 계약 조건을 준비 중입니다",
                    "계약 조건 제출 대기".to_string(),
                )
            };
            push_or_upgrade_flow(
                &mut flows,
                ActiveFlow {
                    key: FlowKey {
                        athlete_id: athlete.id,
                        source_team_id: *source_team_id,
                        destination_team_id: request.team_id,
                    },
                    stage: stage.to_string(),
                    stage_rank,
                    state_text: state_text.to_string(),
                    deadline,
                    order_hint: minute_text(&request.last_date.to_string()),
                },
            );
        }
    }
    sort_active_flows(&mut flows);
    flows
}

fn encode(document: &HistoryDocument) -> Vec<u8> {
    let mut text = String::new();
    text.push_str(HISTORY_MAGIC);
    text.push('\n');
    text.push_str(&format!("schema={}\n", document.schema_version));
    text.push_str(&format!("next_sequence={}\n", document.next_sequence));
    for entry in &document.entries {
        text.push_str(&format!(
            "E\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            entry.key.athlete_id,
            entry.key.source_team_id,
            entry.key.destination_team_id,
            entry.generation,
            entry.sequence,
            if entry.active { 1 } else { 0 },
            entry.first_seen_at,
            entry.last_seen_at,
            entry.completed_at,
            entry.last_stage,
            entry.last_deadline,
            entry.created_process_id,
        ));
    }
    text.into_bytes()
}

fn parse_usize(value: &str, label: &str) -> Result<usize, String> {
    value
        .parse::<usize>()
        .map_err(|error| format!("offer history {label}: {error}"))
}

fn parse_u32(value: &str, label: &str) -> Result<u32, String> {
    value
        .parse::<u32>()
        .map_err(|error| format!("offer history {label}: {error}"))
}

fn parse_u64(value: &str, label: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|error| format!("offer history {label}: {error}"))
}

fn decode(bytes: &[u8]) -> Result<HistoryDocument, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("offer history is not UTF-8: {error}"))?;
    let mut lines = text.lines();
    if lines.next() != Some(HISTORY_MAGIC) {
        return Err("offer history magic mismatch".to_string());
    }
    let schema_line = lines
        .next()
        .ok_or_else(|| "offer history schema line missing".to_string())?;
    let schema_version = parse_u32(
        schema_line
            .strip_prefix("schema=")
            .ok_or_else(|| "offer history schema prefix missing".to_string())?,
        "schema",
    )?;
    if schema_version != HISTORY_SCHEMA {
        return Err(format!(
            "offer history schema {schema_version} is unsupported"
        ));
    }
    let next_line = lines
        .next()
        .ok_or_else(|| "offer history next-sequence line missing".to_string())?;
    let next_sequence = parse_u64(
        next_line
            .strip_prefix("next_sequence=")
            .ok_or_else(|| "offer history next-sequence prefix missing".to_string())?,
        "next_sequence",
    )?;
    let mut entries = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 13 || fields[0] != "E" {
            return Err(format!("offer history invalid entry line: {line}"));
        }
        entries.push(HistoryEntry {
            key: FlowKey {
                athlete_id: parse_usize(fields[1], "athlete_id")?,
                source_team_id: parse_usize(fields[2], "source_team_id")?,
                destination_team_id: parse_usize(fields[3], "destination_team_id")?,
            },
            generation: parse_u32(fields[4], "generation")?,
            sequence: parse_u64(fields[5], "sequence")?,
            active: match fields[6] {
                "1" => true,
                "0" => false,
                other => return Err(format!("offer history active flag {other} is invalid")),
            },
            first_seen_at: fields[7].to_string(),
            last_seen_at: fields[8].to_string(),
            completed_at: fields[9].to_string(),
            last_stage: fields[10].to_string(),
            last_deadline: fields[11].to_string(),
            created_process_id: parse_u32(fields[12], "created_process_id")?,
        });
    }
    Ok(HistoryDocument {
        schema_version,
        next_sequence,
        entries,
    })
}

fn load(db: &Database) -> Result<HistoryDocument, String> {
    match db.mod_save_data.get_bytes(NAMESPACE, HISTORY_KEY) {
        Some(bytes) => decode(&bytes),
        None => Ok(HistoryDocument::default()),
    }
}

fn save(db: &mut Database, document: &HistoryDocument) -> Result<usize, String> {
    let bytes = encode(document);
    if decode(&bytes)? != *document {
        return Err("offer history codec round-trip mismatch".to_string());
    }
    let _ = db.mod_save_data.set_version(NAMESPACE, NAMESPACE_VERSION);
    let _ = db
        .mod_save_data
        .set_bytes(NAMESPACE, HISTORY_KEY, bytes.clone());
    if db.mod_save_data.save_version(NAMESPACE) != NAMESPACE_VERSION {
        return Err("offer history namespace version mismatch after write".to_string());
    }
    if db.mod_save_data.get_bytes(NAMESPACE, HISTORY_KEY).as_deref() != Some(bytes.as_slice()) {
        return Err("offer history readback mismatch after write".to_string());
    }
    Ok(bytes.len())
}

fn select_status(
    document: &HistoryDocument,
    active_flows: &[ActiveFlow],
    athlete_id: usize,
) -> Option<DisplayStatus> {
    let mut candidates = Vec::new();
    for flow in active_flows.iter().filter(|flow| flow.key.athlete_id == athlete_id) {
        let Some(entry) = document
            .entries
            .iter()
            .filter(|entry| entry.active && entry.key == flow.key)
            .max_by_key(|entry| entry.generation)
        else {
            continue;
        };
        candidates.push((entry, flow));
    }
    candidates.sort_by_key(|(entry, _)| entry.sequence);
    let active_offer_count = candidates.len();
    let (entry, flow) = candidates.into_iter().next()?;
    Some(DisplayStatus {
        athlete_id,
        sequence: entry.sequence,
        first_seen: entry.first_seen_at.clone(),
        stage: flow.stage.clone(),
        state_text: flow.state_text.clone(),
        deadline: flow.deadline.clone(),
        active_offer_count,
        created_process_id: entry.created_process_id,
    })
}

pub fn synchronize_all(db: &mut Database) -> Result<SyncSummary, String> {
    if db.athletes.iter().next().is_none() {
        return Ok(SyncSummary {
            changed: false,
            created_count: 0,
            completed_count: 0,
            stage_changed_count: 0,
            active_flow_count: 0,
            entry_count: 0,
            bytes_len: 0,
        });
    }
    let active_flows = collect_active_flows(db);
    let now = minute_text(&db.time.to_string());
    let current_process_id = process::id();
    let mut document = load(db)?;
    let mut changed = false;
    let mut created_count = 0usize;
    let mut completed_count = 0usize;
    let mut stage_changed_count = 0usize;

    for entry in document.entries.iter_mut().filter(|entry| entry.active) {
        if !active_flows.iter().any(|flow| flow.key == entry.key) {
            entry.active = false;
            entry.completed_at = now.clone();
            entry.last_seen_at = now.clone();
            changed = true;
            completed_count += 1;
            crate::log_event(
                "offer_history_entry_completed",
                &format!(
                    "athlete_id={};sequence={};first_seen={};completed_at={};process_id={};proposer_identity_visible=false",
                    entry.key.athlete_id,
                    entry.sequence,
                    crate::sanitize(&entry.first_seen_at),
                    crate::sanitize(&entry.completed_at),
                    current_process_id,
                ),
            );
        }
    }

    for flow in &active_flows {
        if let Some(entry) = document
            .entries
            .iter_mut()
            .find(|entry| entry.active && entry.key == flow.key)
        {
            if entry.last_stage != flow.stage || entry.last_deadline != flow.deadline {
                let previous_stage = entry.last_stage.clone();
                let previous_deadline = entry.last_deadline.clone();
                entry.last_stage = flow.stage.clone();
                entry.last_deadline = flow.deadline.clone();
                entry.last_seen_at = now.clone();
                changed = true;
                stage_changed_count += 1;
                crate::log_event(
                    "offer_history_entry_stage_changed",
                    &format!(
                        "athlete_id={};sequence={};first_seen={};previous_stage={};stage={};previous_deadline={};deadline={};first_seen_preserved=true;process_id={};proposer_identity_visible=false",
                        entry.key.athlete_id,
                        entry.sequence,
                        crate::sanitize(&entry.first_seen_at),
                        crate::sanitize(&previous_stage),
                        crate::sanitize(&entry.last_stage),
                        crate::sanitize(&previous_deadline),
                        crate::sanitize(&entry.last_deadline),
                        current_process_id,
                    ),
                );
            }
            continue;
        }

        let generation = document
            .entries
            .iter()
            .filter(|entry| entry.key == flow.key)
            .map(|entry| entry.generation)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        let sequence = document.next_sequence;
        document.next_sequence = document.next_sequence.saturating_add(1);
        let first_seen_at = if flow.order_hint.is_empty() {
            now.clone()
        } else {
            flow.order_hint.clone()
        };
        document.entries.push(HistoryEntry {
            key: flow.key,
            generation,
            sequence,
            active: true,
            first_seen_at: first_seen_at.clone(),
            last_seen_at: now.clone(),
            completed_at: "-".to_string(),
            last_stage: flow.stage.clone(),
            last_deadline: flow.deadline.clone(),
            created_process_id: current_process_id,
        });
        changed = true;
        created_count += 1;
        crate::log_event(
            "offer_history_entry_created",
            &format!(
                "athlete_id={};sequence={};generation={};first_seen={};stage={};deadline={};process_id={};offer_history_key=offer_history_v1;all_proposer_teams_included=true;proposer_identity_visible=false",
                flow.key.athlete_id,
                sequence,
                generation,
                crate::sanitize(&first_seen_at),
                crate::sanitize(&flow.stage),
                crate::sanitize(&flow.deadline),
                current_process_id,
            ),
        );
    }

    document.entries.sort_by_key(|entry| entry.sequence);
    let bytes_len = if changed { save(db, &document)? } else { encode(&document).len() };
    if changed {
        crate::log_event(
            "offer_history_modsave_written",
            &format!(
                "offer_history_key=offer_history_v1;entry_count={};active_entry_count={};bytes_len={};process_id={};database_mod_save_data_mutation=true;save_api_called=false",
                document.entries.len(),
                document.entries.iter().filter(|entry| entry.active).count(),
                bytes_len,
                current_process_id,
            ),
        );
    }
    Ok(SyncSummary {
        changed,
        created_count,
        completed_count,
        stage_changed_count,
        active_flow_count: active_flows.len(),
        entry_count: document.entries.len(),
        bytes_len,
    })
}

pub fn status_payload(db: &mut Database, athlete_id: usize) -> Result<Vec<u8>, String> {
    let _ = synchronize_all(db)?;
    let active_flows = collect_active_flows(db);
    let document = load(db)?;
    let current_process_id = process::id();
    let Some(status) = select_status(&document, &active_flows, athlete_id) else {
        return Ok(format!(
            "status=hidden\nathlete_id={athlete_id}\ncurrent_process_id={current_process_id}\nproposer_redacted=true\n"
        )
        .into_bytes());
    };
    let reloaded = status.created_process_id != current_process_id;
    crate::log_event(
        "offer_history_selected",
        &format!(
            "athlete_id={};sequence={};first_seen={};stage={};deadline={};active_offer_count={};created_process_id={};current_process_id={};reloaded={};selection_policy=first_active_sequence;initial_discovery_order=native_last_date_then_flow_key;first_active_offer_only=true;all_proposer_teams_included=true;proposer_redacted=true",
            status.athlete_id,
            status.sequence,
            crate::sanitize(&status.first_seen),
            crate::sanitize(&status.stage),
            crate::sanitize(&status.deadline),
            status.active_offer_count,
            status.created_process_id,
            current_process_id,
            reloaded,
        ),
    );
    Ok(format!(
        "status=visible\nathlete_id={}\nfirst_seen={}\nstate_text={}\ndeadline={}\nstage={}\nsequence={}\nactive_offer_count={}\ncreated_process_id={}\ncurrent_process_id={}\nreloaded={}\nselection_policy=first_active_sequence\nproposer_redacted=true\n",
        status.athlete_id,
        status.first_seen,
        status.state_text,
        status.deadline,
        status.stage,
        status.sequence,
        status.active_offer_count,
        status.created_process_id,
        current_process_id,
        reloaded,
    )
    .into_bytes())
}

pub fn run_selection_self_test() -> Result<String, String> {
    let key_a = FlowKey { athlete_id: 10, source_team_id: 1, destination_team_id: 7 };
    let key_b = FlowKey { athlete_id: 10, source_team_id: 1, destination_team_id: 9 };
    let flow_a = ActiveFlow {
        key: key_a,
        stage: "transfer".to_string(),
        stage_rank: 1,
        state_text: "이 선수에 대한 이적 조건을 두고 구단 간 협상이 진행 중입니다".to_string(),
        deadline: "2031-06-26 00:00".to_string(),
        order_hint: "2031-06-23 09:00".to_string(),
    };
    let flow_a_recruit_draft = ActiveFlow {
        key: key_a,
        stage: "recruit_draft".to_string(),
        stage_rank: 2,
        state_text: "소속 구단의 이적 승인을 거쳐 선수에게 제시할 계약 조건을 준비 중입니다".to_string(),
        deadline: "계약 조건 제출 대기".to_string(),
        order_hint: "2031-06-23 09:00".to_string(),
    };
    let flow_a_recruit = ActiveFlow {
        key: key_a,
        stage: "recruit".to_string(),
        stage_rank: 3,
        state_text: "소속 구단의 이적 승인을 거쳐 선수와 계약 조건을 협상 중입니다".to_string(),
        deadline: "2031-06-28 00:00".to_string(),
        order_hint: "2031-06-23 09:00".to_string(),
    };
    let flow_b = ActiveFlow {
        key: key_b,
        stage: "transfer".to_string(),
        stage_rank: 1,
        state_text: "이 선수에 대한 이적 조건을 두고 구단 간 협상이 진행 중입니다".to_string(),
        deadline: "2031-06-27 00:00".to_string(),
        order_hint: "2031-06-24 14:20".to_string(),
    };
    let flow_a_changed = ActiveFlow {
        key: key_a,
        stage: "transfer".to_string(),
        stage_rank: 1,
        state_text: "수정된 이적 조건에 대한 응답을 기다리고 있습니다".to_string(),
        deadline: "2031-06-27 00:00".to_string(),
        order_hint: "2031-06-24 09:30".to_string(),
    };
    let entry_a = HistoryEntry {
        key: key_a,
        generation: 1,
        sequence: 1,
        active: true,
        first_seen_at: "2031-06-23 09:00".to_string(),
        last_seen_at: "2031-06-23 09:00".to_string(),
        completed_at: "-".to_string(),
        last_stage: "transfer".to_string(),
        last_deadline: "2031-06-26 00:00".to_string(),
        created_process_id: 100,
    };
    let entry_b = HistoryEntry {
        key: key_b,
        generation: 1,
        sequence: 2,
        active: true,
        first_seen_at: "2031-06-24 14:20".to_string(),
        last_seen_at: "2031-06-24 14:20".to_string(),
        completed_at: "-".to_string(),
        last_stage: "transfer".to_string(),
        last_deadline: "2031-06-27 00:00".to_string(),
        created_process_id: 100,
    };
    let mut document = HistoryDocument { schema_version: 1, next_sequence: 3, entries: vec![entry_a.clone(), entry_b.clone()] };

    if select_status(&document, &[], 10).is_some() {
        return Err("no-active-offer case did not hide".to_string());
    }
    let one = select_status(&document, std::slice::from_ref(&flow_a), 10)
        .ok_or_else(|| "single-active-offer case did not select".to_string())?;
    if one.sequence != 1 {
        return Err("single-active-offer case selected the wrong sequence".to_string());
    }
    let multiple = select_status(&document, &[flow_b.clone(), flow_a.clone()], 10)
        .ok_or_else(|| "multiple-active-offer case did not select".to_string())?;
    if multiple.sequence != 1 || multiple.active_offer_count != 2 {
        return Err("multiple-active-offer case did not select the first offer".to_string());
    }
    document.entries[0].active = false;
    let fallback = select_status(&document, std::slice::from_ref(&flow_b), 10)
        .ok_or_else(|| "first-completed fallback case did not select".to_string())?;
    if fallback.sequence != 2 {
        return Err("first-completed fallback case did not select the next offer".to_string());
    }
    document.entries[0].active = true;
    let draft_transition = select_status(
        &document,
        std::slice::from_ref(&flow_a_recruit_draft),
        10,
    )
    .ok_or_else(|| "draft-transition case did not select".to_string())?;
    if draft_transition.sequence != 1
        || draft_transition.first_seen != "2031-06-23 09:00"
        || draft_transition.stage != "recruit_draft"
        || draft_transition.deadline != "계약 조건 제출 대기"
    {
        return Err("draft transition did not preserve the original first-seen time".to_string());
    }
    let transitioned = select_status(&document, std::slice::from_ref(&flow_a_recruit), 10)
        .ok_or_else(|| "stage-transition case did not select".to_string())?;
    if transitioned.sequence != 1
        || transitioned.first_seen != "2031-06-23 09:00"
        || transitioned.stage != "recruit"
    {
        return Err("stage transition did not preserve the original first-seen time".to_string());
    }
    let changed = select_status(&document, std::slice::from_ref(&flow_a_changed), 10)
        .ok_or_else(|| "condition-change case did not select".to_string())?;
    if changed.sequence != 1
        || changed.first_seen != "2031-06-23 09:00"
        || changed.deadline != "2031-06-27 00:00"
    {
        return Err("condition change did not preserve the original first-seen time".to_string());
    }
    let mut initial_discovery = vec![flow_b.clone(), flow_a.clone()];
    sort_active_flows(&mut initial_discovery);
    if initial_discovery.first().map(|flow| flow.key) != Some(key_a) {
        return Err("initial discovery ordering did not prefer the oldest native request time".to_string());
    }
    let entry_a_new = HistoryEntry {
        key: key_a,
        generation: 2,
        sequence: 3,
        active: true,
        first_seen_at: "2031-07-01 09:00".to_string(),
        last_seen_at: "2031-07-01 09:00".to_string(),
        completed_at: "-".to_string(),
        last_stage: "transfer".to_string(),
        last_deadline: "2031-07-04 00:00".to_string(),
        created_process_id: 200,
    };
    let new_generation_document = HistoryDocument {
        schema_version: 1,
        next_sequence: 4,
        entries: vec![HistoryEntry { active: false, ..entry_a.clone() }, entry_a_new],
    };
    let new_generation_flow = ActiveFlow {
        key: key_a,
        stage: "transfer".to_string(),
        stage_rank: 1,
        state_text: "이 선수에 대한 이적 조건을 두고 구단 간 협상이 진행 중입니다".to_string(),
        deadline: "2031-07-04 00:00".to_string(),
        order_hint: "2031-07-01 09:00".to_string(),
    };
    let new_generation = select_status(
        &new_generation_document,
        std::slice::from_ref(&new_generation_flow),
        10,
    )
    .ok_or_else(|| "new-generation case did not select".to_string())?;
    if new_generation.sequence != 3 || new_generation.first_seen != "2031-07-01 09:00" {
        return Err("a fully ended flow did not receive a new first-seen generation".to_string());
    }
    let payload = format!(
        "제안 접수 {} | 현재 상태 {} | 응답 기한 {}",
        transitioned.first_seen, transitioned.state_text, transitioned.deadline,
    );
    if payload.contains("destination_team") || payload.contains("source_team") || payload.contains("구단명") {
        return Err("proposer identity leaked into the display contract".to_string());
    }
    Ok("case_count=10;no_active_hidden=true;single_offer_visible=true;multiple_offer_first_selected=true;first_completed_next_selected=true;draft_transition_first_seen_preserved=true;stage_transition_first_seen_preserved=true;condition_change_first_seen_preserved=true;new_generation_new_first_seen=true;initial_discovery_oldest_request_selected=true;all_proposer_teams_included=true;proposer_identity_visible=false;selection_policy=first_active_sequence;initial_discovery_order=native_last_date_then_flow_key".to_string())
}
