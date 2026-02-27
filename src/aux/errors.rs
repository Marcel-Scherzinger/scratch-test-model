use crate::aux::{AttrLocation, JsonCtx};

/// General block parsing failed because of the json structure, this is not about [attributes](`crate::attrs`).
#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum BlockJsonStructureError {
    /// The requested block [`Id`](`crate::Id`) wasn't found in the
    /// [target](`crate::scopes::Target`)'s json mapping `blocks`
    ///
    /// This should rarely occur as multiple program parts only request blocks
    /// that were previously found in the file by e. g. iterating over all
    /// block `Id`s
    #[error("the requested id doesn't exist in the current target")]
    IdNotFound,
    /// The found block data (for the requested [`Id`](`crate::Id`)) is not
    /// a json mapping (`{ ... }`). This should never happen in a valid file.
    #[error("block is not a json object/mapping, fields not found")]
    BlockIsNoMapping,
    /// Mapping is missing a needed key-value-pair, like `inputs` or `fields`.
    ///
    /// Regardless of the block's kind or opcode, some attributes need to be present.
    /// Examples of such attributes are `opcode` or `inputs`.
    /// For procedures the `mutation` key has to be present additionally.
    #[error("block mapping doesn't contain mandatory '{0}' option")]
    MissingMandatoryAttr(&'static str),
    /// A block property is of invalid form, this is not about attributes like in [`crate::attrs`].
    ///
    /// `inputs`, `fields` and `mutation` need to be – if present – json mappings from
    /// attribute names to their values. An opcode should be a string. This error
    /// indicates that one of those expectations was violated.
    #[error("the '{attr}' attribute is of the wrong type, it should be {correct_type}")]
    AttrWrongType {
        attr: &'static str,
        correct_type: &'static str,
    },
}

/// Specific error for the current block kind's requirements.
///
/// The most interesting one is [BlockKindError::AttrParsing] as this is produced
/// by individual block attributes (the data associated with a particular block).
#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum BlockKindError<'a> {
    /// The found opcode is no valid choice for the tried block kind.
    ///
    /// Either the wrong kind type was tried, the block opcode is not supported or it is
    /// just invalid. Every opcode that is supported by a variant of
    /// [`BlockKind`](crate::blocks::BlockKind) is valid.
    #[error("the found opcode is no valid choice for the tried block kind: {0}")]
    UnknownOpcode(svalue::ARc<str>),
    #[error("json-repr of block: {0}")]
    JsonStructure(#[from] BlockJsonStructureError),
    #[error("missing block attr: {0}")]
    MissingMandatoryAttr(#[from] MandatoryAttrMissingError),
    #[error("parsing attribute {location}.'{key}': {error}")]
    AttrParsing {
        location: AttrLocation,
        key: &'static str,
        error: AttributeParseError<'a>,
    },
}

/// Inner error indicating an attribute's content is invalid.
///
/// This is typically wrapped by [`AttributeParseError`] and focuses
/// on errors of the type-specific format, when shadow information is
/// already resolved.
#[derive(derive_more::Debug, PartialEq, Clone, thiserror::Error)]
pub enum AttributeContentError<'a> {
    // Custom-Assoc-Type? / Subblock für alle?
    #[error("{0}")]
    #[debug("{_0:?}")]
    Other(&'static str),

    /// Some attributes can be one of multiple types. This is represented with [`either::Either`].
    /// If the `Left` variant fails to parse, `Right` is tried with the same input.
    /// If both fail the errors are returned.
    #[error("'Either' attribute failed twice: first={first_err}, second={second_err}")]
    BothEitherFailed {
        first_err: Box<AttributeContentError<'a>>,
        second_err: Box<AttributeContentError<'a>>,
    },

    #[error("expected an array (fixed-len={specified_length:?}): {value}")]
    ExpectedArray {
        value: &'a serde_json::Value,
        specified_length: Option<usize>,
    },

    /// The found value is invalid, some attribute types provide a list with valid
    /// alternatives/hints.
    #[error("value {invalid:?} is invalid for dropdown, valid examples/hints: {hint_list}")]
    InvalidOptionForDropdown {
        invalid: String,
        hint_list: &'static str,
    },

    #[error("this dropdown is expected to have a string id as reference to it's menu, not: {0}")]
    ExternalDropdownMenuReferenceNotString(&'a serde_json::Value),

    /// Some attributes don't contain the real data but only a reference to another block.
    /// This variant means that such a block occured and wasn't totally valid.
    #[error("there was an error parsing an indirection subblock: {0}")]
    Subblock(Box<JsonCtx<'a, BlockKindError<'a>>>),
}

/// Main error for parsing of attributes.
#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum AttributeParseError<'a> {
    /// The attribute's only value is a singleton array indicating that it contains shadow
    /// information, but the array has no real data
    #[error("attribute is not providing a real value, only shadow indication")]
    ShadowWithoutValue,
    /// The attribute has two nested shadow levels,
    /// when the first is traversed (to the block hiding the possible shadow)
    /// another shadow information was found. This is invalid
    #[error("attribute contains two levels of shadow information, only one allowed")]
    DuplicateShadow,
    /// The attribute's value is invalid for the expected type.
    #[error("content, shadowing={shadow_resolved}: {error}")]
    Content {
        /// If there was shadow information that was traversed to parse only the active value
        ///
        /// This can be useful to spot if an invalid value triggered shadow-resolving,
        /// even though it shouldn't happen
        shadow_resolved: bool,
        /// The details about the error the content produced
        error: AttributeContentError<'a>,
    },
}

/// A mapping like `inputs` or `fields` misses an [attribute](`crate::attrs`) that is expected for the opcode.
#[derive(Debug, PartialEq, PartialOrd, Clone, thiserror::Error)]
#[error(
    "the mandatory block attribute '{key}' is missing from the expected property set '{location}'"
)]
pub struct MandatoryAttrMissingError {
    pub(super) location: AttrLocation,
    pub(super) key: &'static str,
}
