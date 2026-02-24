use crate::{
    Id,
    attrs::RefBlock,
    blocks::BlockKindUnit,
    error::BlockReferenceInvalid,
    scopes::{block_wrapper::BlockWrapper, target_blocks::WrappableKind},
};

use super::target::Target;
use itertools::Itertools;
use svalue::ARc;

/// Represents an entire sb3 program file with all [`Target`]s,
/// blocks ([`TargetBlocks`](crate::TargetBlocks)) and
/// procedures ([`TargetProcedures`](crate::TargetProcedures))
///
/// See the [`crate`] level docs for a general explaination
#[derive(Debug, derive_getters::Getters, Clone, PartialEq)]
pub struct ProjectDoc {
    /// Targets of the document, see [`Target`] for details
    pub(crate) targets: ARc<[Target]>,
    /// The version number stored as metadata in the document
    ///
    /// This field is currently unused by the implementation
    /// but may be used in the future to detect compatible files
    pub(crate) semver: Option<ARc<str>>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[error("The requested block id isn't associated with a valid block")]
pub struct NoValidBlockForId;

impl ProjectDoc {
    pub fn ids_with_opcodes(&self) -> impl Iterator<Item = (Id, BlockKindUnit)> {
        self.targets()
            .iter()
            .flat_map(|t| t.blocks().ids_with_opcodes())
    }

    pub fn su_ids_with_blocks(&self) -> impl Iterator<Item = (Id, BlockKindUnit)> {
        self.ids_with_opcodes().sorted().unique()
    }
    /*
    /// Returns the block with the given [`Id`] regardless in which target it's stored.
    pub fn get_block_owned(&self, id: &crate::Id) -> Result<ARc<BlockWrapper>, NoValidBlockForId> {
        self.get_block(id).cloned()
    }
    */
    /// Returns the block with the given [`Id`] regardless in which target it's stored.
    pub fn get_block<'a>(
        &'a self,
        id: &crate::Id,
    ) -> Result<&'a ARc<BlockWrapper>, NoValidBlockForId> {
        self.targets()
            .iter()
            .flat_map(|trgt| trgt.blocks().iter_blocks())
            .find(|blk| blk.id() == id)
            .ok_or(NoValidBlockForId)
    }

    pub fn get_specific_kind<T: WrappableKind + PartialEq>(
        &self,
        reference: &RefBlock<T>,
    ) -> Result<&T, BlockReferenceInvalid> {
        self.targets()
            .iter()
            .map(|trgt| trgt.blocks().get_specific_kind(reference))
            .find(|blk| !matches!(blk, Err(BlockReferenceInvalid::IdNotFound)))
            .ok_or(BlockReferenceInvalid::IdNotFound)?
    }
    pub fn get_specific_with_wrapper<T: WrappableKind>(
        &self,
        reference: &RefBlock<T>,
    ) -> Result<(&T, &svalue::ARc<BlockWrapper>), BlockReferenceInvalid> {
        self.targets()
            .iter()
            .map(|trgt| trgt.blocks().get_specific_with_wrapper(reference))
            .find(|blk| !matches!(blk, Err(BlockReferenceInvalid::IdNotFound)))
            .ok_or(BlockReferenceInvalid::IdNotFound)?
    }
}
