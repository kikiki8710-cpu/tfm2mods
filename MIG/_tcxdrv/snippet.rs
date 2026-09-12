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

struct Cb;
impl rustc_driver::Callbacks for Cb {
    fn after_analysis<'tcx>(&mut self, _c: &rustc_interface::interface::Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        let cstore = CStore::from_tcx(tcx);
        let sm = tcx.sess.source_map();
        for &cnum in tcx.crates(()) {
            if tcx.crate_name(cnum).to_string() != "game_ai" { continue; }
            let n = cstore.num_def_ids_untracked(cnum);
            for i in 0..n {
                let did = DefId { krate: cnum, index: rustc_hir::def_id::DefIndex::from_usize(i) };
                let p = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tcx.def_path_str(did))).unwrap_or_default();
                if !p.contains("EpicHuntAndBattlePlan::sub_plan") { continue; }
                let s = tcx.def_span(did);
                eprintln!("{} -> snippet={:?}", p, sm.span_to_snippet(s));
                let f = sm.lookup_source_file(s.lo());
                eprintln!("  file={:?} src_is_some={} external_src={:?} n_lines={}",
                    f.name, f.src.is_some(), f.external_src.borrow().get_source().is_some(), f.lines().len());
            }
        }
        Compilation::Stop
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::run_compiler(&args, &mut Cb);
}
