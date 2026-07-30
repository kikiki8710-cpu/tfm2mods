# mod_api SDK API — 컴파일러 심문으로 복구한 시그니처 (재심문 방지)

> 순수 SDK 모드. raw 오프셋/RVA 금지. 아래는 `rustup run nightly-2026-05-24 rustc … --extern mod_api=…` 로
> `let _: () = expr;` 타입불일치 에러를 읽어 복구. 심문 probe = `_probe/`.

## Node (mod_api::Node)
- `n.id: String`
- `n.child: Vec<Node>`         — 자식(재귀 워킹)
- `n.visible: bool`
- `n.runner: Box<dyn NodeRunner>`

## 러너 downcast
- `n.runner.type_name() -> &str`          — 러너 종류 문자열
- `n.runner.as_any() -> &dyn Any`         → `.downcast_ref::<T>()`
- `n.runner.as_any_mut() -> &mut dyn Any` → `.downcast_mut::<T>()`
- 러너 타입(rmeta 확인): LabelRunner, ButtonRunner, ImageRunner, DropdownRunner,
  SelectableRunner, ColorSelectableRunner, CheckboxRunner, SliderRunner, ScrollViewRunner,
  CanvasRunner, SvgRunner, AssetSelectorRunner, TextEditRunner, TreeViewRunner, ColorRunner,
  AnimationRunner, NodeRunner

## LabelRunner
- `lr.text: String`
- `lr.style: Style<LabelProperty>`   (`Style`·`Color` 는 prelude 미노출 — 필드경유로만 접근)
- `lr.style.normal.color: Color`     — Color{ r,g,b,a: f32 }
- 색 변경(타입 임포트 불필요, 제자리 변경):
  ```rust
  if let Some(lr) = n.runner.as_any_mut().downcast_mut::<LabelRunner>() {
      let c = &mut lr.style.normal.color; c.r=0.3; c.g=0.9; c.b=0.45;
  }
  ```
- Style 상태: `.normal` 확인됨. hover/active/disabled 는 미확인(필요 시 심문).

## Node 주입 (순수-SDK, RVA 훅 없이)
> ⚠ 우리 draft_overlay 는 RVA 훅(LOADER/PARSER)으로 주입 = 패치취약. 여기선 쓰지 않음.
> 순수-SDK 주입 = **기존 형제 노드 clone → 수정 → 부모 child 에 push**.
- `Node: Clone` ✓
- `n.layout: Style<Layout>` — 위치/크기는 `n.layout.normal.{x,y,width,height}: Length` (px/percent)
- 패턴:
  ```rust
  let mut cell = row_name_label.clone();
  cell.id = "csb_0".to_string();
  cell.layout.normal.x = /* Length */;              // 헤더 csb_h0 x 와 정렬
  if let Some(lr)=cell.runner.as_any_mut().downcast_mut::<LabelRunner>(){ lr.text="80".into(); }
  row.child.push(cell);
  ```
- ⚠ 재주입 방지: 이미 주입한 id(csb_0..) 존재하면 값만 갱신, 없을 때만 push.
- `Length`(prelude 미노출, 생성자 불명) — **구성 불필요**: 주입 셀 위치는 헤더노드
  `csb_hN.layout.normal.x/width` **값을 복사**하면 컬럼 정렬 자동. (Length=작은 enum, Copy/Clone 추정)

## 데이터 타입 (SDK 심문으로 확인)
- `Staff.stat -> StaffStat`     — 코치 능력치 구조체(밴픽/전술/협상/판단/잠재/피드백/전력/컨트롤/판단/멘탈 10개)
- `Athlete.stat -> AthleteStat` — 선수 능력치 구조체(12개)
- `Team`, `Scene`(enum, def: game-view/src/lib.rs:19) 는 존재하나 하위 접근 메서드/필드가 명시적이지 않음.
- ❌ 미해결: **Scene/게임상태 → 현재 화면 Staff/Athlete "리스트" 꺼내는 경로**.
  Team 에 athletes/staffs/money 필드·메서드 없음. Scene 에 team/staffs/athletes 메서드 없음.
  블라인드 심문서 did-you-mean 힌트도 안 나옴 → 이 접근경로가 유일 난관.
  (StaffStat/AthleteStat 개별 필드명도 rmeta grep 불가 → 심문 필요)

