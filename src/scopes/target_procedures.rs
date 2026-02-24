use std::collections::HashMap;

use svalue::ARc;

use crate::{
    Id,
    blocks::{ProcedureArgumentDef, ProcedureId},
    scopes::{TargetBlocks, block_wrapper::BlockWrapper},
};

/// Contains all information about a defined custom/own block, also called procedure.
///
/// This information is collected by combining to categories of blocks from the target blocks:
/// - a definition block
/// - a prototype block
#[derive(Debug, PartialEq, derive_getters::Getters)]
pub struct Procedure {
    proccode: ARc<str>,
    definition_block: ARc<BlockWrapper>,
    prototype_block_id: Id,
    /// generated using proccode and arguments
    procedure_id: ProcedureId,
    arguments: ARc<[ProcedureArgumentDef]>,
}

/// This component has no equivalent in the scratch file format
#[derive(Debug, PartialEq)]
pub struct TargetProcedures(HashMap<ProcedureId, Procedure>);

impl TargetProcedures {
    pub(crate) fn new(blocks: &TargetBlocks) -> Self {
        let mut cb = HashMap::new();
        let mut proto_to_def_block = HashMap::new();

        for block in blocks.iter_blocks() {
            use crate::blocks::BlockKind as B;
            match block.inner() {
                B::ProceduresPrototype(proto) => {
                    cb.insert(block.id().clone(), proto);
                }
                B::ProceduresDefinition(def) => {
                    proto_to_def_block.insert(def.prototype().o_id(), block.clone());
                }
                _ => {}
            }
        }

        let res = cb
            .into_iter()
            .filter_map(|(id, prototype)| {
                Some((
                    prototype.procedure_id().clone(),
                    Procedure {
                        proccode: prototype.proccode().clone(),
                        arguments: prototype.arguments().clone(),
                        definition_block: proto_to_def_block.get(&id)?.clone(),
                        procedure_id: prototype.procedure_id().clone(),
                        prototype_block_id: id,
                    },
                ))
            })
            .collect();
        Self(res)
    }
    pub fn get(&self, id: &ProcedureId) -> Option<&Procedure> {
        self.0.get(id)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&ProcedureId, &Procedure)> {
        self.0.iter()
    }
    pub fn iter_procedures(&self) -> impl Iterator<Item = &Procedure> {
        self.0.values()
    }
}
