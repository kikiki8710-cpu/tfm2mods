#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_span;
extern crate rustc_session;

use rustc_driver::Compilation;
use rustc_middle::ty::TyCtxt;

fn span_str(tcx: TyCtxt<'_>, s: rustc_span::Span) -> String {
    let sm = tcx.sess.source_map();
    let lo = sm.lookup_char_pos(s.lo());
    let hi = sm.lookup_char_pos(s.hi());
    format!("{}:{}:{}-{}:{}", sm.filename_for_diagnostics(&lo.file.name), lo.line, lo.col.0 + 1, hi.line, hi.col.0 + 1)
}

struct Cb;
impl rustc_driver::Callbacks for Cb {
    fn after_analysis<'tcx>(&mut self, _c: &rustc_interface::interface::Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        for id in tcx.hir_crate_items(()).definitions() {
            let did = id.to_def_id();
            eprintln!("{:?}\t{}\t{}", tcx.def_kind(did), tcx.def_path_str(did), span_str(tcx, tcx.def_span(did)));
        }
        Compilation::Stop
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::run_compiler(&args, &mut Cb);
}
