use crate::{
    _exports::{AsOpcodeUnit, BlockKindError, ParseJsonBlock},
    aux::{JsonBlocks, JsonCtx},
    blocks::{
        CmpBlockKind, CmpBlockKindUnit, EventBlockKind, EventBlockKindUnit, ExprBlockKind,
        ExprBlockKindUnit, StmtBlockKind, StmtBlockKindUnit,
        procedures::{ProceduresDefinition, ProceduresPrototype},
    },
};

/// opcode block type is [`BlockKindUnit`]
#[derive(Debug, derive_more::From, PartialEq, Clone)]
pub enum BlockKind {
    ProceduresDefinition(ProceduresDefinition),
    ProceduresPrototype(ProceduresPrototype),

    Event(EventBlockKind),
    Cmp(CmpBlockKind),
    Expr(ExprBlockKind),
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
#[display("{_variant}")]
pub enum BlockKindUnit {
    Event(EventBlockKindUnit),
    Cmp(CmpBlockKindUnit),
    Expr(ExprBlockKindUnit),
    Stmt(StmtBlockKindUnit),
    #[display("procedures_prototype")]
    ProceduresPrototype,
    #[display("procedures_definition")]
    ProceduresDefinition,
}

impl AsOpcodeUnit for BlockKind {
    type OpcodeUnit = BlockKindUnit;

    fn opcode(&self) -> Self::OpcodeUnit {
        match self {
            Self::Expr(u) => u.opcode().into(),
            Self::Event(u) => u.opcode().into(),
            Self::Cmp(u) => u.opcode().into(),
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
