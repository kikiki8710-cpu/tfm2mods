---

### `259` v48_projectile_profile — 캐스터 슬롯(0=skill,1=skill2,2=ult)의 논타겟 투사체 프로필 (속도,half폭,고정지연) — ExpectedGame 에 1회 가상 시전해 실측, TLS 캐시(캐스터 name×slot)

| 항목 | 값 |
|---|---|
| id | `fight_check__v48_projectile_profile` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai11fight_check22v48_projectile_profile` |
| 소스 | `game-ai\src\fight_check.rs:184` |
| IR | `m15.ll` 30617~31042행 |
| 경로·가시성 | `game_ai::v48_projectile_profile` · **pub** |
| 계층 | 점수화·술어 |
| exe | `eba320` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::OperationData, &game_core::Entity, u8) -> std::option::Option<(u64, u64, u64)>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[259]/sig/tls/<키>`)**

- `name`: game_ai::fight_check::V48_PROJ_PROFILE (thread_local! RefCell<V48ProfileCache>) — 접점 = fn-포인터 상수 `@anon.e5f9a016f1220e5cee86475419aee1e5.194 = constant ptr @…V48_PROJ_PROFILE…call_once…`(m15.ll:213), LocalKey::with 호출 2회(m15.ll:30654 조회 · 31032 저장)
- `role`: 작성자+소비자 (같은 함수가 조회→계산→저장). 다른 소비자는 이 명세 범위 밖(미탐색)
- `key`: (캐스터 name: String — Entity+0x248 을 clone 해 HashMap 키(m00.ll:74974~74975 String::clone) · 조회는 ptr@0x250/len@0x258 로 equivalent 비교(m00.ll:74661~74664)) × slot(u8, 배열 인덱스 0..3). ⚠id 가 아니라 이름이 키 — 동명 챔피언이면 같은 항목을 공유
- `layout`: RefCell<V48ProfileCache> 64B: +0 borrow 플래그 i64(0=free · -1=borrow_mut · 재진입 시 panic_already_borrowed m00.ll:74654) · +8 V48ProfileCache.map HashMap<String,[Option<Option<(u64,u64,u64)>>;3]>(48B, hashbrown: ctrl ptr@+8 · bucket_mask@+16 · growth_left@+24 · items@+32 · hasher RandomState@+40) · +56 V48ProfileCache.seed u64. 버킷 항목 120B: String 키 @+0(24B) · 값 배열 @+24 = 3×32B(tag i64: -1=None(미기록) · 0=Some(None)(프로필 없음으로 확정) · 1=Some(Some) 이어 +8 speed·+16 halfwidth·+24 delay) — 신규 항목 기본값은 세 슬롯 tag -1(m00.ll:75073~75077 `store i64 -1` @+24/+56/+88). tcxdict V48ProfileCache = map@0x0(48B)·seed@0x30(8B)
- `invalidation`: 조회 클로저(L188~190): `c.seed != data.cache.game.seed()` 이면 `c.seed = seed; c.map.clear()` 후 None 반환(m00.ll:74646~74651 비교 · 74830 seed store · 74833 clear). 즉 게임 seed 가 바뀌면(새 판) 전량 폐기. 같은 seed 안에서는 영구(레벨업으로 range 가 바뀌어도 재계산 없음 — halfwidth/speed/delay 는 range 비의존이라 설계상 무해, 단 level 게이트(skill2 lv>2·ult lv>4)로 None 이 저장되면 그 판에서 계속 None ★). 저장 클로저(L234): `c.seed == seed` 일 때만 기록(m00.ll:74956~74961) — 불일치면 조용히 버림
- `call_conditions`: 조회 with: 무조건(함수 진입 직후, L186). 저장 with: 캐시 미스(tag -1)로 계산 경로를 탄 경우 무조건(L232) — 결과가 None 이어도 Some(None)(tag 0) 으로 기록. 프로세스당 스레드 1개면 전역 메모 = 「케이스당 프로세스 1개」 오라클 주의 대상

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<(u64,u64,u64)> 32B | `dead_on_unwind noalias writable writeonly sret([32 x i8]) captures(none) dereferenceable(32)` · initializes 속성 없음 — 아래 returns 의 live 바이트 참조 | 4 |
| 1 | 1 | data | &OperationData(24B) | `noalias readonly captures(none) dereferenceable(24)` · +0 cache(&AbstractGameWithCache) → .game(&dyn AbstractGame) 로 seed() 만 호출(L185) · +8 context(&GameContext) 를 adjust_position·apply 에 전달(L205·213) | 4 |
| 2 | 2 | caster | &Entity(1728B) | `noalias readonly captures(address, read_provenance) dereferenceable(1728)` · 슬롯 effect·level·stat range·id·x/y·name 을 읽음. 쓰기 없음 | 4 |
| 3 | 3 | slot | u8 | `noundef` · 0=skill_effect / 1=skill2_effect() / 그 외=ult_effect() (인라인 caster_slot_effect fight_check.rs:173~177 · switch m15.ll:30689). 캐시 배열 인덱스로도 쓰여 ≥3 이면 aux 클로저에서 panic_bounds_check(m00.ll:74857·75090) — 호출자 계약: slot∈{0,1,2} | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// fight_check.rs:184  pub fn v48_projectile_profile(data:&OperationData, caster:&Entity, slot:u8) -> Option<(u64,u64,u64)>  // (speed, halfwidth, delay)
// L185
seed = data.cache.game.seed()                                   // vtable+0x20
// L186~196  TLS 조회 (aux closure#0, m00.ll:74604~)
cached = V48_PROJ_PROFILE.with(|c| {
   let mut c = c.borrow_mut();                                  // 재진입이면 panic_already_borrowed
   if c.seed != seed { c.seed = seed; c.map.clear(); return None }   // L188~190 (seed store → clear 순)
   let slots = c.map.get(&caster.name)?;                        // L193 키 = 캐스터 이름(String)
   slots[slot as usize]                                         // slot ≥3 → panic_bounds_check
})
if cached.tag != -1 { return cached (32B 그대로) }               // L196 적중: Some(None)→None · Some(Some)→Some
// L199~231  계산 (closure#1 인라인)
profile = (|| {
   // L200 caster_slot_effect(caster, slot)  (fight_check.rs:173~177 인라인)
   effect = match slot { 0 => caster.skill_effect.as_ref(),                       // 레벨 게이트 없음
                         1 => if caster.level > 2 {&caster.skill2_effect} else {&NONE},   // entity.rs:1693
                         _ => if caster.level > 4 {&caster.ult_effect}   else {&NONE} }   // entity.rs:1701
   let effect = effect?;                                          // tag(casting)==-1 → None
   // L201 EffectType::is_nontarget (type.rs:149 인라인)
   if !((effect.casting - 1) <u 2) { return None }                 // casting ∉ {Position(1), Direction(2)} → None  (L202)
   // L204 Effect::range(caster)  (effect.rs:26 인라인)
   range = max(1, effect.range + (caster.level - 1) * effect.growth_range + caster.stat_buff_cached.range)
   // L205
   (tx, ty) = Game::adjust_position(ctx.map, ctx.setting, caster.x + range, caster.y)
   // L206  가상 입력
   target = if effect.casting == Position(1) { InputTarget::Pos{x:tx, y:ty} (tag 2) } else { InputTarget::Dir{dir_x:1000, dir_y:0} (tag 1) }
   // L211  고정 시드 프로브 rnd
   probe_rnd = StdRng::seed_from_u64(((caster.name.len() as u64) << 16 | 0x7648) ^ ((slot as u64) << 8))
   // L212
   expected = ExpectedGame::from_game(&mut probe_rnd, data.cache.game)
   // L213  1회 가상 시전
   effect.ty.apply(&mut probe_rnd, &mut expected as &mut dyn AbstractGame, ctx, caster.id, &target, effect.attack_type, &None::<EffectOptionalInfo>, &mut frame(null))
   // L218  스폰 투사체 중 마지막 것 (closure#0: 필터)
   p = expected.iter_projectile().filter(|p| p.caster_id == caster.id && p.dodgeable()).last()?   // dodgeable(projectile.rs:208) = move_type ∈ {LinearDist, Delayed, Periodic, Parabolic}  (L219 `?` → None)
   // L219~220
   halfwidth = if p.shape == Circle { p.shape.radius } else { 20000 }
   // L223~228
   (speed, delay) = match p.move_type {
      LinearDist{speed,..}      => (speed, 0),                 // L224 (+0x60)
      Delayed{applyed,..}       => (0, applyed),               // L225 (+0x50)
      Parabolic{tick,..}        => (0, tick),                  // L226 (+0x60)
      Periodic{first_delay,..}  => (0, first_delay),           // L227 (+0x78)
      _                         => (Projectile::speed(p), 0),  // L228 — 필터 때문에 실행상 도달 불가
   }
   Some((speed, halfwidth, delay))                              // L230
})()   // L231 expected drop
// L232~235  TLS 저장 (aux closure#2, m00.ll:74908~)
V48_PROJ_PROFILE.with(|c| { let mut c = c.borrow_mut(); if c.seed == seed { c.map.entry(caster.name.clone()).or_insert([None;3])[slot as usize] = Some(profile) } })
// L238~239
return profile

