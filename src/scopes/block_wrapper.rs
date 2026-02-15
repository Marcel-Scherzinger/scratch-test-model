use crate::{
    Id,
    aux::{JsonBlocks, JsonCtx, WithJsonContextExt},
    blocks::BlockKind,
    error::BlockError,
};

/// This wraps a Scratch [block](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Blocks)
///
/// It stores the unique [`Id`] of the block Scratch has assigned to it,
/// the [`Id`] of the `next` block (especially important for statements)
/// and the `parent` block (`parent` is not actively used in the application).
///
/// **parent** is not the defined to be the previous block
/// (next and parent don't form a double-linked-list)
/// but it can be the next-outer-block in the hierarchy.
/// [See here for details](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Blocks)
#[derive(Debug, derive_getters::Getters, PartialEq)]
pub struct BlockWrapper {
    /// The unique id of this block, alphanumeric and special characters are used
    id: Id,
    /// The [`BlockKind`] contains specific information for this kind of block
    inner: BlockKind,
    /// An optional id to the next block following this
    next: Option<Id>,
    /// `parent` is not actively used in the application
    ///
    /// **parent** is not the defined to be the previous block
    /// (next and parent don't form a double-linked-list)
    /// but it can be the next-outer-block in the hierarchy.
    /// [See here for details](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Blocks)
    parent: Option<Id>,
    /// not needed for the application, but it is present in the file so it's parsed
    shadow: bool,
    /// `topLevel`/`x`/`y` is the position of blocks, if they are top-level
    ///
    /// If and (as it seems) only if the property `topLevel` is set to `true`,
    /// the two properties `x` and `y` are also present.
    ///
    /// They then form the position of the block on a grid.
    top_level_pos: Option<Position>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Position {
    x: i64,
    y: i64,
}

impl BlockWrapper {
    pub(crate) fn from_json<'a>(
        all_blocks: JsonBlocks<'a>,
        id: Id,
        obj: &'a serde_json::Value,
    ) -> Result<Self, JsonCtx<'a, BlockError<'a>>> {
        let next = read_maybe_string(obj, "next").with_ctx(obj)?.map(Id::from);
        let parent = read_maybe_string(obj, "parent")
            .with_ctx(obj)?
            .map(Id::from);
        let shadow = read_bool(obj, "shadow").with_ctx(obj)?;

        let top_level_pos = {
            let top_level = read_bool(obj, "topLevel").with_ctx(obj)?;
            if top_level {
                let x = read_i64(obj, "x").with_ctx(obj)?;
                let y = read_i64(obj, "y").with_ctx(obj)?;
                Some(Position { x, y })
            } else {
                None
            }
        };

        let inner =
            BlockKind::from_json(all_blocks, &id).map_err(|ctx| ctx.map_err(BlockError::Kind))?;

        Ok(Self {
            id,
            inner,
            next,
            parent,
            shadow,
            top_level_pos,
        })
    }
}

fn read_maybe_string<'a>(
    obj: &'a serde_json::Value,
    key: &'static str,
) -> Result<Option<&'a str>, BlockError<'a>> {
    Ok(obj
        .get(key)
        .ok_or(BlockError::MissingMandatoryAttr(key))?
        .as_str())
}

fn read_bool<'a>(obj: &serde_json::Value, key: &'static str) -> Result<bool, BlockError<'a>> {
    obj.get(key)
        .ok_or(BlockError::MissingMandatoryAttr(key))?
        .as_bool()
        .ok_or(BlockError::AttrType {
            attr_name: key,
            expected_type: "bool",
        })
}

fn read_i64<'a>(obj: &'a serde_json::Value, key: &'static str) -> Result<i64, BlockError<'a>> {
    obj.get(key)
        .ok_or(BlockError::MissingMandatoryAttr(key))?
        .as_i64()
        .ok_or(BlockError::AttrType {
            attr_name: key,
            expected_type: "i64",
        })
}
