//! draft_data.json 역직렬화 + 통계 접근 헬퍼.
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Default)]
pub struct Meta {
    #[serde(default)]
    pub total_matches: u64,
    #[serde(default)]
    pub relation_matches: u64,
    #[serde(default)]
    pub champion_count: u64,
}

#[derive(Deserialize, Clone)]
pub struct Champion {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub candidate_index: i64,
    #[serde(default)]
    pub positions: Vec<String>,
}

#[derive(Deserialize, Clone, Default)]
pub struct PosStat {
    pub wins: u64,
    pub games: u64,
}

#[derive(Deserialize, Clone, Default)]
pub struct ChampStat {
    #[serde(default)]
    pub games: u64,
    #[serde(default)]
    pub wins: u64,
    #[serde(default)]
    pub winrate: f64,
    #[serde(default)]
    pub picks: u64,
    #[serde(default)]
    pub pickrate: f64,
    #[serde(default)]
    pub bans: u64,
    #[serde(default)]
    pub banrate: f64,
    #[serde(default)]
    pub by_position: HashMap<String, PosStat>,
}

#[derive(Deserialize, Clone, Default)]
pub struct Rel {
    pub games: u64,
    pub wins: u64,
    pub winrate: f64,
}

#[derive(Deserialize, Clone)]
pub struct DraftData {
    #[serde(default)]
    pub meta: Meta,
    pub champions: Vec<Champion>,
    #[serde(default)]
    pub stats: HashMap<String, ChampStat>,
    #[serde(default)]
    pub synergy: HashMap<String, Rel>,
    #[serde(default)]
    pub counter: HashMap<String, Rel>,
}

impl DraftData {
    pub fn load(path: &str) -> Result<Self, String> {
        let raw = std::fs::read(path).map_err(|e| format!("{path} 읽기 실패: {e}"))?;
        serde_json::from_slice(&raw).map_err(|e| format!("JSON 파싱 실패: {e}"))
    }

    pub fn stat(&self, id: &str) -> ChampStat {
        self.stats.get(id).cloned().unwrap_or_default()
    }

    pub fn name(&self, id: &str) -> String {
        self.champions
            .iter()
            .find(|c| c.id == id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| id.to_string())
    }

    /// 같은 팀 동반 시너지 (a,b 순서 무관).
    pub fn synergy(&self, a: &str, b: &str) -> Option<&Rel> {
        let (x, y) = if a <= b { (a, b) } else { (b, a) };
        self.synergy.get(&format!("{x}|{y}"))
    }

    /// a 가 b 를 상대했을 때의 a 기준 매치업 (방향성).
    pub fn counter(&self, a: &str, b: &str) -> Option<&Rel> {
        self.counter.get(&format!("{a}>{b}"))
    }

    /// 전 챔프 평균 presence(픽률+밴률) — 압력점수 baseline.
    pub fn baseline_presence(&self) -> f64 {
        if self.champions.is_empty() {
            return 0.2;
        }
        let mut s = 0.0;
        for c in &self.champions {
            let st = self.stat(&c.id);
            s += st.pickrate + st.banrate;
        }
        s / self.champions.len() as f64
    }
}
