use std::collections::HashMap;

use itertools::Itertools;
use svalue::ARc;

use crate::{
    _exports::AsOpcodeUnit,
    Id,
    aux::JsonBlocks,
    blocks::BlockKindUnit,
    scopes::{TargetBlocksError, block_wrapper::BlockWrapper},
};

#[derive(Debug, PartialEq)]
pub struct TargetBlocks {
    blocks: HashMap<Id, ARc<BlockWrapper>>,
}

impl TargetBlocks {
    pub(crate) fn from_json(value: &serde_json::Value) -> Result<Self, TargetBlocksError<'_>> {
        let dict = value.as_object().ok_or(TargetBlocksError::ExpectedObject)?;
        let all_blocks = JsonBlocks::new(value);

        let valid: Result<HashMap<_, _>, _> = dict
            .into_iter()
            .map(|(id, obj): (&String, &serde_json::Value)| {
                let id: Id = id.clone().into();
                if obj
                    .get("opcode")
                    .and_then(|o| o.as_str())
                    .map(|o| crate::attrs::dropdowns::META_DROPDOWN_MENUES.contains(&o))
                    == Some(true)
                    // sometimes there are array blocks that belong to
                    // displays in the ui for variables
                    || obj.is_array()
                {
                    return Ok(None);
                }

                match BlockWrapper::from_json(all_blocks, id.clone(), obj) {
                    Ok(b) => Ok(Some((id, b.into()))),
                    Err(error) => Err(TargetBlocksError::AtLeastOneInvalid { id, error }),
                }
            })
            .flatten_ok()
            .collect();

        Ok(Self { blocks: valid? })
    }

    /// All blocks
    pub fn iter_blocks(&self) -> impl Iterator<Item = &ARc<BlockWrapper>> {
        self.blocks.values()
    }
    /// Get block by [`Id`]
    pub fn get(&self, id: &Id) -> Option<&ARc<BlockWrapper>> {
        self.blocks.get(id)
    }

    /// Iterator of [`Id`]s and [opcode names](https://en.scratch-wiki.info/wiki/List_of_Block_Opcodes)
    /// of valid and invalid blocks
    pub fn ids_with_opcodes(&self) -> impl Iterator<Item = (Id, BlockKindUnit)> {
        self.blocks
            .iter()
            .map(|(id, bw)| (id.clone(), bw.inner().opcode()))
    }
}
