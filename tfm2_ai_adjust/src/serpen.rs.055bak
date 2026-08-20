// serpen.rs — Serpent(disc12)/오브젝티브 전투 재현 + disc4보조 write.
// 들어있는 것: my_serpen_battle/my_serpen_poke/serpen_* 헬퍼, my_c8c520(다이버 선별)/c8c_*, my_defense_nexus_050,
//   write_serpen_out/write_disc4_aux(itemnet aux write), SERPEN_PHASE3_ALLY 상수.
// 언제 손대나: 세르펜/오브젝티브 전투 판단·다이버 로직 변경 시. RVA는 rva_051.rs.

#[inline] fn serpen_diag(i: usize) { SERPEN_DIAG[i].fetch_add(1, Ordering::Relaxed); }



// ════ disc11(실명 Hide — 구라벨 SerpenPoke, §11.8) 0.5.0 FUN_1422d1870 완전재작성(REWRITE, §12.13). char 반환, RNG-free. ════
//   P1=byte[subp+0x28](=p2+0x20, p2=subp+8) · P3=side(SimState p5+0x820) · l80=*(p6) · vobj=*(p6+8).
//   웨이포인트 슬롯 조회 → wp==0 코드 / wp!=0 시 vt+0x138 candidate-resolve(P1==1만) + type/side_test별 char.
//   반환 char {2,3,4,6,8,9,0xb,0xc,0xd,0xe,0xf,0x10,0x11,0x12,0x14,0x15} / -99=passthrough(가드).
unsafe fn my_serpen_poke(p2: usize, _p3: u64, p5: usize, p6: usize, _p7: usize, _p8: usize) -> i64 {
    let _pg = perf_guard(7);
    let l80 = rd_u64(p6).unwrap_or(0) as usize;              // *P6
    if !ptr_ok(l80) { return -99; }
    let serpen_slot = tune("poke_serpen_slot", 5) as u8;    // ✅[재배선] serpen 웨이포인트 점유 임계
    let side = rd_u64(p5 + 0x930).unwrap_or(2);             // P3 (0.5.0 SimState side, was 0x6a8)  ★0.5.4 오프셋 이동 반영
    if side > 1 { return -99; }
    let p3s = side as usize;
    let p1b = rd_u8(p2 + 0x20);                              // P1 = byte[subp+0x28] = byte[(subp+8)+0x20]
    let (prim_base, fb_base) = match p1b {                   // 카테고리별 웨이포인트 슬롯 베이스
        0 => (0x180usize, 0x190usize),
        1 => (0x1a0, 0x1b0),
        _ => (0x1c0, 0x1d0),
    };
    let mut wp = rd_u64(l80 + prim_base + p3s * 8).unwrap_or(0) as usize;   // primary
    if wp == 0 { wp = rd_u64(l80 + fb_base + p3s * 8).unwrap_or(0) as usize; }   // fallback(primary==0)
    if wp == 0 {
        let (lo, hi): (i64, i64) = match p1b { 0 => (0x02, 0x10), 1 => (0x04, 0x11), _ => (0x09, 0x15) };
        return if p3s == 0 { lo } else { hi };
    }
    // wp != 0
    let ty = rd_i32(wp + 0x68).unwrap_or(0);                 // type
    match p1b {
        0 => {
            if ty != 2 { return -99; }                       // 게임 assert(type==2) → 가드 passthrough
            let cond = rd_u8(wp + 0x128) < serpen_slot && rd_u64(wp + 0x88).unwrap_or(0) != 0;
            if cond { if p3s == 0 { 0x06 } else { 0x03 } } else if p3s == 0 { 0x03 } else { 0x06 }
        }
        1 => {
            // candidate-resolve(vt+0x138, 2단 handle→entity) — P1==1(side_test)서만 필요. 순수재현(dd7_slot128).
            let sim = rd_u64(l80).unwrap_or(0) as usize;
            let vt = rd_u64(l80 + 8).unwrap_or(0) as usize;
            if !ptr_ok(sim) || !ptr_ok(vt) { return -99; }
            let cand = dd7_slot128(sim, rd_u64(p5 + 0x938).unwrap_or(0));   // vt+0x138 2단 resolve, handle=*(p5+0x818) self-handle → 순수재현(shadow-call 제거=AV방지)
            if !ptr_ok(cand) { return -99; }
            let vobj = rd_u64(p6 + 8).unwrap_or(0) as usize;   // *(P6+8)
            let m = rd_u64(vobj + 8).unwrap_or(0) as usize;
            let mapymax = rd_u64(m + 0x12c0).unwrap_or(0);
            let cy = rd_u64(cand + 0x668).unwrap_or(0);
            let cx = rd_u64(cand + 0x660).unwrap_or(0);
            let side_test = mapymax.wrapping_sub(cy) < cx;    // 부호없는 비교
            if ty == 2 && rd_u8(wp + 0x128) >= serpen_slot {
                if side_test { if p3s == 0 { 0x0d } else { 0x12 } } else if p3s == 0 { 0x08 } else { 0x0c }
            } else if side_test { 0x0e } else { 0x0b }        // P3 미사용
        }
        _ => {
            if ty != 2 { return -99; }
            let b2 = rd_u8(wp + 0x128) < serpen_slot && rd_u64(wp + 0x88).unwrap_or(0) != 0;
            let flag = if b2 { p3s == 0 } else { p3s != 0 };
            if flag { 0x14 } else { 0x0f }
        }
    }
}


// (0.4.14 serpen_uvar6 제거: 0.5.0 SerpenPoke는 char 반환·디스패처가 out+0=0xb/out+8=char/out+0x10=1 래핑 — uVar6 aux 없음.)

// ★disc4 full-output writer: [p1]=code. code!=7이면 aux([p1+8]=active(*(p2+0x48)), [p1+0x10]=facet byte(*(p2+0x60)), [p1+0x11]=0). code7=코드만(0x206ee7b 직행, aux 미터치). p2sj=subp(disc4는 rdx직접 사용).
unsafe fn write_disc4_aux(p1: usize, code: i64, p2sj: usize) -> bool {
    if !writable(p1, 0x18) || !ptr_ok(p2sj) || !readable(p2sj + 0x68, 8) { return false; }
    std::ptr::write_unaligned(p1 as *mut u64, code as u64);
    if code != 7 {
        std::ptr::write_unaligned((p1 + 8) as *mut u64, rd_u64(p2sj + 0x48).unwrap_or(0));
        std::ptr::write_unaligned((p1 + 0x10) as *mut u8, rd_u8(p2sj + 0x60));
        std::ptr::write_unaligned((p1 + 0x11) as *mut u8, 0u8);
    }
    true
}

// ★0.5.0 SerpenPoke(disc11) 출력 래핑: 디스패처 0x22e2f88 = out+0=0xb(q) / out+8=char(q) / out+0x10=1(u16) / out+0x12=0(b).
//   char_code = my_serpen_poke 반환(0x02..0x15). <0 이면 passthrough.
unsafe fn write_serpen_out(p1: usize, char_code: i64) -> bool {
    if char_code < 0 || !writable(p1, 0x14) { return false; }
    std::ptr::write_unaligned(p1 as *mut u64, 0xbu64);
    std::ptr::write_unaligned((p1 + 8) as *mut u64, char_code as u64);
    std::ptr::write_unaligned((p1 + 0x10) as *mut u16, 1u16);
    std::ptr::write_unaligned((p1 + 0x12) as *mut u8, 0u8);
    true
}

//   ② phase3 아군임계 (abs 0x143868990): return (enemy==0) && (SERPEN_PHASE3_ALLY[role] <= ally)
const SERPEN_PHASE3_ALLY: [i64; 18] = [4, 2, 1, 2, 1, 3, 1, 4, 4, 3, 2, 1, 2, 1, 3, 1, 3, 3];

// ⑤-b FUN_14211ad10 (RVA 0x211ad10): 적기지밖·거리 술어 bool. ctx=PredCtxHeader(deref됨), cand=엔티티ptr(deref됨).
//   dist²(self,cand)>150000²→false. self.type0면 cand의 k슬롯 점유→false. 적기지박스 판정→reject^1.
//   상수 800000=0xC3500·64001=0xFA01·160001=0x27101·896000=0xDAC00.
unsafe fn serpen_pred_b(ctx: usize, cand: usize) -> bool {
    let selfp = rd_u64(ctx).unwrap_or(0) as usize;                 // *(ctx+0) = self A
    if !ptr_ok(selfp) || !ptr_ok(cand) { return false; }
    let sx = rd_u64(selfp + 0x660).unwrap_or(0);
    let sy = rd_u64(selfp + 0x668).unwrap_or(0);
    let cx = rd_u64(cand + 0x660).unwrap_or(0);
    let cy = rd_u64(cand + 0x668).unwrap_or(0);
    if sqd(sx, sy, cx, cy) > 22500000000u64 { return false; }      // 150000²
    if rd_u64(selfp).unwrap_or(1) == 0 {                            // self.type==0
        let k = rd_u64(selfp + 8).unwrap_or(0) as usize;           // k=self+0x08 (>1 panic가드=재현X)
        if rd_u64(cand + k * 0x18 + 0x38).unwrap_or(0) != 0 { return false; }
    }
    // reject 판정: (*(*(ctx+8))>0xb && cand.type==0 && cand.side==1-side)일 때 적기지박스
    let bptr = rd_u64(ctx + 8).unwrap_or(0) as usize;              // ptr B
    let bval = rd_u64(bptr).unwrap_or(0);                          // *(*(ctx+8))
    let cent = rd_u64(ctx + 0x10).unwrap_or(0) as usize;           // entity C
    let side = rd_u64(cent + 0x930).unwrap_or(2);//  ★0.5.4 오프셋 이동 반영
    let mut reject = false;
    if bval > 0xb && rd_u64(cand).unwrap_or(1) == 0
        && rd_u64(cand + 8).unwrap_or(99) == 1u64.wrapping_sub(side) {
        if side == 1 {   // side1
            reject = (cx < 0xFA01 && cy.wrapping_sub(0xC3500) < 0x27101)
                  || (cx < 0x27101 && cy.wrapping_sub(0xDAC00) < 0xFA01);
        } else {         // side0 = 대칭 x/y스왑
            reject = (cy < 0xFA01 && cx.wrapping_sub(0xC3500) < 0x27101)
                  || (cy < 0x27101 && cx.wrapping_sub(0xDAC00) < 0xFA01);
        }
    }
    !reject
}

// ⑤-e FUN_141d01ac0 (RVA 0x1d01ac0, 0.5.0_2): type3 스킬 ready 술어 bool. RNG-free.
//   스킬리스트 list=*(entity+0x2b0)/count=*(entity+0x2b8) stride0x28, 각 스킬 +0x00=타입태그.
//   type3 진행중이면 0. 없으면 entity+0x68 점프테이블로 클래스별 필드 널체크(idx 0..13).
unsafe fn serpen_skill_type3(entity: usize) -> bool {
    if !ptr_ok(entity) { return false; }
    let list = rd_u64(entity + 0x2b0).unwrap_or(0) as usize;
    let n = (rd_u64(entity + 0x2b8).unwrap_or(0) as usize).min(256);
    if list != 0 {
        for i in 0..n { if rd_i32(list + i * 0x28).unwrap_or(0) == 3 { return false; } }
    }
    let idx = rd_i32(entity + 0x68).unwrap_or(-1);          // 점프테이블 셀렉터
    let field: usize = match idx {
        0 | 3 => return false,
        1 => 0xb8, 2 => 0x110, 4 => 0xf0, 5 | 6 => 0x1f0, 7 => 0xe8,
        8 => 0xb0, 9 => 0xc8, 10 => 0xf0, 11 => 0xd8, 12 => 0xd0, 13 => 0xb0,
        _ => return false,                                  // ⬜idx>13 default 미규명 → 보수적 false
    };
    if list != 0 {   // Loop B: 전 스킬 2<=type<=5 확인
        for i in 0..n {
            let ty = rd_i32(list + i * 0x28).unwrap_or(0);
            if !(2..=5).contains(&ty) { return false; }
        }
    }
    rd_u64(entity + field).unwrap_or(1) == 0
}

// ⑤-d 데미지코어 FUN_1422e85a0 (RVA 0x22e85a0, 0.5.0_3): 상대별 유효뎀 1회(phys/magic). 콜러가 which=0,1 합산. RNG-free.
//   ★getStat=raw memcpy(champion+0x600블록) 확정 → entity+0x610=maxHP·+0x618=defP·+0x620=defM (별개 statA 없음, %maxHP 휴리스틱).
//   coef=atk+0x358: +0xa8 phys관통%·+0xb0 magic관통%·+0xd0 phys배율·+0xd8 self배율·+0xe0 magic배율·+0xf0 amp.
//   base=어빌리티 vtable(*(atk+0x480)+0x28) 산출 스킬원본뎀(콜러 제공). dmgtype=*(atk+0x4a4). which 0=phys/1=magic.
unsafe fn serpen_dmg_core(atk: usize, tgt: usize, base_in: i64, dmgtype: i32, which: u32) -> i64 {
    if !ptr_ok(atk) || !ptr_ok(tgt) { return 0; }
    let coef = atk + 0x358;
    let atk_maxhp = rd_i64(atk + 0x628).unwrap_or(0);
    let tgt_maxhp = rd_i64(tgt + 0x628).unwrap_or(0);
    let mut base = base_in;
    let out = if dmgtype == 2 || dmgtype == 3 {
        (rd_i64(coef + 0xf0).unwrap_or(0) + 100) * base / 100                      // amp/true
    } else {
        let mult_t = if dmgtype == 0 { rd_i64(coef + 0xd0).unwrap_or(0) } else { rd_i64(coef + 0xe0).unwrap_or(0) };
        base += mult_t * tgt_maxhp / 100;
        rd_i64(coef + 0xd8).unwrap_or(0) * atk_maxhp / 100 + base
    };
    let pen = if which == 0 { rd_i64(coef + 0xa8).unwrap_or(0) } else { rd_i64(coef + 0xb0).unwrap_or(0) };
    let def = if which == 0 { rd_i64(tgt + 0x618).unwrap_or(0) } else { rd_i64(tgt + 0x620).unwrap_or(0) };
    let red = if pen < 101 { 100 - pen } else { 0 };
    let eff = ((red * def) >> 2) / 25;    // = red*def/100 (⚠asm 2-step 정확형은 harness 검증)
    let res = out * 100 / (eff + 100);
    res + if res == 0 { 1 } else { 0 }    // 최소1
}

