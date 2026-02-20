use super::{CmpBlockKind, CmpBlockKindUnit, ExprBlockKind, ExprBlockKindUnit};

#[derive(Debug, PartialEq, Clone, derive_more::From)]
pub enum ExprOrCmpBlockKind {
    Expr(ExprBlockKind),
    Cmp(CmpBlockKind),
}

#[derive(
    derive_more::Display,
    Debug,
    PartialEq,
    Clone,
    derive_more::From,
    Copy,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum ExprOrCmpBlockKindUnit {
    #[display("{_0}")]
    Expr(ExprBlockKindUnit),
    #[display("{_0}")]
    Cmp(CmpBlockKindUnit),
}
