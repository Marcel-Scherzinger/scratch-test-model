/// copied from [<https://github.com/scratchfoundation/scratch-vm/blob/develop/src/serialization/sb3.js>]
pub(crate) mod constants;
pub(crate) mod do_for_attrs;
pub(crate) mod errors;
mod json_ctx;
mod location;
pub(crate) mod opcode_trait;
pub(crate) mod parse_attr;
pub(crate) mod parse_block;

pub use json_ctx::{JsonCtx, WithJsonContextExt};
pub use location::AttrLocation;
pub use parse_block::JsonBlocks;
