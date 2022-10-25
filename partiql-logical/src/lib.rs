use partiql_value::{BindingsName, Value};
use std::collections::HashMap;

// TODO we should replace this enum with some identifier that can be looked up in a symtab/funcregistry?
#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum UnaryOp {}

// TODO we should replace this enum with some identifier that can be looked up in a symtab/funcregistry?
#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum BinaryOp {
    And,
    Or,
    Concat,
    Eq,
    Neq,
    Gt,
    Gteq,
    Lt,
    Lteq,
}

#[derive(Debug)]
pub enum PathComponent {
    Key(String),
    Index(i64),
}

#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum Expr {
    Value(ValueExpr),   // e.g. 2+3
    Bindings(BindingsToValueExpr),  // <- should be Bindings(BindingsToBindings); e.g. where expr, order by, group by
    BindingsToValue(BindingsToValueExpr),   // e.g. select value
    ValueToBindings(ValueToBindingsExpr),   // from, unpivot
}

#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum ValueExpr {
    // TODO other variants
    UnExpr(UnaryOp, Box<ValueExpr>),
    BinaryExpr(BinaryOp, Box<ValueExpr>, Box<ValueExpr>),
    Lit(Box<Value>),
    Path(Box<ValueExpr>, Vec<PathComponent>),
    VarRef(BindingsName),
}

// Bindings -> Bindings : Where, OrderBy, Offset, Limit, Join, SetOp, Select, Distinct, GroupBy, Unpivot, Let
// Values   -> Bindings : From
// Bindings -> Values   : Select Value

#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum BindingsExpr {
    From(From),
    Unpivot,
    Where(Where),
    OrderBy,
    Offset,
    Limit,
    Join,
    SetOp,
    SelectValue(SelectValue),
    Select(Select),
    Distinct(Distinct),
    GroupBy,
    Output, //--TODO(Alan) why is Output modeled as a `BindingsExpr` -- signal to come out (marker/sentinel value)
            //     value expr don't have notion yet; possible ValueExpr will have its own output marker
}

#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum BindingsToValueExpr {}

#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum ValueToBindingsExpr {}

/// [`From`] bridges from [`ValueExpr`]s to [`BindingExpr`]s
#[derive(Debug)]
pub struct From {
    pub expr: ValueExpr,
    pub as_key: String,
    pub at_key: Option<String>,
    pub out: Box<BindingsExpr>,
}

#[derive(Debug)]
pub struct Where {
    pub expr: ValueExpr,
    pub out: Box<BindingsExpr>,
}

#[derive(Debug)]
pub struct Select {
    pub exprs: HashMap<String, ValueExpr>,
    pub out: Box<BindingsExpr>,
}

#[derive(Debug)]
pub struct SelectValue {
    pub exprs: ValueExpr,
    pub out: Box<ValueExpr>,
}

#[derive(Debug)]
pub struct Distinct {
    pub out: Box<BindingsExpr>,
}
