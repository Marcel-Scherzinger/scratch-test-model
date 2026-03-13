#[cfg(feature = "serde")]
use crate::blocks::{BlockKindUnit, EventBlockKindUnit, ExprOrCmpBlockKindUnit, StmtBlockKindUnit};

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub(crate) enum BlockKindUnitSerDe {
    Event(EventBlockKindUnit),
    ExprCmp(ExprOrCmpBlockKindUnit),
    Stmt(StmtBlockKindUnit),
    Proc(ProceduresSerDe),
}

#[cfg(feature = "serde")]
impl From<BlockKindUnitSerDe> for BlockKindUnit {
    fn from(value: BlockKindUnitSerDe) -> Self {
        match value {
            BlockKindUnitSerDe::Stmt(a) => Self::Stmt(a),
            BlockKindUnitSerDe::Proc(ProceduresSerDe::ProceduresDefinition) => {
                Self::ProceduresDefinition
            }
            BlockKindUnitSerDe::Proc(ProceduresSerDe::ProceduresPrototype) => {
                Self::ProceduresPrototype
            }
            BlockKindUnitSerDe::Event(a) => Self::Event(a),
            BlockKindUnitSerDe::ExprCmp(e) => Self::ExprCmp(e),
        }
    }
}
#[cfg(feature = "serde")]
impl From<BlockKindUnit> for BlockKindUnitSerDe {
    fn from(value: BlockKindUnit) -> Self {
        match value {
            BlockKindUnit::Stmt(a) => Self::Stmt(a),
            BlockKindUnit::Event(a) => Self::Event(a),
            BlockKindUnit::ExprCmp(e) => Self::ExprCmp(e),
            BlockKindUnit::ProceduresPrototype => Self::Proc(ProceduresSerDe::ProceduresPrototype),
            BlockKindUnit::ProceduresDefinition => {
                Self::Proc(ProceduresSerDe::ProceduresDefinition)
            }
        }
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub(crate) enum ProceduresSerDe {
    #[serde(rename = "procedures_prototype")]
    ProceduresPrototype,
    #[serde(rename = "procedures_definition")]
    ProceduresDefinition,
}