// FUN_142076fb0: side별 5-슬롯 non-null 카운트(role7 그리드셀). g0=geom[0]. RNG-free. (확정)
// ★★[07-23 전면 재작성, 0.5.2 `0x14238f130` 전수 disasm 근거 = ANA\disc12-epiccheck-tail-spec.md §B count5]
//   ~~구현: 5칸 중 non-null 개수만 셈(`role==7` 체크가 **아예 없었음**)~~ → 원본 `count5`는 3중 필터가 붙는다.
//   슬롯 열거 자체(`g0+0x1e0+sd*0x28+i*8`, i<5, non-null)는 원본 `FUN_1422c8c80`과 일치했고 **필터만 통째 누락**이었다.
//   ⚠★`vis_side`는 **루프 sd가 아니라 side 고정**(원본 `@14238f81b`/`@14238f971` 둘 다 `MOV RDX,[RBP+0x70]`=side).
//     lastSeen 테이블 베이스만 루프 sd(`@f760` `IMUL RCX,RBX(1-side),0x2e8` / `@f8ab` `IMUL RAX,RDI(side),0x2e8`).
//     ⟹ 인자를 vis_side/sd 둘로 분리했다. 하나로 합치면 조용히 틀린다.
//   vt 매핑(전부 순수재현, shadow-call 없음): `vt[0xd0]`=geom_vt68(가시성, 반환극성 = 필드==0이 true) ·
//     `vt[0x128]`=geom_vtc0(시야레코드) · `vt[0x28]`=dd7_slot20(현재 tick, `gc+0xeac0`). 필드 오프셋 0.5.2 유효 확정(07-23).
unsafe fn serpen_role7_count(g0: usize, gchild: usize, ctx2: usize, geom: usize, vis_side: usize, sd: usize) -> u64 {
    if !ptr_ok(g0) || !ptr_ok(gchild) || !ptr_ok(ctx2) || sd > 1 || vis_side > 1 { return 0; }
    let mapdef = rd_u64(ctx2 + 0x20).unwrap_or(0) as usize;      // mapdef = *(ctx2+0x20)
    if !ptr_ok(mapdef) { return 0; }
    let role_tbl = mapdef + 0x38b8;                              // 30×30 u64, row stride 0xf0
    let ls_base = rd_u64(geom + 0x10).unwrap_or(0) as usize;     // p6[2] = lastSeen base
    let now = dd7_slot20(gchild);                                // vt[0x28] = 현재 tick
    let mut n = 0u64;
    for i in 0..5usize {
        let u = rd_u64(g0 + 0x1e0 + sd * 0x28 + i * 8).unwrap_or(0) as usize;
        if u == 0 { continue; }
        // 셀 인덱스: 좌표/32000 후 29 클램프 (원본 magic 0x20c49ba5e353f7d = /32000)
        let yi = (rd_u64(u + 0x668).unwrap_or(0) / 32000).min(29) as usize;
        let xi = (rd_u64(u + 0x660).unwrap_or(0) / 32000).min(29) as usize;
        let role = rd_u64(role_tbl + yi * 0xf0 + xi * 8).unwrap_or(0);
        if role != 7 { continue; }                               // ★원본은 `n += (role==7) & ok`(양쪽 평가)이나 카운트 결과는 동일
        let id = rd_u64(u + 0x5c0).unwrap_or(0);
        let ok = if geom_vt68(gchild, vis_side, id) { true }      // vt[0xd0] != 0 → 즉시 통과
        else {
            let look = geom_vtc0(gchild, id);                    // vt[0x128] = 시야레코드
            if look == 0 { false }
            else {
                let li = rd_i32(look + 0x9c0).unwrap_or(0) as usize;   // lane idx  ★0.5.4 오프셋 이동 반영
                if !ptr_ok(ls_base) { false }
                else { rd_i64(ls_base + sd * 0x2e8 + 0x1e0 + li * 8).unwrap_or(0) + 0x78 >= now }
            }
        };
        if ok { n += 1; }
    }
    n
}

// ★FUN_141fe2500 충실재현 v2(07-10 정밀RE, ghidra 확정). Option discriminant(0/1)만 반환(disc12 콜러가 판별값만 씀).
//   구모델 버그 3건 정정: ①plan측 Vec = *(ctx+0x30)이 헤더 "포인터"(인라인으로 오독→항상 None이던 원인) ②local_50 기본 0(-1 아님, tier≤3 필터 max)
//   ③첫 이름일치 j에서 조건 통과/실패 불문 break. getter vt 구현체=raw 필드(0x1409be9c0=self+0 name/0x14226f070=+0x188 tier/0x14226f080=+0x180 price/0x1409bea90=self+0x30 연관이름Vec) → raw read 재현(shadow-call 불요).
//   owned 원소=카탈로그와 동일 타입(buy가 카탈로그 pair 딥클론 push, vtA1-4 0x1438a1d18../vtB 0x1438af4a0 공유).
//   ⚠라이브 대체 시: 후보 n>0이면 Lemire 균등픽 RNG 2+워드 소비 재현 필요(검증경로=discriminant만이라 생략).
unsafe fn serpen_rng_pick(rng: usize, athlete: usize, plan: usize, live: bool) -> i64 {
    let obase = rd_u64(athlete + 0x4b0).unwrap_or(0) as usize;   // owned Vec{cap@0x448,ptr@0x450,len@0x458}, 원소 {data,vt} stride 0x10
    let ocnt = rd_u64(athlete + 0x4b8).unwrap_or(0) as usize;
    // ★★[07-23] 임의 상한 완화(~~`ocnt > 256`~~ → `> 0x10000`). 원본 `0x2135350`엔 대응 상한이 **없다**.
    //   걸리면 n이 과소 산출되어 **Lemire draw 수가 어긋남 = RNG desync**(live 대체 시 치명). 쓰레기 포인터 방어용으로만 크게 남긴다.
    if obase == 0 || ocnt == 0 || ocnt > 0x10000 { return 0; }
    let gold = rd_u64(athlete + 0x9a8).unwrap_or(0);
    // local_50 = max{tier : tier<=3} over owned, 없으면 0 (tier=*(data+0x188), unsigned)
    let mut local50: u64 = 0;
    for i in 0..ocnt {
        let data = rd_u64(obase + i * 0x10).unwrap_or(0) as usize;
        if data == 0 { continue; }
        let t = rd_u64(data + 0x188).unwrap_or(0);
        if t <= 3 && t > local50 { local50 = t; }
    }
    // plan측 = *(ctx+0x30) → Vec 헤더 {cap@0, ptr@8, len@0x10}, 원소={data,vt} stride 0x10 (머지된 아이템 카탈로그)
    let hdr = rd_u64(plan + 0x30).unwrap_or(0) as usize;
    if !ptr_ok(hdr) { return 0; }
    let pbase = rd_u64(hdr + 8).unwrap_or(0) as usize;
    let pcnt = rd_u64(hdr + 0x10).unwrap_or(0) as usize;
    if pbase == 0 || pcnt == 0 || pcnt > 0x10000 { return 0; }   // ★07-23 상한 완화(원본 무상한, 위 사유와 동일)
    // ★fe2500 후보 수집(RNG-free): (owned,연관이름) 쌍 중 첫 카탈로그 이름일치가 (tier>local50 && price<=gold)면 후보 1개. n=후보수(=게임 Vec len).
    let mut n: u64 = 0;
    for i in 0..ocnt {
        let data = rd_u64(obase + i * 0x10).unwrap_or(0) as usize;
        if data == 0 { continue; }
        // 연관이름 Vec<String> 인라인 @data+0x30: {cap@+0x30, ptr@+0x38, len@+0x40}; String stride 0x18 = {cap@0, ptr@8, len@0x10}
        let sptr = rd_u64(data + 0x38).unwrap_or(0) as usize;
        let slen = rd_u64(data + 0x40).unwrap_or(0) as usize;
        if sptr == 0 || slen == 0 || slen > 0x10000 { continue; }   // ★07-23 상한 완화(원본 무상한)
        for k in 0..slen {
            let s = sptr + k * 0x18;
            let s_p = rd_u64(s + 8).unwrap_or(0) as usize;
            let s_l = rd_u64(s + 0x10).unwrap_or(0) as usize;
            if s_p == 0 || s_l == 0 || s_l > 0x10000 { continue; }   // ★07-23 상한 완화(원본 무상한)
            for j in 0..pcnt {
                let pd = rd_u64(pbase + j * 0x10).unwrap_or(0) as usize;
                if pd == 0 { continue; }
                if rd_u64(pd + 0x10).unwrap_or(0) as usize != s_l { continue; }   // name String 인라인 @pd+0: ptr@+8, len@+0x10
                let pn = rd_u64(pd + 8).unwrap_or(0) as usize;
                if pn == 0 || !mem_eq(pn, s_p, s_l) { continue; }
                // ★★[07-23 신설] **(c) 가용 게이트 `vt[0x50]`** — 모드에 **없던 조건**(원본 `@0x142135562` `TEST AL,AL / JZ`).
                //   누락 시 후보가 **과다 계상** → n 상이 → Lemire draw 수 상이 = **RNG desync 직접원인**.
                //   ⚠실패 시 **`break`**(다음 "이름"으로) — `continue`면 같은 이름에 대해 카탈로그를 계속 뒤져 또 과다계상된다.
                //   구현: vtable 주소 하드코딩(패치마다 썩음) 대신 **슬롯 타겟 첫 바이트 분류**(shadow-call 없음·버전 강건).
                //     0.5.2 실측 = `0x1405d17d0`(vtA1-4) → `B0 01 C3`(mov al,1) = 상수 true
                //                / `0x142145970`(vtB)    → `48 83 B9 90 01 00 00 00 / 0F 95 C0 / C3` = `[self+0x190] != 0`(disp32 @f50+3)
                let pvt = rd_u64(pbase + j * 0x10 + 8).unwrap_or(0) as usize;   // 원소 = {data, vt}
                let avail = if !ptr_ok(pvt) { true } else {
                    let f50 = rd_u64(pvt + 0x50).unwrap_or(0) as usize;
                    if !ptr_ok(f50) { true } else {
                        match rd_u8(f50) {
                            0xB0 => true,                                                    // mov al,1; ret
                            0x48 => rd_u64(pd + rd_u32(f50 + 3) as usize).unwrap_or(0) != 0, // cmp qword[rcx+disp32],0; setne al
                            _ => true,                                                       // 미지 impl = 보수적 통과
                        }
                    }
                };
                if !avail { break; }
                // ★첫 이름일치 j: 조건 평가 후 통과/실패 불문 break (게임 semantics)
                let t = rd_u64(pd + 0x188).unwrap_or(0);
                let p = rd_u64(pd + 0x180).unwrap_or(u64::MAX);
                if t > local50 && p <= gold { n += 1; }   // 후보 1개 push (게임 Vec append)
                break;
            }
        }
    }
    // ★live 대체 경로에서만 fe2500 RNG 소비 재현: n>0이면 Lemire gen_range(0,n-1) 1회 → 게임 RNG 스트림 전진(write-back).
    //   결과 idx는 미사용(tag=후보 존재여부만) — 목적은 스트림 동기화. 검증(리턴훅, live=false)은 게임이 이미 소비했으므로 무드로우.
    //   write-back = dd7700(my_dd7700_rng_final) 패턴 동일: refills>0시 buf 64워드+counter(+0x130), idx(+0x100). wr_*는 폴트세이프.
    if live && n > 0 {
        if let Some(mut r) = RngSim::new(rng) {
            if r.gen_range(0, (n - 1) as u64).is_some() {
                let refills = r.refills;
                let mut ok = true;
                if refills > 0 {
                    for i in 0..64usize { if !wr_u32(rng + i * 4, r.buf[i]) { ok = false; break; } }
                    if ok { let c0 = rd_u64(rng + 0x130).unwrap_or(0); ok = wr_u64(rng + 0x130, c0.wrapping_add(4u64.wrapping_mul(refills))); }
                }
                if ok { let _ = wr_u64(rng + 0x100, r.idx); }
            }
        }
    }
    (n != 0) as i64
}

// engage 게이트 valid_target 술어: HP%>=40 + 위치게이트(p6=4/5). mapobj=ctx[1]=*(geom[1]+8).
unsafe fn serpen_valid_target(e: usize, mapobj: usize, p6: u8) -> bool {
    let maxhp = rd_u64(e + 0x628).unwrap_or(0);
    if maxhp == 0 || rd_u64(e + 0x670).unwrap_or(0).wrapping_mul(100) / maxhp < (tune("ec_valid_hp", 0x28) as u64) { return false; }
    let x = rd_u64(e + 0x660).unwrap_or(0);
    let y = rd_u64(e + 0x668).unwrap_or(0);
    let mapy = rd_u64(mapobj + 0x12c0).unwrap_or(0);
    let mapx = rd_u64(mapobj + 0x12b8).unwrap_or(0);
    if p6 == 4 {
        if x <= 0x2ee00 { return true; }
        let d = mapy.wrapping_sub(y);
        if x <= d { return true; }
        if d >= mapx.wrapping_sub(0x2ee00) { return true; }
        x.wrapping_sub(d) < 64000
    } else {   // p6=5 대칭(x↔v 스왑)
        let v = mapy.wrapping_sub(y);
        if v <= 0x2ee00 { return true; }
        if v <= x { return true; }
        if x >= mapx.wrapping_sub(0x2ee00) { return true; }
        v.wrapping_sub(x) < 64000
    }
}

