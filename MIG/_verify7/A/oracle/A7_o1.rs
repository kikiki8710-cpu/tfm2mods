#![allow(unused, dead_code, non_snake_case)]
//! A7-O1 (7차 배치A) — ev>=4 로 남은 행을 실행으로 내리고, 7차에 새로 복원한 소스 표기를 컴파일로 검증한다.
//!
//! 표적:
//!  (T1) `specs[2]` consts[1](5=Recall) · consts[4](16=AttackNexus) — **줄길이 산술로 복원한 소스 표기**
//!       `SubPlan::Recall(RecallSubPlan::default())` / `SubPlan::AttackNexus(AttackNexusSubPlan::default())`
//!       가 ① 실제로 컴파일되고 ② 메모리 태그 5 / 16 을 만드는지. (페이로드가 0B ZST 라 store 가 없다는 것도 확인)
//!  (T2) `specs[3]` consts[6](1 = Option<usize> 의 Some 판별자) — 레이아웃 실측.
//!  (T3) `specs[1]` consts[8](tag 1 = Minion) · consts[9](-10) · knobs[4] — **6차까지 미니언 케이스가 오라클에
//!       한 번도 안 들어갔다**(A6_o3 은 Nexus/Tower/Jungle/Champion 만). 유효 Entity 의 EntityType 판별자만
//!       1 로 덮어써서(수법 ⑦) 계수 -10 을 실행 확인하고, **원복 후** drop 한다(페이로드 오해석 drop 방지).
//!  (T4) `specs[1]` closed[1] — tag 1 경로의 술어가 `is_any_type_minion` 인지(`is_minion(line)` 이 아닌지)
//!       를 세 라인 전부로 실행 대조.
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn tag8<T>(v: &T) -> u8 { unsafe { *(v as *const T as *const u8) } }
fn tag64<T>(v: &T) -> i64 { unsafe { std::ptr::read_unaligned(v as *const T as *const i64) } }

