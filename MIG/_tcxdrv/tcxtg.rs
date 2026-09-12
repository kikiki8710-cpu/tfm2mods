#![feature(rustc_private)]
//! tcxtg — 타입 레이아웃 **그래프** 덤프.
//! tcxdump.rs 가 "DefId 별 아이템"을 덤프한다면, 이 드라이버는 대상 크레이트의 모든 ADT 를 씨앗으로
//! **구체 타입(concrete Ty)** 을 BFS 하며 `layout_of` 결과를 전부 적는다.
//! 그래서 `Vec<Shield, Global>` · `[[Option<&Entity>; 5]; 2]` · 튜플 같은
//! **표준 라이브러리/제네릭 인스턴스의 실제 오프셋**까지 컴파일러가 직접 말해준다.
//! 출력: JSON Lines (`TCX_OUT`), 한 줄 = 한 타입.
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_span;
extern crate rustc_session;
extern crate rustc_metadata;
extern crate rustc_abi;

use rustc_driver::Compilation;
use rustc_middle::ty::{self, Ty, TyCtxt, TypingEnv, TypeVisitableExt};
use rustc_hir::def_id::DefId;
use rustc_metadata::creader::CStore;
use rustc_abi::HasDataLayout;
use std::collections::HashSet;
use std::io::Write;

fn jesc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o
}

fn catch<T>(f: impl FnOnce() -> T) -> Option<T> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).ok()
}

struct Cb;

impl rustc_driver::Callbacks for Cb {
    fn after_analysis<'tcx>(&mut self, _c: &rustc_interface::interface::Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        let cstore = CStore::from_tcx(tcx);
        let want = std::env::var("TCX_CRATE").unwrap_or_else(|_| String::from("game_ai"));
        let out = std::env::var("TCX_OUT").unwrap_or_else(|_| format!("C:/tfm2mods/MIG/_tcx/tg_{}.jsonl", want));
        let mut f = std::io::BufWriter::new(std::fs::File::create(&out).unwrap());
        let env = TypingEnv::fully_monomorphized();

        // ---- 씨앗 수집: 대상 크레이트의 모든 Struct/Enum/Union 의 identity 타입
        let mut work: Vec<Ty<'tcx>> = Vec::new();
        for &cnum in tcx.crates(()) {
            if tcx.crate_name(cnum).to_string() != want { continue; }
            let n = cstore.num_def_ids_untracked(cnum);
            eprintln!("crate {} ndefs={}", want, n);
            for i in 0..n {
                let did = DefId { krate: cnum, index: rustc_hir::def_id::DefIndex::from_usize(i) };
                let dk = match catch(|| tcx.def_kind(did)) { Some(v) => v, None => continue };
                use rustc_hir::def::DefKind as DK;
                // 씨앗 ②: 함수 시그니처의 인자/반환 타입도 넣는다.
                //   (구조체 필드에서 도달 못하는 `Option<Input>` 같은 sret 반환 슬롯을 잡기 위함)
                if matches!(dk, DK::Fn | DK::AssocFn) {
                    if let Some(Some(tys)) = catch(|| {
                        if tcx.generics_of(did).count() != 0 { return None; }
                        // ★late-bound 리전을 반드시 지운다. skip_binder() 만 쓰면 `'^3.Named(DefId(..))`
                        //   같은 bound var 가 타입 문자열에 남아 같은 타입이 함수 수만큼 갈라진다.
                        let sig = tcx.instantiate_bound_regions_with_erased(
                            tcx.fn_sig(did).instantiate_identity().skip_normalization());
                        let mut v: Vec<Ty<'tcx>> = sig.inputs().iter().copied().collect();
                        v.push(sig.output());
                        Some(v)
                    }) {
                        for t0 in tys {
                            if let Some(t1) = catch(|| tcx.erase_and_anonymize_regions(t0)) {
                                if !t1.has_param() { work.push(t1); }
                            }
                        }
                    }
                }
                // 씨앗 ③: MIR 지역변수 타입(있을 때만). 지역변수 전용 모노모피 인스턴스
                //   (`bumpalo::Vec<JungleType>` 같은 것)를 잡는다. TCX_LOCALS=0 으로 끌 수 있다.
                if std::env::var("TCX_LOCALS").ok().as_deref() != Some("0")
                    && matches!(dk, DK::Fn | DK::AssocFn | DK::Closure) {
                    if let Some(Some(tys)) = catch(|| {
                        if tcx.generics_of(did).count() != 0 { return None; }
                        if !tcx.is_mir_available(did) { return None; }
                        let m = tcx.optimized_mir(did);
                        Some(m.local_decls.iter().map(|l| l.ty).collect::<Vec<_>>())
                    }) {
                        for t0 in tys {
                            if let Some(t1) = catch(|| tcx.erase_and_anonymize_regions(t0)) {
                                if !t1.has_param() { work.push(t1); }
                            }
                        }
                    }
                }
                if matches!(dk, DK::Fn | DK::AssocFn) { continue; }
                if !matches!(dk, DK::Struct | DK::Enum | DK::Union) { continue; }
                if let Some(Some(t)) = catch(|| {
                    let t0 = tcx.type_of(did).instantiate_identity().skip_normalization();
                    let t1 = tcx.erase_and_anonymize_regions(t0);
                    if t1.has_param() { None } else { Some(t1) }
                }) { work.push(t); }
            }
        }
        eprintln!("seeds={}", work.len());

