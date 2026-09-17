//! hl — 관전 하이라이트 수집(원작 `hl_collect`/`hl_finalize` 의 stable 판).
//! 원작: 클라 `db.game_view.client.events`(GameFrameData) 를 프레임 스캔해 EntityEvent/KillEvent/PlayerStatistics 문자열 파싱.
//! stable: 이벤트 스트림 접근 없음 → `StableMatchHook` 이 sim 안에서 매 tick 델타를 뽑는다(Spectator_Chat 과 같은 기법).
//!   · 대상 sim = `sim_origin().kind` ∈ {ClientMatchView, ClientSpectate, ClientReplay} (서버 presim 제외)
//!   · 킬 = kill_log 델타(killer_team+lane → pid) / 궁·스킬 시전 = cooldown 0→상승 / 피격 = 한 tick hp 하락 ≥100 또는 cc 추가
//!   · 포탑·처형 데스 = is_alive 하강인데 ±10tick 안에 그 선수가 killed 인 kill_log 없음
//!   · 골드 = 150tick 마다 팀합 / 승자 = nexus 사망(엔티티 이름에 nexus) 또는 최종 킬스코어
//!   · 완성 시점 = sim.is_end() / 다음 세트 on_match_start / export 시 5초 이상 tick 없음(부분 완성)
//! 블록 키 = origin (match_id, replay_id, set_index) → export 가 우리 매치·세트 번호로 정확 매칭. 선수명은 export 시 리플레이 로스터로 해석
//!   (sim 에는 athlete id 가 없어 pid → (팀, 챔피언) 만 저장; 라인 텍스트는 `{p:<pid>}` 자리표시자).
//! 틱 = 30/초(0.6.0 db-reference §3; Spectator_Chat 과 동일 가정 ⬜미검증).
use mod_api_stable::{SimOriginKindV1, StableMatchHook, StableSim};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

pub const TICKS_PER_SEC: i64 = 30;
const MULTIKILL_WINDOW: i64 = 10 * TICKS_PER_SEC;
const HIT_WINDOW: i64 = 25;            // 시전 후 적중 집계 창(~0.8초)
const CHAIN_WINDOW: i64 = 10 * TICKS_PER_SEC;
const ULT_MIN_HITS: usize = 3;
const SKILL_MIN_HITS: usize = 4;
const HIT_MIN_DMG: i64 = 100;
const GOLD_COMEBACK_GAP: i64 = 10_000;
const GOLD_SAMPLE: i64 = 5 * TICKS_PER_SEC;
const MAX_BLOCKS: usize = 8;
const IDLE_FINALIZE_SECS: u64 = 5;

pub type OriginKey = (u32, u64, u64, u64); // (kind, match_id, replay_id, set_index)

#[derive(Clone, Debug)]
enum Ev {
    Kill { killer: usize, killed: usize },
    Death(usize),
    Cast { pid: usize, is_ult: bool },
    Hit(usize),            // 피격(피해 ≥100 또는 cc) 당한 pid
    Gold(i64, i64),
}

#[derive(Default)]
struct Raw {
    origin: Option<OriginKey>,
    players: HashMap<usize, (String, usize, u32)>, // pid → (champion, team, lane)
    events: Vec<(i64, Ev)>,
    kill_seen: usize,
    alive: HashMap<usize, bool>,
    hp: HashMap<usize, usize>,
    cc_seen: HashMap<usize, usize>,
    cds: HashMap<usize, (usize, usize, usize)>, // (skill, skill2, ult)
    nexus: HashMap<usize, usize>,               // entity id → team
    nexus_die: Option<usize>,
    last_gold_tick: i64,
    last_tick: i64,
    last_wall: Option<Instant>,
    ended: bool,
}
static RAW: Mutex<Option<Raw>> = Mutex::new(None);

