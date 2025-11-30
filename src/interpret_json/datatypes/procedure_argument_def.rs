use super::RefBlock;
use crate::{BlockKind, ExprBlockKind, VariableValue};

#[derive(Debug, derive_getters::Getters, PartialEq, Clone)]
pub struct ProcedureArgumentDef {
    pub(crate) name: String,
    // TODO: check if BlockKind is the most specific type
    pub(crate) argument_id: RefBlock<BlockKind>,
    // TODO: check if BlockKind is the most specific type
    pub(crate) reporter_id_in_this_block: RefBlock<BlockKind>,
    pub(crate) default_value: VariableValue,
}
