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
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExprOrCmpBlockKindUnit {
    #[display("{_0}")]
    Expr(ExprBlockKindUnit),
    #[display("{_0}")]
    Cmp(CmpBlockKindUnit),
}

#[cfg(feature = "utoipa")]
impl utoipa::PartialSchema for ExprOrCmpBlockKindUnit {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        let inner = [ExprBlockKindUnit::schema(), CmpBlockKindUnit::schema()];

        use itertools::Itertools;
        use utoipa::openapi::RefOr;
        use utoipa::openapi::schema::Schema;

        let enum_values = inner
            .into_iter()
            .flat_map(|s| match s {
                RefOr::T(Schema::Object(o)) => o.enum_values,
                _ => panic!("unexpected schema kind"),
            })
            .flatten()
            .sorted_by_key(ToString::to_string)
            .collect_vec();

        let mut object = utoipa::openapi::schema::Object::new();
        object.enum_values = Some(enum_values);

        RefOr::T(Schema::Object(object))
    }
}
#[cfg(feature = "utoipa")]
impl utoipa::ToSchema for ExprOrCmpBlockKindUnit {
    fn name() -> std::borrow::Cow<'static, str> {
        "ExprOrCmpBlockKindUnit".into()
    }
}

impl<'a, S: DoForAttrsStrategy<'a>> DoForAttrs<'a, S> for ExprOrCmpBlockKind
where
    ExprBlockKind: DoForAttrs<'a, S>,
    CmpBlockKind: DoForAttrs<'a, S>,
{
    fn do_for_attrs(
        &'a self,
        inputs: &<S as DoForAttrsStrategy<'a>>::Inputs,
        outputs: &mut <S as DoForAttrsStrategy<'a>>::Outputs,
    ) -> Result<(), <S as DoForAttrsStrategy<'a>>::Error> {
        match self {
            Self::Expr(k) => k.do_for_attrs(inputs, outputs),
            Self::Cmp(k) => k.do_for_attrs(inputs, outputs),
        }
    }
}
