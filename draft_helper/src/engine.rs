//! 추천 엔진 2종.
//!  A) 게임 AI 재현 — compute_features 10특징 로지스틱 승률예측
//!     (discovered-banpick-ai.md §2/§14: 베이지안 평활 승률 + 기본가중치).
//!     단, 게임의 feature-hash 시너지/카운터 테이블 대신 경기기록 실측 시너지/카운터를 사용
//!     (구조는 동일, 더 해석가능).
//!  B) 메타 통계 — 베이지안 강도점수 100·(0.7·mean+0.3·lower) + 압력점수 (TFM2.gg 대시보드 식).
use crate::model::DraftData;

// ── 게임 AI 기본가중치 (default_meta_weights, §11) 그룹 합산 ──
//  [1]=포지션승률 0.6 (최대), 솔로 0.15, 시너지/조합(2+4+6+8) ≈ 0.42, 카운터(3+5) ≈ 0.16
const W_SOLO: f32 = 0.15;
const W_POS: f32 = 0.60;
const W_SYN: f32 = 0.42;
const W_CTR: f32 = 0.16;

/// 베이지안 평활 승률항 — §2 표준식: 2·((wins+1)/(games+2) − 0.5)·min(games/6, 1) ∈ [−1,+1].
fn winrate_term(wins: u64, games: u64) -> f32 {
    if games == 0 {
        return 0.0;
    }
    let p = (wins as f32 + 1.0) / (games as f32 + 2.0);
    let conf = (games as f32 / 6.0).min(1.0);
    2.0 * (p - 0.5) * conf
}

#[derive(Clone)]
pub struct Breakdown {
    pub solo: f32,
    pub pos: f32,
    pub syn: f32,
    pub ctr: f32,
}

#[derive(Clone)]
pub struct Reco {
    pub id: String,
    /// 게임 AI 예측 점수 (≈ 0..100, 50=호각).
    pub score_a: f32,
    /// 메타 강도+압력 점수 (≈ 0..100).
    pub score_b: f32,
    /// 블렌드 점수.
    pub blended: f32,
    pub tier: &'static str,
    pub bd: Breakdown,
}

/// 게임 AI 재현: own 팀에 c 를 넣었을 때의 예측 승률 기여(특징 가중합).
/// own=평가 주체 팀(픽=내팀 / 밴=상대팀), opp=상대.
fn score_game_ai(d: &DraftData, c: &str, own: &[String], opp: &[String]) -> (f32, Breakdown) {
    let st = d.stat(c);

    // 솔로 강도 (전체 승률)
    let solo = winrate_term(st.wins, st.games);

    // 포지션별 승률 — 후보 포지션 중 max (가장 잘 맞는 자리)
    let mut pos = -2.0f32;
    for (_p, ps) in st.by_position.iter() {
        let t = winrate_term(ps.wins, ps.games);
        if t > pos {
            pos = t;
        }
    }
    if pos < -1.5 {
        pos = solo; // 포지션 데이터 없으면 솔로로 대체
    }

    // 시너지/조합 — own 팀 기픽들과의 동반 승률 평균
    let mut syn = 0.0f32;
    let mut sn = 0;
    for p in own {
        if let Some(r) = d.synergy(c, p) {
            syn += winrate_term(r.wins, r.games);
            sn += 1;
        }
    }
    if sn > 0 {
        syn /= sn as f32;
    }

    // 카운터 — opp 픽들 상대 매치업 평균 (c 가 이기면 +)
    let mut ctr = 0.0f32;
    let mut cn = 0;
    for e in opp {
        if let Some(r) = d.counter(c, e) {
            ctr += winrate_term(r.wins, r.games);
            cn += 1;
        }
    }
    if cn > 0 {
        ctr /= cn as f32;
    }

    let bd = Breakdown {
        solo: W_SOLO * solo,
        pos: W_POS * pos,
        syn: W_SYN * syn,
        ctr: W_CTR * ctr,
    };
    let raw = bd.solo + bd.pos + bd.syn + bd.ctr; // ≈ [−1.33, +1.33]
    let score = (50.0 + 40.0 * raw).clamp(0.0, 100.0);
    (score, bd)
}

/// 베이지안 메타 강도+압력 (TFM2.gg 대시보드 식). 컨텍스트 무관(메타 기준점).
fn score_meta(d: &DraftData, c: &str, baseline: f64) -> (f32, &'static str) {
    let st = d.stat(c);
    let kappa = 42.0f64;
    let prior = 0.5f64;
    let a = st.wins as f64 + prior * kappa;
    let b = (st.games.saturating_sub(st.wins)) as f64 + (1.0 - prior) * kappa;
    let n = a + b;
    let mean = a / n;
    let var = a * b / (n * n * (n + 1.0));
    let z = 0.84f64;
    let lower = (mean - z * var.sqrt()).max(0.0);
    let strength = 100.0 * (0.7 * mean + 0.3 * lower);

    let eps = 0.001f64;
    let presence = st.pickrate + st.banrate;
    let pressure = 50.0 + 16.0 * ((presence + eps) / (baseline + eps)).ln();
    let final_b = strength * 0.84 + pressure * 0.16;

    // 티어: lower 승률% 기준 (meta-data.js tiers minLower 56/51/48/44)
    let lp = lower * 100.0;
    let tier = if lp >= 56.0 {
        "OP"
    } else if lp >= 51.0 {
        "1"
    } else if lp >= 48.0 {
        "2"
    } else if lp >= 44.0 {
        "3"
    } else {
        "4"
    };
    (final_b as f32, tier)
}

/// 현재 드래프트 상태에서 한 스텝(밴 또는 픽)의 후보 추천 목록.
/// for_my=true 면 "내 픽/밴" 관점, false 면 "상대 픽/밴" 관점.
/// is_ban=true 면 밴 추천(상대에게 가장 위협적인/강한 챔프).
pub fn recommend(
    d: &DraftData,
    available: &[String],
    my_picks: &[String],
    enemy_picks: &[String],
    for_my: bool,
    is_ban: bool,
    blend_a: f32, // 0..1 (게임AI 비중)
) -> Vec<Reco> {
    let baseline = d.baseline_presence();

    // 평가 주체/상대 팀 결정.
    //  내 픽: 내팀 가치 (own=my, opp=enemy)
    //  내 밴: 상대가 뽑으면 강할 챔프 (own=enemy, opp=my)
    //  상대 픽: 상대팀 가치 (own=enemy, opp=my)
    //  상대 밴: 내가 뽑으면 강할 챔프 (own=my, opp=enemy)
    let (own, opp): (&[String], &[String]) = match (for_my, is_ban) {
        (true, false) => (my_picks, enemy_picks),
        (true, true) => (enemy_picks, my_picks),
        (false, false) => (enemy_picks, my_picks),
        (false, true) => (my_picks, enemy_picks),
    };

    let mut out: Vec<Reco> = available
        .iter()
        .map(|c| {
            let (score_a, bd) = score_game_ai(d, c, own, opp);
            let (score_b, tier) = score_meta(d, c, baseline);
            let blended = blend_a * score_a + (1.0 - blend_a) * score_b;
            Reco {
                id: c.clone(),
                score_a,
                score_b,
                blended,
                tier,
                bd,
            }
        })
        .collect();

    out.sort_by(|x, y| y.blended.partial_cmp(&x.blended).unwrap_or(std::cmp::Ordering::Equal));
    out
}
