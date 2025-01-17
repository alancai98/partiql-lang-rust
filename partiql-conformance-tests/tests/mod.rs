use partiql_ast_passes::error::AstTransformationError;
use partiql_catalog::{Catalog, PartiqlCatalog};
use partiql_eval as eval;
use partiql_eval::env::basic::MapBindings;
use partiql_eval::error::PlanErr;
use partiql_eval::eval::{EvalPlan, EvalResult};
use partiql_logical as logical;
use partiql_parser::{Parsed, ParserResult};
use partiql_value::Value;

mod test_value;
pub(crate) use test_value::TestValue;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub(crate) enum EvaluationMode {
    Coerce,
    Error,
}

#[track_caller]
#[inline]
pub(crate) fn parse(statement: &str) -> ParserResult {
    partiql_parser::Parser::default().parse(statement)
}

#[track_caller]
#[inline]
pub(crate) fn lower(
    catalog: &dyn Catalog,
    parsed: &Parsed,
) -> Result<logical::LogicalPlan<logical::BindingsOp>, AstTransformationError> {
    let planner = partiql_logical_planner::LogicalPlanner::new(catalog);
    planner.lower(parsed)
}

#[track_caller]
#[inline]
pub(crate) fn compile(
    catalog: &dyn Catalog,
    logical: logical::LogicalPlan<logical::BindingsOp>,
) -> Result<EvalPlan, PlanErr> {
    let mut planner = eval::plan::EvaluatorPlanner::new(catalog);
    planner.compile(&logical)
}

#[track_caller]
#[inline]
pub(crate) fn evaluate(mut plan: EvalPlan, bindings: MapBindings<Value>) -> EvalResult {
    plan.execute_mut(bindings)
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn fail_syntax(statement: &str) {
    let res = parse(statement);
    assert!(
        res.is_err(),
        "When parsing `{statement}`, expected `Err(_)`, but was `{res:#?}`"
    );
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn pass_syntax(statement: &str) -> Parsed {
    let res = parse(statement);
    assert!(
        res.is_ok(),
        "When parsing `{statement}`, expected `Ok(_)`, but was `{res:#?}`"
    );
    res.unwrap()
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn fail_semantics(statement: &str) {
    let catalog = PartiqlCatalog::default();
    let parsed = parse(statement);
    let lowered = parsed.map(|parsed| lower(&catalog, &parsed));
    assert!(
        lowered.is_err(),
        "When semantically verifying `{statement}`, expected `Err(_)`, but was `{lowered:#?}`"
    );
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn pass_semantics(statement: &str) {
    let catalog = PartiqlCatalog::default();
    let parsed = pass_syntax(statement);
    let lowered = lower(&catalog, &parsed);
    assert!(
        lowered.is_ok(),
        "When semantically verifying `{statement}`, expected `Ok(_)`, but was `{lowered:#?}`"
    );
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn fail_eval(statement: &str, mode: EvaluationMode, env: &Option<TestValue>) {
    let catalog = PartiqlCatalog::default();

    let parsed = parse(statement);
    let lowered_result = lower(&catalog, &parsed.expect("parse"));
    let lowered = lowered_result.expect("lower");
    let bindings = env
        .as_ref()
        .map(|e| (&e.value).into())
        .unwrap_or_else(MapBindings::default);
    let plan = compile(&catalog, lowered).expect("compile");
    let out = evaluate(plan, bindings);

    assert!(
        out.is_err(),
        "When evaluating (mode = {mode:#?}) `{statement}`, expected `Err(_)`, but was `{out:#?}`"
    );
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn pass_eval(
    statement: &str,
    mode: EvaluationMode,
    env: &Option<TestValue>,
    expected: &TestValue,
) {
    let catalog = PartiqlCatalog::default();

    let parsed = parse(statement);
    let lowered_result = lower(&catalog, &parsed.expect("parse"));
    let lowered = lowered_result.expect("lower");
    let bindings = env
        .as_ref()
        .map(|e| (&e.value).into())
        .unwrap_or_else(MapBindings::default);
    let plan = compile(&catalog, lowered).expect("compile");
    let out = evaluate(plan, bindings);

    match out {
        Ok(v) => assert_eq!(v.result, expected.value),
        Err(err) => {
            panic!(
                "When evaluating (mode = {mode:#?}) `{statement}`, expected `Ok(_)`, but was `Err({err:#?})`"
            )
        }
    }
}

#[allow(dead_code)]
pub(crate) fn environment() -> Option<TestValue> {
    None
}

// The `partiql_tests` module will be generated by `build.rs` build script.
#[cfg(feature = "conformance_test")]
mod partiql_tests;
