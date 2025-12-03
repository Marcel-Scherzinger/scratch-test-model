use itertools::Itertools;

use super::Target;
use crate::ARc;
use crate::error::JsonCtxError;
use crate::{
    Id,
    blocks::{BlockKindError, UnsupportedBlockKind},
};

#[derive(Debug, thiserror::Error)]
#[error("The requested block id isn't associated with a valid block")]
pub struct NoValidBlockForId;

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
impl ProjectDoc {
    /// Iterator of all _invalid_ blocks regardless of the [target](`Target`)
    pub fn invalid_blocks(
        &self,
    ) -> impl Iterator<Item = (&Id, &ARc<JsonCtxError<BlockKindError>>)> {
        self.targets.iter().flat_map(|t| t.blocks().iter_invalid())
    }
    pub fn unsupported_blocks(&self) -> impl Iterator<Item = (&Id, &UnsupportedBlockKind)> {
        self.targets
            .iter()
            .flat_map(|t| t.blocks().iter_unsupported_blocks())
    }
    pub fn unknown_blocks(&self) -> impl Iterator<Item = (&Id, &ARc<str>)> {
        self.targets
            .iter()
            .flat_map(|t| t.blocks().iter_unknown_blocks())
    }
    pub fn ensure_no_invalid_blocks(self) -> Result<Self, Self> {
        if self.invalid_blocks().next().is_some() {
            Err(self)
        } else {
            Ok(self)
        }
    }
    pub fn ensure_no_unknown_blocks(self) -> Result<Self, Self> {
        if self.unknown_blocks().next().is_some() {
            Err(self)
        } else {
            Ok(self)
        }
    }
    pub fn ensure_no_unsupported_blocks(self) -> Result<Self, Self> {
        if self.unsupported_blocks().next().is_some() {
            Err(self)
        } else {
            Ok(self)
        }
    }

    pub fn ids_with_blocks(&self) -> impl Iterator<Item = (Id, ARc<str>)> {
        self.targets()
            .iter()
            .flat_map(|t| t.blocks().ids_with_blocks())
    }

    pub fn su_ids_with_blocks(&self) -> impl Iterator<Item = (Id, ARc<str>)> {
        self.ids_with_blocks().sorted().unique()
    }
    /// Returns the _valid_ block with the given [`Id`] regardless in which target it's
    /// stored. **Invalid or unsupported blocks can't be retreived with this**
    pub fn get_block(
        &self,
        id: &crate::Id,
    ) -> Result<crate::ARc<crate::BlockWrapper>, NoValidBlockForId> {
        self.targets()
            .iter()
            .flat_map(|trgt| trgt.blocks().iter_blocks())
            .find(|blk| blk.id() == id)
            .cloned()
            .ok_or(NoValidBlockForId)
    }
}