## 데이터 바인딩 (미해결 — daram2 dll 참조 필요)
- 코치진 능력치 헤더 = layout.ui `csb_h0..9` (banpick/strategy/negotiation/judge_ability/
  judge_potential/feedback/power_analysis/control_coaching/judgment_coaching/mental_coaching).
- 행별 값 셀은 .ui 에 없음 → **DLL 이 런타임 주입**. 각 행 ↔ Staff 객체 매핑 + 스탯 읽기 방법이
  아직 미확인. rmeta 의 player_* 접근자는 인-매치용 가능성. → daram2 dll 디컴파일로 확정 예정.

## ★마스터 API 문서 = sdk_050_src/docs/native-mod-api-reference.md (1075줄)
> 관리화면 데이터·세이브·러너·챔피언·아이템 전부 문서화. 심문 전에 여기부터.

## ★game_core 직접 링크 = 전 게임구조체 타입드 접근 (라이트/생성)
> mod_api 는 Team/Athlete/Staff 는 재수출하나 하위 타입(ChampionTier/MerchandiseProduct 등)은 미노출.
> deps 에 `libgame_core-*.rlib` 있음 → `--extern game_core=<rlib>` 로 직접 링크(같은 rlib=TypeId 일치, 안전).
> 빌드: `tfm2_view_plus/build.ps1`(mod_api + game_core 둘 다 extern).
- `use game_core::{ChampionTier, MerchandiseProduct};` (크레이트 루트 재수출; `data::` 는 private)
- 구조체 전체 필드 = `let T {} = x;` E0027 / enum 변형 = 비망라 match E0004 로 열거.

## ★Team 구조체 필드 (전부 pub — db_mut().teams.get_mut(id) 로 write)
`id name manager_name logo league_id last_strategy strategy popularity last_starting news
 total_balance(f64) transfer_budget salary_budget scout_budget team_color_strategy
 pending_installments resale_clauses scout_dispatch stadium total_home_attendance home_match_count
 total_entrance_income watched_athletes champion_personal_tactics release_list_athletes
 no_transfer_athletes champion_tiers fan_expectation fan_satisfaction fan_count
 training_facility_grade merchandise_facility_grade gaming_house_level merchandise_products
 watched_staffs release_list_staffs fan_momentum gaming_house_customization welfare gaming_house_inventory`
- **statistics 티어**: `champion_tiers: HashMap<String, ChampionTier>`. 변형 = `S A B C D NoTier`.
  → `team.champion_tiers.insert(champ_name, ChampionTier::S)`; 무티어=NoTier(또는 remove). 세이브 자동저장.
  게임이 champion-info/banpick 에서 같은 필드 읽어 자동 전파(UI relabel 불요).
- **facility 생산**: `merchandise_products: Vec<MerchandiseProduct>`. MerchandiseProduct 필드:
  `product_type athlete_id stock sell_price yearly_sales yearly_revenue total_sales total_revenue
   daily_purchase_rate last_produced_date last_sold_date base_player_daily_purchase_rate
   base_team_daily_purchase_rate price_elasticity_x100 daily_sales_remainder`
  → 전체 추가생산 = 각 product `.stock += qty` (daram2 +0x10=stock 일치). 신규생산 = 신규 MerchandiseProduct push.
  ⚠ 비용은 게임 파이프라인이 stock 델타 처리(daram2도 차감코드 없음). product_type/sell_price 세팅 필요.

## ★관리화면 데이터 접근 (순수 SDK, 검증 완료)
- 진입: `Scene::InGame { data }` → `data: ClientData` (관리 게임플레이)
- `data.db() -> Ref<ClientDatabase>`, `data.db_mut() -> RefMut<..>`
- ClientData read 헬퍼:
  - `player_team_id() -> usize`, `player_team()/team(id) -> Option<Ref<Team>>`, `team_ids()`
  - `athlete_ids() -> Vec<usize>`, `athlete(id) -> Option<Ref<Athlete>>`
  - `staff_ids() -> Vec<usize>`, `staff(id) -> Option<Ref<Staff>>`
  - `champion_info(name:&str) -> Option<Arc<dyn ChampionInfo>>`
