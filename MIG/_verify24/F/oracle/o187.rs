#![allow(unused, dead_code, non_snake_case)]
#![feature(thread_local)]
//! 24차 배치F · 187 position_eval_at 오라클 (pub 직접 호출).
//!  관측 기구 ①prof::PHASE_CALLS[90](hit)/[91](miss) 카운터(prof::enable 후) ②TLS 정본
//!  `POS_EVAL_CACHE0023___RUST_STD_INTERNAL_VAL`(32B: RefCell<PosEvalCache> 24B + lazy state 1B)을
//!  extern #[thread_local] 로 직접 읽어 슬롯 인덱스(해시식)·키 레이아웃·hit/miss 를 실행으로 확정한다.
//!  한 프로세스 = 한 시나리오(argv[1]). 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh <이 파일>
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;
use std::sync::atomic::Ordering;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[thread_local]
    #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai13position_eval14POS_EVAL_CACHE0023___RUST_STD_INTERNAL_VAL"]
    static mut POS_EVAL_CACHE_RAW: [u8; 32];
}

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn arg(a: &[String], i: usize, d: i64) -> i64 { a.get(i).and_then(|s| s.parse().ok()).unwrap_or(d) }

const SLOT: usize = 104;
const NSLOT: usize = 512;

/// 명세 purpose_ord 표(태그 → ord)
fn ord_of(p: game_ai::PositionEvalPurpose) -> u64 {
    use game_ai::PositionEvalPurpose as P;
    match p {
        P::General => 0, P::RunAway => 512, P::Recall => 1024, P::Around => 1536, P::Positioning => 2048,
        P::Trace => 2560, P::Lane => 3072,
        P::LineStyle(LineStyle::Aggressive) => 4096, P::LineStyle(LineStyle::Defensive) => 4608,
        P::LaneSafe => 5120, P::Objective => 5632, P::AttackStance => 6144,
    }
}
fn purpose_tag(p: game_ai::PositionEvalPurpose) -> u8 { unsafe { *(&p as *const _ as *const u8) } }
/// 명세 pe_slot_index 재구현
fn idx_of(id: usize, x: u64, y: u64, p: game_ai::PositionEvalPurpose) -> usize {
    let h = x ^ (y << 21) ^ ((id as u64) << 42) ^ ord_of(p);
    (h.wrapping_mul(0x9E3779B97F4A7C15) >> 55) as usize
}
fn purposes() -> Vec<game_ai::PositionEvalPurpose> {
    use game_ai::PositionEvalPurpose as P;
    vec![P::General, P::RunAway, P::Recall, P::Around, P::Positioning, P::Trace, P::Lane,
         P::LineStyle(LineStyle::Aggressive), P::LineStyle(LineStyle::Defensive), P::LaneSafe, P::Objective, P::AttackStance]
}

struct Tls { base: *const u8 }
impl Tls {
    fn get() -> Tls { Tls { base: unsafe { std::ptr::addr_of!(POS_EVAL_CACHE_RAW) as *const u8 } } }
    fn state(&self) -> u8 { rd(self.base, 24) }
    fn borrow(&self) -> i64 { rd(self.base, 0) }
    fn slots(&self) -> *const u8 { rd(self.base, 8) }
    fn seed(&self) -> u64 { rd(self.base, 16) }
    fn slot(&self, i: usize) -> *const u8 { unsafe { self.slots().add(i * SLOT) } }
    fn tag(&self, i: usize) -> u8 { rd(self.slot(i), 0x61) }
    fn some_count(&self) -> usize { if self.state() != 1 { return 0; } (0..NSLOT).filter(|&i| self.tag(i) != 2).count() }
    fn find_key(&self, id: usize, x: u64, y: u64, ptag: u8, ver: usize) -> Vec<usize> {
        if self.state() != 1 { return vec![]; }
        (0..NSLOT).filter(|&i| self.tag(i) != 2 && rd::<usize>(self.slot(i), 8) == id && rd::<u64>(self.slot(i), 16) == x
                 && rd::<u64>(self.slot(i), 24) == y && rd::<u8>(self.slot(i), 32) == ptag && rd::<usize>(self.slot(i), 40) == ver).collect()
    }
    fn dump(&self, i: usize) -> String {
        let s = self.slot(i);
        format!("slot[{}] tag={} tick={} key=(id={},x={},y={},p={},ver={}) val.risk={} tower_risk={} gain={} gain_me={} adjust={} unseen={} on_traj={} on_ptraj={} pad={:02x?}",
            i, self.tag(i), rd::<usize>(s, 0), rd::<usize>(s, 8), rd::<u64>(s, 16), rd::<u64>(s, 24), rd::<u8>(s, 32), rd::<usize>(s, 40),
            rd::<i64>(s, 48), rd::<i64>(s, 56), rd::<i64>(s, 64), rd::<i64>(s, 72), rd::<i64>(s, 80), rd::<i64>(s, 88), rd::<u8>(s, 96), rd::<u8>(s, 97),
            (0..6).map(|k| rd::<u8>(s, 98 + k)).collect::<Vec<u8>>())
    }
}

