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
        let want = std::env::var("TCX_CRATE").unwrap_or_else(|_| String::from("game_ai"));
        let out = std::env::var("TCX_OUT").unwrap_or_else(|_| String::from("C:/tfm2mods/MIG/_tcx/mirdump_game_ai.txt"));
        let mut f = std::io::BufWriter::new(std::fs::File::create(&out).unwrap());
        let mut n_fn = 0usize;
        for &cnum in tcx.crates(()) {
            if tcx.crate_name(cnum).to_string() != want { continue; }
            let n = cstore.num_def_ids_untracked(cnum);
            for i in 0..n {
                let did = DefId { krate: cnum, index: rustc_hir::def_id::DefIndex::from_usize(i) };
                let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tcx.is_mir_available(did))).unwrap_or(false);
                if !ok { continue; }
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let body = tcx.optimized_mir(did);
                    let mut s = String::new();
                    s.push_str(&format!("### {} [{}] {}\n", tcx.def_path_str(did), i, span_str(tcx, tcx.def_span(did))));
                    for (bb, data) in body.basic_blocks.iter_enumerated() {
                        s.push_str(&format!("  {:?}:\n", bb));
                        for st in &data.statements {
                            s.push_str(&format!("    {:?}   @{}\n", st, span_str(tcx, st.source_info.span)));
                        }
                        if let Some(t) = &data.terminator {
                            s.push_str(&format!("    T {:?}   @{}\n", t.kind, span_str(tcx, t.source_info.span)));
                        }
                    }
                    s
                }));
                match r { Ok(s) => { f.write_all(s.as_bytes()).unwrap(); n_fn += 1; }, Err(_) => { writeln!(f, "### <ERR> [{}]", i).unwrap(); } }
            }
        }
        eprintln!("dumped {} bodies -> {}", n_fn, out);
        Compilation::Stop
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::run_compiler(&args, &mut Cb);
}