#[derive(Clone)]
pub struct HlBlock {
    pub origin: OriginKey,
    /// pid → (champion, team, lane)
    pub players: HashMap<usize, (String, usize, u32)>,
    pub score: (i64, i64),
    /// (tick, 텍스트 — `{p:<pid>}` 자리표시자 포함)
    pub lines: Vec<(i64, String)>,
    pub partial: bool,
}
pub static BLOCKS: Mutex<Vec<HlBlock>> = Mutex::new(Vec::new());

fn origin_key(sim: &StableSim<'_>) -> Option<OriginKey> {
    let o = sim.sim_origin()?;
    let k = o.kind;
    if k != SimOriginKindV1::ClientMatchView as u32 && k != SimOriginKindV1::ClientSpectate as u32 && k != SimOriginKindV1::ClientReplay as u32 { return None; }
    Some((k, o.match_id, o.replay_id, o.set_index))
}

pub fn fmt_tick(tk: i64) -> String { let s = tk.max(0) / TICKS_PER_SEC; format!("{:02}:{:02}", s / 60, s % 60) }

pub struct Hook;
impl StableMatchHook for Hook {
    fn on_match_start(&self, sim: &mut StableSim<'_>) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let Some(key) = origin_key(sim) else { return };
            let mut g = RAW.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(prev) = g.take() { if !prev.ended { finalize(prev, true); } }
            let mut r = Raw { origin: Some(key), last_wall: Some(Instant::now()), ..Default::default() };
            for i in 0..sim.player_count() {
                let Some(p) = sim.player_at(i) else { continue };
                let champ = p.champion().and_then(|e| e.name()).unwrap_or_default();
                r.players.insert(p.id(), (champ, p.team(), p.lane().map(|l| l as u32).unwrap_or(9)));
                r.alive.insert(p.id(), true);
                if let Some(e) = p.champion() { r.hp.insert(p.id(), e.hp().0); }
                r.cds.insert(p.id(), p.cooldowns().map(|c| (c.1, c.2, c.3)).unwrap_or((0, 0, 0)));
            }
            for i in 0..sim.entity_count() {
                if let Some(e) = sim.entity_at(i) {
                    if !e.is_champion() && !e.is_tower() && !e.is_minion() && e.name().map(|n| n.to_ascii_lowercase().contains("nexus")).unwrap_or(false) { r.nexus.insert(e.id(), e.team()); }
                }
            }
            crate::log(&format!("[hl] start origin={:?} players={} nexus={:?}", key, r.players.len(), r.nexus));
            *g = Some(r);
        }));
    }
    fn on_match_tick(&self, sim: &mut StableSim<'_>, _seed: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let Some(key) = origin_key(sim) else { return };
            let mut g = RAW.lock().unwrap_or_else(|e| e.into_inner());
            let Some(r) = g.as_mut() else { return };
            if r.origin != Some(key) || r.ended { return; }
            let t = sim.tick() as i64;
            r.last_tick = t;
            r.last_wall = Some(Instant::now());
            // 킬 로그 델타
            let kc = sim.kill_log_count();
            while r.kill_seen < kc {
                if let Some(k) = sim.kill_log_at(r.kill_seen) {
                    let find = |team: usize, pos: u32| r.players.iter().find(|(_, (_, tm, ln))| *tm == team && *ln == pos).map(|(pid, _)| *pid);
                    let killer = find(k.killer_team, k.killer_position);
                    let killed = find(1 - k.killer_team.min(1), k.killed_position);
                    if let (Some(a), Some(b)) = (killer, killed) { r.events.push((k.tick as i64, Ev::Kill { killer: a, killed: b })); }
                }
                r.kill_seen += 1;
            }
            let pids: Vec<usize> = r.players.keys().copied().collect();
            for pid in pids {
                let Some(p) = sim.get_player(pid) else { continue };
                let alive = p.is_alive();
                let was = r.alive.insert(pid, alive).unwrap_or(true);
                if was && !alive { r.events.push((t, Ev::Death(pid))); }
                if let Some(e) = p.champion() {
                    let (hp, _mx) = e.hp();
                    let prev = r.hp.insert(pid, hp).unwrap_or(hp);
                    let mut hit = alive && prev > hp && (prev - hp) as i64 >= HIT_MIN_DMG;
                    let cc_n = e.cc_count();
                    let seen = r.cc_seen.entry(pid).or_insert(0);
                    if cc_n > *seen { for i in *seen..cc_n { if let Some(c) = e.cc_at(i) { if c.kind <= 2 { hit = true; } } } }
                    *seen = cc_n;
                    if hit { r.events.push((t, Ev::Hit(pid))); }
                }
                if let Some(cd) = p.cooldowns() {
                    let prev = r.cds.insert(pid, (cd.1, cd.2, cd.3)).unwrap_or((0, 0, 0));
                    if prev.2 == 0 && cd.3 > 15 { r.events.push((t, Ev::Cast { pid, is_ult: true })); }
                    if (prev.0 == 0 && cd.1 > 15) || (prev.1 == 0 && cd.2 > 15) { r.events.push((t, Ev::Cast { pid, is_ult: false })); }
                }
            }
            if t - r.last_gold_tick >= GOLD_SAMPLE {
                r.last_gold_tick = t;
                let (mut gb, mut gr) = (0i64, 0i64);
                for i in 0..sim.player_count() { if let Some(p) = sim.player_at(i) { if p.team() == 0 { gb += p.gold() as i64; } else { gr += p.gold() as i64; } } }
                r.events.push((t, Ev::Gold(gb, gr)));
            }
            if t % 30 == 0 && r.nexus_die.is_none() {
                for (id, tm) in r.nexus.clone() { if let Some(e) = sim.get_entity(id) { if !e.is_alive() { r.nexus_die = Some(tm); } } else { r.nexus_die = Some(tm); } }
            }
            let ended = sim.is_end();
            if ended { r.ended = true; }
            if ended { if let Some(done) = g.take() { finalize(done, false); } }
        }));
    }
}

