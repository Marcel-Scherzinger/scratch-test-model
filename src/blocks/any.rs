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
#[cfg(feature = "serde")]
use crate::blocks::serde_any_unit::BlockKindUnitSerDe;
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", serde(from = "BlockKindUnitSerDe"))]
#[cfg_attr(feature = "serde", serde(into = "BlockKindUnitSerDe"))]
#[display("{_variant}")]
pub enum BlockKindUnit {
    Event(#[cfg_attr(feature = "utoipa", schema(inline))] EventBlockKindUnit),
    ExprCmp(#[cfg_attr(feature = "utoipa", schema(inline))] ExprOrCmpBlockKindUnit),
    Stmt(#[cfg_attr(feature = "utoipa", schema(inline))] StmtBlockKindUnit),
    #[display("procedures_prototype")]
    ProceduresPrototype,
    #[display("procedures_definition")]
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
            Self::ProceduresPrototype(_) => BlockKindUnit::ProceduresPrototype,
            Self::ProceduresDefinition(_) => BlockKindUnit::ProceduresDefinition,
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