// engage 게이트 FUN_141fdc2a0(=구 FUN_141d1f7d0): true=교전보류(0xc)/false=진행(dist기반 0xe/0xc). RNG-free. (완전 재구현, §12.16 ⑩+tail byte-exact 07-10)
//   판별 early게이트(tick/sf/count<REQUIRED/valid/앵커/vt0x138/슬롯스캔+최근접/out.idx==sim+0x8b0)=정확.
//   tail(ghidra-re 확정 스펙): ①fe8f30 lane-order 스캐너(후보≠-1→보류) ②fe87c0 committed-order 타겟(≠0→보류)
//   ③타이밍 데드라인(now>deadline+2U→보류) ④앵커셀 도달성(vt0x70==0→보류) ⑤f872f0 MIA 매복위협(1→보류). 전부 통과=false(진행).
unsafe fn serpen_engage_gate(sf: usize, tick: i64, sim: usize, geom: usize, tp: usize, p6: u8) -> bool {
    sgt(6, 0);   // ★SGT stage: 진입
    // ★★[07-23] ~~`if tick < tune("ec_gate_tick", 0x18) { return false; }`~~ **게이트 삭제**(0.5.2 `0x2117ae0` 실disasm).
    //   0.5.2 원본은 `param_2`(=tick)를 **본문에서 한 번도 참조하지 않는다** — 프롤로그 `@117b1b MOVZX EDX,[RCX+0x3f7]`에서
    //   RDX가 즉시 파괴되고 재사용 없음. ⟹ `ec_gate_tick` = **死레버**(설정편집기 제거 대상).
    //   영향: 매치 초반 `tick<0x18` 구간에서 game=0xc/0xe vs mine=0xe 불일치가 나던 원인.
    //   ⚠이 함수는 disc12(p6=4)뿐 아니라 **라이브 disc14(p6=5, my_defense_nexus_050 L1043)** 도 쓴다 ⟹ disc14 재검증 필요.
    let _ = tick;   // 0.5.2 원본 미사용(시그니처는 호출부 호환 위해 유지)
    let sf_ok = if p6 == 4 { rd_u8(sf + 0x3f6) == 0 && rd_u8(sf + 0x3f7) == 1 } else { rd_u8(sf + 0x3f6) == 1 && rd_u8(sf + 0x3f7) == 1 };
    if !sf_ok { sgt(6, 2); return false; }   // p6=4/5 sf플래그
    let g0 = rd_u64(geom).unwrap_or(0) as usize;        // planptr=geom[0](roster+0x1e0, vt=+8)
    let ctx = rd_u64(geom + 8).unwrap_or(0) as usize;   // ctx=geom[1](+0x38 lane-id, +0x20 앵커컨테이너)
    if !ptr_ok(g0) || !ptr_ok(ctx) { sgt(6, 3); return false; }
    let side = rd_u64(sim + 0x930).unwrap_or(2);//  ★0.5.4 오프셋 이동 반영
    if side > 1 { sgt(6, 4); return false; }
    let s = side as usize;
    let mapobj = rd_u64(ctx + 8).unwrap_or(0) as usize;
    if !ptr_ok(mapobj) { sgt(6, 5); return false; }
    // ⬜fe8f30 게이트 = default -1(pass)
    let lane_id = rd_u8(ctx + 0x38) as usize;   // count<REQUIRED[lane-id] 게이트
    let mut cnt = 0i64;
    for i in 0..5usize {
        let e = rd_u64(g0 + 0x1e0 + s * 0x28 + i * 8).unwrap_or(0) as usize;
        if e != 0 && serpen_valid_target(e, mapobj, p6) { cnt += 1; }
    }
    // ★[07-23] REQUIRED 유효범위 정정: 0.5.2 `DAT_1438acf40` 실측 = `[2,2,1,2,1,2,1,2,2]`(idx 0..8)뿐이고
    //   idx9=0·10=2·11=3·**12+ = 무관 데이터**(원본은 bound-check 없이 인덱싱). 모드 테이블의 9~17 값은 0.5.2와 다르다.
    //   ⟹ 유효범위를 idx<9로 좁힌다(그 이상은 도달 불가로 간주 = 기존 `<18` 가드보다 정확).
    if cnt < if lane_id < 9 { SERPEN_REQUIRED[lane_id] } else { 0 } { sgt(6, 6); sgt(7, cnt); sgt(8, lane_id as i64); return false; }
    let (refx, refy) = serpen_anchor_pickup(ctx, s, p6);
    let gchild = rd_u64(g0).unwrap_or(0) as usize;
    let gvt = rd_u64(g0 + 8).unwrap_or(0) as usize;
    let w = rd_u64(sim + 0x938).unwrap_or(0) as usize;
    if !ptr_ok(gchild) || !ptr_ok(gvt) { sgt(6, 7); return false; }   // ★07-10 sgate 진단: ptr_ok(w) 제거 — w=*(sim+0x818)은 핸들(작은 정수)이라 ptr_ok 상시 실패→게이트 상시 false→라이브매치 game(0xc)≠my(0xe). 핸들 검증=dd7_slot128 내부(범위+in-use)
    if dd7_slot128(gchild, w as u64) == 0 { sgt(6, 8); return false; }   // vt0x138 resolve 게이트 → 순수재현(2-stage=dd7_slot128 비트동일)
    // 슬롯스캔+최근접: argmin over valid slots of isqrt(anchor→E)+LANE_BIAS[idx] (포화)
    let (mut best_score, mut best_idx, mut best_ent) = (u64::MAX, -1i64, 0usize);
    for i in 0..5usize {
        let e = rd_u64(g0 + 0x1e0 + s * 0x28 + i * 8).unwrap_or(0) as usize;
        if e == 0 || !serpen_valid_target(e, mapobj, p6) { continue; }
        let d = isqrt(sqd(refx, refy, rd_u64(e + 0x660).unwrap_or(0), rd_u64(e + 0x668).unwrap_or(0)));
        let score = d.saturating_add(SERPEN_LANE_BIAS[i] as u64);
        if score < best_score { best_score = score; best_idx = i as i64; best_ent = e; }
    }
    if best_ent == 0 || best_idx != rd_u32(sim + 0x9c0) as i64 { sgt(6, 9); sgt(7, best_idx); sgt(8, rd_u32(sim + 0x9c0) as i64); return false; }   // out.idx==sim lane  ★0.5.4 오프셋 이동 반영
    // ═══ tail byte-exact (07-10 ghidra-re 확정 스펙). RNG-free. ═══
    let geom2 = rd_u64(geom + 0x10).unwrap_or(0) as usize;
    // ★★[07-23 오류 수정] ~~`rd_u64(ctx + 0x12f8)`~~ → **`rd_u64(mapobj + 0x12f8)`**. **객체가 달랐다**(ctx ≠ mapobj).
    //   0.5.2 원본은 두 곳에서 동일하게 `*( *(ctx+8) + 0x12f8 )`를 읽는다: `@142118224-14211822c` + `FUN_1423b5f70` 첫 줄.
    //   `mapobj`(= `*(ctx+8)`)는 L352에 이미 있으므로 그대로 사용. UNIT은 ②(committed-order `tp <= UNIT*20`)와
    //   ③(데드라인 `sf[..] + 2*UNIT`) **양쪽에 들어가므로 영향이 크다**. (0.5.0_3에서도 틀렸을 가능성 — ⬜별도 확인 대상.)
    let unit = rd_u64(mapobj + 0x12f8).unwrap_or(0);   // UNIT = *(mapobj+0x12f8)
    // ① fe8f30 lane-order 스캐너: ★07-10 ghidra 확정 — 후보(≠-1)면 즉시 return 0(진행), -1이어야 체인 계속.
    //    (구 "후보→보류(true)" 해석은 반대 — sgate stage11 게임=0xe 9건으로 발각, 디컴 `if(cVar7!=-1) return 0` 확증.)
    //    실물 순서는 fe8f30이 cnt/argmin 조기게이트보다 앞이나 read-only+실패경로 동일(0)이라 출력 비트동일 — 현 위치 유지.
    //    L6 = g0+side*8 → cnt=*(L6+0x2170+k*0x10): commit_gate의 rd_u64(g0+0x2170..+s*8)과 동일 컨테이너(순서무관 합산=정합).
    let pat: [u8; 2] = if p6 == 4 { [0x00, 0x01] } else { [0x02, 0x01] };
    let l6 = g0 + s * 8;             // L6 = geom0 + side*8
    let l8 = geom2 + s * 0x2e8;      // L8
    let mut best_p: i64 = -1;
    let mut best_lane_score = i64::MAX;
    for &p in pat.iter() {
        // eligibility: lane_id에 따라 (p,k) 매핑
        let k: i64 = match lane_id {
            0 | 1 | 3 | 5 | 7 | 8 => match p {
                0 => if matches!(lane_id, 0 | 2 | 7 | 8) { 0 } else { -1 },
                1 => if matches!(lane_id, 0 | 4 | 5 | 7 | 8) { 1 } else { -1 },
                _ => 2,
            },
            2 => if p == 0 { 0 } else { -1 },
            4 => if p == 1 { 1 } else { -1 },
            _ => -1,   // lane 6: 후보 없음
        };
        if k < 0 { continue; }
        let ocnt = rd_u64(l6 + 0x2170 + (k as usize) * 0x10).unwrap_or(0);
        let prog = rd_u64(l8 + (k as usize) * 0x28 + 0x10).unwrap_or(0);
        let num = rd_i32(l8 + (k as usize) * 0x28 + 0x20).unwrap_or(0) as i64;
        if ocnt >= 3 && prog >= 2000 && num >= 2 { continue; }   // skip = 후보 아님
        let score = num * 1000 + (ocnt as i64) * 10000 + prog as i64;
        if score < best_lane_score { best_lane_score = score; best_p = p as i64; }
    }
    if best_p != -1 { sgt(6, 11); return false; }   // lane-order 후보 존재 → return 0(진행) [ghidra 확증]
    // ② fe87c0 committed-order 타겟 체크: !=0 → 보류
    let lane_ok = if p6 == 4 { lane_id == 0 || lane_id >= 7 } else { lane_id < 9 && ((0x1a1u32 >> lane_id) & 1) != 0 };
    let mut c87 = false;
    if lane_ok {
        let now = rd_i64(gchild + 0xec68).unwrap_or(0) as u64;
        // ★★[07-23 회귀 롤백] ~~`let w87 = geom;`(같은 날 "정정"으로 넣었던 것)~~ → **`gchild + 0xeaf0` 복원 = 원래가 맞았다**.
        //   오판 경위: Ghidra 디컴이 `FUN_1423b5f70`을 `param_2[0x34]/[0x35]` 기준으로 표시해 "param_4(geom) 기준"으로 읽었으나,
        //   **실제 asm은 `@1423b5fe5 CALL [RBP+0x30]` 직후의 RDX**(= `vt+0x30`의 2번째 반환값 = `gchild+0xeaf0`)를 쓴다.
        //   `geom`은 그 콜 이전에 RDX에 있던 값이라 디컴파일러가 **콜 클로버를 놓친** 전형적 케이스(감사 07-23, 0.5.2 실disasm).
        //   ⟹ base는 dd7700 STAGE6·disc12 W큐와 **동일한 `gc+0xeaf0` 계열**로 통일된다(그쪽은 오늘 정방향 교정 완료).
        //   근거 주소: `@1423b5ff1 CMP [RDX+0x1d8],0` / `@1423b6046 CMP [RDX+0x1a8],0`.
        let w87 = gchild + 0xec98;
        let (l_off, h_off): (usize, usize) = if p6 == 4 { (0x1a8, 0x1a0) } else { (0x1d8, 0x1d0) };
        if rd_u64(w87 + l_off).unwrap_or(0) != 0 {
            let hp0 = rd_u64(w87 + h_off).unwrap_or(0) as usize;
            let tgt = if ptr_ok(hp0) { geom_resolve150(gchild, rd_u64(hp0).unwrap_or(0)) } else { 0 };
            if ptr_ok(tgt) && geom_vt68(gchild, s, rd_u64(tgt + 0x5c0).unwrap_or(0)) {
                let p78 = rd_u64(sf + if p6 == 4 { 0x80 } else { 0x88 }).unwrap_or(0);
                if now <= p78.wrapping_add(unit) {
                    let full = rd_u64(tgt + 0x670).unwrap_or(0) >= rd_u64(tgt + 0x628).unwrap_or(1);
                    if !full {
                        let p56 = rd_u64(tp + if p6 == 4 { 0x88 } else { 0xc0 }).unwrap_or(u64::MAX);
                        c87 = p56 <= unit.wrapping_mul(20);
                    }
                }
            }
        }
    }
    if c87 { sgt(6, 12); return true; }
    // ③ 타이밍 데드라인: now > deadline + 2*UNIT → 보류
    let now = rd_i64(gchild + 0xec68).unwrap_or(0) as u64;
    let deadline = rd_u64(sf + if p6 == 4 { 0x80 } else { 0x88 }).unwrap_or(0);
    if now > deadline.wrapping_add(unit.wrapping_mul(2)) { sgt(6, 13); return true; }
    // ④ 앵커 셀 도달성: vt0x70==0 → 보류
    if !geom_vt70(gchild, s, (refx / 32000) as usize, (refy / 32000) as usize) { sgt(6, 14); return true; }
    // ⑤ f872f0 MIA 매복 위협: 1 → 보류
    let eside = 1 - s;
    for idx in 0..5usize {
        let e = rd_u64(g0 + eside * 0x28 + 0x1e0 + idx * 8).unwrap_or(0) as usize;
        if e == 0 { continue; }
        let maxhp = rd_u64(e + 0x628).unwrap_or(0);
        if maxhp == 0 || rd_u64(e + 0x670).unwrap_or(0).wrapping_mul(100) / maxhp <= 0x31 { continue; }
        let id = rd_u64(e + 0x5c0).unwrap_or(0);
        if geom_vt68(gchild, s, id) { continue; }                 // now-visible = 매복 아님
        let rec = geom_vtc0(gchild, id);
        if rec != 0 {
            let last = rd_u64(geom2 + eside * 0x2e8 + 0x1e0 + (rd_u32(rec + 0x9c0) as usize) * 8).unwrap_or(0);//  ★0.5.4 오프셋 이동 반영
            if last.wrapping_add(0x78) >= now { continue; }       // 최근 목격(<=120틱) = 매복 아님
        }
        let lx = rd_i64(sf + 0x218 + idx * 0x10).unwrap_or(0);
        let ly = rd_i64(sf + 0x220 + idx * 0x10).unwrap_or(0);
        let d = isqrt(sqd(refx, refy, lx as u64, ly as u64));
        let gap = d.saturating_sub(150000);
        let seen_t = rd_u64(sf + 0x2b8 + idx * 8).unwrap_or(0);
        let elapsed = now.saturating_sub(seen_t);
        if gap <= elapsed.saturating_mul(rd_u64(e + 0x640).unwrap_or(0)) { sgt(6, 15); return true; }   // 도달가능 MIA 위협 → 보류
    }
    sgt(6, 10);
    false   // 전 게이트 통과 = 진행(본체서 dist² 판정→0xe/0xc)
}

