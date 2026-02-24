use crate::attrs::{DoForAttrs, DoForAttrsStrategy};

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

impl<S: DoForAttrsStrategy> DoForAttrs<S> for ExprOrCmpBlockKind
where
    ExprBlockKind: DoForAttrs<S>,
    CmpBlockKind: DoForAttrs<S>,
{
    fn do_for_attrs(
        &self,
        inputs: &<S as DoForAttrsStrategy>::Inputs,
        outputs: &mut <S as DoForAttrsStrategy>::Outputs,
    ) -> Result<(), <S as DoForAttrsStrategy>::Error> {
        match self {
            Self::Expr(k) => k.do_for_attrs(inputs, outputs),
            Self::Cmp(k) => k.do_for_attrs(inputs, outputs),
        }
    }
}