- ClientDatabase pub 필드: `athletes/staffs/teams: HashMap<usize,_>`, `champion_info_sheet`,
  `item_setting`, `game_setting`, `mod_save_data`, `time: NaiveDateTime`, `scene: ClientScene`
- 검증된 필드 접근:
  - `Staff`: `.name: String`, `.stat: StaffStat`, `.role`
  - `StaffStat`(usize): banpick strategy negotiation judge_ability judge_potential feedback
    power_analysis control_coaching judgment_coaching mental_coaching (헤더 csb_h0..9 순서)
  - `Athlete`: `.stat: AthleteStat`, `.contract` (`.contract.team_id()` = **메서드**), `.hidden`
  - `AthleteStat`(usize): last_hit skill_avoid skill_hit positioning control_speed concentration
    mental judgement order roaming aggressive ego
- 열거 패턴: `athlete_ids()` 순회 → `a.contract.team_id()==player_team_id()` 필터 → `.stat` 렌더.
  (코치진 화면은 UI 행 자체가 내 팀 코치라 이름매칭으로 바인딩 가능)

## save_compat 판단 = 거의 N/A (단일 통합모드)
- 우리 영속화 = **네이티브 Team 필드 직접 write**(champion_tiers/merchandise_products) → 게임이 일반
  게임상태로 자동 저장. **mod_save 네임스페이스 미사용**(모드 등록 게이트 불필요). 설정 = 로컬 .cfg.
- daram2 legacy_save_patcher = 별도 뷰모드들을 구 세이브 enabled_mods 에 등록하던 것 → 단일 모드엔 불요.
- ⚠ 인게임 확인: 구 세이브 로드 시 champion_tiers write 가 정상 persist 되는지(can_write_mod_save 무관할 것).

## ★세이브 저장 (순수 SDK — 참고용, 현재 미사용)
- ClientData: `mod_save_get_string/set_string(mod_id,key,val)`, `mod_save_get_bytes/set_bytes`,
  `mod_save_contains_key`, `mod_save_keys`, `can_write_mod_save()`, `mod_save_set_version`.
  write 는 서버로 save mutation 패킷 자동 큐잉(허용 시).
- ClientDatabase.mod_save_data: `ModSaveData`(set_string/get_string/set_bytes...). MAX_VALUE_LEN=1MB.
- 모드 메시징: `send_mod_command(mod_id,cmd,payload)`, `mod_events(mod_id)`.

## 진입/확장 (template)
- `impl ModExtension { fn post_update(&self, scene:&mut Scene, ui:&mut GameUI, assets:&mut Assets, dt:f32) }`
- `ModRegistration::new(id)`, `reg.set_extension(...)`, `declare_mod!(init)`
- `ui.root: Node`, `ui.filter_handler` (클릭 라우팅 — scroll_fix 참고)
- `UIEvent::Click { path: String, .. }`

## rmeta 로 존재 확인된 게임 데이터 타입/함수 (시그니처는 추후 심문)
- 선수/코치: Athlete, Staff, EntityStat, Stat
- 챔피언: ChampionInfo, ModChampionInfo, ChampionInfoSheet, ChampionPatchStatistics,
  ChampionCategory/SubCategory/Tag, champion_name, champion_id_at, champion_count, data_champion
- 티어: next_tier, previous_tier, tier    ← 티어 부여 1급 API
- 선수 스탯 접근자: player_kills/deaths/assists/cs/gold/level/position/champion/team/
  is_alive/respawn_time, player_at, player_count, get_player, players
- 영입: ScoutDispatchInfo, RecruitDoneAthlete
- 아이템/생산: Item, ModItemInfo, ItemCategory, ItemTag, Price, add_item, produced, price
- 에셋/스킨: Assets, loaded_assets, AssetSelectorRunner, AssetSelectorProperty,
  custom_champion_native_effect_model, add_native_effect, native_effects, asset_loader
- 팀: Team, TeamResearchData, TeamTrainingPlan

## 빌드
```
$env:NoDefaultCurrentDirectoryInExePath=""
cmd /c "C:\tfm2mods\sdk_050_hotfix\mod-sdk\build_mod.bat" "src\lib.rs"   # → lib.dll (cwd)
```