// reposition/engage 공용 앵커픽업: plan+0x20 컨테이너(base@+0x68/cnt@+0x70 stride0x28), entry+0x20==tag면 side별 좌표(+0/+8 vs +0x10/+0x18). 미매칭=(0,0).
unsafe fn serpen_anchor_pickup(plan: usize, side: usize, tag: u8) -> (u64, u64) {
    let container = rd_u64(plan + 0x20).unwrap_or(0) as usize;
    if !ptr_ok(container) { return (0, 0); }
    let base = rd_u64(container + 0x68).unwrap_or(0) as usize;
    let cnt = rd_u64(container + 0x70).unwrap_or(0) as usize;
    if base == 0 || cnt > 4096 { return (0, 0); }
    for i in 0..cnt {
        let e = base + i * 0x28;
        if rd_u8(e + 0x20) == tag {
            let co = side * 0x10;
            return (rd_u64(e + co).unwrap_or(0), rd_u64(e + co + 8).unwrap_or(0));
        }
    }
    (0, 0)
}

// FUN_141fe85b0: 반경내 아군 카운트(side=sim+0x820, base=geom[0], 5슬롯 HP%>=hpthr & dist²<=R²). vt필터無.
unsafe fn serpen_count_allies_at(sim: usize, g0: usize, ax: u64, ay: u64, r: u64, hpthr: u64) -> i64 {
    let side = rd_u64(sim + 0x930).unwrap_or(2);//  ★0.5.4 오프셋 이동 반영
    if side > 1 || !ptr_ok(g0) { return 0; }
    let r2 = r.wrapping_mul(r);
    let mut c = 0i64;
    for i in 0..5usize {
        let e = rd_u64(g0 + 0x1e0 + (side as usize) * 0x28 + i * 8).unwrap_or(0) as usize;
        if e == 0 { continue; }
        let maxhp = rd_u64(e + 0x628).unwrap_or(0);
        if maxhp == 0 || rd_u64(e + 0x670).unwrap_or(0).wrapping_mul(100) / maxhp < hpthr { continue; }
        if sqd(ax, ay, rd_u64(e + 0x660).unwrap_or(0), rd_u64(e + 0x668).unwrap_or(0)) <= r2 { c += 1; }
    }
    c
}

// FUN_141fe9220: 반경내 적 카운트 + 시야필터(visible-now OR last-seen<=120틱, [[tfm2-vision-fog-in-ai]]). eside=1-side.
//   ⚠vt0xc0(시야lookup)/vt0x68(now-visible)/vt0x28(curtick) shadow-call(SERPEN_VERIFY 게이트내 실행).
unsafe fn serpen_count_enemies_at(sim: usize, geom: usize, ax: u64, ay: u64, r: u64, hpthr: u64) -> i64 {
    let side = rd_u64(sim + 0x930).unwrap_or(2);//  ★0.5.4 오프셋 이동 반영
    if side > 1 { return 0; }
    let eside = 1 - side as usize;
    let g0 = rd_u64(geom).unwrap_or(0) as usize;
    let g2 = rd_u64(geom + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(g0) { return 0; }
    let this = rd_u64(g0).unwrap_or(0) as usize;
    let vt = rd_u64(g0 + 8).unwrap_or(0) as usize;
    if !ptr_ok(this) || !ptr_ok(vt) { return 0; }
    let r2 = r.wrapping_mul(r);
    let mut c = 0i64;
    for i in 0..5usize {
        let e = rd_u64(g0 + 0x1e0 + eside * 0x28 + i * 8).unwrap_or(0) as usize;
        if e == 0 { continue; }
        let maxhp = rd_u64(e + 0x628).unwrap_or(0);
        if maxhp == 0 || rd_u64(e + 0x670).unwrap_or(0).wrapping_mul(100) / maxhp < hpthr { continue; }
        if sqd(ax, ay, rd_u64(e + 0x660).unwrap_or(0), rd_u64(e + 0x668).unwrap_or(0)) > r2 { continue; }
        let key = rd_u64(e + 0x5c0).unwrap_or(0) as usize;
        if geom_vt68(this, side as usize, key as u64) { c += 1; continue; }          // now-visible → 순수재현(vt0x68)
        let vis = geom_vtc0(this, key as u64);                                        // 시야레코드 → 순수재현(vt0xc0)
        if vis == 0 || !ptr_ok(g2) { continue; }                                      // 한번도 못봄
        let laneidx = rd_u32(vis + 0x9c0) as usize;//  ★0.5.4 오프셋 이동 반영
        let last_seen = rd_u64(g2 + eside * 0x2e8 + 0x1e0 + laneidx * 8).unwrap_or(0);
        if (rd_i64(this+ 0xec68).unwrap_or(0) as u64) <= last_seen.wrapping_add(tune("ec_vision_ticks", 0x78) as u64) { c += 1; }   // last-seen<=120틱, tick=this+0xeac0 직접읽기(vt0x28)
    }
    c
}

// FUN_141fe8b90(sim,geom,4) commit/throttle 게이트: 밴드내·앵커먼 아군 n>=1 && (팀 시도카운터/타이머 게이트).
unsafe fn serpen_commit_gate(sim: usize, geom: usize, plan: usize, mode: u8, ax: u64, ay: u64) -> bool {
    let side = rd_u64(sim + 0x930).unwrap_or(2);//  ★0.5.4 오프셋 이동 반영
    if side > 1 { return false; }
    let s = side as usize;
    let g0 = rd_u64(geom).unwrap_or(0) as usize;
    let g2 = rd_u64(geom + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(g0) { return false; }
    let mut n = 0i64;
    for i in 0..5usize {
        let e = rd_u64(g0 + 0x1e0 + s * 0x28 + i * 8).unwrap_or(0) as usize;
        if e == 0 { continue; }
        let maxhp = rd_u64(e + 0x628).unwrap_or(0);
        if maxhp == 0 || rd_u64(e + 0x670).unwrap_or(0).wrapping_mul(100) / maxhp < (tune("ec_commit_hp", 0x28) as u64) { continue; }
        let (ex, ey) = (rd_u64(e + 0x660).unwrap_or(0), rd_u64(e + 0x668).unwrap_or(0));
        if poke_f6f720(plan, ex, ey, mode) && (sqd(ax, ay, ex, ey) >> 8) > 0xe8d4a50 { n += 1; }
    }
    if n == 0 { return false; }
    let (g0off, tsub): (usize, usize) = match mode { 0 => (0x2170, 0x00), 1 => (0x2180, 0x28), _ => (0x2190, 0x50) };
    if rd_u64(g0 + g0off + s * 8).unwrap_or(0) > 1 { return true; }
    if !ptr_ok(g2) { return false; }
    let tbase = g2 + s * 0x2e8 + tsub;
    let t10 = rd_i64(tbase + 0x10).unwrap_or(0);
    if t10 > 2999 { return true; }
    let cnt = rd_i32(tbase + 0x20).unwrap_or(0) as i64;
    if t10 > 999 { cnt > 1 } else { cnt > 3 }
}

// reposition FIGHT 체크 FUN_141fe8980(tick,sim,geom,4): true=FIGHT(2/3)/false=0xd. RNG-free. sf+0x3eb==3서만. (완전 재구현, §12.16 ⑨)
unsafe fn serpen_reposition_fight(tick: i64, sim: usize, geom: usize, p4: u8) -> bool {
    // ★★[07-23] ~~`if tick <= 0x18 { return false; }`~~ **게이트 삭제**(0.5.2 `0x23b6800` 전수 disasm).
    //   원본은 `param_1`(tick)을 본문에서 **단 한 번도 읽지 않는다** — RCX 첫 등장이 `@1423b683c MOVZX ECX,DL`(덮어쓰기).
    //   ⟹ **5번째 사전게이트 삭제 확인**(disc1 p3 · disc12/14 code3 · engage_gate tick · 여기).
    //   ⚠라이브 disc14(L946, p4=5)도 이 함수를 쓴다 ⟹ disc14 재검증 대상에 포함.
    let _ = tick;   // 0.5.2 원본 미사용(시그니처는 호출부 호환 위해 유지)
    // ⚠원본은 `p4 < 4 → false` 후 `4→2 / 5→0 / 그 외→0xFF(mask 0x1ab)`. 모드의 `_ => return false`는
    //   살아있는 호출부가 4·5뿐이라 **출력 동등**(p4>=6 경로는 실재하지 않음). 표기만 다름.
    let mode: u8 = match p4 { 4 => 2, 5 => 0, _ => return false };   // p4=4→mode2 / p4=5→mode0
    let g0 = rd_u64(geom).unwrap_or(0) as usize;
    let plan = rd_u64(geom + 8).unwrap_or(0) as usize;
    if !ptr_ok(g0) || !ptr_ok(plan) { return false; }
    let role = rd_u8(plan + 0x38);
    let mask: u32 = match mode { 0 => 0x185, 1 => 0x1b1, _ => 0x1ab };
    if role > 8 || (mask >> (role & 0x1f)) & 1 == 0 { return false; }   // mode별 role 마스크
    let side = rd_u64(sim + 0x930).unwrap_or(2);//  ★0.5.4 오프셋 이동 반영
    if side > 1 { return false; }
    let gchild = rd_u64(g0).unwrap_or(0) as usize;
    let gvt = rd_u64(g0 + 8).unwrap_or(0) as usize;
    let w = rd_u64(sim + 0x938).unwrap_or(0) as usize;
    if !ptr_ok(gchild) || !ptr_ok(gvt) { return false; }   // ★07-10: ptr_ok(w) 제거(w=핸들, engage_gate와 동일 버그). 검증=dd7_slot128 내부
    let selfe = dd7_slot128(gchild, w as u64);   // self resolve → 순수재현(vt0x138=2-stage=dd7_slot128)
    if !ptr_ok(selfe) { return false; }
    // ★★[07-23 라이브 버그 수정] ~~하드코딩 `4`~~ → **`p4`**. 원본 `@1423b68e9`는 앵커 엔트리 매칭 키로
    //   **`BL`(=`param_4`)** 를 쓴다(`entry+0x20 == p4`인 첫 엔트리). 하드코딩 4는 **p4=5 경로에서 엉뚱한 앵커를 집는다**.
    //   ⚠"현재 호출부가 p4=4뿐이라 무영향"은 **오판** — L946의 **라이브 disc14가 p4=5로 호출** 중이었다.
    let (ax, ay) = serpen_anchor_pickup(plan, side as usize, p4);
    let sx = rd_u64(selfe + 0x660).unwrap_or(0);
    let sy = rd_u64(selfe + 0x668).unwrap_or(0);
    if !poke_f6f720(plan, sx, sy, mode) { return false; }   // ★self 좌표(앵커 아님)
    if (sqd(ax, ay, sx, sy) >> 8) <= 0xe8d4a50 { return false; }
    if !serpen_commit_gate(sim, geom, plan, mode, ax, ay) { return false; }
    let allies = serpen_count_allies_at(sim, g0, ax, ay, tune("ec_count_radius", 180000) as u64, tune("ec_count_hp", 0x28) as u64);
    let enemies = serpen_count_enemies_at(sim, geom, ax, ay, tune("ec_count_radius", 180000) as u64, tune("ec_count_hp", 0x28) as u64);
    let role_thr = if (role as usize) < 18 { SERPEN_PHASE3_ALLY[role as usize] } else { 0 };
    enemies == 0 && role_thr <= allies
}

// 캐스팅 컴포넌트 getter dispatch(07-10 RE: concrete 전수=raw 필드/상수 → fnptr RVA 분기, 게임코드 실행 0).
//   vt0x80: *(data+0x38)(42/42)·디폴트=0. vt0x90: *(data+0x30)(38/42)·+0x18/+8/+0 희귀 raw·0.
//   vt0xa8: 1(디폴트+41/42)·*(data+0x28). vt0xc8(캐스팅 트레이트)=true. 미등재(어빌리티 다형 vt0xc8 등)=보수적 0.
// ★[0.5.3 재핀 2026-07-30] 이 표도 disc19_repro.rs 슬롯표와 **같이 0.5.0_3 값이 방치돼 있었다**(0.5.1·0.5.2 마이그 모두 누락).
//   매핑(전부 명령 완전동일 + vtable 슬롯분포 일치 검증, 도구 = vtslot7~9_053.py):
//     0x19ed660→0xee4130 · 0x19f2f60→0xee24c0 · 0x19ed250→0xfe5c20 · 0x1a3a240→0xf025c0 · 0xb024b0→0xff71b0
//     0x50fc80→0x9d2a0 · 0x9a1230→0xf45090 · 0x1a13cb0→0x2f840 · 0x5418a0→0xc04f0
//   ⛔구값 병기 금지 — 구 RVA 전부가 0.5.3 `.text` 안이라 방치하면 **엉뚱한 함수를 이 게터로 오인**한다.
unsafe fn c8c_cast_get(pair: usize, slot: usize) -> u64 {
    let data = rd_u64(pair).unwrap_or(0) as usize;
    let vt = rd_u64(pair + 8).unwrap_or(0) as usize;
    if !ptr_ok(vt) { return 0; }
    let f = rd_u64(vt + slot).unwrap_or(0) as usize;
    let b = exe_base();
    if f <= b { return 0; }
    match f - b {
        0xee4130 => rd_u64(data + 0x38).unwrap_or(0),
        0xee24c0 => rd_u64(data + 0x30).unwrap_or(0),
        0xfe5c20 => rd_u64(data + 0x18).unwrap_or(0),
        0xf025c0 => rd_u64(data + 8).unwrap_or(0),
        0xff71b0 => rd_u64(data).unwrap_or(0),
        0x9d2a0 => 0,
        0xf45090 => 1,
        0x2f840 => rd_u64(data + 0x28).unwrap_or(0),
        0xc04f0 => 1,
        _ => 0,
    }
}

// 이펙트 kind 스캔: (any4, any5, all∈2..=5)
unsafe fn c8c_kinds(e: usize) -> (bool, bool, bool) {
    let list = rd_u64(e + 0x2b0).unwrap_or(0) as usize;
    let n = (rd_u64(e + 0x2b8).unwrap_or(0) as usize).min(256);
    let (mut a4, mut a5, mut all) = (false, false, true);
    if list != 0 {
        for i in 0..n {
            let k = rd_i32(list + i * 0x28).unwrap_or(0);
            if k == 4 { a4 = true; }
            if k == 5 { a5 = true; }
            if !(2..=5).contains(&k) { all = false; }
        }
    }
    (a4, a5, all)
}

// 타입별 타이머 필드 OFF(serpen_skill_type3와 동일 표)
#[inline] fn c8c_type_off(idx: i32) -> Option<usize> {
    Some(match idx { 1 => 0xb8, 2 => 0x110, 4 | 10 => 0xf0, 5 | 6 => 0x1f0, 7 => 0xe8, 8 | 13 => 0xb0, 9 => 0xc8, 11 => 0xd8, 12 => 0xd0, _ => return None })
}

// FUN_141fce700 재현: 무기A(type 0xd 평타) 가용성
unsafe fn c8c_fce700(e: usize) -> bool {
    let (a4, a5, all) = c8c_kinds(e);
    if a4 { return false; }
    if a5 && rd_i32(e + 0x4f8).unwrap_or(-1) != -1 && c8c_cast_get(e + 0x4c8, 0xc8) == 1 { return false; }
    if rd_i32(e + 0x68).unwrap_or(-1) != 0xd { return false; }
    let itv = ((c8c_cast_get(e + 0x568, 0x90) as i64) * 100 / (rd_i32(e + 0x3e8).unwrap_or(0) as i64 + 100).max(1)).max(3);
    let cap = (c8c_cast_get(e + 0x568, 0xa8) as i64).max(1);
    if itv - itv / cap < rd_i64(e + 0xb8).unwrap_or(0) { return false; }
    all && rd_i32(e + 0x4f8).unwrap_or(-1) != -1
}

// FUN_141fbe950 재현: 무기B(*(e+0x5b0)>=3) 가용성 (comp=e+0x578/0x580, 최소1, 비교필드 e+0xc0)
unsafe fn c8c_fbe950(e: usize) -> bool {
    if rd_i64(e + 0x5c8).unwrap_or(0) < 3 { return false; }
    let (a4, a5, all) = c8c_kinds(e);
    if a4 { return false; }
    if a5 && rd_i32(e + 0x500 + 0x30).unwrap_or(-1) != -1 && c8c_cast_get(e + 0x500, 0xc8) == 1 { return false; }
    let itv = ((c8c_cast_get(e + 0x578, 0x90) as i64) * 100 / (rd_i32(e + 0x3e8).unwrap_or(0) as i64 + 100).max(1)).max(1);
    let cap = (c8c_cast_get(e + 0x578, 0xa8) as i64).max(1);
    if itv - itv / cap < rd_i64(e + 0xc0).unwrap_or(0) { return false; }
    all
}

// TTD 가중치표(geom0+0x230, u64[2][5][100]): idx = side*500 + slot*100 + col
#[inline] unsafe fn c8c_tbl(g0: usize, rs: usize, rslot: usize, col: usize) -> i64 {
    rd_i64(g0 + 0x230 + (rs * 500 + rslot * 100 + col) * 8).unwrap_or(0)
}

