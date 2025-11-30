use super::{CmpBlockKind, EventBlockKind, ExprBlockKind, NoopStmtBlockKind, StmtBlockKind};
use super::{
    CmpBlockKindUnit, EventBlockKindUnit, ExprBlockKindUnit, NoopStmtBlockKindUnit,
    StmtBlockKindUnit, UnsupportedBlockKind, UnsupportedBlockKindUnit,
};
use crate::ARc;
use crate::attr::{ProcedureArgumentDef, RefBlock};
use crate::blocks::dt_interface::GetOpcodeUnit;

/// opcode block type is [`BlockKindUnit`]
#[derive(Debug, derive_more::From, PartialEq, Clone)]
pub enum BlockKind {
    #[from(skip)]
    ProceduresDefinition {
        /// block id of matching [Self::ProceduresPrototype] instance
        custom_block: RefBlock,
    },
    #[from(skip)]
    ProceduresPrototype {
        /// representation of procedure's name with placeholders for arguments
        proccode: String,
        /// a sequence of arguments this procedure accepts
        arguments: ARc<[ProcedureArgumentDef]>,
    },

    Event(EventBlockKind),
    Cmp(CmpBlockKind),
    Expr(ExprBlockKind),
    Stmt(StmtBlockKind),
    Noop(NoopStmtBlockKind),
    Unsup(UnsupportedBlockKind),
}

/// main block type is [`BlockKind`]
#[derive(Debug, derive_more::From, PartialEq, Clone, Copy, derive_more::Display)]
#[display("{_variant}")]
pub enum BlockKindUnit {
    Event(EventBlockKindUnit),
    Cmp(CmpBlockKindUnit),
    Expr(ExprBlockKindUnit),
    Stmt(StmtBlockKindUnit),
    Noop(NoopStmtBlockKindUnit),
    Unsup(UnsupportedBlockKindUnit),
    #[display("procedures_prototype")]
    ProceduresPrototype,
    #[display("procedures_definition")]
    ProceduresDefinition,
}

impl GetOpcodeUnit for BlockKind {
    type Opcode = BlockKindUnit;

    fn get_opcode(&self) -> Self::Opcode {
        match self {
            Self::Expr(u) => u.get_opcode().into(),
            Self::Event(u) => u.get_opcode().into(),
            Self::Cmp(u) => u.get_opcode().into(),
            Self::Stmt(u) => u.get_opcode().into(),
            Self::Noop(u) => u.get_opcode().into(),
            Self::Unsup(u) => u.get_opcode().into(),
            Self::ProceduresPrototype { .. } => BlockKindUnit::ProceduresPrototype,
            Self::ProceduresDefinition { .. } => BlockKindUnit::ProceduresDefinition,
        }
    }
}
