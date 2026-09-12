#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_span;
extern crate rustc_session;
extern crate rustc_metadata;

use rustc_driver::Compilation;
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;
use rustc_metadata::creader::CStore;
use std::io::Write;

fn span_str(tcx: TyCtxt<'_>, s: rustc_span::Span) -> String {
    let sm = tcx.sess.source_map();
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let lo = sm.lookup_char_pos(s.lo());
        let hi = sm.lookup_char_pos(s.hi());
        format!("{}:{}:{}-{}:{}", sm.filename_for_diagnostics(&lo.file.name), lo.line, lo.col.0 + 1, hi.line, hi.col.0 + 1)
    })) { Ok(v) => v, Err(_) => "<span?>".to_string() }
}

struct Cb;
impl rustc_driver::Callbacks for Cb {
    fn after_analysis<'tcx>(&mut self, _c: &rustc_interface::interface::Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        let cstore = CStore::from_tcx(tcx);
        let out = std::env::var("TCX_OUT").unwrap_or_else(|_| String::from("C:/tfm2mods/MIG/_tcx/mirchk.txt"));
        let mut f = std::fs::File::create(&out).unwrap();
        for &cnum in tcx.crates(()) {
            let name = tcx.crate_name(cnum).to_string();
            if name != std::env::var("TCX_CRATE").unwrap_or_else(|_| String::from("game_ai")) { continue; }
            let n = cstore.num_def_ids_untracked(cnum);
            let mut kinds: std::collections::BTreeMap<String,(usize,usize)> = Default::default();
            for i in 0..n {
                let did = DefId { krate: cnum, index: rustc_hir::def_id::DefIndex::from_usize(i) };
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let dk = tcx.def_kind(did);
                    let mir = tcx.is_mir_available(did);
                    (format!("{:?}", dk), mir)
                }));
                let (dk, mir) = match r { Ok(v) => v, Err(_) => ("<ERR>".to_string(), false) };
                let e = kinds.entry(dk.clone()).or_insert((0,0));
                e.0 += 1; if mir { e.1 += 1; }
                if dk == "Fn" || dk == "AssocFn" || dk == "Closure" || dk == "Ctor" || dk.starts_with("Const") || dk == "Static" {
                    let p = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tcx.def_path_str(did)))
                        .unwrap_or("<path?>".into());
                    let sp = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tcx.def_span(did)))
                        .map(|s| span_str(tcx, s)).unwrap_or("<span?>".into());
                    let inl = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tcx.cross_crate_inlinable(did))).unwrap_or(false);
                    writeln!(f, "{}\t{}\t{}\tmir={}\txinl={}\t{}\t{}", name, i, dk, mir as u8, inl as u8, p, sp).unwrap();
                }
            }
            eprintln!("=== {} ndefs={} ===", name, n);
            for (k,(t,m)) in &kinds { eprintln!("  {:20} total={:6} mir={}", k, t, m); }
        }
        Compilation::Stop
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::run_compiler(&args, &mut Cb);
}