// 공격자 j가 피격슬롯 col에 주는 burst(가용성 게이트별 max)/dps(무조건 3열 합) 누적
unsafe fn c8c_accum(g0: usize, gchild: usize, j: usize, col: usize, scalar: i64, burst: &mut i64, dps: &mut i64) {
    let athj = dd7_slot_a8(gchild, rd_u64(j + 0x5c0).unwrap_or(0));
    if athj == 0 { return; }
    let rs = rd_u64(athj + 0x930).unwrap_or(2) as usize;//  ★0.5.4 오프셋 이동 반영
    if rs > 1 { return; }
    let rslot = (rd_u32(athj + 0x9c0) as usize).min(4);//  ★0.5.4 오프셋 이동 반영
    let ty = rd_i32(j + 0x68).unwrap_or(-1);
    let a0 = ty == 0 || ty == 3 || serpen_skill_type3(j)
        || c8c_type_off(ty).map(|o| rd_i64(j + o).unwrap_or(i64::MAX) <= scalar).unwrap_or(false);
    let a1 = ty != 0xd || c8c_fce700(j) || rd_i64(j + 0xb8).unwrap_or(i64::MAX) <= scalar;
    let a2 = ty != 0xd || c8c_fbe950(j) || rd_i64(j + 0xc0).unwrap_or(i64::MAX) <= scalar;
    let mut m = 0i64;
    if a0 { m = m.max(c8c_tbl(g0, rs, rslot, col)); }
    if a1 { m = m.max(c8c_tbl(g0, rs, rslot, col + 5)); }
    if a2 { m = m.max(c8c_tbl(g0, rs, rslot, col + 10)); }
    *burst += m;
    *dps += c8c_tbl(g0, rs, rslot, col + 0x32) + c8c_tbl(g0, rs, rslot, col + 0x37) + c8c_tbl(g0, rs, rslot, col + 0x3c);
}

// 구조물 리스트 FUN_142076820: 고정 6슬롯(이 순서) non-null ++ 동적 Vec 전부
unsafe fn c8c_structures(g0: usize, side: usize, out: &mut Vec<usize>) {
    for off in [0x180usize, 0x1a0, 0x1c0, 0x190, 0x1b0, 0x1d0] {
        let e = rd_u64(g0 + off + side * 8).unwrap_or(0) as usize;
        if e != 0 { out.push(e); }
    }
    let p = rd_u64(g0 + 0x130 + side * 0x20).unwrap_or(0) as usize;
    let n = (rd_u64(g0 + 0x148 + side * 0x20).unwrap_or(0) as usize).min(64);
    if p != 0 { for i in 0..n { let e = rd_u64(p + i * 8).unwrap_or(0) as usize; if e != 0 { out.push(e); } } }
}

// 아군 홈존 판정(c8c520 (b)루프 제외조건; side0: x≤0xfa00&&0xc3500≤y<0xea601 || x≤0x27100&&0xdac00≤y≤0xea600 / side1 대칭)
#[inline] fn c8c_in_home(side: usize, x: u64, y: u64) -> bool {
    if side == 0 {
        (x <= 0xfa00 && y >= 0xc3500 && y < 0xea601) || (x <= 0x27100 && y >= 0xdac00 && y <= 0xea600)
    } else {
        (y <= 0xfa00 && x >= 0xc3500 && x < 0xea601) || (y <= 0x27100 && x >= 0xdac00 && x <= 0xea600)
    }
}

unsafe fn my_c8c520(level: i64, ss: usize, geom: usize, selfe: usize) -> u8 {
    let g0 = rd_u64(geom).unwrap_or(0) as usize;
    let plan = rd_u64(geom + 8).unwrap_or(0) as usize;
    if !ptr_ok(g0) || !ptr_ok(plan) { return 2; }
    let gchild = rd_u64(g0).unwrap_or(0) as usize;
    if !ptr_ok(gchild) { return 2; }
    let side = rd_u64(ss + 0x930).unwrap_or(2) as usize;//  ★0.5.4 오프셋 이동 반영
    if side > 1 { return 2; }
    let my_slot = (rd_u32(ss + 0x9c0) as usize).min(4);//  ★0.5.4 오프셋 이동 반영
    let tick = rd_i64(gchild + 0xec68).unwrap_or(0);              // vt0x28 concrete(0x19f0620)
    let mapctx = rd_u64(plan + 8).unwrap_or(0) as usize;
    let scalar = rd_i64(mapctx + 0x12f8).unwrap_or(0);
    let (sx, sy) = (rd_u64(selfe + 0x660).unwrap_or(0), rd_u64(selfe + 0x668).unwrap_or(0));
    // ① tag5: 자기 구조물 보호권 — t*=own 중 최근접, 없으면 *(g0+0x170+side*8), 그래도 0 → tag5
    let mut own: Vec<usize> = Vec::new();
    c8c_structures(g0, side, &mut own);
    let mut tstar = 0usize; let mut best = u64::MAX;
    for &st in &own {
        let d2 = sqd(rd_u64(st + 0x660).unwrap_or(0), rd_u64(st + 0x668).unwrap_or(0), sx, sy);
        if d2 < best { best = d2; tstar = st; }
    }
    if tstar == 0 { tstar = rd_u64(g0 + 0x170 + side * 8).unwrap_or(0) as usize; }
    if tstar == 0 { return 5; }
    let self_reach = (rd_i32(selfe + 0x470).unwrap_or(0) as i64 + 100) * rd_i64(selfe + 0x680).unwrap_or(0) / 100;
    if rd_i32(tstar + 0x4c0).unwrap_or(-1) != -1 {
        let reach = rd_i64(tstar + 0x438).unwrap_or(0) + rd_i64(tstar + 0x4a0).unwrap_or(0)
            + self_reach + (rd_i64(tstar + 0x5c8).unwrap_or(1) - 1) * rd_i64(tstar + 0x4a8).unwrap_or(0);
        let d2 = sqd(rd_u64(tstar + 0x660).unwrap_or(0), rd_u64(tstar + 0x668).unwrap_or(0), sx, sy);
        if reach >= 0 && d2 <= (reach as u64).wrapping_mul(reach as u64) { return 5; }
    }
    // ② 후보 = 적 5슬롯 × FUN_141c932c0 술어(=serpen_pred_b 재사용, ctx{self,&level,ss})
    let lvl_u = level as u64;
    let ctx: [u64; 3] = [selfe as u64, (&lvl_u as *const u64) as u64, ss as u64];
    let ctxp = ctx.as_ptr() as usize;
    let mut cands = [0usize; 5]; let mut nc = 0usize;
    for k in 0..5usize {
        let cand = rd_u64(g0 + 0x1e0 + (1 - side) * 0x28 + k * 8).unwrap_or(0) as usize;
        if cand != 0 && serpen_pred_b(ctxp, cand) { cands[nc] = cand; nc += 1; }
    }
    if nc == 0 { return 1; }
    // ③ 셋업
    let self_itv = ((c8c_cast_get(selfe + 0x558, 0x80) as i64) * 100 / (rd_i32(selfe + 0x3e4).unwrap_or(0) as i64 + 100).max(1)).max(3);
    let self_spd = rd_i64(selfe + 0x640).unwrap_or(0);
    let self_hp = rd_i64(selfe + 0x670).unwrap_or(0);
    let t1 = (sqd(rd_u64(tstar + 0x660).unwrap_or(0), rd_u64(tstar + 0x668).unwrap_or(0), sx, sy).isqrt() as i64) / self_spd.max(1);
    let budget = t1 + self_itv + 6;
    let roleb = (rd_u8(plan + 0x38) as usize).min(7);
    let scalar2 = rd_i64(mapctx + C8C_ROLE_OFF[roleb]).unwrap_or(0);
    let list2: Vec<usize> = if tick <= scalar2 {
        own.iter().copied().filter(|&e| sqd(rd_u64(e + 0x660).unwrap_or(0), rd_u64(e + 0x668).unwrap_or(0), sx, sy) <= 0x53d1ac100).collect()
    } else { Vec::new() };
    let exe = exe_base();
    // ④ 후보 루프
    let (mut danger, mut pushed) = (false, false);
    for ci in 0..nc {
        let cand = cands[ci];
        let id = rd_u64(cand + 0x5c0).unwrap_or(0);
        let ath = dd7_slot_a8(gchild, id);
        if ath == 0 { continue; }
        let s_stat = rd_i64(ath + 0x230).unwrap_or(0).min(100);
        let th_a = 0x50 + (80000 - 800 * s_stat) / 1000;
        let th_b = 0x2d + (450 * s_stat) / 1000;
        let c_slot = (rd_u32(ath + 0x9c0) as usize).min(4);//  ★0.5.4 오프셋 이동 반영
        let (cx, cy) = (rd_u64(cand + 0x660).unwrap_or(0), rd_u64(cand + 0x668).unwrap_or(0));
        // (a) 내 생존시간: 적 j(5슬롯) 중 dist²(j,cand)≤150000² && hp%≥0x28
        let (mut burst, mut dps) = (0i64, 0i64);
        for jj in 0..5usize {
            let j = rd_u64(g0 + 0x1e0 + (1 - side) * 0x28 + jj * 8).unwrap_or(0) as usize;
            if j == 0 { continue; }
            if sqd(rd_u64(j + 0x660).unwrap_or(0), rd_u64(j + 0x668).unwrap_or(0), cx, cy) > 0x53d1ac100 { continue; }
            let jmax = rd_i64(j + 0x628).unwrap_or(0);
            if jmax <= 0 || rd_i64(j + 0x670).unwrap_or(0) * 100 / jmax < 0x28 { continue; }
            c8c_accum(g0, gchild, j, my_slot, scalar, &mut burst, &mut dps);
        }
        let my_surv = (self_hp - burst).max(0) * 0x3c / dps.max(1);
        // (b) cand 처치시간: 아군 k(5슬롯) + list2 구조물
        let (mut burst2, mut dps2) = (0i64, 0i64);
        for kk2 in 0..5usize {
            let k = rd_u64(g0 + 0x1e0 + side * 0x28 + kk2 * 8).unwrap_or(0) as usize;
            if k == 0 { continue; }
            if sqd(rd_u64(k + 0x660).unwrap_or(0), rd_u64(k + 0x668).unwrap_or(0), cx, cy) > 0x53d1ac100 { continue; }
            if rd_u64(cand).unwrap_or(1) == 0 {   // cand 은신/시야 필드 → k 제외
                let ck = rd_u64(cand + 8).unwrap_or(0) as usize;
                if rd_u64(k + ck * 0x18 + 0x38).unwrap_or(0) != 0 { continue; }
            }
            if level > 0xb && rd_u64(k).unwrap_or(1) == 0 && rd_u64(k + 8).unwrap_or(99) as usize == side {
                let (kx, ky) = (rd_u64(k + 0x660).unwrap_or(0), rd_u64(k + 0x668).unwrap_or(0));
                if c8c_in_home(side, kx, ky) { continue; }
            }
            c8c_accum(g0, gchild, k, c_slot, scalar, &mut burst2, &mut dps2);
        }
        for &st in &list2 {
            if rd_i32(st + 0x4c0).unwrap_or(-1) == -1 { continue; }
            // ⛔★★[07-22] **이 shadow-call이 disc14 크래시 3건의 진범**(2차·3차 AV, RIP=exe+0x1c8f785 동일).
            //   `RVA_C8C_DMG_SHEET`는 **0.5.2 미마이그(보류)** 상태의 0.5.0_3 값 `0x3830c58`인데, 0.5.2에서 그 주소는
            //   **Rust 타입명 문자열 블롭**("…ltAction")이다. 그걸 vt+0x28 다형 shadow-call의 this로 넘겨 `call [r9+0x30]`을
            //   수행 → 문자열을 함수 포인터로 호출 → **non-canonical → #GP**(모드 crash_log의 `faultAddr=-1`이 이 표식).
            //   ★크래시 덤프의 `R9 = exe+0x3830c58`이 정확히 이 상수 — "게임이 만든 쓰레기값"이 아니라 **모드가 하드코딩한 stale 상수**였다.
            //   ⚠rva_052.rs의 C8C 주석 "d19thr 게이트 기본 OFF라 미사용"은 **오기**: 이 사이트(my_c8c520)는 disc14 재현 경로에서
            //     게이트 없이 호출되므로, 보류 상태를 방치하면 **재현이 실행되는 즉시** AV다(대체 여부와 무관 — 3차는 mpcap 캡처만으로 발생).
            //   ⟹ RVA 확정 전까지 shadow-call 자체를 봉인(§3 "위험 shadow-call은 게이트로 격리" 원칙).
            //   영향: b0=b1=0 → 이 항목 dmg=0 → **재현 정확도 저하(game≠mine 가능)**. 크래시는 원천 차단.
            //   해제 조건 = `RVA_C8C_DMG_SHEET` 0.5.2 확정(ghidra-re: desc 11→9 개수변동으로 순서대응 불가 → 별도 변별 필요) 후 true.
            //   ✅**[07-22] 봉인 해제**: `RVA_C8C_DMG_SHEET` 0.5.2 확정(`0x381e1e0`, 3버전 대조 + 0.5.0_3 ground-truth로 방법 검증).
            //     desc 7중복본의 slot 0x30이 전부 동일하므로 오동작 위험 사실상 0. 재현 정확도(b0/b1) 복구.
            const C8C_SHEET_MIGRATED: bool = true;
            let (b0, b1) = if C8C_SHEET_MIGRATED {
                probe_basedmg_r9(st, plan, exe, exe + RVA_C8C_DMG_SHEET)   // ★어빌리티 vt+0x28 base쌍(Arc payload; 다형이라 잠정 shadow-call)
            } else { (0i64, 0i64) };
            let dmg = if b0 > 0 || b1 > 0 {
                let dt = rd_i32(st + 0x4a4).unwrap_or(0);
                serpen_dmg_core(st, cand, b0.max(0), dt, 0) + serpen_dmg_core(st, cand, b1.max(0), dt, 1)
            } else { 0 };
            let itv_s = ((c8c_cast_get(st + 0x558, 0x90) as i64) * 100 / (rd_i32(st + 0x3e4).unwrap_or(0) as i64 + 100).max(1)).max(3);
            dps2 += dmg * scalar / itv_s.max(1);
            burst2 += dmg;
        }
        let t_kill = (rd_i64(cand + 0x670).unwrap_or(0) - burst2).max(0) * 0x3c / dps2.max(1);
        if th_a + my_surv > t_kill { continue; }   // 잡을 수 있음 → skip
        let cand_reach = if rd_i32(cand + 0x4c0).unwrap_or(-1) != -1 {
            rd_i64(cand + 0x438).unwrap_or(0) + rd_i64(cand + 0x4a0).unwrap_or(0)
                + (rd_i64(cand + 0x5c8).unwrap_or(1) - 1) * rd_i64(cand + 0x4a8).unwrap_or(0)
        } else { 0 };
        let cand_bonus = (rd_i32(cand + 0x470).unwrap_or(0) as i64 + 100) * rd_i64(cand + 0x680).unwrap_or(0) / 100;
        let d = sqd(cx, cy, sx, sy).isqrt() as i64;
        if d > cand_reach + self_reach + cand_bonus + rd_i64(cand + 0x640).unwrap_or(0) * (self_itv + 6) { continue; }
        if my_surv < th_b || rd_i64(cand + 0x640).unwrap_or(0) > self_spd {
            danger = true;
            if my_surv <= budget { pushed = true; }
        }
    }
    if pushed { 0 } else if danger { 4 } else { 1 }
}