fn main() {
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };

    // ── (T1) 복원한 소스 표기가 컴파일되고 같은 태그를 만드는가 ───────────────
    {
        use game_ai::plan_legacy::sub_plan::{SubPlan, RecallSubPlan, AttackNexusSubPlan,
                                             LineDefenseSubPlan};
        println!("ZST\tRecallSubPlan={}B\tAttackNexusSubPlan={}B\tSubPlan={}B",
                 std::mem::size_of::<RecallSubPlan>(),
                 std::mem::size_of::<AttackNexusSubPlan>(),
                 std::mem::size_of::<SubPlan>());
        let r = SubPlan::Recall(RecallSubPlan::default());
        let a = SubPlan::AttackNexus(AttackNexusSubPlan::default());
        println!("SUBPLAN_SRC\tRecall(RecallSubPlan::default())\ttag={}\t기대=5\t{}",
                 tag64(&r), if tag64(&r) == 5 { "MATCH" } else { "**MISMATCH**" });
        println!("SUBPLAN_SRC\tAttackNexus(AttackNexusSubPlan::default())\ttag={}\t기대=16\t{}",
                 tag64(&a), if tag64(&a) == 16 { "MATCH" } else { "**MISMATCH**" });
        // 페이로드 3B 를 쓰는 LineDefense 와 대조 — 0B 인 두 variant 는 태그 말고 쓰는 바이트가 없다
        let bytes_r: Vec<u8> = unsafe {
            std::slice::from_raw_parts(&r as *const _ as *const u8, std::mem::size_of::<SubPlan>()).to_vec() };
        println!("SUBPLAN_SRC\tRecall 바이트[0..16]={:?}", &bytes_r[..16]);
    }

    // ── (T2) Option<usize> 판별자 ────────────────────────────────────────────
    {
        let s: Option<usize> = Some(7);
        let n: Option<usize> = None;
        println!("OPTUSIZE\tsize={}B\tSome(7).discr={}\tNone.discr={}\t기대 Some=1\t{}",
                 std::mem::size_of::<Option<usize>>(), tag64(&s), tag64(&n),
                 if tag64(&s) == 1 && tag64(&n) == 0 { "MATCH" } else { "**MISMATCH**" });
        // effect_cc_time 의 반환형과 같은 형태인지(명세 consts[6] 의 주장)
        let cc: Option<usize> = Some(0);
        println!("OPTUSIZE\tSome(0).discr={}\t(Some(0) 도 is_some ⟹ 위협으로 센다는 history[9] 와 정합)", tag64(&cc));
    }

    // ── (T3)(T4) Minion 계수 ───────────────────────────────────────────────
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let player: &PlayerState = game.get_player_by_position(0, Position::Jungle).unwrap();
        let champ: &Entity = cache.player_champion[0][1].unwrap();
        let dynchamp: &dyn AbstractEntity = champ as &dyn AbstractEntity;
        let ef = Effect {
            range: 200000, growth_range: 0, start_timing: 0,
            casting: CastingType::Targeting, target: CastingTarget::Enemy,
            ty: Arc::new(AttackEffect::new(500, 0)) as Arc<dyn EffectType>,
            attack_type: AttackType::Skill,
        };
        let zp: std::mem::MaybeUninit<game_ai::ScoreParameter> = std::mem::MaybeUninit::zeroed();
        let za: std::mem::MaybeUninit<Box<dyn Action>> = std::mem::MaybeUninit::zeroed();
        let parameter: &game_ai::ScoreParameter = unsafe { zp.assume_init_ref() };
        let action: &Box<dyn Action> = unsafe { za.assume_init_ref() };

        // 기준 엔티티 = 타워(유효 Entity). 판별자만 잠깐 1(Minion) 로 바꿔 재본 뒤 **반드시 원복**한다.
        let mut base: Option<Entity> = None;
        for id in game.world.tower_ids.iter().take(1) {
            if let Some(e) = game.world.entity.get(*id) { base = Some(e.clone()); }
        }
        let mut e = base.expect("tower entity 없음");
        let orig = tag64(&e);
        let tyaddr = (&e as *const Entity as usize) + 0x68;
        for (label, newtag) in [("Tower(원본)", orig), ("Minion(판별자 주입)", 1i64), ("None(0)", 0i64)] {
            unsafe { std::ptr::write_unaligned(tyaddr as *mut i64, newtag) };
            let hp = e.hp as i64;
            if hp == 0 { println!("MINION\t{}\thp=0 skip", label); continue; }
            let value = Effect::expected_damage_target(&ef, &ctx, dynchamp, &e) as i64;
            let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
            let got = game_ai::calculate_jungle_action_score(&mut rnd, player, &data, parameter, action, &ef, &e);
            let coef: i64 = match newtag { 3 => 200, 2 => 80, 1 => -10, _ => 0 };
            let pred = std::cmp::min(coef, coef.wrapping_mul(value) / hp);
            // 대립가설: 미니언 계수가 0(= 감점 없음)이라면?
            let pred_zero = 0i64;
            println!("MINION\t{}\ttag={}\thp={}\tvalue={}\tcoef(명세)={}\tgame={}\tmine={}\t대립(coef=0)={}\t{}",
                     label, newtag, hp, value, coef, got, pred, pred_zero,
                     if got == pred && pred != pred_zero { "MATCH(판별)" }
                     else if got == pred { "MATCH" } else { "**MISMATCH**" });
        }
        unsafe { std::ptr::write_unaligned(tyaddr as *mut i64, orig) };   // ★원복(drop 전)

        // (T4) tag 1 경로의 술어 — is_any_type_minion vs is_minion(line)
        let ty_minion_probe = &e.ty;
        println!("PRED\tTower.ty: is_any_type_minion={}\tis_minion(Top)={}\tis_minion(Mid)={}\tis_minion(Bottom)={}",
                 ty_minion_probe.is_any_type_minion(),
                 ty_minion_probe.is_minion(LineType::Top),
                 ty_minion_probe.is_minion(LineType::Mid),
                 ty_minion_probe.is_minion(LineType::Bottom));
        unsafe { std::ptr::write_unaligned(tyaddr as *mut i64, 1i64) };
        println!("PRED\ttag1(주입): is_any_type_minion={}\tis_minion(Top)={}\tis_minion(Mid)={}\tis_minion(Bottom)={}\t(★is_minion 이 라인별로 갈리면 IR 의 「태그 1 → 곧바로 -10」과 모순 ⟹ is_any_type_minion 이 맞다)",
                 e.ty.is_any_type_minion(),
                 e.ty.is_minion(LineType::Top),
                 e.ty.is_minion(LineType::Mid),
                 e.ty.is_minion(LineType::Bottom));
        unsafe { std::ptr::write_unaligned(tyaddr as *mut i64, orig) };
        println!("PRED\t원복 tag={}", tag64(&e));
    }
    println!("DONE\tsetting_ok={}", ok);
}