## post_render / RenderState / RenderCommand 심문 확정 (2026-07-26, tfm2_banpick_illust v1.3.0 patchviz 실사용 — 재심문 금지)
- post_render 시그니처 = `(&self, &Scene, &GameUI, &Assets, &mut RenderState)`
- `RenderState` pub 필드: `commands: HashMap<String, Vec<RenderCommand>>` · `pass: Vec<RenderPass>`(pub 필드 없음) · `map_size: HashMap<String,(f32,f32)>` · `exports: Vec<ExportCommand>` · `base_map: String`
- `RenderCommand` = **engine_core::render_state 소속 — mod_api 미노출** ⇒ rustc `--extern engine_core` 직접 링크 필요(**build_inj.ps1에 추가됨** — 전 모드 공용, 미사용 모드 무해)
  - 변형 18종: SetCamera/SetImageScale/Svg/Sprite/NinePatch/Mesh{vertices,indices}/SpriteInstance/Text/DrawLine/DrawLineEx/RoundingBox/FogOverlay/AnnularSector/FilledPath/StartMaskingLayer/ApplyMaskingLayer/AdjustCanvas/RestoreCanvas
  - `Text{rect:Rect, z:i32, text:String, font, rot, size:f32, scale, color:Color, align_x/y, line_height, shader, underline_ranges}`
  - `Sprite{x,y:f32, z, rot, flip_x/y, texture:String, texture_rect:Rect, pivot_x/y, sample_nearest, shader}`
  - `DrawLine{from_x/y, to_x/y, from_color/to_color:Color, width:f32, z}`
  - `FilledPath{points:Vec<(f32,f32)>, z, color:Color, gradient:Option<...>}`
  - ⚠`Color`(common::color {r,g,b,a})·`Rect`(common::rect {x,y,w,h})는 prelude 미노출 ⇒ **기존 커맨드 clone 후 필드 수정** 트릭으로 신규 생성 회피(참조구현 = `tfm2_banpick_illust\src\patchviz.rs`)

## 챔피언 데이터 API 심문 확정 (2026-07-26 — 재심문 금지)
- `ChampionInfo`(dyn) 메서드: `stat()→EntityStat` · `growth()→EntityStat` · `category()` · `attack()→Box<dyn Action>` (name/id/attack_speed/range **없음**)
- `dyn Action`: `cooltime(&Entity)→usize`(⚠밴픽 컨텍스트 사용불가) · `duration()→usize` · `effect()→Option<Effect{range, growth_range, start_timing, casting, target, +2}>`
- `EntityStat` 필드 9종: attack, magic_power, hp, defence, magic_resistance, move_speed, hp_regen, stack, crit_chance
- `ChampionInfoSheet`: **Default 구현됨**(=무패치 원본 스탯 생성 경로 — ⚠챔피언 세팅 코드 전체가 정적 링크돼 dll +2.5MB 팽창), 필드=mod_champions+챔프별 61종(예 fighter: FighterChampionInfo), `get_champion_mut(&str)→Option<&mut dyn ChampionInfo>` · `get_id(&str)→Option<usize>`
- ★추가 심문 확정 (2026-07-29, banpick_illust v1.3.2 modchamps 실사용 — 재심문 금지)
  - `ChampionInfoSheet.mod_champions: Vec<ModChampionEntry>` = **pub 필드**(prelude 경유 접근 가능)
  - `ModChampionEntry` 필드 = `id: String` · `stat: EntityStat` · `growth: EntityStat` · `attack: DataActionDef` · `skill: DataActionDef` … 외 11
  - **`ModChampionEntry`는 `ChampionInfo` 트레잇을 구현** ⟹ `&entry`를 `&dyn ChampionInfo`로 그대로 전달 가능(stat()/growth()/attack() 사용 가능)
  - ★실측(0.5.2, 07-29): **`get_champion_mut`은 모드(워크샵) 챔프도 찾는다**(base 출처 sheet 92 / 스냅샷 mod_champions 4) — "하드코딩 61종만 매칭" 가설은 **틀림**
- `ClientDatabase.get_historical_sheet(&str, Option<NaiveDate>)→&ChampionInfoSheet` · `version_at_date(NaiveDate)` 실존(v1.3.0에선 미사용)
- LabelRunner 텍스트 raw 읽기 = **text@+352** (참조구현 = `tfm2_banpick_illust\src\raw.rs::label_text`)
