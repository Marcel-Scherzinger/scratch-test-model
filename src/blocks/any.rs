use crate::{
    _exports::{AsOpcodeUnit, BlockKindError, ParseJsonBlock},
    attrs::{DoForAttrs, DoForAttrsStrategy},
    aux::{JsonBlocks, JsonCtx},
    blocks::{
        CmpBlockKind, CmpBlockKindUnit, EventBlockKind, EventBlockKindUnit, ExprBlockKind,
        ExprBlockKindUnit, ExprOrCmpBlockKind, ExprOrCmpBlockKindUnit, StmtBlockKind,
        StmtBlockKindUnit,
        procedures::{ProceduresDefinition, ProceduresPrototype},
    },
};

/// opcode block type is [`BlockKindUnit`]
#[derive(Debug, derive_more::From, PartialEq, Clone)]
pub enum BlockKind {
    ProceduresDefinition(ProceduresDefinition),
    ProceduresPrototype(ProceduresPrototype),

    Event(EventBlockKind),
    ExprCmp(ExprOrCmpBlockKind),
    Stmt(StmtBlockKind),
}
/// main block type is [`BlockKind`]
#[derive(
    Debug,
    derive_more::From,
    PartialEq,
    Clone,
    Copy,
    derive_more::Display,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[display("{_variant}")]
pub enum BlockKindUnit {
    Event(EventBlockKindUnit),
    ExprCmp(ExprOrCmpBlockKindUnit),
    Stmt(StmtBlockKindUnit),
    Proc(ProcKindUnit),
}

#[cfg(feature = "utoipa")]
impl utoipa::PartialSchema for BlockKindUnit {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        let inner = [
            EventBlockKindUnit::schema(),
            ExprBlockKindUnit::schema(),
            CmpBlockKindUnit::schema(),
            StmtBlockKindUnit::schema(),
            ProcKindUnit::schema(),
        ];

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
        object.description = Some(r#"For more information see <a href="https://marcel-scherzinger.github.io/scratch-test-model/scratch_test_model/blocks/enum.BlockKindUnit.html">BlockKindUnit</a>"#.to_string());

        RefOr::T(Schema::Object(object))
    }
}
#[cfg(feature = "utoipa")]
impl utoipa::ToSchema for BlockKindUnit {
    fn name() -> std::borrow::Cow<'static, str> {
        "BlockKindUnit".into()
    }
}

#[derive(
    Debug,
    derive_more::From,
    PartialEq,
    Clone,
    Copy,
    derive_more::Display,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ProcKindUnit {
    #[display("procedures_prototype")]
    #[cfg_attr(feature = "serde", serde(rename = "procedures_prototype"))]
    ProceduresPrototype,
    #[display("procedures_definition")]
    #[cfg_attr(feature = "serde", serde(rename = "procedures_definition"))]
    ProceduresDefinition,
}

impl From<ExprBlockKindUnit> for BlockKindUnit {
    fn from(value: ExprBlockKindUnit) -> Self {
        Self::ExprCmp(value.into())
    }
}
impl From<CmpBlockKindUnit> for BlockKindUnit {
    fn from(value: CmpBlockKindUnit) -> Self {
        Self::ExprCmp(value.into())
    }
}
impl From<ExprBlockKind> for BlockKind {
    fn from(value: ExprBlockKind) -> Self {
        Self::ExprCmp(value.into())
    }
}
impl From<CmpBlockKind> for BlockKind {
    fn from(value: CmpBlockKind) -> Self {
        Self::ExprCmp(value.into())
    }
}

impl AsOpcodeUnit for BlockKind {
    type OpcodeUnit = BlockKindUnit;

    fn opcode(&self) -> Self::OpcodeUnit {
        match self {
            Self::ExprCmp(ExprOrCmpBlockKind::Cmp(u)) => u.opcode().into(),
            Self::ExprCmp(ExprOrCmpBlockKind::Expr(u)) => u.opcode().into(),
            Self::Event(u) => u.opcode().into(),
            Self::Stmt(u) => u.opcode().into(),
            Self::ProceduresPrototype(_) => BlockKindUnit::Proc(ProcKindUnit::ProceduresPrototype),
            Self::ProceduresDefinition(_) => {
                BlockKindUnit::Proc(ProcKindUnit::ProceduresDefinition)
            }
        }
    }
}

impl BlockKind {
    pub(crate) fn from_json<'a>(
        all_blocks: JsonBlocks<'a>,
        id: &str,
    ) -> Result<Self, JsonCtx<'a, BlockKindError<'a>>> {
        use crate::aux::parse_block::ParseJsonBlock;

        Ok(
            if let Some(opt) = unknown_no_error::<EventBlockKind>(all_blocks, id)? {
                opt.into()
            } else if let Some(opt) = unknown_no_error::<CmpBlockKind>(all_blocks, id)? {
                opt.into()
            } else if let Some(opt) = unknown_no_error::<ExprBlockKind>(all_blocks, id)? {
                opt.into()
            } else if let Some(opt) = unknown_no_error::<ProceduresDefinition>(all_blocks, id)? {
                opt.into()
            } else if let Some(opt) = unknown_no_error::<ProceduresPrototype>(all_blocks, id)? {
                opt.into()
            } else {
                let opt = StmtBlockKind::ctx_parse_json_block(all_blocks, id)?;
                opt.into()
            },
        )
    }
}

fn unknown_no_error<'a, T: ParseJsonBlock>(
    all_blocks: JsonBlocks<'a>,
    id: &str,
) -> Result<Option<T>, JsonCtx<'a, BlockKindError<'a>>> {
    match T::ctx_parse_json_block(all_blocks, id) {
        Err(ctx) => {
            let (err, json) = ctx.into_inner();
            if let BlockKindError::UnknownOpcode(_) = err {
                return Ok(None);
            }
            Err(JsonCtx {
                inner_error: err,
                json_ctx: json,
            })
        }
        Ok(ok) => Ok(Some(ok)),
    }
}

impl<'a, S: DoForAttrsStrategy<'a>> DoForAttrs<'a, S> for BlockKind
where
    ExprOrCmpBlockKind: DoForAttrs<'a, S>,
    StmtBlockKind: DoForAttrs<'a, S>,
    EventBlockKind: DoForAttrs<'a, S>,
    ProceduresPrototype: DoForAttrs<'a, S>,
    ProceduresDefinition: DoForAttrs<'a, S>,
{
    fn do_for_attrs(
        &'a self,
        inputs: &<S as DoForAttrsStrategy<'a>>::Inputs,
        outputs: &mut <S as DoForAttrsStrategy<'a>>::Outputs,
    ) -> Result<(), <S as DoForAttrsStrategy<'a>>::Error> {
        match self {
            Self::ExprCmp(b) => b.do_for_attrs(inputs, outputs),
            Self::Stmt(b) => b.do_for_attrs(inputs, outputs),
            Self::Event(b) => b.do_for_attrs(inputs, outputs),
            Self::ProceduresDefinition(b) => b.do_for_attrs(inputs, outputs),
            Self::ProceduresPrototype(b) => b.do_for_attrs(inputs, outputs),
        }
    }
}
