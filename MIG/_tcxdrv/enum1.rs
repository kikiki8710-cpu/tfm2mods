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
        for &cnum in tcx.crates(()) {
            let name = tcx.crate_name(cnum);
            if name.as_str() != "game_ai" { continue; }
            let n = cstore.num_def_ids_untracked(cnum);
            eprintln!("crate {} cnum={:?} ndefs={}", name, cnum, n);
            let mut mir_ok = 0usize; let mut total = 0usize;
            for i in 0..n {
                let did = DefId { krate: cnum, index: rustc_hir::def_id::DefIndex::from_usize(i) };
                total += 1;
                if tcx.is_mir_available(did) { mir_ok += 1; }
            }
            eprintln!("mir_ok={} / {}", mir_ok, total);
        }
        Compilation::Stop
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::run_compiler(&args, &mut Cb);
}
