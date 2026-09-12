#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_span;
extern crate rustc_session;
extern crate rustc_metadata;
extern crate rustc_abi;

use rustc_driver::Compilation;
use rustc_middle::ty::{TyCtxt, TypingEnv, TypeVisitableExt};
use rustc_hir::def_id::DefId;
use rustc_metadata::creader::CStore;
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

fn span_json(tcx: TyCtxt<'_>, s: rustc_span::Span) -> String {
    let sm = tcx.sess.source_map();
    match catch(|| {
        let lo = sm.lookup_char_pos(s.lo());
        let hi = sm.lookup_char_pos(s.hi());
        format!("{{\"f\":\"{}\",\"l\":{},\"c\":{},\"l2\":{},\"c2\":{}}}",
            jesc(&format!("{}", sm.filename_for_diagnostics(&lo.file.name))),
            lo.line, lo.col.0 + 1, hi.line, hi.col.0 + 1)
    }) { Some(v) => v, None => "null".to_string() }
}

struct Cb;
impl rustc_driver::Callbacks for Cb {
    fn after_analysis<'tcx>(&mut self, _c: &rustc_interface::interface::Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        let cstore = CStore::from_tcx(tcx);
        let want = std::env::var("TCX_CRATE").unwrap_or_else(|_| String::from("game_ai"));
        let out = std::env::var("TCX_OUT").unwrap_or_else(|_| format!("C:/tfm2mods/MIG/_tcx/{}.json", want));
        let mut f = std::io::BufWriter::new(std::fs::File::create(&out).unwrap());
        let env = TypingEnv::fully_monomorphized();
        for &cnum in tcx.crates(()) {
            if tcx.crate_name(cnum).to_string() != want { continue; }
            let n = cstore.num_def_ids_untracked(cnum);
            eprintln!("crate {} ndefs={}", want, n);
            write!(f, "{{\"crate\":\"{}\",\"n\":{},\"items\":[", want, n).unwrap();
            let mut first = true;
            for i in 0..n {
                if i % 10000 == 0 { eprintln!("  .. {}/{}", i, n); }
                let did = DefId { krate: cnum, index: rustc_hir::def_id::DefIndex::from_usize(i) };
                let dk = match catch(|| tcx.def_kind(did)) { Some(v) => v, None => continue };
                let dks = format!("{:?}", dk);
                let path = catch(|| tcx.def_path_str(did)).unwrap_or_else(|| String::from("<?>"));
                let sp = catch(|| tcx.def_span(did)).map(|s| span_json(tcx, s)).unwrap_or_else(|| String::from("null"));
                use rustc_hir::def::DefKind as DK;
                let has_vis = matches!(dk, DK::Mod | DK::Struct | DK::Enum | DK::Union | DK::Trait | DK::TraitAlias
                    | DK::TyAlias | DK::ForeignTy | DK::Fn | DK::AssocFn | DK::Const { .. } | DK::AssocConst { .. }
                    | DK::Static { .. } | DK::Variant | DK::Field | DK::Ctor(..) | DK::Macro(..) | DK::AssocTy);
                let vis = if has_vis {
                    catch(|| match tcx.visibility(did) {
                        rustc_middle::ty::Visibility::Public => String::from("pub"),
                        rustc_middle::ty::Visibility::Restricted(d) => format!("in:{}", tcx.def_path_str(d)),
                    }).unwrap_or_else(|| String::from("?"))
                } else { String::from("-") };
                let is_body = matches!(dk, DK::Fn | DK::AssocFn | DK::Closure | DK::Const { .. } | DK::AssocConst { .. }
                    | DK::Static { .. } | DK::AnonConst | DK::InlineConst | DK::Ctor(..));
                let mir = if is_body { catch(|| tcx.is_mir_available(did)).unwrap_or(false) } else { false };
                let xinl = if is_body { catch(|| tcx.cross_crate_inlinable(did)).unwrap_or(false) } else { false };
                if !first { write!(f, ",\n").unwrap(); }
                first = false;
                write!(f, "{{\"i\":{},\"k\":\"{}\",\"p\":\"{}\",\"v\":\"{}\",\"sp\":{},\"mir\":{},\"xinl\":{}",
                    i, jesc(&dks), jesc(&path), jesc(&vis), sp, mir, xinl).unwrap();

                if matches!(dk, rustc_hir::def::DefKind::Fn | rustc_hir::def::DefKind::AssocFn) {
                    if let Some(sig) = catch(|| format!("{:?}", tcx.fn_sig(did).instantiate_identity().skip_binder())) {
                        write!(f, ",\"sig\":\"{}\"", jesc(&sig)).unwrap();
                    }
                    let gen_cnt = catch(|| tcx.generics_of(did).own_params.len()).unwrap_or(0);
                    write!(f, ",\"ngen\":{}", gen_cnt).unwrap();
                    if let Some(ia) = catch(|| format!("{:?}", tcx.codegen_fn_attrs(did).inline)) {
                        write!(f, ",\"inline\":\"{}\"", jesc(&ia)).unwrap();
                    }
                }

                if matches!(dk, rustc_hir::def::DefKind::Struct | rustc_hir::def::DefKind::Enum | rustc_hir::def::DefKind::Union) {
                    if let Some(adt) = catch(|| tcx.adt_def(did)) {
                        let mut s = String::from(",\"adt\":{\"repr\":\"");
                        s.push_str(&jesc(&format!("{:?}", adt.repr())));
                        s.push_str("\",\"variants\":[");
                        let mut vf = true;
                        for v in adt.variants() {
                            if !vf { s.push(','); }
                            vf = false;
                            let discr = catch(|| {
                                let ty = tcx.type_of(did).instantiate_identity().skip_normalization();
                                let vi = adt.variant_index_with_id(v.def_id);
                                format!("{:?}", ty.discriminant_for_variant(tcx, vi))
                            }).unwrap_or_else(|| String::from("null"));
                            let vdsp = catch(|| tcx.def_span(v.def_id)).map(|x| span_json(tcx, x)).unwrap_or_else(|| String::from("null"));
                            s.push_str(&format!("{{\"n\":\"{}\",\"discr\":\"{}\",\"sp\":{},\"fields\":[",
                                jesc(v.name.as_str()), jesc(&discr), vdsp));
                            let mut ff = true;
                            for fd in &v.fields {
                                if !ff { s.push(','); }
                                ff = false;
                                let ty = catch(|| format!("{:?}", tcx.type_of(fd.did).instantiate_identity().skip_normalization())).unwrap_or_else(|| String::from("?"));
                                s.push_str(&format!("{{\"n\":\"{}\",\"t\":\"{}\"}}", jesc(fd.name.as_str()), jesc(&ty)));
                            }
                            s.push_str("]}");
                        }
                        s.push_str("]}");
                        write!(f, "{}", s).unwrap();
                    }
                    let lay = catch(|| {
                        let ty0 = tcx.type_of(did).instantiate_identity().skip_normalization();
                        let ty = tcx.erase_and_anonymize_regions(ty0);
                        if ty.has_param() { return None; }
                        tcx.layout_of(env.as_query_input(ty)).ok().map(|l| {
                            let mut s = format!("{{\"size\":{},\"align\":{}", l.size.bytes(), l.align.abi.bytes());
                            if let rustc_abi::FieldsShape::Arbitrary { offsets, .. } = &l.fields {
                                s.push_str(",\"offsets\":[");
                                for (k, o) in offsets.iter().enumerate() {
                                    if k > 0 { s.push(','); }
                                    s.push_str(&format!("{}", o.bytes()));
                                }
                                s.push(']');
                            }
                            match &l.variants {
                                rustc_abi::Variants::Multiple { tag, tag_encoding, tag_field, variants } => {
                                    s.push_str(&format!(",\"tag\":\"{}\",\"tag_enc\":\"{}\",\"tag_field\":\"{}\",\"vlayouts\":[",
                                        jesc(&format!("{:?}", tag)), jesc(&format!("{:?}", tag_encoding)), jesc(&format!("{:?}", tag_field))));
                                    for (k, v) in variants.iter().enumerate() {
                                        if k > 0 { s.push(','); }
                                        s.push_str(&format!("{{\"size\":{},\"offsets\":[", v.size.bytes()));
                                        for (m, o) in v.field_offsets.iter().enumerate() {
                                            if m > 0 { s.push(','); }
                                            s.push_str(&format!("{}", o.bytes()));
                                        }
                                        s.push_str("]}");
                                    }
                                    s.push(']');
                                }
                                rustc_abi::Variants::Single { index } => {
                                    s.push_str(&format!(",\"single\":{}", index.as_usize()));
                                }
                                _ => {}
                            }
                            s.push('}');
                            s
                        })
                    }).flatten();
                    if let Some(l) = lay { write!(f, ",\"layout\":{}", l).unwrap(); }
                }
                write!(f, "}}").unwrap();
            }
            write!(f, "],\"files\":[").unwrap();
            let sm = tcx.sess.source_map();
            let mut ffirst = true;
            for sf in sm.files().iter() {
                let nm = format!("{}", sm.filename_for_diagnostics(&sf.name));
                if !ffirst { write!(f, ",\n").unwrap(); }
                ffirst = false;
                let lines = sf.lines();
                write!(f, "{{\"f\":\"{}\",\"start\":{},\"len\":{},\"nl\":{},\"lines\":[",
                    jesc(&nm), sf.start_pos.0, sf.normalized_source_len.0, lines.len()).unwrap();
                for (k, l) in lines.iter().enumerate() {
                    if k > 0 { write!(f, ",").unwrap(); }
                    write!(f, "{}", l.0).unwrap();
                }
                write!(f, "]}}").unwrap();
            }
            write!(f, "]}}").unwrap();
        }
        f.flush().unwrap();
        eprintln!("wrote {}", out);
        Compilation::Stop
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::run_compiler(&args, &mut Cb);
}
