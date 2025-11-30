use super::{CmpBlockKind, CmpBlockKindUnit, ExprBlockKind, ExprBlockKindUnit};

#[derive(Debug, PartialEq, Clone)]
pub enum ExprOrCmpBlockKind {
    Expr(ExprBlockKind),
    Cmp(CmpBlockKind),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprOrCmpBlockKindUnit {
    Expr(ExprBlockKindUnit),
    Cmp(CmpBlockKindUnit),
}
