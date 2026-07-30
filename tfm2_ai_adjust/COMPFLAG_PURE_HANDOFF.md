# disc19 Gate1 compFlag 순수화 — 잔여 2항 구현 핸드오프

> 대상: `src/tfm2_ai_adjust.rs`의 `d19_g1_compflag_pure`. 이 문서 하나로 컨텍스트 없이 통합 구현 가능.
> 출처: 정본 `ANA\tfm2-0.5.0-migration.md §11.9.10.2` (ghidra-re, FUN_14209a750 디컴 정독, **0.5.0_3 buildid 24125999**).
> 두 항 모두 **RNG-free·바이트 재현 가능** 확정. 게임 헬퍼 FFI 호출 없음(완전 재구현 원칙 준수).

## 배경 (현 상태)
`d19_g1_compflag_pure(g0, nexus, other, qx, qy) -> (cf0, cf1)`는 이미 ~95% 완성(de40 오브젝티브 루프·list_a/b·nearest·flag 매핑·loop2 threat 게이트·위협레코드 빌더 `d19_cf_threat`). 순수 계산은 `d19_g1cf_shadow` cfg로 shadow(FUN_142090ec0) 롤백 가능(기본 ON=shadow, 검증 전 안전).

**잔여 = 2항뿐:**
1. loop2 skillType∈{3,4} 항 — 기존 loop2 안 `// ★loop2 skillType∈{3,4} 항(a0211)은 1차 미구현` 주석 자리(미구현)
2. knight_ult de40 엔트리게이트 — `if rd_u8(desc + 0x131) == 0 { continue; }` 한 줄(근사, knight_ult 특수유닛 오탈락)

재사용 헬퍼(전부 소스에 존재): `d19_target_valid(desc,c,t)->bool`, `d19_cf_castrange(u,pad)->i64`, `geom_vt28(gc)->i64`, `isqrt(u64)->u64`, `rd_i64/rd_u64/rd_i32(->Option)`, `rd_u8/rd_u32(->값)`, `ptr_ok`. (`rd_u16` 없음 → `rd_u32(a) & 0xffff` 사용.)

---

## 변경 1 — loop2 skillType∈{3,4} 항 (cf0 추가 OR)

기존 loop2(`for k in 0..nb { … }`)는 `list_b`(이미 관계+거리 필터 완료)를 순회하며 threat 게이트를 계산한다. **같은 루프 안**, `// ★loop2 skillType…` 주석 자리에 아래를 삽입(같은 `u = list_b[k]` 재사용):

```rust
// ── loop2 skillType∈{3,4} → cf0 (§11.9.10.2 A) ──
let s = rd_i64(u + 0x2f0).unwrap_or(0);
let st: u64 = if s >= 0 { 4 } else { (s as u64) ^ 0x8000_0000_0000_0000 };
if st == 3 || st == 4 {
    // st별 오프셋: st3 기준, st4는 usability/선분B/threshold/vt28비교 전부 +0x10
    let d = if st == 3 { 0usize } else { 0x10usize };
    let uv_desc = u + 0x344 + d;   // usability descriptor arg
    let bx = rd_i64(u + 0x318 + d).unwrap_or(0);   // 선분 끝점 B.x
    let by = rd_i64(u + 0x320 + d).unwrap_or(0);   // 선분 끝점 B.y
    let thr_base = rd_i64(u + 0x328 + d).unwrap_or(0);
    let cmp_fld  = rd_i64(u + 0x330 + d).unwrap_or(i64::MAX);   // vt28 비교대상
    // (1) usability
    if d19_target_valid(uv_desc, u, nexus) {
        // (2) ctx_vt28 >= *(u+0x330/0x340)   ⚠아래 "미확정" 참조
        let ctx28 = geom_vt28(go);
        if ctx28 >= cmp_fld {
            // (3) 점-선분 최근접 거리(정수) — A=유닛앵커, B=위 끝점, P=nexus(qx,qy)
            let ax = rd_u64(u + 0x648).unwrap_or(0) as i64;   // 선분 끝점 A (st 무관 고정)
            let ay = rd_u64(u + 0x650).unwrap_or(0) as i64;
            let (px, py) = (qx as i64, qy as i64);
            let (dx, dy) = (bx - ax, by - ay);
            let (pdx, pdy) = (px - ax, py - ay);
            let len2 = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
            let t10000: i64 = if len2 == 0 { 0 } else {
                let num = dy.wrapping_mul(pdy).wrapping_add(dx.wrapping_mul(pdx)).wrapping_mul(10000);
                (num / len2).clamp(0, 10000)
            };
            // 매직 -0x346dc5d63886594b = 표준 ÷10000·부호반전 → P-proj 벡터. Rust 절삭÷와 동치.
            let ex = pdx - dx.wrapping_mul(t10000) / 10000;
            let ey = pdy - dy.wrapping_mul(t10000) / 10000;
            let dist2 = ex.wrapping_mul(ex).wrapping_add(ey.wrapping_mul(ey)) as u64;
            let dist = isqrt(dist2) as i64;   // floor sqrt (원본 Newton과 동치)
            // (4) threshold: 둘째 항 = d19_cf_castrange(nexus, 20000)과 동일
            let thr = thr_base + d19_cf_castrange(nexus, 20000);
            if dist <= thr {
                cf0 = true;   // 게이트 통과=무조건(damage core score와 독립·OR 누적)
            }
        }
    }
}
```

