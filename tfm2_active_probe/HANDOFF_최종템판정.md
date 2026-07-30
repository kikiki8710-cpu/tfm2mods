# 인수인계 — 최종빌드(최종템) 판정 탐색방법 → item_tactics 교체용

> 출처: `tfm2_active_probe` 진단모드 (2026-07-05 인게임 검증 완료).
> 목적: item_tactics 개인전술 드롭다운에 push할 "최종템" 판정을, 검증된 이 방법으로 교체.
> ⚠ item_tactics 는 별도 세션에서 수정 중 — 이 노트는 그쪽에서 교체할 때 참조용.

## 1. 검증된 판정 알고리즘

**최종템 = `next_tier`(상위조합) 가 확실히 빈 것 `Some(empty)` AND `built_set` 포함.**

- `next_tier` = element 의 "이 아이템으로 조합되는 상위 아이템들" (upgrade 방향).
  - 레이아웃: `element + o` = `Vec<String>{ len@o, ptr@o+8, cap@o+0x10 }`, 원소 String stride `0x18`.
  - 오프셋 `o` = **바닐라/모드 둘 다 `+0x30`** (실측 확인, votes 바닐라13/모드37). 단 하드코딩 말고 배열별 프로브 권장.
- `built_set` = 모든 아이템의 next_tier 타겟 합집합 = "다른 아이템의 상위조합 결과로 등장하는 key".
  - 이유: `needlessly_large_rod` 같은 **베이스 컴포넌트도 next_tier 가 비어있음**. built_set 미포함으로 제외해야 오판 없음.

## 2. ★핵심 교훈 (item_tactics 가 놓치고 있는 함정)

item_tactics `src/lib.rs` (2026-07-05 기준) `dump_mod_items` 안:
```rust
let nt = read_nt(elem, best_off).unwrap_or_default();   // ← L623 부근. 함정!
if nt.is_empty() && built.contains(&k) { finals.push(...); }
```
`read_nt` 가 `None`(그 오프셋에서 next_tier 가 아님 = 읽기 실패)을 반환하면 `unwrap_or_default()` 가
**빈 Vec 으로 취급 → 최종템으로 오판**.

- 지금 item_tactics 는 **모드 배열만** 다루고 nt 오프셋을 그 배열에서 탐지하므로 `None` 이 거의 안 나 실질 무해.
- 하지만 **다른 모드가 모드템을 override 하면 참조가 틀어져 `None` 발생 → 오판** 가능.
- tfm2_active_probe 에서 바닐라를 포함시키자 이 함정이 실제로 터짐: riot 이 바닐라 아이템을 override →
  `wind_dagger`/`night_hood`/`hardened_heart`(전부 t1 중간템) 의 next_tier 가 `+0x30`에서 `None` →
  최종템으로 3개 오판. **`None` 을 최종에서 제외**하니 정확해짐(34개, 오판0).

## 3. 교체 코드 (None 제외 + 배열별 오프셋 분리)

item_tactics 의 `read_nt` 는 그대로 두고, **최종 판정 루프만** 아래처럼 교체:

```rust
// 최종 = next_tier '확실히' 빈것(Some(empty)) AND built 포함. None(판정불가)/베이스컴포넌트는 제외.
let mut built: std::collections::HashSet<String> = std::collections::HashSet::new();
for i in 0..cnt {
    if let Some(nt) = read_nt(buf + i * st, best_off) { for k in nt { built.insert(k); } }
}
let mut finals: Vec<u64> = Vec::new();
for i in 0..cnt {
    let elem = buf + i * st;
    let k = key_of_elem(elem).unwrap_or_default();
    match read_nt(elem, best_off) {
        Some(nt) if nt.is_empty() => {
            if built.contains(&k) { finals.push(30 + i as u64); }   // ★최종
            // else: 베이스컴포넌트(재료) → 제외
        }
        Some(_) => {}     // 중간템(상위조합 존재)
        None => {}        // ★ 이 오프셋서 next_tier 아님(판정불가) → 제외 (기존 unwrap_or_default 버그 수정점)
    }
}
```

- item_tactics 는 모드 배열 단일이라 **오프셋 분리는 불필요**(단일 best_nt 로 충분). None 제외만 반영하면 됨.
- 바닐라 최종은 item_tactics 가 이미 `VANILLA_FINAL = [4,24,9,14,19,29]`(카테고리별 t4) 하드코딩으로 처리 →
  **그대로 유지 권장.** 바닐라 동적판정은 override 로 중간템이 NONE 나서 불완전(최종 t4 는 안전하나 굳이).

## 4. 검증 근거 (tfm2_active_probe, riot+more_item 활성, 96아이템)

- 최종빌드 34개 = 바닐라 6 + 모드 28, 오판 0.
  - 바닐라 6 = `VANILLA_FINAL` 과 정확 일치: warlords_final_judgement(4)/storm_sovereign(9)/
    impregnable_fortress(14)/veil_of_annihilation(19)/prophet_of_the_abyss(24)/giants_horn_shard(29).
  - 모드 28 = `A_1_5`,`A_2_5` + `radiant_*`×26. = item_tactics `mod_final_opts()` 가 넣는 것과 동일.
- 트리 정상: `bf_sword → infinity_edge, deathblade`, `infinity_edge → radiant_infinity_edge`,
  크로스조합 `gatekeepers_armor → ...deaths_dance, jaksho_the_protean...`(바닐라→모드 override).
- NONE(판정불가) 12개 = 전부 바닐라 중간/베이스티어. 최종템은 len=0=Some(empty)라 NONE 불가 →
  최종빌드 리스트 누락 위험 구조상 없음.

## 5. 요약 — item_tactics 에서 할 일

1. `dump_mod_items` 의 최종 판정 루프에서 `read_nt(...).unwrap_or_default()` → **`match` 로 `None` 제외** (위 §3).
2. 바닐라 최종(`VANILLA_FINAL`)·드롭다운 옵션 구성(`mod_final_opts`/`item_opt_label`)은 **변경 불필요**.
3. (선택) 검증하려면 이 방법을 tfm2_active_probe 로그(`tfm2_active_probe.txt` 최종빌드 섹션)와 대조.