// 분기 순서: 캐시 적중 → slot effect 존재 → is_nontarget → (range·target·rnd·expected·apply) → 투사체 존재 → 프로필. None 이 되는 지점 4곳: 슬롯 effect 없음(L200) · 타겟팅/None 캐스팅(L202) · 스폰 투사체 없음(L219) — 모두 Some(None) 으로 캐시됨.
// gen_range 호출 사이트: 이 함수 본문 0회. probe_rnd 는 from_game(L212)·apply(L213) 순으로 전달.
```

**`mem` 메모리 접근 41건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | m15.ll:30639 → +0 game 팻포인터(data@+0, vtable@+8) → vtable+0x20 = AbstractGame::seed (divtable AbstractGame 0x20) 호출(L185) → seed:u64 | 3 | OK |  |
| 1 | OperationData | 0x8 | context (&GameContext) | r | m15.ll:30666 · L205 adjust_position 인자(map@+0x20, setting@+0x8) · L213 apply 의 context 인자 | 4 | OK |  |
| 2 | GameContext | 0x20 | map (&MapDef 28112B) | r | m15.ll:30764~30765 → Game::adjust_position 1번째 인자 | 4 | OK |  |
| 3 | GameContext | 0x8 | setting (&GameSetting 5432B) | r | m15.ll:30766~30767 → adjust_position 2번째 인자 | 4 | OK |  |
| 4 | Entity | 0x4c8 | skill_effect (Option<Effect>) — slot 0 | r | m15.ll:30714 `%2+1224` (as_ref, 레벨 게이트 없음 · caster_slot_effect L175) | 4 | OK |  |
| 5 | Entity | 0x4f8 | skill_effect@tag (i32, -1=None) | r | m15.ll:30708~30711 | 4 | OK |  |
| 6 | Entity | 0x5c8 | level (usize) | r | m15.ll:30696·30719 인라인 skill2_effect()(entity.rs:1693 `level>2`)·ult_effect()(entity.rs:1701 `level>4`) 게이트 · m15.ll:30753 Effect::range 의 `(level-1)×growth_range` | 4 | OK |  |
| 7 | Entity | 0x500 | skill2_effect (Option<Effect>) — slot 1 | r | m15.ll:30721~30722 level>2 이면 필드, 아니면 상수 None(@anon.58, m15.ll:64) | 4 | OK |  |
| 8 | Entity | 0x530 | skill2_effect@tag (i32) — select 결과 +48 | r | m15.ll:30724~30726 (gep 는 select 결과 기준이라 1328 리터럴 없음) | 4 | OK |  |
| 9 | Entity | 0x538 | ult_effect (Option<Effect>) — slot 그 외(2) | r | m15.ll:30698~30699 `%2+1336` level>4 이면 필드, 아니면 상수 None | 4 | OK |  |
| 10 | Entity | 0x568 | ult_effect@tag (i32) — select 결과 +48 | r | m15.ll:30701~30703 (tcxdict Entity 0x568 = ult_effect@tag) | 3 | OK |  |
| 11 | Effect | 0x30 | casting (CastingType i32) | r | m15.ll:30730~30736 phi 로 모은 태그(=Option 니치 판별에 쓴 값과 동일 로드) → 인라인 EffectType::is_nontarget(type.rs:149) `(casting-1) <u 2` = Position(1)\|Direction(2) · L206 `casting==1`(Position) 으로 InputTarget 종류 결정(m15.ll:30776) | 4 | OK |  |
| 12 | Effect | 0x10 | range (u64) | r | m15.ll:30748~30749 인라인 Effect::range(effect.rs:26) | 4 | OK |  |
| 13 | Effect | 0x18 | growth_range (u64) | r | m15.ll:30750~30751 `(level-1)×growth_range` | 4 | OK |  |
| 14 | Entity | 0x438 | stat_buff_cached.range (usize) | r | m15.ll:30756~30757 Effect::range 의 스탯 가산항 | 4 | OK |  |
| 15 | Entity | 0x660 | x (u64) | r | m15.ll:30768~30770 `x + range` → adjust_position(L205) | 4 | OK |  |
| 16 | Entity | 0x668 | y (u64) | r | m15.ll:30771~30772 → adjust_position | 4 | OK |  |
| 17 | Entity | 0x258 | name.vec.len (String::len) | r | m15.ll:30789~30790 (dloc: string.rs:1870 len ← fight_check.rs:211) ★프로브 rnd 시드 재료 = `(name.len() << 16 \| 0x7648) ^ (slot << 8)` — id 가 아니라 이름 길이 | 4 | OK |  |
| 18 | Entity | 0x5c0 | id (usize) | r | m15.ll:30812~30813 apply 의 caster_id 인자(L213) · 필터 `p.caster_id == caster.id`(m15.ll:30855) | 4 | OK |  |
| 19 | Effect | 0x0 | ty (Arc<dyn EffectType> data ptr) | r | m15.ll:30803 ArcInner → 데이터 = ptr+16+((vtable.align-1)&~15) (m15.ll:30806~30812 Arc::deref 인라인) | 4 | OK |  |
| 20 | Effect | 0x8 | ty vtable ptr | r | m15.ll:30804~30805 · vtable+0x10 align(30807) · vtable+0x20 = EffectType::apply(divtable EffectType 0x20, m15.ll:30820~30822) | 3 | OK |  |
| 21 | Effect | 0x2c | attack_type (AttackType i32) | r | m15.ll:30814~30816 → apply 의 attack_type 인자 | 4 | OK |  |
| 22 | Projectile | 0xf8 | caster_id (usize) | r | m15.ll:30853~30855 필터 closure#0(L218) `== caster.id` | 4 | OK |  |
| 23 | Projectile | 0x40 | move_type@tag (i64 니치: 2=LinearDist 3=Delayed 4=Periodic … 10=Parabolic · 암묵 BouncingTarget) | r | m15.ll:30860~30872 인라인 Projectile::dodgeable(projectile.rs:208): 논리idx = tag≥2 ? tag-2 : 7 → {0,1,2,8} 만 true · m15.ll:30932~30944 L223 match 도 같은 환산 | 4 | OK |  |
| 24 | Projectile | 0x10 | shape@tag (i64: 0=Circle 1=Line 2=Rect 3=DirDot) | r | m15.ll:30889~30892 L219 `shape==Circle` 판별 | 4 | OK |  |
| 25 | Projectile | 0x18 | shape.Circle.radius (u64) | r | m15.ll:30924~30925 L220 halfwidth = radius (Circle 일 때만) | 4 | OK |  |
| 26 | Projectile | 0x60 | move_type.LinearDist.speed / move_type.Parabolic.tick (u64, 같은 오프셋 0x40+0x20) | r | m15.ll:30952~30953 LinearDist→speed · m15.ll:30976~30977 Parabolic→delay(tick) | 4 | OK |  |
| 27 | Projectile | 0x50 | move_type.Delayed.applyed (u64, 0x40+0x10) | r | m15.ll:30960~30961 Delayed→delay(DI `applyed`) | 4 | OK |  |
| 28 | Projectile | 0x78 | move_type.Periodic.first_delay (u64, 0x40+0x38) | r | m15.ll:30968~30969 Periodic→delay(DI `first_delay`) | 4 | OK |  |
| 29 | RefCell<V48ProfileCache>(TLS) | 0x0 | borrow 플래그 | r | aux m00.ll:74635~74640 / 74945~74951 borrow_mut(0→-1) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 30 | RefCell<V48ProfileCache>(TLS) | 0x38 | V48ProfileCache.seed (RefCell 값부 +0x30) | r | aux m00.ll:74646~74650 / 74956~74960 게임 seed 와 비교 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 31 | RefCell<V48ProfileCache>(TLS) | 0x8 | V48ProfileCache.map (hashbrown 48B: ctrl@+0x8 mask@+0x10 items@+0x20 hasher@+0x28) | r | aux m00.ll:74674~74676 items==0 이면 즉시 미스 · 74680~74682 hash_one(name) · 74791 String equivalent · 74877~74884 값 배열 [slot] 32B memcpy | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 32 | Entity | 0x248 | name (String 24B) — 저장 클로저에서 clone 해 키로 | r | aux m00.ll:74973~74975 `%26+584` String::clone · 조회 클로저는 ptr@0x250(m00.ll:74661~74662)·len@0x258(74663~74664) | 4 | OK |  |
| 33 | sret | 0x0 | Option<(u64,u64,u64)>.tag | w | m15.ll:30740·30744·30895 `store i64 0` / 30993 `store i64 1` → %12 → memcpy 32B to %0 (31034) · 캐시 적중 시 %14 의 32B 복사(30661) | 4 | 확인불가(tcx 사전에 타입 없음) | 0(None) \| 1(Some) |
| 34 | sret | 0x8 | speed | w | m15.ll:30987~30988 (Some 일 때만) | 4 | 확인불가(tcx 사전에 타입 없음) | LinearDist.speed \| Projectile::speed(p)(기타) \| 0 |
| 35 | sret | 0x10 | halfwidth | w | m15.ll:30989~30990 | 4 | 확인불가(tcx 사전에 타입 없음) | Circle.radius \| 20000 |
| 36 | sret | 0x18 | delay | w | m15.ll:30991~30992 | 4 | 확인불가(tcx 사전에 타입 없음) | Delayed.applyed \| Periodic.first_delay \| Parabolic.tick \| 0 |
| 37 | RefCell<V48ProfileCache>(TLS) | 0x0 | borrow 플래그 | w | aux m00.ll:74640 / 74873·74890 (조회) · 74951 / 75118 (저장) | 4 | 확인불가(tcx 사전에 타입 없음) | -1 → 복원(+1) |
| 38 | RefCell<V48ProfileCache>(TLS) | 0x38 | V48ProfileCache.seed | w | aux m00.ll:74830 — seed 불일치 시(L189). 저장 클로저는 seed 를 안 바꿈 | 4 | 확인불가(tcx 사전에 타입 없음) | data.cache.game.seed() |
| 39 | RefCell<V48ProfileCache>(TLS) | 0x8 | V48ProfileCache.map | w | aux m00.ll:74833 clear · 74990 rustc_entry · 75067~75078 신규 120B 항목(키 + 3×tag -1) insert_no_grow · 75094~75096 `[slot]`(stride 32) memcpy 32B | 4 | 확인불가(tcx 사전에 타입 없음) | clear()(seed 불일치, L190) / entry(name.clone()).or_insert([None;3])[slot] = profile(32B memcpy, L235) |
| 40 | stack | 0x0 | probe_rnd StdRng(320B alloca %9) · expected ExpectedGame(168B %8) · target InputTarget(24B %10) · _optional_info None(40B %7 tag -2) · frame ptr null(8B %6) | w | m15.ll:30798·30800·30782~30786·30817·30819 — 함수 밖 &mut 인자 없음(전 인자 readonly). ExpectedGame 은 L231 drop(Entity Vec @+16, Projectile Vec @+80 drop 30901·30918) | 4 | 확인불가(★모호: 동명 def_path 3개 [('game_core::Hunter) | L211~213 로컬 |

**`consts` 상수 14건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 196 | 센티널 | m15.ll:30657 캐시 조회 결과 tag -1 = 미기록(None) → 계산 경로. 같은 -1 이 Option<Effect> 니치(None) 판별에도 쓰임(m15.ll:30703·30710·30726, entity.rs:742 as_ref) | 4 |
| 1 | 0 | 200 | 태그 | slot switch case 0 = skill_effect (m15.ll:30690, caster_slot_effect fight_check.rs:175) · sret tag None(30740·30744·30895) · L206 dir_y=0 · L224 delay=0 / 다른 arm speed=0 | 4 |
| 2 | 1 | 200 | 태그 | slot switch case 1 = skill2_effect() (m15.ll:30691, fight_check.rs:176) · L204 `max(1, range)` 하한(30762) · L206 InputTarget 태그 1=Dir(30781) · L230 sret tag Some(30993) | 4 |
| 3 | 2 | 176 | 태그 | entity.rs:1693 skill2_effect() 레벨 게이트 `level > 2`(m15.ll:30720) · type.rs:149 is_nontarget `(casting-1) <u 2` 의 폭(30736: Position1·Direction2) · L206 InputTarget 태그 2=Pos(30781) · L208/L223 move_type 태그→논리idx `tag-2`(30864·30936) | 4 |
| 4 | 4 | 177 | 임계 | entity.rs:1701 ult_effect() 레벨 게이트 `level > 4`(m15.ll:30697) — 5레벨 미만이면 ult 슬롯 None | 4 |
| 5 | 30280 | 211 | 미상 | 0x7648 — 프로브 rnd 시드 상수: seed = (caster.name.len() << 16 \| 0x7648) ^ (slot << 8) (m15.ll:30793~30797). 고정 시드라 호출측 rnd 소비 없음 | 4 |
| 6 | 16 | 211 | 계수 | 시드 산술 `name.len() << 16`(m15.ll:30793 shl) — 곱셈 65536 접힘 | 4 |
| 7 | 8 | 211 | 태그 | 시드 산술 `slot << 8`(m15.ll:30796 shl) — 곱셈 256 접힘 | 4 |
| 8 | 1000 | 206 | 산출값 | Direction 캐스팅일 때 가상 입력 InputTarget::Dir{dir_x:1000, dir_y:0}(m15.ll:30779) — +x 단위 방향(투사체 기하는 방향 무관이라 임의 고정) | 4 |
| 9 | 20000 | 219 | 산출값 | shape 가 Circle 이 아닐 때(Line/Rect/DirDot) halfwidth 기본값 20000(m15.ll:30930 phi) = 0.625셀(셀 32000) | 4 |
| 10 | 9 | 208 | 센티널 | `llvm.assume(tag != 9)`(m15.ll:30862·30934) — ProjectileMoveType 니치 태그 9 는 untagged BouncingTarget 자리라 불가(tcxdict --enum ProjectileMoveType). 판정 아님 | 3 |
| 11 | 7 | 208 | 센티널 | move_type 태그 <2 이면 논리idx 7 = BouncingTarget(untagged)(m15.ll:30866·30938 select) — tcxdict 니치 환산과 일치 | 3 |
| 12 | -2 | 213 | 센티널 | apply 의 `_optional_info: &Option<EffectOptionalInfo>` = None(니치 태그 -2, tcxdict --enum EffectOptionalInfo: Area=-1 · None=-2)(m15.ll:30817) | 3 |
| 13 | 3 | 193 | 인덱스 | aux: 캐시 값 배열 길이 3 — `slot < 3` bounds check(m00.ll:74857·75090) 실패 시 panic_bounds_check. 인덱스 상한이라 판정 아님 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 프로브 rnd 시드 상수 | fight_check.rs:211 | 30280 | 0x7648. 바꿔도 결정론만 바뀜(투사체 기하는 시드 비의존이 설계 의도 — 실제 의존 여부는 apply 구현체별 미확인) | 4 | 기존 |
| 1 | 비-Circle 투사체 half 폭 기본값 | fight_check.rs:219 | 20000 | Line/Rect/DirDot 투사체의 회피 판정 폭. 올리면 소비자(v48 회피)가 더 넓게 피함 — 단 소비자 미독해 | 4 | 기존 |
| 2 | Direction 캐스팅 가상 방향 | fight_check.rs:206 | 1000 | dir=(1000,0). 기하가 방향 비의존이면 무영향. 맵 경계 근처 캐스터에서 +x 방향 투사체가 경계 처리로 달라질 수 있음(미검증) | 4 | 기존 |
| 3 | skill2/ult 레벨 게이트 | entity.rs:1693/1701 (인라인) | 2 / 4 | level≤2 → skill2 None, level≤4 → ult None 이 캐시에 Some(None) 으로 박혀 그 판 내내 None(★캐시가 레벨을 키에 안 넣음) | 4 | 기존 |
| 4 | 캐시 키 = 캐스터 이름 | fight_check.rs:193/235 | String | 같은 이름(같은 챔피언)의 두 엔티티가 항목 공유 — 레벨·아이템 사거리가 달라도 speed/halfwidth/delay 는 range 비의존이라 설계상 무해, 레벨 게이트만 예외 | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | apply | game_view::GameView::apply | in:game_view::view::game | fn(&mut game_view::GameView, &engine_core::assets::Assets, &impl Platform/#0, &game_view::GameViewSharedConfig, &mut std::option::Option<&mut engine_core::ui::node::Node>, &mut game_view::GameViewSystem, usize, bool, &game_core::GameFrameData) | game-view\src\view\game.rs:889 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 145개 중 상위 3개 |
| 2 | apply | game_core::BuffState::apply | pub | fn(&game_core::BuffState, game_core::EntityStat) -> game_core::EntityStat | game-core\src\simulation\entity.rs:1113 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 145개 중 상위 3개 |
| 3 | apply | game_core::EffectType::apply | pub | fn(&Self/#0, &mut rand::rngs::std::StdRng, &mut dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext, usize, game_core::InputTarget, game_core::AttackType, std::option::Option<game_core::EffectOptionalInfo>, &mut std::option::Option<&mut game_core::GameFrameData>) | game-core\src\simulation\effect\type.rs:271 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 145개 중 상위 3개 |
| 4 | caster_slot_effect | game_ai::caster_slot_effect | pub | fn(&game_core::Entity, u8) -> std::option::Option<&game_core::Effect> | game-ai\src\fight_check.rs:173 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | dodgeable | game_core::Projectile::dodgeable | pub | fn(&game_core::Projectile) -> bool | game-core\src\simulation\projectile.rs:207 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | frame | game_view::ui::gaming_house_ui::HouseCharacter::frame | in:game_view::ui::gaming_house_ui | fn(&game_view::ui::gaming_house_ui::HouseCharacter, &engine_core::assets::Assets) -> usize | game-view\src\ui\gaming_house_ui.rs:1227 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | from_game | game_core::ExpectedGame::<'a>::from_game | pub | fn(&mut rand::rngs::std::StdRng, &'a/#0 dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + 'a/#0) -> game_core::ExpectedGame<'a/#0> | game-core\src\simulation\expected_game.rs:20 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | is_nontarget | game_core::CastingType::is_nontarget | pub | fn(&game_core::CastingType) -> bool | game-core\src\simulation\effect\type.rs:148 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | iter_projectile | <game_core::ExpectedGame<'a> as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::ExpectedGame<'a/#0>) -> game_core::ProjectileIter | game-core\src\simulation\expected_game.rs:230 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | seed | game_core::AbstractGame::seed | pub | fn(&Self/#0) -> u64 | game-core\src\simulation.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | seed | <game_core::Game as game_core::AbstractGame>::seed | pub | fn(&game_core::Game) -> u64 | game-core\src\simulation\game.rs:1564 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | seed | <game_core::SingleLaneGame as game_core::AbstractGame>::seed | pub | fn(&game_core::SingleLaneGame) -> u64 | game-core\src\simulation\game.rs:3781 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | speed | game_core::Projectile::speed | pub | fn(&game_core::Projectile) -> u64 | game-core\src\simulation\projectile.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | v48_projectile_profile | game_ai::v48_projectile_profile | pub | fn(&game_core::OperationData, &game_core::Entity, u8) -> std::option::Option<(u64, u64, u64)> | game-ai\src\fight_check.rs:184 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 13개**: `borrow_mut`, `cached`, `clear  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `entry`, `last`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `llvm.umax.i64`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `or_insert`, `seed_from_u64`, `with  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 2개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m00.ll:93603) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | Option<Option<(u64,u64,u64)>> 의 외측 None 메모리 태그가 -1 인 것은 IR 관측(m00.ll:74899 phi -1 · 75073 default -1) — rustc 니치 규칙상 2 가 예상되나 tcxdict 에 이 제네릭 인스턴스 layout 이 없어 이론 근거는 미확정. IR 값(-1) 을 정본으로 씀. -2 는 LocalKey::try_with 의 AccessError 니치(m00.ll:74891 → panic_access_error) | 3 |  |
| 1 | 미탐색 | EffectType::apply 구현체별 내부(투사체 스폰 조건·rnd 소비)는 미독해 — 어떤 effect 가 dodgeable 투사체를 0개 스폰해 None 이 되는지는 구현체 IR 필요(범위 밖) | 4 |  |
| 2 | 미탐색 | Projectile::dodgeable 이 정확히 {LinearDist, Delayed, Periodic, Parabolic} 인 것은 인라인 switch(m15.ll:30867~30872) 로 확정. 단 Delayed/Periodic 을 「회피 가능」으로 치는 의도는 미확인 | 4 |  |
| 3 | 미탐색 | L228 `_ => (Projectile::speed(p), 0)` arm 은 필터 때문에 실행상 도달 불가로 판정 — 단 dodgeable 과 L223 match 가 소스에서 별도 정의라 향후 dodgeable 확장 시 살아남. reach 도구는 live 로 봄(상수 접기 아님) | 4 |  |
| 4 | 표기 불가 | seed 재료가 `caster.name.len()`(Entity+0x258) 인 것은 IR 정본. 지시문 요지의 `handle<<16` 과 다름 — 소스가 `caster.name.len()` 인지 다른 표현의 접힘인지는 소스 부재로 표기 불가(동작은 확정) | 3 |  |
| 5 | 미탐색 | V48_PROJ_PROFILE 의 다른 소비자·작성자 존재 여부 미탐색(이 명세 범위 = 이 함수 + 두 with 클로저) | 4 |  |
| 6 | 미탐색 | sret 에 `initializes` 속성 없음 → None 경로에서 +8..+32 가 undef 로 복사되는 것은 IR 구조(alloca 부분 초기화 후 32B memcpy)로 확정. sweep 비교 시 None 이면 tag 8B 만 대조할 것 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | Game::adjust_position 내부(맵 경계 보정 규칙) 미독해 — 반환 (x,y) 만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

