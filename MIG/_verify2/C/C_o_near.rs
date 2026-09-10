#![allow(unused, dead_code, non_snake_case)]
// 2차 배치 C — GoalData::has_near_line_enemy(m09.ll:3948) 본문 확정 오라클
// 1차는 "region_dist 홉거리 < 2 구조로 추정" 으로 남겼다. IR 전수독해 + 실행으로 닫는다.
use game_core::*;
use game_ai::{GoalData, EnemyRegionInfo};

fn main() {
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
    let map = MapDef::moba(&setting);
    let lines = [LineType::Top, LineType::Mid, LineType::Bottom];
    let ln = ["Top", "Mid", "Bottom"];

    // ① line_region(line, side, k) 표 — IR 은 side=0, k∈{2,3,4} 만 쓴다
    for li in 0..3usize {
        for side in 0..2usize {
            let v: Vec<String> = (0..7usize).map(|k| format!("{}", map.line_region(lines[li], side, k))).collect();
            println!("line_region\tline={}\tside={}\tk0..6={}", ln[li], side, v.join(","));
        }
        for side in 0..2usize {
            println!("lane_seq\tline={}\tside={}\t{:?}", ln[li], side, map.lane_seq(lines[li], side));
        }
    }

    // ② region_dist 27x27 행렬 덤프
    for a in 0..27usize {
        let v: Vec<String> = (0..27usize).map(|b| format!("{}", map.region_dist(a, b))).collect();
        println!("region_dist\ta={}\t{}", a, v.join(","));
    }

    // ③ has_near_line_enemy 진리표 — 슬롯 하나만 Some(region=r) 로 두고 r 을 0..27 로 훑는다
    for li in 0..3usize {
        let mut hits: Vec<String> = Vec::new();
        for r in 0..27usize {
            let mut gd: GoalData = Default::default();
            gd.enemy_region[0] = Some(EnemyRegionInfo { region: r, last_known: 0 });
            if gd.has_near_line_enemy(lines[li], &map) { hits.push(r.to_string()); }
        }
        // IR 예측: ∃k∈{2,3,4} : region_dist(line_region(line,0,k), r) < 2
        let mut pred: Vec<String> = Vec::new();
        for r in 0..27usize {
            if (2..=4).any(|k| map.region_dist(map.line_region(lines[li], 0, k), r) < 2) { pred.push(r.to_string()); }
        }
        println!("near\tline={}\tgame=[{}]\tmine=[{}]\t{}", ln[li], hits.join(","), pred.join(","),
            if hits == pred { "MATCH" } else { "★MISMATCH" });
    }

    // ④ 슬롯 5개 전부 독립인가 (어느 슬롯이든 하나만 맞으면 true)
    for slot in 0..5usize {
        let mut gd: GoalData = Default::default();
        let r = map.line_region(LineType::Mid, 0, 3);
        gd.enemy_region[slot] = Some(EnemyRegionInfo { region: r, last_known: 0 });
        println!("slot\tslot={}\tregion={}\tres={}", slot, r, gd.has_near_line_enemy(LineType::Mid, &map));
    }

    // ⑤ last_known 은 판정에 쓰이지 않는가
    let r = map.line_region(LineType::Mid, 0, 3);
    for lk in [0usize, 1, 999999] {
        let mut gd: GoalData = Default::default();
        gd.enemy_region[0] = Some(EnemyRegionInfo { region: r, last_known: lk });
        println!("lastknown\tlk={}\tres={}", lk, gd.has_near_line_enemy(LineType::Mid, &map));
    }

    // ⑥ 빈 GoalData
    let gd: GoalData = Default::default();
    for li in 0..3usize {
        println!("empty\tline={}\tres={}", ln[li], gd.has_near_line_enemy(lines[li], &map));
    }
}