/// export 직전 호출: 5초 이상 tick 이 멈춘 미완성 수집이 있으면 부분 블록으로 확정(원본은 유지 — 세트가 이어지면 다시 갱신).
pub fn flush_idle() {
    let snapshot = {
        let g = RAW.lock().unwrap_or_else(|e| e.into_inner());
        match g.as_ref() {
            Some(r) if !r.ended && r.last_wall.map(|w| w.elapsed().as_secs() >= IDLE_FINALIZE_SECS).unwrap_or(false) && r.last_tick > 0 => {
                Some(Raw { origin: r.origin, players: r.players.clone(), events: r.events.clone(), nexus_die: r.nexus_die, last_tick: r.last_tick, ..Default::default() })
            }
            _ => None,
        }
    };
    if let Some(r) = snapshot { finalize(r, true); }
}

fn finalize(r: Raw, partial: bool) {
    let Some(origin) = r.origin else { return };
    let team_of = |pid: &usize| -> i64 { r.players.get(pid).map(|p| p.1 as i64).unwrap_or(-1) };
    let mut ev: Vec<(i64, String)> = Vec::new();
    let mut kills: Vec<(i64, usize, usize)> = Vec::new();
    let mut deaths: Vec<(i64, usize)> = Vec::new();
    let mut casts: Vec<(i64, usize, bool)> = Vec::new();
    let mut hits: Vec<(i64, usize)> = Vec::new();
    let mut gold: Vec<(i64, i64)> = Vec::new();
    let (mut kb, mut kr) = (0i64, 0i64);
    for (t, e) in &r.events {
        match e {
            Ev::Kill { killer, killed } => { kills.push((*t, *killer, *killed)); if team_of(killer) == 0 { kb += 1; } else { kr += 1; } }
            Ev::Death(p) => deaths.push((*t, *p)),
            Ev::Cast { pid, is_ult } => casts.push((*t, *pid, *is_ult)),
            Ev::Hit(p) => hits.push((*t, *p)),
            Ev::Gold(b, rr) => gold.push((*t, b - rr)),
        }
    }
    // 1) 연속킬
    {
        let mut per: HashMap<usize, Vec<i64>> = HashMap::new();
        for (t, k, _) in &kills { per.entry(*k).or_default().push(*t); }
        for (pid, mut ticks) in per {
            ticks.sort();
            let mut start = 0usize;
            for i in 1..=ticks.len() {
                if i == ticks.len() || ticks[i] - ticks[i - 1] > MULTIKILL_WINDOW {
                    let n = i - start;
                    if n >= 2 {
                        let label = match n { 2 => "더블킬".to_string(), 3 => "트리플킬".to_string(), 4 => "쿼드라킬".to_string(), 5 => "펜타킬".to_string(), m => format!("{}연속킬", m) };
                        let champ = r.players.get(&pid).map(|p| p.0.clone()).unwrap_or_default();
                        ev.push((ticks[start], format!("{{p:{}}}({{c:{}}}) {}!", pid, champ, label)));
                    }
                    start = i;
                }
            }
        }
    }
    // 2) n인 궁 / 광역 스킬 — 시전자 귀속 정보가 없어 "창 안에 아군 다른 시전 없음"(단독 시전)일 때만 적중을 인정
    {
        casts.sort_by_key(|c| c.0);
        let mut last_cast: HashMap<(usize, bool), i64> = HashMap::new();
        let mut hl: Vec<(i64, usize, usize, bool)> = Vec::new();
        for &(f, pid, is_ult) in casts.iter() {
            if let Some(&lf) = last_cast.get(&(pid, is_ult)) { if f - lf < 60 { continue; } }
            last_cast.insert((pid, is_ult), f);
            let tc = team_of(&pid);
            if tc < 0 { continue; }
            let exclusive = !casts.iter().any(|&(of, oc, _)| oc != pid && team_of(&oc) == tc && of + HIT_WINDOW >= f && of <= f + HIT_WINDOW);
            if !exclusive { continue; }
            let mut amb: Vec<usize> = Vec::new();
            for (ht, hid) in hits.iter() {
                if *ht < f { continue; }
                if *ht > f + HIT_WINDOW { break; }
                if *hid != pid && team_of(hid) >= 0 && team_of(hid) != tc && !amb.contains(hid) { amb.push(*hid); }
            }
            let min = if is_ult { ULT_MIN_HITS } else { SKILL_MIN_HITS };
            if amb.len() >= min { hl.push((f, pid, amb.len(), is_ult)); }
        }
        hl.sort_by_key(|e| e.0);
        let mut i = 0;
        while i < hl.len() {
            let mut j = i + 1;
            while j < hl.len() && hl[j].0 - hl[j - 1].0 <= CHAIN_WINDOW { j += 1; }
            if j - i == 1 {
                let (t, pid, k, iu) = hl[i];
                ev.push((t, format!("{{p:{}}} {} — 적 {}명 적중!", pid, if iu { "궁극기" } else { "광역 스킬" }, k)));
            } else {
                let parts: Vec<String> = hl[i..j].iter().map(|(_, pid, k, iu)| format!("{{p:{}}} {}({}명)", pid, if *iu { "궁" } else { "스킬" }, k)).collect();
                ev.push((hl[i].0, format!("대규모 한타! 궁·스킬 연쇄 {}회 — {}", j - i, parts.join("·"))));
            }
            i = j;
        }
    }
    // 3) 포탑/처형 데스 — 죽었는데 그 시점 킬 로그에 피살자로 없음
    for (t, pid) in &deaths {
        let killed_by_champ = kills.iter().any(|(kt, _, kd)| kd == pid && (kt - t).abs() <= 10);
        if !killed_by_champ {
            let champ = r.players.get(pid).map(|p| p.0.clone()).unwrap_or_default();
            ev.push((*t, format!("{{p:{}}}({{c:{}}}) 포탑에 맞아 사망!", pid, champ)));
        }
    }
    // 4) 골드 역전
    if gold.len() >= 2 {
        let winner: Option<i64> = match r.nexus_die { Some(l) => Some(1 - (l as i64).min(1)), None => if kb > kr { Some(0) } else if kr > kb { Some(1) } else { None } };
        if let Some(w) = winner {
            let sgn: i64 = if w == 0 { 1 } else { -1 };
            let (mut min_t, mut min_v) = (0i64, 0i64);
            for &(t, d) in gold.iter() { let v = d * sgn; if v < min_v { min_v = v; min_t = t; } }
            if min_v <= -GOLD_COMEBACK_GAP {
                let cross = gold.iter().find(|&&(t, d)| t >= min_t && d * sgn >= 0).map(|&(t, _)| t).unwrap_or_else(|| gold.last().map(|&(t, _)| t).unwrap_or(min_t));
                ev.push((cross, format!("{}팀, 최대 {}G 골드 열세를 뒤집고 역전승! (최대 열세 시점 {})", if w == 0 { "블루" } else { "레드" }, -min_v, fmt_tick(min_t))));
            }
        }
    }
    crate::log(&format!("[hl] finalize origin={:?} partial={} kills={} casts={} hits={} gold={} nexus_die={:?} score={}:{} lines={}", origin, partial, kills.len(), casts.len(), hits.len(), gold.len(), r.nexus_die, kb, kr, ev.len()));
    ev.sort_by_key(|e| e.0);
    let mut blocks = BLOCKS.lock().unwrap_or_else(|e| e.into_inner());
    blocks.retain(|b| b.origin != origin);
    if ev.is_empty() { return; }
    blocks.push(HlBlock { origin, players: r.players.clone(), score: (kb, kr), lines: ev, partial });
    while blocks.len() > MAX_BLOCKS { blocks.remove(0); }
}

