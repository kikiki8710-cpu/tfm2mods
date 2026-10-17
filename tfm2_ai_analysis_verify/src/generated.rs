// 자동 생성: 현행 PE/proof 고정 근거. 직접 편집하지 않는다.
pub const BASELINE_EXE_SHA256: &str = "a1d05045405a12e2bed182daeafcad8c8a40cf29ab68ba6ea5bfd30257439582";
pub const CENSUS_FUNCTIONS: usize = 741;
pub struct ProbeDef { pub id: usize, pub rva: usize, pub semantic_id: &'static str, pub name: &'static str, pub body: &'static [u8], pub prologue: &'static [u8], pub source: &'static str }
pub static PROBES: &[ProbeDef] = &[
 ProbeDef { id: 3, rva: 0xe49c00, semantic_id: "game_finish.all_present_own_roles_within_600k_of_opponent_nexus", name: "존재하는아군5역할모두상대기지600000반경안인지판정", body: include_bytes!("../tools/data/probe_e49c00.bin"), prologue: &[0x56,0x57,0x48,0x83,0xec,0x28,0xb8,0x1,0x0,0x0,0x0,0x48,0x29,0xc8], source: "game-ai\\src\\game_finish.rs" },
 ProbeDef { id: 4, rva: 0xfa9dc0, semantic_id: "handler.obs.other_role_same_lane_strict_near150k", name: "다른 역할의 같은 라인 관측과 엄격한150k거리", body: include_bytes!("../tools/data/probe_fa9dc0.bin"), prologue: &[0x48,0x83,0xec,0x28,0x4c,0x8b,0x1,0x48,0x8b,0xa,0x49,0x8b,0x0], source: "game-ai\\src\\plan\\handler\\obs.rs" },
 ProbeDef { id: 5, rva: 0xf51e50, semantic_id: "battle.role_dual_timestamp_rejection_predicate", name: "battle.role_dual_timestamp_rejection_predicate", body: include_bytes!("../tools/data/probe_f51e50.bin"), prologue: &[0x48,0x83,0xec,0x28,0x48,0x8b,0x1,0x4c,0x8b,0x0,0x49,0x8b,0x40,0x8], source: "game-ai\\src\\plan\\big_plan\\battle.rs" },
 ProbeDef { id: 6, rva: 0xe9eb70, semantic_id: "tfm2.team_plan.064.0xe9eb70", name: "BB rawtag0 참여 record 존재와 자기 HP25 게이트", body: include_bytes!("../tools/data/probe_e9eb70.bin"), prologue: &[0x48,0x83,0xec,0x28,0x48,0x8b,0x8a,0x0,0x8,0x0,0x0,0x48,0x83,0xf9,0x1], source: "game-ai\\src\\plan\\team_plan\\gank_gates.rs" },
 ProbeDef { id: 7, rva: 0xfdb9a0, semantic_id: "shared.forecast_record_lookup.first_of_12", name: "12개176B 레코드에서 key 첫 일치 포인터 조회", body: include_bytes!("../tools/data/probe_fdb9a0.bin"), prologue: &[0x48,0x83,0xec,0x28,0x48,0x89,0xc8,0x48,0x8b,0x89,0x40,0x8,0x0,0x0], source: "game-ai\\src\\ai_utils.rs" },
];
