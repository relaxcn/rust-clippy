use clippy_utils::mir::{PossibleBorrowerMap, enclosing_mir, expr_local, local_assignments, used_exactly_once};
use clippy_utils::path_to_local;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::mir::{Body, Local, Operand, Place, Rvalue, Statement, StatementKind};
use rustc_session::impl_lint_pass;
extern crate rustc_mir_dataflow;
use crate::dataflow_test::rustc_mir_dataflow::GenKill;
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_index::bit_set::DenseBitSet;
use rustc_middle::mir::pretty::{PrettyPrintMirOptions, write_mir_fn};
use rustc_middle::mir::{Location, TerminatorKind};
use rustc_mir_dataflow::value_analysis::StateData;
use rustc_mir_dataflow::{Analysis, ResultsVisitor};
use std::collections::HashMap;

declare_clippy_lint! {
    /// ### What it does
    /// Dataflow Testing
    #[clippy::version = "1.87.0"]
    pub DATAFLOW_TEST,
    pedantic,
    "default lint description"
}

pub struct DataflowTest {}

impl_lint_pass!(DataflowTest => [DATAFLOW_TEST]);

impl LateLintPass<'_> for DataflowTest {
    fn check_fn(
        &mut self,
        cx: &LateContext<'_>,
        _: rustc_hir::intravisit::FnKind<'_>,
        _: &'_ rustc_hir::FnDecl<'_>,
        _: &'_ rustc_hir::Body<'_>,
        _: rustc_span::Span,
        def_id: rustc_span::def_id::LocalDefId,
    ) {
        let mir = cx.tcx.optimized_mir(def_id.to_def_id());
        let mut possible_borrower = PossibleBorrowerMap::new(cx, mir);

        dbg!(&possible_borrower.map);

        // dbg!(mir);

        if let Some((ss, ss1)) = possible_borrower.map.iter().next() {
            if let Some(sslocal) = mir.local_decls.get(*ss) {
                span_lint_and_help(
                    cx,
                    DATAFLOW_TEST,
                    sslocal.source_info.span,
                    "source from here",
                    None,
                    "",
                );
            }
            for sss in ss1.iter() {
                if let Some(localdecl) = mir.local_decls.get(sss) {
                    // dbg!(localdecl);
                    span_lint_and_help(
                        cx,
                        DATAFLOW_TEST,
                        localdecl.source_info.span,
                        "here msg",
                        None,
                        "help msg",
                    );
                }
            }

            // span_lint_and_help(cx, DATAFLOW_TEST, mir.span , "here msg", None, "help msg");
        }
    }
}

struct MyStorage {}

impl<'tcx> Analysis<'tcx> for MyStorage {
    type Domain = DenseBitSet<Local>;

    const NAME: &'static str = "mystorage";

    fn bottom_value(&self, body: &Body<'tcx>) -> Self::Domain {
        eprintln!("Calling bottom_value for body with {} blocks", body.basic_blocks.len());
        // bottom = dead
        DenseBitSet::new_empty(body.local_decls.len())
    }

    fn initialize_start_block(&self, body: &Body<'tcx>, state: &mut Self::Domain) {}

    fn apply_primary_statement_effect(&mut self, state: &mut Self::Domain, stmt: &Statement<'tcx>, _: Location) {
        match &stmt.kind {
            StatementKind::Assign(l) => {
                dbg!("assign");
            },
            _ => (),
        }
    }
}