pub fn blocks() -> Vec<HlBlock> { BLOCKS.lock().unwrap_or_else(|e| e.into_inner()).clone() }

#[cfg(test)]
mod tests {
    use super::*;
    fn raw() -> Raw {
        let mut r = Raw { origin: Some((2, 7, 70, 1)), ..Default::default() };
        for (pid, champ, team, lane) in [(1, "knight", 0, 0), (2, "archer", 0, 3), (3, "ninja", 1, 1), (4, "bard", 1, 4), (5, "ogre", 1, 0)] { r.players.insert(pid, (champ.to_string(), team, lane)); }
        r
    }
    #[test]
    fn multikill_and_tower_and_gold() {
        let mut r = raw();
        r.events.push((100, Ev::Kill { killer: 1, killed: 3 }));
        r.events.push((100, Ev::Death(3)));
        r.events.push((200, Ev::Kill { killer: 1, killed: 4 }));
        r.events.push((200, Ev::Death(4)));
        r.events.push((900, Ev::Death(2))); // 킬로그 없음 → 포탑
        r.events.push((0, Ev::Gold(0, 0)));
        r.events.push((3000, Ev::Gold(1000, 13000)));
        r.events.push((6000, Ev::Gold(20000, 15000)));
        r.events.push((1000, Ev::Cast { pid: 5, is_ult: true }));
        r.events.push((1005, Ev::Hit(1))); r.events.push((1010, Ev::Hit(2))); r.events.push((1012, Ev::Hit(1)));
        r.nexus_die = Some(1);
        finalize(r, false);
        let b = blocks();
        assert_eq!(b.len(), 1);
        let lines: Vec<String> = b[0].lines.iter().map(|(t, l)| format!("[{}] {}", fmt_tick(*t), l)).collect();
        println!("{:#?}", lines);
        assert!(lines.iter().any(|l| l.contains("더블킬")));
        assert!(lines.iter().any(|l| l.contains("포탑에 맞아 사망")));
        assert!(lines.iter().any(|l| l.contains("역전승")));
        assert!(!lines.iter().any(|l| l.contains("궁극기 — 적")), "2명 적중은 궁 하이라이트 아님");
        assert_eq!(b[0].score, (2, 0));
    }
}