**확정 사항**
- cf0=1은 damage core(FUN_1422e85a0) score와 **독립**(게이트 통과만으로 세팅, 클리어 없는 OR 누적).
- `list_b`가 이미 관계/거리 필터를 거쳤으므로 별도 프리게이트 불필요.
- st3/st4 델타: usability desc/선분B.x/선분B.y/threshold base/vt28비교 = **전부 +0x10** (위 `d`로 처리). effects-list base(원본 local_b0)만 -0x8이나 cf0 술어와 무관하여 생략.

**⚠ 미확정 (구현 시 확인 필요) — `ctx28`**
- 원본은 `(*local_350)()` = **`local_108`(AI/쿼리 컨텍스트 객체)의 vt슬롯 0x28**, self/nexus가 아님. 함수 진입부에서 동일 호출로 이미 사용됨.
- 위 코드는 `geom_vt28(go)`(=`rd_i64(go+0xeac0)`)로 근사. **`go`와 `local_108`이 동일 객체인지 검증**할 것. 기존 코드가 loop2 threat 경로에서 이미 `geom_vt28(go)`를 쓰고 있어 재사용이 자연스럽지만, 대상 불일치면 A/B에서 cf0 mismatch로 드러난다 → 그때 `local_108` 대응 포인터로 교정.

---

## 변경 2 — knight_ult de40 엔트리게이트

de40 오브젝티브 루프에서 아래 한 줄:
```rust
if rd_u8(desc + 0x131) == 0 { continue; }   // 근사 — knight_ult 특수유닛 오탈락
```
을 다음으로 교체:
```rust
// de40 엔트리게이트 — skip-flag 또는 knight_ult 특수유닛 (§11.9.10.2 B)
let entry_ok = rd_u8(desc + 0x131) != 0 || (
    // mode>0xf: Gate1 도달=phase>=0x1d>0xf라 항상 참(완전성 위해 명시)
    rd_u64(desc).unwrap_or(0) == rd_u64(nexus).unwrap_or(1)
    && (rd_u64(desc).unwrap_or(0) != 0 || rd_u64(desc + 8).unwrap_or(0) == rd_u64(nexus + 8).unwrap_or(1))
    && rd_i64(desc + 0xa0).unwrap_or(0) == 10                       // 이름 길이 == 10
    && {
        let np = rd_u64(desc + 0x98).unwrap_or(0) as usize;         // 이름 str ptr
        ptr_ok(np)
        && rd_u64(np).unwrap_or(0) == 0x755f_7468_6769_6e6b         // "knight_u"
        && (rd_u32(np + 8) & 0xffff) == 0x746c                      // "lt"  → "knight_ult"
    }
);
if !entry_ok { continue; }
```

**확정 사항**
- name은 **해시가 아니라 리터럴 바이트 비교** ("knight_ult", 10바이트). 길이는 `desc+0xa0==10`로 선검사.
- owner-key(`desc+0`/`desc+8`)는 self와 동일여부만 필요(값 의미 불요, 바이트 비교로 충분).
- `mode` = FUN_14209a750 param2. **Gate1은 phase>=0x1d(>0xf)에서만 도달**하므로 `mode>0xf`는 이 문맥에서 항상 참 → 위 코드처럼 생략. 완전성을 원하면 `d19_g1_compflag_pure`에 `phase` 인자를 추가해 `phase > 0xf &&`를 앞에 붙일 것.
- desc `+0xa0`=이름 길이, `+0x98`=str ptr.

---

## 검증 절차 (구현 후)
1. 빌드·배포: `powershell -File C:\tfm2mods\build_inj.ps1 -Src <lib.rs> -ModId tfm2_ai_adjust` (1MB 초과 모드라 rustc 직접 경로일 수 있음 — `/build` 참조). 게임 실행중이면 dll 락 → 종료 요청.
2. A/B 대조: cfg `d19gate1=1`, `d19_g1cf_shadow=0`(순수 활성), `d19_g1cf_cmp=1`(순수 vs shadow 대조), **`coef_mult=100` 필수**.
3. 실제 매치 1판 구동 → `g1cfcmp.txt`에서 cf0/cf1 **순수 vs shadow mismatch(MM)=0** 확인.
4. MM=0이면 순수화 완료 → DONE 승격(§7 record-keeper). MM>0이면 mismatch 케이스로 어느 항 문제인지 특정(우선 `ctx28` 대상, 다음 knight_ult) → 교정 후 재검증.
5. 진단 토글: `d19_g1cf_loop2`(loop2 threat항 격리), 필요시 skillType 항도 격리 토글 추가 권장.

## 판정
잔여 2항 정밀 규명 완료(RNG-free·바이트 재현). 위 구현+A/B(MM=0) 통과 전까지 **DONE 미승격**. 구현은 이 문서 기준으로 통합 진행.