        let mut seen: HashSet<String> = HashSet::new();
        let mut done = 0usize;
        while let Some(ty) = work.pop() {
            let key = match catch(|| format!("{:?}", ty)) { Some(k) => k, None => continue };
            if !seen.insert(key.clone()) { continue; }
            done += 1;
            if done % 20000 == 0 { eprintln!("  .. types={} queue={}", done, work.len()); }

            let lay = catch(|| tcx.layout_of(env.as_query_input(ty)).ok()).flatten();
            let mut s = String::with_capacity(256);
            s.push_str(&format!("{{\"t\":\"{}\"", jesc(&key)));
            if let Some(l) = &lay {
                s.push_str(&format!(",\"sz\":{},\"al\":{}", l.size.bytes(), l.align.abi.bytes()));
            }

            // 자식 타입을 안전하게 정규화해서 얻는다.
            let norm = |u: ty::Unnormalized<'tcx, Ty<'tcx>>| -> Option<Ty<'tcx>> {
                catch(|| tcx.normalize_erasing_regions(env, u))
            };

            let mut push = |t: Ty<'tcx>, w: &mut Vec<Ty<'tcx>>| {
                if !t.has_param() { w.push(t); }
            };

            match ty.kind() {
                ty::Adt(def, args) => {
                    let kind = if def.is_enum() { "enum" } else if def.is_union() { "union" } else { "struct" };
                    s.push_str(&format!(",\"k\":\"{}\",\"p\":\"{}\"", kind, jesc(&tcx.def_path_str(def.did()))));
                    // 태그 인코딩
                    if let Some(l) = &lay {
                        if let rustc_abi::Variants::Multiple { tag, tag_encoding, tag_field, .. } = &l.variants {
                            let toff = if let rustc_abi::FieldsShape::Arbitrary { offsets, .. } = &l.fields {
                                offsets.get(*tag_field).map(|o| o.bytes())
                            } else { None };
                            let tsz = catch(|| tag.size(&tcx).bytes());
                            s.push_str(&format!(",\"tag\":{{\"off\":{},\"sz\":{}",
                                toff.map(|x| x.to_string()).unwrap_or_else(|| "null".into()),
                                tsz.map(|x| x.to_string()).unwrap_or_else(|| "null".into())));
                            match tag_encoding {
                                rustc_abi::TagEncoding::Direct => s.push_str(",\"enc\":\"Direct\"}"),
                                rustc_abi::TagEncoding::Niche { untagged_variant, niche_variants, niche_start } => {
                                    s.push_str(&format!(",\"enc\":\"Niche\",\"untagged\":{},\"nv0\":{},\"nv1\":{},\"nstart\":{}}}",
                                        untagged_variant.as_u32(), niche_variants.start().as_u32(),
                                        niche_variants.end().as_u32(), niche_start));
                                }
                            }
                        }
                    }
                    s.push_str(",\"vs\":[");
                    for (vi, v) in def.variants().iter_enumerated() {
                        if vi.as_u32() != 0 { s.push(','); }
                        let discr = catch(|| {
                            format!("{:?}", ty.discriminant_for_variant(tcx, vi))
                        }).unwrap_or_else(|| "null".into());
                        s.push_str(&format!("{{\"n\":\"{}\",\"i\":{},\"d\":\"{}\",\"f\":[",
                            jesc(v.name.as_str()), vi.as_u32(), jesc(&discr)));
                        // 이 variant 의 필드 오프셋
                        let voff: Option<Vec<u64>> = lay.as_ref().and_then(|l| {
                            match &l.variants {
                                rustc_abi::Variants::Multiple { variants, .. } =>
                                    variants.get(vi).map(|x| x.field_offsets.iter().map(|o| o.bytes()).collect()),
                                _ => match &l.fields {
                                    rustc_abi::FieldsShape::Arbitrary { offsets, .. } => Some(offsets.iter().map(|o| o.bytes()).collect()),
                                    rustc_abi::FieldsShape::Union(_) => Some(v.fields.iter().map(|_| 0u64).collect()),
                                    _ => None,
                                },
                            }
                        });
                        for (fi, fd) in v.fields.iter_enumerated() {
                            if fi.as_u32() != 0 { s.push(','); }
                            let fty = catch(|| fd.ty(tcx, args)).and_then(norm);
                            let fts = fty.map(|t| format!("{:?}", t)).unwrap_or_else(|| "?".into());
                            let o = voff.as_ref().and_then(|v| v.get(fi.as_usize()).copied());
                            s.push_str(&format!("{{\"n\":\"{}\",\"t\":\"{}\",\"o\":{}}}",
                                jesc(fd.name.as_str()), jesc(&fts),
                                o.map(|x| x.to_string()).unwrap_or_else(|| "null".into())));
                            if let Some(t) = fty { push(t, &mut work); }
                        }
                        s.push_str("]}");
                    }
                    s.push(']');
                }
                ty::Array(el, len) => {
                    let cnt = catch(|| len.try_to_target_usize(tcx)).flatten();
                    let stride = lay.as_ref().and_then(|l| match &l.fields {
                        rustc_abi::FieldsShape::Array { stride, count } => Some((stride.bytes(), *count)),
                        _ => None,
                    });
                    s.push_str(&format!(",\"k\":\"array\",\"el\":\"{}\",\"cnt\":{},\"stride\":{}",
                        jesc(&format!("{:?}", el)),
                        stride.map(|x| x.1).or(cnt).map(|x| x.to_string()).unwrap_or_else(|| "null".into()),
                        stride.map(|x| x.0.to_string()).unwrap_or_else(|| "null".into())));
                    push(*el, &mut work);
                }
                ty::Slice(el) => {
                    s.push_str(&format!(",\"k\":\"slice\",\"el\":\"{}\"", jesc(&format!("{:?}", el))));
                    push(*el, &mut work);
                }
                ty::Tuple(ts) => {
                    let off: Option<Vec<u64>> = lay.as_ref().and_then(|l| match &l.fields {
                        rustc_abi::FieldsShape::Arbitrary { offsets, .. } => Some(offsets.iter().map(|o| o.bytes()).collect()),
                        _ => None,
                    });
                    s.push_str(",\"k\":\"tuple\",\"f\":[");
                    for (i, t) in ts.iter().enumerate() {
                        if i > 0 { s.push(','); }
                        let o = off.as_ref().and_then(|v| v.get(i).copied());
                        s.push_str(&format!("{{\"n\":\"{}\",\"t\":\"{}\",\"o\":{}}}", i, jesc(&format!("{:?}", t)),
                            o.map(|x| x.to_string()).unwrap_or_else(|| "null".into())));
                        push(t, &mut work);
                    }
                    s.push(']');
                }
                ty::Ref(_, el, _) => {
                    s.push_str(&format!(",\"k\":\"ref\",\"el\":\"{}\"", jesc(&format!("{:?}", el))));
                    push(*el, &mut work);
                }
                ty::RawPtr(el, _) => {
                    s.push_str(&format!(",\"k\":\"ptr\",\"el\":\"{}\"", jesc(&format!("{:?}", el))));
                    push(*el, &mut work);
                }
                _ => {
                    s.push_str(&format!(",\"k\":\"{}\"", jesc(&format!("{:?}", ty.kind()).split(&['(', '{', ' '][..]).next().unwrap_or("other"))));
                }
            }
            s.push_str("}\n");
            f.write_all(s.as_bytes()).unwrap();
        }
        f.flush().unwrap();
        eprintln!("wrote {} ({} types)", out, done);
        Compilation::Stop
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::run_compiler(&args, &mut Cb);
}
