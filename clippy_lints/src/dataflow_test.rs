use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::mir::{Body, Local, Statement, StatementKind, Rvalue, Operand, Place};
use rustc_session::impl_lint_pass;
use clippy_utils::mir::{PossibleBorrowerMap, enclosing_mir, expr_local, local_assignments, used_exactly_once};
use clippy_utils::path_to_local;
extern crate rustc_mir_dataflow;
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_mir_dataflow::{Analysis, ResultsVisitor};
use rustc_mir_dataflow::value_analysis::StateData;
use rustc_middle::mir::Location;
use rustc_index::bit_set::DenseBitSet;
use std::collections::HashMap;
use crate::dataflow_test::rustc_mir_dataflow::GenKill;
use rustc_middle::mir::TerminatorKind;
use rustc_middle::mir::pretty::write_mir_fn;
use rustc_middle::mir::pretty::PrettyPrintMirOptions;

declare_clippy_lint! {
    /// ### What it does
    /// Dataflow Testing
    #[clippy::version = "1.87.0"]
    pub DATAFLOW_TEST,
    pedantic,
    "default lint description"
}

pub struct DataflowTest {

}

impl_lint_pass!(DataflowTest => [DATAFLOW_TEST]);

impl LateLintPass<'_> for DataflowTest {
    // fn check_fn(&mut self, cx: &LateContext<'_>,_:rustc_hir::intravisit::FnKind<'_>,_: &'_ rustc_hir::FnDecl<'_>, _: &'_ rustc_hir::Body<'_>,_:rustc_span::Span, def_id :rustc_span::def_id::LocalDefId) {
    //     dbg!("fn");
    //     let tcx = cx.tcx;
    //     let body = cx.tcx.optimized_mir(def_id);
    //     let storage = MyStorage{};
    //     let mut results = storage.iterate_to_fixpoint(tcx, body, None).into_results_cursor(body);
    // }

    // fn check_stmt(&mut self, cx: &LateContext<'_>, stmt: &'_ rustc_hir::Stmt<'_>) {
    //     if let StmtKind::Let(letstmt) = stmt.kind {
    //         if let PatKind::Binding(_, _, ident, _) = letstmt.pat.kind {
    //             if 'a' == ident.name {
    //                 let body = enclosing_mir(tcx, stmt.hir_id); 
    //                 let pp = PossibleBorrowerMap::new(cx, body);
    //             }
    //         }
    //     }
    // }

    fn check_fn(&mut self, cx: &LateContext<'_>,_:rustc_hir::intravisit::FnKind<'_>,_: &'_ rustc_hir::FnDecl<'_>, _: &'_ rustc_hir::Body<'_>,_:rustc_span::Span, def_id:rustc_span::def_id::LocalDefId) {
        let mir = cx.tcx.optimized_mir(def_id.to_def_id());
        let body: &Body<'_> = cx.tcx.optimized_mir(def_id);
        let mut possible_borrower = PossibleBorrowerMap::new(cx, mir);

        let options = PrettyPrintMirOptions::from_cli(cx.tcx);
        // write_mir_fn(
        //     cx.tcx,
        //     body,
        //     &mut |_, _| Ok(()),
        //     &mut std::io::stdout(),
        //     options,
        // )
        // .unwrap();

        // dbg!(&mir.local_decls);

        for (ss, ss1) in possible_borrower.map {
            for sss in ss1.iter() {
                if let Some(localdecl) = mir.local_decls.get(sss) {
                    // dbg!(localdecl);
                    span_lint_and_help(cx, DATAFLOW_TEST, localdecl.source_info.span , "here msg", None, "help msg");
                }
            }

            // span_lint_and_help(cx, DATAFLOW_TEST, mir.span , "here msg", None, "help msg");
        }

        
    }

    // fn check_(&mut self, cx: &LateContext<'_>,_:rustc_hir::intravisit::FnKind<'_>,_: &'_ rustc_hir::FnDecl<'_>, body: &'_ rustc_hir::Body<'_>,_:rustc_span::Span, def_id:rustc_span::def_id::LocalDefId) {
    //     println!("fn");
    //     body.
    //     let pp = PossibleBorrowerMap::new(cx, body);
    //     dbg!(pp.map);
    // }
}

struct MyStorage{}

impl<'tcx> Analysis<'tcx> for MyStorage {
    type Domain = DenseBitSet<Local>;

    const NAME: &'static str = "mystorage";

    fn bottom_value(&self, body: &Body<'tcx>) -> Self::Domain {
        eprintln!("Calling bottom_value for body with {} blocks", body.basic_blocks.len());
        // bottom = dead
        DenseBitSet::new_empty(body.local_decls.len())

    }

    fn initialize_start_block(&self, body: &Body<'tcx>, state: &mut Self::Domain) {
    }

    fn apply_primary_statement_effect(
        &mut self,
        state: &mut Self::Domain,
        stmt: &Statement<'tcx>,
        _: Location,
    ) {

        match &stmt.kind {
            StatementKind::Assign(l) => {
                dbg!("assign");
            },
            _ => (),
        }
    }
}