// ════ disc12(실명 EpicCheck — 구라벨 SerpenBattle, §11.8) body = FUN_14235c440 (0.5.0_3 RVA 0x235c440) 재구현. out-writer(직접기록), 코드반환(-99=passthrough). ════
//   시그: (out,mem=subp+8,tick=r8,rng=r9,sim=r14,geom=r15,tp,sf). 출력{0x14,0xc,0xd,0xe,7,2,3}=디스패처 추적 확정.
// ★★[07-28 결정성 수정] entry 스냅샷 4종을 **전역(MP_AUX_*) 대신 인자로** 수령.
//   구조결함이었던 것: 라이브 arm이 MP_AUX_M18/M19/WLEN/WH(프로세스 전역 Atomic)에 store→여기서 load.
//   배경 rayon 워커 다수 + 관전 sim이 **동시에** disc12를 지나면 store↔load 사이에 타 스레드가 덮어써(lost update)
//   m18/m19/W큐가 **남의 경기 값**이 됨 ⟹ 리액티브 분기(0x14/0xc)가 갈려 **배경≠관전 비결정 발산**(스코어까지).
//   `aux: Option<(u64 m18, u64 m19, u64 wlen, u64 wh)>` = Some(라이브 대체·인자직결) / None(검증 리턴훅 경로=전역 스냅샷 사용).
unsafe fn my_serpen_battle(out: usize, mem: usize, tick: i64, rng: usize, sim: usize, geom: usize, tp: usize, sf: usize, live: bool, aux: Option<(u64, u64, u64, u64)>) -> i64 {
    serpen_diag(12);   // 총 호출
    for i in 0..12usize { SGT[i].store(-1, Ordering::Relaxed); }   // ★SGT: 호출당 리셋(stale 오독 방지)
    // ★[07-23] 진입가드 인자별 세분(미해결 ① "733/1350 튕김" 원인 특정용). geom/sim/mem은 캡처 경로에서 MP_AUX 공유 오염 의심.
    if !ptr_ok(out)  { serpen_diag(0);  return -99; }   // out=bufp(스택)라 정상 0이어야 함 — 0이면 하네스 버그
    if !ptr_ok(geom) { serpen_diag(24); return -99; }   // geom=MP_AUX_P6(r15)
    if !ptr_ok(sim)  { serpen_diag(25); return -99; }   // sim=MP_AUX_P5(r14)
    if !ptr_ok(mem)  { serpen_diag(26); return -99; }   // mem=MP_AUX_P2(subp+8)
    let g0 = rd_u64(geom).unwrap_or(0) as usize;          // geom[0]
    let plan = rd_u64(geom + 8).unwrap_or(0) as usize;    // geom[1]
    if !ptr_ok(g0) || !ptr_ok(plan) { serpen_diag(1); return -99; }
    let gchild = rd_u64(g0).unwrap_or(0) as usize;        // geom[0][0]
    let gvt = rd_u64(g0 + 8).unwrap_or(0) as usize;       // geom[0][1]
    if !ptr_ok(gchild) || !ptr_ok(gvt) { serpen_diag(2); return -99; }
    let side = rd_u64(sim + 0x930).unwrap_or(2);//  ★0.5.4 오프셋 이동 반영
    if side > 1 { serpen_diag(3); return -99; }
    let self_handle = rd_u64(sim + 0x938).unwrap_or(0) as usize;   // ★self-unit 핸들(vt0x138 arg, a994)
    let selfe = dd7_slot128(gchild, self_handle as u64);           // self resolve → 순수재현(vt0x138=2-stage=dd7_slot128)
    if !ptr_ok(selfe) { serpen_diag(4); return -99; }
    // ★order-context W = vt0x30(gchild) RDX = gchild+0xeaf0 (순수재현). 리액티브·메인 공용 target 소스.
    //   ★07-10: W큐도 entry 스냅샷(MP_AUX_WLEN/WH) 사용 — 게임핸들러가 오더 소비 후 리턴훅이 len=0을 읽던 0xc/0xd 아티팩트 해소.
    //   entry훅이 호출 직전 저장하므로 라이브(mp_repl entry시점)에서도 snapshot==live 동일값.
    // ★[07-28] aux=Some이면 인자직결(스레드 안전), None이면 구 전역 스냅샷(검증 경로 전용).
    let (a_m18, a_m19, a_wlen, a_wh) = match aux {
        Some(t) => t,
        None => (MP_AUX_M18.load(Ordering::Relaxed), MP_AUX_M19.load(Ordering::Relaxed),
                 MP_AUX_WLEN.load(Ordering::Relaxed), MP_AUX_WH.load(Ordering::Relaxed)),
    };
    let have_order = a_wlen != 0;
    let mut target = 0usize;
    if have_order {
        let h = a_wh;                                 // handle = *(*(W+0x1a0)) entry 스냅샷
        target = geom_resolve150(gchild, h);          // target resolve → 순수재현(vt0x150)
    }
    // ── 리액티브 가드(mem[0x18]||mem[0x19]) ──
    // ★one-shot 플래그(07-10 확정): 게임핸들러가 소비/클리어 → 검증(리턴훅) 호출은 항상 0을 봄(5.5% DIFF 원인).
    //   entry 스냅샷(MP_AUX_M18/19, 캡처가 disc12 진입시 저장) 사용. 라이브 대체(mp_repl, entry시점 호출)로 전환시엔 live-read와 동일값.
    let m18 = a_m18 as u8;   // ★[07-28] 인자직결(구 MP_AUX_M18 전역 load = 스레드 레이스원)
    let m19 = a_m19 as u8;   // ★[07-28] 인자직결(구 MP_AUX_M19)
    // ★SGT: 진입부 상태 스냅(0xe/0xc divergence 특정용)
    sgt(1, have_order as i64); sgt(2, ptr_ok(target) as i64); sgt(3, m18 as i64); sgt(4, m19 as i64); sgt(11, tick);
    if m18 != 0 || m19 != 0 {
        if have_order && ptr_ok(target) {                 // 도달 → 0x14
            serpen_diag(6);
            sgt(0, 0x14);
            wr_u64(out, 0x14); wr_u64(out + 8, 0); wr_u8(out + 0x10, 0); wr_u8(out + 0x11, m18);
            return 0x14;
        }
        serpen_diag(if !have_order { 20 } else { 21 });   // ★DIAG(0x14 갭 추격): 20=오더無 / 21=오더but타겟해석실패
        serpen_diag(7); sgt(0, 0xc); wr_u8(out + 8, 0); wr_u64(out, 0xc); return 0xc;  // 미도달 → 0xc
    }
    // ── 메인경로 ──
    if have_order { serpen_diag(22); }   // ★DIAG: 메인경로인데 오더 존재(게임이 여기서 0x14 낼 가능성 추적)
    if !ptr_ok(target) {
        // ★게임 fe2500은 타겟 생존체크보다 먼저 무조건 1회 호출 → 타겟-null(0xc) 반환 전에도 RNG 소비. live 대체시 스트림 동기화 위해 여기서 드로우.
        if live { let _ = serpen_rng_pick(rng, sim, plan, true); }
        serpen_diag(7); sgt(0, 0xc); wr_u8(out + 8, 0); wr_u64(out, 0xc); return 0xc;   // 타겟없음 → 0xc
    }
    let self_max = rd_i64(selfe + 0x628).unwrap_or(0);
    if self_max == 0 { serpen_diag(5); return -99; }   // ★-99 passthrough(게임 재실행이 드로우) — 이 지점은 아직 미드로우라 double-draw 없음
    let self_cur = rd_i64(selfe + 0x670).unwrap_or(0);
    let hp_pct = self_cur.wrapping_mul(100) / self_max;
    // ★★[07-28 이중드로우 수정] `[E]` 재구축 경로(미구현)는 아래 `:~1011`에서 **-99 passthrough**로 빠지는데,
    //   여기서 이미 live draw(=게임 RNG write-back 전진)를 해버리면 **원본이 재실행되며 picker를 또 소비** ⟹ 같은 호출에 draw 2회
    //   = 스트림이 한쪽 sim에서만 앞서감 = **배경≠관전 영구 desync**(스코어까지 발산). 구 주석 "미드로우 지점이라 double-draw 없음"은
    //   disc14엔 맞지만 **disc12 [E]엔 거짓**이었다.
    //   ⟹ passthrough로 갈 조건을 **draw 전에 선판정**해, 그 경우엔 draw 자체를 하지 않는다(원본이 자기 draw를 온전히 수행).
    //   판정식은 아래 [D]/[E] 게이트와 동일(중복 계산이나 read-only·부작용 없음).
    let will_passthrough_e = {
        let mapobj_pre = rd_u64(plan + 8).unwrap_or(0) as usize;
        let scalar_pre = if ptr_ok(mapobj_pre) { rd_i64(mapobj_pre + 0x12f8).unwrap_or(0) } else { 0 };
        let a_pre = rd_i64(tp + 0x88).unwrap_or(0);
        let b_pre = rd_i64(tp + 0x98).unwrap_or(0);
        let min_pre = if a_pre < b_pre { a_pre } else { b_pre };
        let skip_ef_pre = min_pre <= scalar_pre.wrapping_mul(5);
        !skip_ef_pre && rd_u8(mem + 0x1a) == 0   // [E] 진입 && 재구축 경로(미구현) = 아래서 -99
    };
    let tag = serpen_rng_pick(rng, sim, plan, live && !will_passthrough_e);     // tag=후보존재(0/1). live시 여기서 fe2500 Lemire 드로우(target-null 미도달 경로=위에서 처리)
    dl_rec(gchild, 13, tag as u64 | (will_passthrough_e as u64) << 8);   // ★[07-29 detlog] d12 픽tag(site13): draw유무 대조
    if tag != 0 { serpen_diag(15); }
    let plan4 = rd_u64(plan + 0x20).unwrap_or(0) as usize;   // plan[4]
    let zone = plan4 + (side as usize) * 0x20;
    let self_x = rd_u64(selfe + 0x660).unwrap_or(0);
    let self_y = rd_u64(selfe + 0x668).unwrap_or(0);
    let out_of_zone = self_x < rd_u64(zone + 0x6d70).unwrap_or(0)
        || rd_u64(zone + 0x6d80).unwrap_or(0) < self_x
        || self_y < rd_u64(zone + 0x6d78).unwrap_or(0);
    serpen_diag(if out_of_zone { 13 } else { 14 });
    let tgt_cur = rd_i64(target + 0x670).unwrap_or(0);
    let tgt_max = rd_i64(target + 0x628).unwrap_or(1);
    if tgt_cur == tgt_max { serpen_diag(16); }
    sgt(12, tag);   // ★sgate 진단: picker tag
    sgt(13, (out_of_zone as i64) | (((tgt_cur == tgt_max) as i64) << 1) | (hp_pct << 8));   // zenc=oz|tgtfull<<1|hp%<<8
    let engage: bool;
    if out_of_zone {
        if (hp_pct > tune("ec_oz_hp", 0x32) && tag == 0) || tgt_cur != tgt_max { engage = true; }
        else { serpen_diag(8); sgt(0, 7); wr_u64(out, 7); return 7; }
    } else if tgt_cur == tgt_max {   // in-zone & 타겟 풀피
        if tag != 0 || hp_pct < tune("ec_iz_hp", 0x33) || (self_cur < self_max && self_y <= rd_u64(zone + 0x6d88).unwrap_or(0)) {
            serpen_diag(8); sgt(0, 7); wr_u64(out, 7); return 7;
        } else { engage = true; }
    } else { engage = true; }        // in-zone & 타겟 비풀피
    if !engage { serpen_diag(8); sgt(0, 7); wr_u64(out, 7); return 7; }
    serpen_diag(19);   // main-engage 도달
    // ── 메인교전(LAB_14235c6af) ──
    if rd_u8(sf + 0x3f6) != 0 { serpen_diag(18); serpen_diag(8); sgt(0, 7); wr_u64(out, 7); return 7; }
    if rd_u8(sf + 0x3f7) == 3 {   // 재배치 페이즈
        serpen_diag(17);
        // ★★[07-23 정정] ~~`(tag as u64) < 5 && role_self == tag as u32`~~ = **확정 오독**.
        //   `tag`는 RNG picker의 Option discriminant(0/1)일 뿐 **레인이 아니다**(테일은 tag를 [A]에서 불리언으로만 소비).
        //   0.5.2 원본 `@f3df~f5b2`의 실제 레인 산출(순수 오프셋형, 근거 = MIGRATION.md §7.2-A7 §6 + 조건표 §B):
        //     lane = if u8[gchild+0xeae9] != 0 { read24(sim+0x4a8) } else { read24(gchild+0xb248 + side*0x18) }
        //   (원본은 `vt[0xc0]`(바이트게터 `0x1dc87d0`) 분기 후 `vt[0xe0]`(24B sret `0x231eed0`) 호출 — 둘 다 순수재현으로 대체)
        let lane: u32 = if rd_u8(gchild + 0xec91) != 0 {
            rd_u32(sim + 0x508)                                  // 16B movups + 8B = 24B 중 선두 u32
        } else {
            rd_u32(gchild + 0xb248 + (side as usize) * 0x18)
        };
        let role_self = rd_u32(sim + 0x9c0);//  ★0.5.4 오프셋 이동 반영
        let fight = lane <= 4 && role_self == lane && serpen_reposition_fight(tick, sim, geom, 4);
        if fight {
            let role = rd_u8(plan + 0x38);   // FIGHT: role_val 상수(0x1eb마스크→2 / role2→0 / else 1)
            let role_val: u8 = if (0x1ebu32 >> (role & 0x1f)) & 1 != 0 { 2 } else if role == 2 { 0 } else { 1 };
            serpen_diag(10);
            // ★★[07-23] ~~`if tick < 0x21 { → code 3 }`~~ **분기 삭제**(0.5.2에서 `cmp rbx,0x21` 게이트 제거됨).
            //   0.5.2 출력집합 = {0x14,0xc,7,0xd,0xe,2} — code 3은 없다. disc14와 동일 부류의 게이트 삭제.
            sgt(0, 2); wr_u64(out, 2); wr_u8(out + 8, 0); wr_u8(out + 9, role_val); wr_u8(out + 0xa, 2); return 2;
        }
        serpen_diag(9); sgt(0, 0xd); wr_u8(out + 8, 0); wr_u64(out, 0xd); return 0xd;
    }
    // ══ [C] 홈 코너 박스 (0.5.2 `@f416~f4da`) ★★07-23 신설 — 기존 재현에 **통째로 없던 종단** ══
    //   원본은 여기서 self를 재취득(vt[0x1a0], 동일 엔티티)하고 hpPct2를 **재계산**한다. 아래 [F]가 쓰는 HP가 이것.
    //   좌표 리터럴 = `0xfa00`(64000)·`0xd9c60`(892000)·`0xea600`(960000)·`0xdac00`(896000) @f46e~f4c1.
    let s_us = side as usize;
    let in_x = if s_us == 0 { self_x <= 64000 } else { self_x >= 892000 && self_x <= 960000 };
    let in_y = if s_us == 0 { self_y >= 896000 && self_y <= 960000 } else { self_y <= 64000 };
    if in_x && in_y && self_cur < self_max { serpen_diag(8); sgt(0, 7); wr_u64(out, 7); return 7; }   // @f4da
    let hp_pct2 = self_cur.wrapping_mul(100) / self_max;   // @f4f0 재계산(self_max!=0은 L834서 가드됨)

    // ══ [D] 위협 게이트 (0.5.2 `@f5c2~f5e2`) ★★07-23 신설 — 통과 실패시 [E][F]를 **건너뛰고** [G]로 직행 ══
    //   scalar = *( *(ctx2+8) + 0x12f8 ) = mapobj+0x12f8 (★ctx2가 아니라 mapobj — engage_gate의 UNIT과 동일 함정)
    let mapobj = rd_u64(plan + 8).unwrap_or(0) as usize;
    let scalar = if ptr_ok(mapobj) { rd_i64(mapobj + 0x12f8).unwrap_or(0) } else { 0 };
    let th_a = rd_i64(tp + 0x88).unwrap_or(0);
    let th_b = rd_i64(tp + 0x98).unwrap_or(0);
    let min_th = if th_a < th_b { th_a } else { th_b };
    let skip_ef = min_th <= scalar.wrapping_mul(5);   // @f5db `LEA rax,[rax+rax*4]` = ×5. '>' 일 때만 [E][F] 진입

    if !skip_ef {
        // ══ [E] 다이브 추적 (0.5.2 `@f5e8~f6fc`) ══
        //   ⬜**재구축 경로(st.track==0)는 미구현 → -99 passthrough**. 콜리 `FUN_141bd73a0`(다이브 후보 빌더) 미전개라
        //     `st.len`(후보 수)을 산출할 수 없다. 그 경로는 출구 3개가 **전부 code 7 수렴**(@fb88/fb9a/fba8)이라
        //     힙 재구축 자체는 생략 가능하나, `st.track=1`·`st.len` 갱신이 **다음 tick 동작을 바꾸므로** 값 없이 쓰면 안 된다.
        //     ⟹ passthrough(게임 원본이 수행) = 바닐라 비트동일 = 안전. 편입 후 별건으로 채운다.
        // ★[07-23] 진단 태그 분리: ~~`serpen_diag(0)`(진입 가드와 동일 태그)~~ → **`serpen_diag(23)`**.
        //   0번을 재사용해 `[0]`이 부풀었고, 이를 "진입 가드로 733/1350 튕김"으로 **오독**했다(실제는 설계대로의 [E] passthrough).
        //   0 = 진짜 인자 가드(L868) / 23 = [E] 재구축 경로 미구현 passthrough. 두 값을 반드시 구분해서 읽을 것.
        if rd_u8(mem + 0x1a) == 0 { serpen_diag(23); return -99; }
        // st.track != 0: payload 순회 — 200000 이내 적 있으면 code 7
        let plen = rd_u64(mem + 0x10).unwrap_or(0);
        let pptr = rd_u64(mem + 8).unwrap_or(0) as usize;
        if plen != 0 && ptr_ok(pptr) {
            for k in 0..(plen.min(4096) as usize) {
                let h = rd_u64(pptr + k * 8).unwrap_or(0);
                let e = geom_resolve150(gchild, h);
                if !ptr_ok(e) { continue; }
                let (ex, ey) = (rd_u64(e + 0x660).unwrap_or(0), rd_u64(e + 0x668).unwrap_or(0));
                if sqd(ex, ey, self_x, self_y) < 0x9502F9001u64 {   // @f61d = 200000²+1
                    serpen_diag(8); sgt(0, 7); wr_u64(out, 7); return 7;   // @f693
                }
            }
        }
        // 완주(또는 len==0) → payload 무효화 후 [F]로 계속(즉시 반환 아님). cap/ptr 절대 미터치. @f6ef
        wr_u8(mem + 0x1a, 0);
        wr_u64(mem + 0x10, 0);
    }

    // ══ [F] 5슬롯 role==7 카운트 (0.5.2 `@f6fc~f9db`) — skip_ef면 건너뛰고 [G]로 ══
    if !skip_ef {
        let cnt_enemy = serpen_role7_count(g0, gchild, plan, geom, s_us, 1 - s_us);   // ★vis_side=side 고정
        let cnt_ally  = serpen_role7_count(g0, gchild, plan, geom, s_us, s_us);
        // ★★07-23 정정: ~~`tgt_hp_pct < 0x15`(target HP)~~ → **`hp_pct2 <= 0x14`(self HP)**. @f9cd `CMP qword[RBP+8],0x14; JA skip`
        //   (`< 0x15` ≡ `<= 0x14`로 수치는 같았고, 틀린 건 **HP 소스**였다.) @f9d8 `CMP R14,R15; JC` = 자팀 < 적팀
        if hp_pct2 <= tune("ec_self_hp_low", 0x14) && cnt_ally < cnt_enemy {
            serpen_diag(8); sgt(0, 7); wr_u64(out, 7); return 7;
        }
    }
    let gate = serpen_engage_gate(sf, tick, sim, geom, tp, 4);   // ★SGT: gate 결과+경로 기록
    sgt(5, gate as i64);
    if !gate {
        let dist2 = sqd(rd_u64(target + 0x660).unwrap_or(0), rd_u64(target + 0x668).unwrap_or(0), self_x, self_y);
        sgt(9, (dist2 >> 8) as i64); sgt(10, (tune("ec_engage_dist2", 0x53d1ac101) as u64 >> 8) as i64);
        if dist2 < (tune("ec_engage_dist2", 0x53d1ac101) as u64) { serpen_diag(11); sgt(0, 0xe); wr_u64(out, 0xe); return 0xe; }
    }
    serpen_diag(7); sgt(0, 0xc); wr_u8(out + 8, 0); wr_u64(out, 0xc); 0xc
}

