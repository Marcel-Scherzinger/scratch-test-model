use std::collections::HashMap;

use itertools::Itertools;
use svalue::ARc;

use crate::{
    _exports::AsOpcodeUnit,
    Id,
    attrs::RefBlock,
    aux::JsonBlocks,
    blocks::{BlockKind, BlockKindUnit},
    scopes::{TargetBlocksError, block_wrapper::BlockWrapper},
};

#[derive(Debug, PartialEq)]
pub struct TargetBlocks {
    blocks: HashMap<Id, ARc<BlockWrapper>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, thiserror::Error)]
pub enum BlockReferenceInvalid {
    #[error("the tried id doesn't belong to a valid block")]
    IdNotFound,
    #[error("the wanted block kind differs from the actual kind")]
    WrongKind,
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
    pub fn get_by_id(&self, id: &Id) -> Option<&ARc<BlockWrapper>> {
        self.blocks.get(id)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&Id, &ARc<BlockWrapper>)> {
        self.blocks.iter()
    }
    pub fn iter_ids(&self) -> impl Iterator<Item = &Id> {
        self.blocks.keys()
    }

    /// Iterator of [`Id`]s and [opcode names](https://en.scratch-wiki.info/wiki/List_of_Block_Opcodes)
    /// of valid and invalid blocks
    pub fn ids_with_opcodes(&self) -> impl Iterator<Item = (Id, BlockKindUnit)> {
        self.blocks
            .iter()
            .map(|(id, bw)| (id.clone(), bw.inner().opcode()))
    }

    pub fn get_specific_kind<T: WrappableKind>(
        &self,
        reference: &RefBlock<T>,
    ) -> Result<&T, BlockReferenceInvalid> {
        let any = self
            .get_by_id(reference.id())
            .ok_or(BlockReferenceInvalid::IdNotFound)?
            .inner();
        T::ensure(any).ok_or(BlockReferenceInvalid::WrongKind)
    }
    pub fn get_specific_with_wrapper<T: WrappableKind>(
        &self,
        reference: &RefBlock<T>,
    ) -> Result<(&T, &svalue::ARc<BlockWrapper>), BlockReferenceInvalid> {
        let wrapped = self
            .get_by_id(reference.id())
            .ok_or(BlockReferenceInvalid::IdNotFound)?;
        let kind = T::ensure(wrapped.inner()).ok_or(BlockReferenceInvalid::WrongKind)?;
        Ok((kind, wrapped))
    }
}

pub trait WrappableKind {
    fn ensure(any: &BlockKind) -> Option<&Self>;
}

impl WrappableKind for crate::blocks::CmpBlockKind {
    fn ensure(any: &BlockKind) -> Option<&Self> {
        if let BlockKind::ExprCmp(crate::blocks::ExprOrCmpBlockKind::Cmp(v)) = any {
            Some(v)
        } else {
            None
        }
    }
}

impl WrappableKind for crate::blocks::ExprBlockKind {
    fn ensure(any: &BlockKind) -> Option<&Self> {
        if let BlockKind::ExprCmp(crate::blocks::ExprOrCmpBlockKind::Expr(v)) = any {
            Some(v)
        } else {
            None
        }
    }
}

impl WrappableKind for crate::blocks::BlockKind {
    fn ensure(any: &BlockKind) -> Option<&Self> {
        Some(any)
    }
}
impl WrappableKind for crate::blocks::ProceduresDefinition {
    fn ensure(any: &BlockKind) -> Option<&Self> {
        if let BlockKind::ProceduresDefinition(v) = any {
            Some(v)
        } else {
            None
        }
    }
}
impl WrappableKind for crate::blocks::ProceduresPrototype {
    fn ensure(any: &BlockKind) -> Option<&Self> {
        if let BlockKind::ProceduresPrototype(v) = any {
            Some(v)
        } else {
            None
        }
    }
}
impl WrappableKind for crate::blocks::EventBlockKind {
    fn ensure(any: &BlockKind) -> Option<&Self> {
        if let BlockKind::Event(v) = any {
            Some(v)
        } else {
            None
        }
    }
}
impl WrappableKind for crate::blocks::StmtBlockKind {
    fn ensure(any: &BlockKind) -> Option<&Self> {
        if let BlockKind::Stmt(v) = any {
            Some(v)
        } else {
            None
        }
    }
}
impl WrappableKind for crate::blocks::ExprOrCmpBlockKind {
    fn ensure(any: &BlockKind) -> Option<&Self> {
        if let BlockKind::ExprCmp(v) = any {
            Some(v)
        } else {
            None
        }
    }
}