fn calls() -> (u64, u64) {
    (game_core::prof::PHASE_CALLS[90].load(Ordering::Relaxed) as u64, game_core::prof::PHASE_CALLS[91].load(Ordering::Relaxed) as u64)
}
fn ps_str(v: &PositioningScore) -> String {
    let b = v as *const _ as *const u8;
    format!("risk={} tower_risk={} gain={} gain_me={} adjust={} unseen={} on_traj={} on_ptraj={} pad={:02x?}",
        v.risk, v.tower_risk, v.gain, v.gain_me, v.adjust, v.unseen_champ_threat, v.on_trajectory, v.on_periodic_trajectory,
        (0..6).map(|k| rd::<u8>(b, 0x32 + k)).collect::<Vec<u8>>())
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let scen = arg(&a, 1, 0);
    if a.len() > 99 { let _ = game_ai::position_eval_at as *const (); }
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tick0 = arg(&a, 2, 1000) as usize;
    game.set_tick(tick0);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Top).expect("player");
    let pid = player.info.id;
    let gseed = (&game as &dyn AbstractGame).seed();
    let gtick = (&game as &dyn AbstractGame).tick();
    game_core::prof::enable();
    println!("game\tseed={}\ttick={}\tpid={}\tprof_enabled={}", gseed, gtick, pid, game_core::prof::ENABLED.load(Ordering::Relaxed));
    let ver = arg(&a, 3, 55) as usize;
    let champ = cache.player_champion[0][0].expect("champ");
    let (x0, y0) = (champ.x, champ.y);
    let tls = Tls::get();
    println!("tls_before\tstate={}", tls.state());

    match scen {
        // ── 0: 기본 — 첫 호출 miss → 슬롯 idx 예측 일치 · 레이아웃 · 2회째 hit(카운터+센티널) · tick/version 불일치 miss · seed 변경 초기화
        0 => {
            let p = game_ai::PositionEvalPurpose::General;
            let (h0, m0) = calls();
            let v1 = game_ai::position_eval_at(ver, player, &data, x0, y0, p);
            let (h1, m1) = calls();
            println!("call1\thit+={}\tmiss+={}\t{}", h1 - h0, m1 - m0, ps_str(&v1));
            println!("tls_after1\tstate={}\tborrow={}\tseed={}\tseed_match={}\tsome={}", tls.state(), tls.borrow(), tls.seed(), tls.seed() == gseed, tls.some_count());
            let idx = idx_of(pid, x0, y0, p);
            let found = tls.find_key(pid, x0, y0, purpose_tag(p), ver);
            println!("slot_pred\tidx={}\tfound={:?}\tMATCH={}", idx, found, found == vec![idx]);
            println!("{}", tls.dump(idx));
            let s = tls.slot(idx);
            println!("slot_tick_eq={}\tslot_val_eq={}", rd::<usize>(s, 0) == gtick, rd::<i64>(s, 48) == v1.risk && rd::<i64>(s, 56) == v1.tower_risk && rd::<i64>(s, 64) == v1.gain && rd::<i64>(s, 72) == v1.gain_me && rd::<i64>(s, 80) == v1.adjust && rd::<i64>(s, 88) == v1.unseen_champ_threat);
            // 2회째: 슬롯 val 에 센티널 → hit 이면 센티널이 그대로 나온다
            wr(s, 48, 777777i64); wr(s, 64, 888888i64);
            let (h0, m0) = calls();
            let v2 = game_ai::position_eval_at(ver, player, &data, x0, y0, p);
            let (h1, m1) = calls();
            println!("call2_same\thit+={}\tmiss+={}\trisk={}\tgain={}\tSENTINEL_RETURNED={}", h1 - h0, m1 - m0, v2.risk, v2.gain, v2.risk == 777777 && v2.gain == 888888);
            // tick 불일치: 슬롯 tick 을 +1 → miss (재계산 · 슬롯 덮어씀)
            wr(s, 0, gtick + 1);
            let (h0, m0) = calls();
            let v3 = game_ai::position_eval_at(ver, player, &data, x0, y0, p);
            let (h1, m1) = calls();
            println!("call3_tickmis\thit+={}\tmiss+={}\trisk={}\tslot_tick_now={}\tslot_risk_now={}\tSENTINEL_RETURNED={}", h1 - h0, m1 - m0, v3.risk, rd::<usize>(s, 0), rd::<i64>(s, 48), v3.risk == 777777);
            // version 불일치: 같은 (id,x,y,purpose) 다른 version → 같은 슬롯 · miss · 덮어씀
            wr(s, 48, 777777i64);
            let (h0, m0) = calls();
            let v4 = game_ai::position_eval_at(ver + 1, player, &data, x0, y0, p);
            let (h1, m1) = calls();
            println!("call4_vermis\thit+={}\tmiss+={}\trisk={}\tslot_ver_now={}\tsome={}\tSENTINEL_RETURNED={}", h1 - h0, m1 - m0, v4.risk, rd::<usize>(s, 40), tls.some_count(), v4.risk == 777777);
            // 원 version 으로 다시 → miss (덮어써졌으니)
            let (h0, m0) = calls();
            let v5 = game_ai::position_eval_at(ver, player, &data, x0, y0, p);
            let (h1, m1) = calls();
            println!("call5_verback\thit+={}\tmiss+={}\tslot_ver_now={}", h1 - h0, m1 - m0, rd::<usize>(s, 40));
            // purpose LineStyle 내부값: Aggressive 와 Defensive 는 ord 가 달라 슬롯이 다르다(예측)
            let pa = game_ai::PositionEvalPurpose::LineStyle(LineStyle::Aggressive);
            let pd = game_ai::PositionEvalPurpose::LineStyle(LineStyle::Defensive);
            println!("linestyle_tags\taggr={}\tdef={}\tidxA={}\tidxD={}", purpose_tag(pa), purpose_tag(pd), idx_of(pid, x0, y0, pa), idx_of(pid, x0, y0, pd));
            // seed 변경: 새 게임(seed 5678) → 전체 초기화 후 miss
            let mut game2 = Game::new(5678u64, false, &setting, &ms, &map);
            {
                let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
                let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
                let mut pid2 = 0usize;
                for t in 0..2usize { for p in 0..5usize {
                    let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
                    let mut st: AthleteStat = Default::default(); st.judgement = 80; st.mental = 60;
                    game2.add_player(GamePlayer::new(pid2, "p", t, poss[p], st, "swordman", ci, Vec::new()));
                    pid2 += 1;
                } }
                game2.start_game(&mut rnd, &ctx);
            }
            game2.set_tick(tick0);
            let cache2 = AbstractGameWithCache::new(&game2 as &dyn AbstractGame, &ctx);
            let data2 = OperationData::new(&cache2, &ctx, &bb);
            let player2 = game2.get_player_by_position(0, Position::Top).expect("player2");
            let some_before = tls.some_count();
            let (h0, m0) = calls();
            let v6 = game_ai::position_eval_at(ver, player2, &data2, x0, y0, p);
            let (h1, m1) = calls();
            println!("call6_seed2\thit+={}\tmiss+={}\ttls_seed={}\tgame2_seed={}\tsome_before={}\tsome_after={}\tRESET={}", h1 - h0, m1 - m0, tls.seed(), (&game2 as &dyn AbstractGame).seed(), some_before, tls.some_count(), tls.some_count() == 1 && tls.seed() == 5678);
        }
        // ── 1: 12 purpose 전부 + 좌표/플레이어 변형 → 슬롯 예측 전수 대조(해시식 확정)
        1 => {
            let mut nmatch = 0usize; let mut ntot = 0usize;
            let mut keys: Vec<(usize, u64, u64, game_ai::PositionEvalPurpose, usize)> = Vec::new();
            let players: Vec<&PlayerState> = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support].iter()
                .map(|p| game.get_player_by_position(0, *p).unwrap()).collect();
            let mut k = 0u64;
            for (pi, pl) in players.iter().enumerate() {
                for p in purposes() {
                    for d in 0..3u64 {
                        k += 1;
                        let x = x0.wrapping_add(d * 12345 + pi as u64 * 777).min(setting.width - 1);
                        let y = y0.wrapping_add(d * 54321 + k * 13).min(setting.height - 1);
                        let v = ver + (d as usize % 2);
                        keys.push((pl.info.id, x, y, p, v));
                    }
                }
            }
            for (id, x, y, p, v) in keys.iter().cloned() {
                let pl = players.iter().find(|pl| pl.info.id == id).unwrap();
                let before = tls.some_count();
                let (h0, m0) = calls();
                let r = game_ai::position_eval_at(v, pl, &data, x, y, p);
                let (h1, m1) = calls();
                let idx = idx_of(id, x, y, p);
                let found = tls.find_key(id, x, y, purpose_tag(p), v);
                let ok = found == vec![idx];
                ntot += 1; if ok { nmatch += 1; }
                println!("key\tid={}\tx={}\ty={}\tp={:?}\tv={}\tidx_pred={}\tfound={:?}\thit+={}\tmiss+={}\t{}", id, x, y, p, v, idx, found, h1 - h0, m1 - m0, if ok { "MATCH" } else { "MISMATCH" });
            }
            println!("SUMMARY\tslot_match={}/{}", nmatch, ntot);
        }
        // ── 2: 슬롯 충돌 — 같은 idx 로 떨어지는 두 키를 찾아 덮어쓰기(체이닝 없음) 확인
        2 => {
            let p = game_ai::PositionEvalPurpose::General;
            let idx1 = idx_of(pid, x0, y0, p);
            let mut x2 = x0 + 1; let mut y2 = y0;
            let mut tries = 0u64;
            while idx_of(pid, x2, y2, p) != idx1 || (x2 == x0 && y2 == y0) { x2 += 1; tries += 1; if x2 >= setting.width { x2 = 1; y2 += 1; } }
            println!("collide\tidx={}\tkey1=({}, {})\tkey2=({}, {})\ttries={}", idx1, x0, y0, x2, y2, tries);
            let v1 = game_ai::position_eval_at(ver, player, &data, x0, y0, p);
            let s = tls.slot(idx1);
            println!("after1\t{}", tls.dump(idx1));
            let (h0, m0) = calls();
            let v2 = game_ai::position_eval_at(ver, player, &data, x2, y2, p);
            let (h1, m1) = calls();
            println!("after2\thit+={}\tmiss+={}\tslot_x_now={}\tOVERWRITTEN={}", h1 - h0, m1 - m0, rd::<u64>(s, 16), rd::<u64>(s, 16) == x2);
            let (h0, m0) = calls();
            let v3 = game_ai::position_eval_at(ver, player, &data, x0, y0, p);
            let (h1, m1) = calls();
            println!("after3_key1\thit+={}\tmiss+={}\tslot_x_now={}\tsome={}", h1 - h0, m1 - m0, rd::<u64>(s, 16), tls.some_count());
        }
        _ => {}
    }
    println!("tls_end\tborrow={}\tstate={}", tls.borrow(), tls.state());
}