// ★★[07-23] `rng`·`live` 인자 신설 = **disc14 RNG 홀 수정**(아래 picker 사이트 주석 참조). 배선 규약은 disc12 `my_serpen_battle`과 동일:
//   **대체 경로만 `live=true`**(tfm2_ai_adjust.rs mp_repl disc14 arm) / **검증(리턴훅) 경로는 `live=false`**(my_movepriority disc14 arm).
//   검증 경로에서 true면 게임이 이미 소비한 draw를 한 번 더 소비 = **이중 소비 desync**.
unsafe fn my_defense_nexus_050(out: usize, cmd: usize, level: i64, rng: usize, sim: usize, geom: usize, tp: usize, sf: usize, live: bool) -> i64 {
    if !ptr_ok(out) || !ptr_ok(cmd) || !ptr_ok(sim) || !ptr_ok(geom) || !ptr_ok(sf) { return -99; }
    let g0 = rd_u64(geom).unwrap_or(0) as usize;
    if !ptr_ok(g0) { return -99; }
    let gchild = rd_u64(g0).unwrap_or(0) as usize;
    let gvt = rd_u64(g0 + 8).unwrap_or(0) as usize;
    if !ptr_ok(gchild) || !ptr_ok(gvt) { return -99; }
    let side = rd_u64(sim + 0x930).unwrap_or(2);//  ★0.5.4 오프셋 이동 반영
    if side > 1 { return -99; }
    let selfe = dd7_slot128(gchild, rd_u64(sim + 0x938).unwrap_or(0));   // self resolve
    if !ptr_ok(selfe) { return -99; }
    // ── 조기 리액티브 (07-10 asm 재RE 정정: 플래그=cmd+0x18|cmd+0x19(구 cmd+3 오식별), S=vt0x30 RDX(구 *(sim+0x818) 오식별)) → 0x14 / 0xf ──
    if rd_u8(cmd + 0x18) != 0 || rd_u8(cmd + 0x19) != 0 {
        let s_ctx = (gchild + 0xec98);
        if !ptr_ok(s_ctx) || rd_u64(s_ctx + 0x1d8).unwrap_or(0) == 0 { wr_u8(out + 8, 0); wr_u64(out, 0xf); return 0xf; }
        let p = rd_u64(s_ctx + 0x1d0).unwrap_or(0) as usize;
        let t = if ptr_ok(p) { geom_resolve150(gchild, rd_u64(p).unwrap_or(0)) } else { 0 };
        if ptr_ok(t) {
            wr_u64(out, 0x14); wr_u64(out + 8, 0); wr_u8(out + 0x10, 1); wr_u8(out + 0x11, rd_u8(cmd + 0x18));
            return 0x14;
        }
        wr_u8(out + 8, 0); wr_u64(out, 0xf); return 0xf;
    }
    // ── ★pre-블록 (07-10 asm 재RE 신규 — phase 게이트들보다 앞. 구모델 누락=game7 vs my 0x10 22건 원인) ──
    //   S=vt0x30 위협리스트: 비면 0xf / T=리스트[0] 엔티티 resolve 실패 0xf / T 풀피일 때만 7 게이트 3종:
    //   ①넥서스렉트(*(geom[1]+0x20)+0x6d70+side*0x20 {xmin,ymin,xmax,ymax}) 안 && HP<max → 7 ②pct<0x33 → 7 ③FUN_141fe2500(=serpen_rng_pick, disc12서 DIFF=0 검증) Some → 7
    let self_max0 = rd_i64(selfe + 0x628).unwrap_or(1).max(1);
    let self_cur0 = rd_i64(selfe + 0x670).unwrap_or(0);
    let pct0 = self_cur0.wrapping_mul(100) / self_max0;
    let s_ctx = (gchild + 0xec98);
    if !ptr_ok(s_ctx) || rd_u64(s_ctx + 0x1d8).unwrap_or(0) == 0 { wr_u8(out + 8, 0); wr_u64(out, 0xf); return 0xf; }   // 위협리스트 비면 0xf
    let p0 = rd_u64(s_ctx + 0x1d0).unwrap_or(0) as usize;
    let t_ent = if ptr_ok(p0) { geom_resolve150(gchild, rd_u64(p0).unwrap_or(0)) } else { 0 };
    if !ptr_ok(t_ent) { wr_u8(out + 8, 0); wr_u64(out, 0xf); return 0xf; }
    let m0 = rd_u64(rd_u64(geom + 8).unwrap_or(0) as usize + 0x20).unwrap_or(0) as usize;   // *(geom[1]+0x20)
    let r0 = m0 + 0x6d70 + (side as usize) * 0x20;   // {x_min@0, y_min@+8, x_max@+0x10, y_max@+0x18}
    let (nx0, ny0) = (rd_u64(selfe + 0x660).unwrap_or(0), rd_u64(selfe + 0x668).unwrap_or(0));
    let inside0 = nx0 >= rd_u64(r0).unwrap_or(u64::MAX) && nx0 <= rd_u64(r0 + 0x10).unwrap_or(0) && ny0 >= rd_u64(r0 + 8).unwrap_or(u64::MAX);
    // ✅★★[07-23 수정 완료 — 구 "disc14 RNG 홀"] ~~`serpen_rng_pick(0, …, false)`~~ → **`(rng, …, live)`**.
    //   결함이었던 것: 0.5.2 원본은 이 지점에서 picker(`0x2135350`)를 **무조건 호출**하고 후보 n>0이면 Lemire 루프로 전역 RNG를
    //   **실제 소비**하는데, 재현이 `rng=0·live=false`라 그 draw를 통째로 빼먹었다 ⟹ disc14 대체 시 **RNG 스트림이 게임보다 뒤처짐 = desync**.
    //   ⚠disc14는 이미 `MP_SAFE_DISC` 편입·라이브였고 code 대조 400/400을 받았으나 **RNG 축은 그 검증이 잡지 못한다**(code만 비교).
    // ★이중소비 안전성 근거(배선 전 확인함): 이 draw 지점 **이후 구간에 `-99` passthrough가 하나도 없다**(함수 내 -99는 전부 진입부
    //   `ptr_ok` 가드 5곳 = draw보다 앞). 따라서 "우리가 draw → -99 반환 → 게임이 재실행하며 또 draw"가 **구조적으로 불가**.
    //   (disc12 `my_serpen_battle`이 target-null 경로에 별도 draw를 넣어야 했던 것과 달리, 여기선 위치 이동 없이 인자만 교체하면 된다.)
    // ★호출 위치는 원본대로 **무조건 호출** 지점 유지 — 아래 "T 풀피" 게이트보다 앞이며, 옮기면 draw 발생 조건이 게임과 갈린다.
    let opt_tag = serpen_rng_pick(rng, sim, rd_u64(geom + 8).unwrap_or(0) as usize, live);
    dl_rec(gchild, 14, opt_tag as u64);   // ★[07-29 detlog] d14 픽tag(site14): draw유무 대조
    if rd_i64(t_ent + 0x670).unwrap_or(0) == rd_i64(t_ent + 0x628).unwrap_or(1) {   // 위협 T 풀피일 때만 7 게이트 활성
        if inside0 && ny0 <= rd_u64(r0 + 0x18).unwrap_or(0) && self_cur0 < self_max0 { wr_u64(out, 7); return 7; }
        if pct0 < tune("sn_self_hp", 0x33) { wr_u64(out, 7); return 7; }
        if opt_tag != 0 { wr_u64(out, 7); return 7; }
    }
    // ── phase1 게이트 (sf+0x3f6 != 1 → DEFEND 7) ──
    //   ★★[07-22] **0.5.2 오프셋 시프트**: ~~0x3ea(0.5.0_3)~~ → 0.5.1 `0x3fa` → **0.5.2 `0x3f6`**.
    //     확증 2점: disc14 원본 `0x2118ef0` asm + 콜리 `FUN_142117ae0`(engage 게이트) 디컴이 `*(char*)(param_1+0x3f6)` 사용.
    //     ⚠0.5.1 마이그에서 이 오프셋이 **통째로 누락**돼 0.5.1·0.5.2 내내 잘못된 분기를 타고 있었다.
    //     ⚠**모드 전역 18곳이 아직 0x3ea/0x3eb**(serpen.rs 291·849·850, tfm2_ai_adjust.rs 3069·3075·3217·3293·3308·3463·3479·
    //       4474·4483·4829·4840·4895). 그 사이트들이 **같은 구조체인지 미검증**이라 일괄치환 금지 — 여기(disc14 sf)만 확정 반영.
    if rd_u8(sf + 0x3f6) != 1 { wr_u64(out, 7); return 7; }
    // ── phase3 (sf+0x3f7 == 3) ──
    if rd_u8(sf + 0x3f7) == 3 {
        if !serpen_reposition_fight(level, sim, geom, 5) {   // FUN_141fe8980 disc5(mode0). ⬜슬롯 lane idx>4||sim+0x8b0!=idx 선체크 생략(best-effort)
            wr_u8(out + 8, 0); wr_u64(out, 0x10); return 0x10;
        }
        let plan = rd_u64(geom + 8).unwrap_or(0) as usize;
        let role = rd_u8(plan + 0x38) as usize;
        let formation = if role < 9 { DEF_FORM_LUT[role] } else { 0 };   // role>=9 가드 = 모드 자체 방어(원본엔 바운즈체크 없음=무경계 JT)
        // ★★[07-22] **code 3 분기 삭제**(0.5.2 대응): ~~`if level > 0x20 { code2 } else { wr_u64(out,3); wr_u8(out+8,formation); return 3 }`~~
        //   0.5.0_3 `0x1c88aea`·0.5.1 `0x1cab960`에 있던 `cmp rbx,0x21; jb` 게이트가 **0.5.2에서 통째 삭제**되고 else-arm이 무조건 실행되도록
        //   상수 인라인됨(대체 분기 없음). ⟹ 0.5.2 출력집합 = {2,7,0xf,0x10,0x11,0x14}로 **code 3 없음**.
        //   재현이 code3을 내면 게임에 존재하지 않는 variant를 커밋하게 됨 = 오염원.
        wr_u64(out, 2); wr_u8(out + 8, 0); wr_u8(out + 9, formation); wr_u8(out + 0xa, 2); return 2;
    }
    // ── [7-A] 홈박스+부상 (07-10 정밀RE: threat 게이트 "앞" 무조건 검사): self 존내 && HP<max → 7 ──
    //   ⚠[08-05 주석 정정] 구 주석의 "하드코딩 넥서스존"은 오기다. 이 영역(geom+0x6d70)은 **부상 유닛 회복존(분수)**이고
    //     넥서스도 에픽도 아니다. 그리고 이 함수는 이름과 달리 **disc14 = EpicPoke(에픽 견제)** 다
    //     — `my_defense_nexus_050`이라는 함수명 자체가 구라벨(tfm2_ai_adjust.rs L7054 참조). 노브는 `sn_home_*`.
    let (sx, sy) = (rd_u64(selfe + 0x660).unwrap_or(0), rd_u64(selfe + 0x668).unwrap_or(0));
    let (ep_lo, ep_hi, ep_x1, ep_y1) = (tune("sn_home_lo", 0xfa00) as u64, tune("sn_home_hi", 0xea600) as u64, tune("sn_home_x1", 0xd9c60) as u64, tune("sn_home_y1", 0xdac00) as u64);
    let in_home = if side == 0 { sx <= ep_lo && sy >= ep_y1 && sy <= ep_hi }
                  else { sx >= ep_x1 && sx <= ep_hi && sy <= ep_lo };
    let self_max = rd_i64(selfe + 0x628).unwrap_or(1).max(1);
    let self_cur = rd_i64(selfe + 0x670).unwrap_or(0);
    if in_home && self_cur < self_max { wr_u64(out, 7); return 7; }
    let pct = self_cur.wrapping_mul(100) / self_max;   // ★self HP%(vt0x138 self 엔티티 — 구 "nexus" 명칭만 정정)
    // ── threat 게이트: min(*(tp+0xc0),*(tp+0xd0)) > 5*scalar(*(geom[1]+8)+0x12f8) ──
    let bctx = rd_u64(rd_u64(geom + 8).unwrap_or(0) as usize + 8).unwrap_or(0) as usize;   // *(geom[1]+8)
    let scalar = rd_i64(bctx + 0x12f8).unwrap_or(0);
    let threat = rd_i64(tp + 0xc0).unwrap_or(0).min(rd_i64(tp + 0xd0).unwrap_or(0));
    if threat > scalar.wrapping_mul(5) {
        // [7-B/C] level>0x2d 다이브추적(cmd+0x1a 상태·cmd Vec{cap@0,ptr@8,len@0x10}).
        //   ★07-10 완전재현: my 계산은 entry 시점(my_movepriority)이라 cmd=pre-state.
        //   pre==0 → 게임은 FUN_141c8c520 호출: my_c8c520(완전재현) tag0(다이버 확보) → 7. (cmd write·오더 emit은 라이브 대체시.)
        //   pre!=0 → 추적 활성: pre-vec 중 self 200k내(dist²<0x9502f9001) 존재 → 7 / 없으면 게임이 리셋 후 폴스루 — 동일 폴스루.
        // ★★[07-22] **진입 게이트 삭제**(0.5.2 대응): ~~`if level > 0x2d {`~~ — 0.5.0_3·0.5.1에 있던 `cmp qword[a3],0x2d; jbe`가
        //   0.5.2에서 제거돼 **threat 게이트 통과 시 무조건 진입**한다(3버전 immediate 기계대조 확정). 조건 없이 실행.
        {
            if rd_u8(cmd + 0x1a) == 0 {
                // pre==0: 게임은 재구축 경로(payload Vec 갱신 + __rust_dealloc = **힙 소유권 이전**)를 타지만,
                //   ★그 경로의 **모든 출구가 예외 없이 code 7로 수렴**(원본 0x2119609 블록 전 출구 추적 확정)
                //   ⟹ 모드는 payload를 건드리지 않고 7만 반환해도 **이번 호출 출력은 원본과 비트동일**.
                //   ⚠게임 힙 재현은 금지(모드 allocator ≠ 게임 __rust_alloc, 교차 free = 힙 파손).
                if my_c8c520(level, sim, geom, selfe) == 0 { wr_u64(out, 7); return 7; }
            } else {
                let vptr = rd_u64(cmd + 8).unwrap_or(0) as usize;
                let vlen = rd_u64(cmd + 0x10).unwrap_or(0) as usize;
                let mut found = false;
                if vptr != 0 && vlen <= 16 {
                    for idx in 0..vlen {
                        let id = rd_u64(vptr + idx * 8).unwrap_or(0) as usize;
                        let e = geom_resolve150(gchild, id as u64);
                        if ptr_ok(e) {
                            let d2 = sqd(rd_u64(e + 0x660).unwrap_or(0), rd_u64(e + 0x668).unwrap_or(0), sx, sy);
                            if d2 < 0x9502f9001 { found = true; break; }   // 다이버 self 200k내 → 추적유지 7
                        }
                    }
                }
                if found { wr_u64(out, 7); return 7; }
                // ★★[07-22] **무효화 write 추가**(원본 0x2119464 재현): 200k내 다이버 없음(또는 len==0) → 캐시 무효화 후 폴스루.
                //   원본은 `payload+0x1a = 0`(b) + `payload+0x10 = 0`(q)만 쓴다. **cap(+0)/ptr(+8)은 절대 건드리지 않는다**
                //   — 그래야 나중에 게임이 재구축할 때 하는 `dealloc(cap*8)`이 여전히 올바르다. 순수 store라 재현 안전.
                //   종전 재현은 이 write를 빠뜨려 게임 상태가 원본과 어긋난 채 폴스루했다.
                wr_u8(cmd + 0x1a, 0);
                wr_u64(cmd + 0x10, 0);
            }
        }
        // [7-D/E/F] 카운팅 서브블록(07-10 정밀RE): obj=vt0x50(sim) 공격자 목록 — *(obj+0x1c0)==0 → 7 /
        //   anchor=vt0x150(sim, **(obj+0x1b8)) 실패 → 7 / pct<=0x14 && allyN<enemyN → 7 (★구모델 부호 반대였음)
        //   카운트=5슬롯 챔프목록(g0+0x1e0+list_side*0x28), 시야게이트(vt0x68 visible-now OR last-seen(geom[2]+list_side*0x2e8+0x1e0+slot*8)+0x78>=tick), 앵커 반경 dist²<0x53d1ac101(150000).
        let obj = geom_vt50(gchild);
        if !ptr_ok(obj) || rd_u64(obj + 0x1c0).unwrap_or(0) == 0 { wr_u64(out, 7); return 7; }
        let hptr = rd_u64(obj + 0x1b8).unwrap_or(0) as usize;
        let anchor = if ptr_ok(hptr) { geom_resolve150(gchild, rd_u64(hptr).unwrap_or(0)) } else { 0 };
        if !ptr_ok(anchor) { wr_u64(out, 7); return 7; }
        let (ax, ay) = (rd_u64(anchor + 0x660).unwrap_or(0), rd_u64(anchor + 0x668).unwrap_or(0));
        let geom2 = rd_u64(geom + 0x10).unwrap_or(0) as usize;
        let tick_now = rd_i64(gchild + 0xec68).unwrap_or(0) as u64;
        let s = side as usize;
        let mut cnts = [0u64; 2];   // [0]=enemyN(list_side=1-s), [1]=allyN(list_side=s)
        for ci in 0..2usize {
            let list_side = if ci == 0 { 1 - s } else { s };
            for k in 0..5usize {
                let e = rd_u64(g0 + 0x1e0 + list_side * 0x28 + k * 8).unwrap_or(0) as usize;
                if e == 0 { continue; }
                let id = rd_u64(e + 0x5c0).unwrap_or(0) as usize;
                let vis = geom_vt68(gchild, s, id as u64);
                if !vis {
                    let ath = geom_vtc0(gchild, id as u64);
                    if !ptr_ok(ath) { continue; }
                    let slot = rd_u32(ath + 0x9c0) as usize;//  ★0.5.4 오프셋 이동 반영
                    if slot > 4 { continue; }
                    let last = rd_u64(geom2 + list_side * 0x2e8 + 0x1e0 + slot * 8).unwrap_or(0);
                    if last.wrapping_add(0x78) < tick_now { continue; }   // last-seen+120틱 창 밖 → 미포함
                }
                let d2 = sqd(rd_u64(e + 0x660).unwrap_or(0), rd_u64(e + 0x668).unwrap_or(0), ax, ay);
                if d2 < 0x53d1ac101 { cnts[ci] += 1; }
            }
        }
        if pct <= tune("sn_hp_crit", 0x14) && cnts[1] < cnts[0] { wr_u64(out, 7); return 7; }   // 아군 열세 → DEFEND
    }
    // ── engage 게이트 (FUN_141fdc2a0 disc5): hold(!=0) → IDLE 0xf / proceed(0) → ENGAGE 0x11 ──
    if serpen_engage_gate(sf, level, sim, geom, tp, 5) { wr_u8(out + 8, 0); wr_u64(out, 0xf); return 0xf; }
    wr_u64(out, 0x11); 0x11
}
