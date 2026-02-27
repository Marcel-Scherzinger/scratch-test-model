use std::path::PathBuf;

pub use crate::aux::JsonCtx;
use crate::{
    Id,
    attrs::data::{ListKind, VariableKind},
};

pub use crate::aux::errors::{
    AttributeContentError, AttributeParseError, BlockJsonStructureError, BlockKindError,
    MandatoryAttrMissingError,
};
pub use crate::scopes::project_doc::NoValidBlockForId;
pub use crate::scopes::target_blocks::BlockReferenceInvalid;

#[allow(unused)]
use crate::attrs::{List, Variable};

/// A [target](`crate::scopes::Target`)'s collection of [`Variable`]s or [`List`]s failed to parse.
///
/// The `Kind` is either `VariableKind` or `ListKind` and isn't
/// present at runtime. It only gives type-level information about
/// the place where the error occured.
///
/// See [`TargetData`](crate::scopes::TargetData)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, thiserror::Error)]
pub enum TargetDataError<Kind> {
    /// The data for variables / lists should always be stored
    /// – for each target – in a json object (`{ ... }`) per kind.
    /// This variant indicates that this wasn't the case.
    #[error("expected object {{...}} for data entities of target")]
    ExpectedObject,
    /// At least one stored variable / list has invalid structure
    #[error("at least one target data entry (id={0:?}) has unknown structure")]
    AtLeastOneInvalid(Id),
    /// This variant can never occur (see [`std::convert::Infallible`])
    /// It is only present for placing a type variable
    #[error("SHOULD NEVER OCCUR")]
    Phantom {
        phantom: std::marker::PhantomData<Kind>,
        delete: std::convert::Infallible,
    },
}

/// Failed to parse [`BlockWrapper`](crate::blocks::BlockWrapper).
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum BlockError<'a> {
    /// A property of [`BlockWrapper`](crate::blocks::BlockWrapper) that is not directly
    /// related to the blocks [kind](crate::blocks::BlockKind) is missing.
    /// This includes ["next"](crate::blocks::BlockWrapper::next),
    /// ["parent"](crate::blocks::BlockWrapper::parent),
    /// ["shadow"](crate::blocks::BlockWrapper::shadow), "topLevel", "x" and "y".
    /// "x" and "y" are only present/needed if "topLevel" is `true`, see
    /// [`top_level_pos`](crate::blocks::BlockWrapper::top_level_pos)
    #[error("missing block attr: {0}")]
    MissingMandatoryAttr(&'static str),

    /// A boolean attribute ("shadow", "topLevel") was not stored as json boolean or
    /// an integer attribute ("x", "y") was not stored as json integer
    #[error("block attr '{attr_name}' should have type '{expected_type}'")]
    AttrType {
        attr_name: &'static str,
        expected_type: &'static str,
    },

    /// Some error occured that is directly related to the blocks [kind](crate::blocks::BlockKind),
    /// e. g. its opcode
    #[error("{0}")]
    Kind(BlockKindError<'a>),
}

/// A [target](`crate::scopes::Target`)'s collection of [blocks](`crate::scopes::TargetBlocks`)
/// is invalid.
///
/// Typically, this means that at least one of the blocks was invalid.
/// Only very invalid files wouldn't have an object as the value of the blocks.
#[derive(Debug, thiserror::Error)]
pub enum TargetBlocksError<'a> {
    /// Is unlikely as files triggering this are so wrong that they shouldn't be considered normal.
    #[error("expected object {{...}} for blocks of target")]
    ExpectedObject,
    /// One of the stored blocks failed to parse
    #[error("at least one target block (id={id:?}) has unknown structure (block-error={error})")]
    AtLeastOneInvalid {
        id: Id,
        error: JsonCtx<'a, BlockError<'a>>,
    },
}

/// Failure while parsing a [`Target`](crate::scopes::Target).
#[derive(Debug, thiserror::Error, derive_more::From)]
pub enum TargetError<'a> {
    /// Every target should say if it is _the_ stage or _a_ sprite.
    #[error("the attribute \"isStage\" of the target is missing")]
    MissingIsStage,
    /// Every target should have a name.
    #[error("the attribute \"name\" of the target is missing")]
    MissingName,

    /// The saved variable values are provided in an invalid way
    #[error("error in target variables: {0}")]
    TargetVar(JsonCtx<'a, TargetDataError<VariableKind>>),
    /// The saved variable values are provided in an invalid way
    #[error("error in target lists: {0}")]
    TargetLis(JsonCtx<'a, TargetDataError<ListKind>>),
    /// The target blocks (the most important compononet) is invalid
    #[error("error in target blocks: {0}")]
    TargetBlocks(TargetBlocksError<'a>),
}

/// The provided json document doesn't represent a valid model.
///
/// Unless the document is very abnormal there should at least be
/// an array of targets, so the variant [`ModelError::Target`]
/// should occur more often.
#[derive(Debug, thiserror::Error, derive_more::From)]
pub enum ModelError<'a> {
    /// One of the `target`-entries produced an error
    #[error("target error: {0}")]
    Target(JsonCtx<'a, TargetError<'a>>),
    /// There was no json array (`[...]`) with targets
    #[error("The document doesn't contain a \"targets\" array: {0}")]
    #[from(skip)]
    NoTargetsArray(&'a serde_json::Value),
}

/// _Not actively used_, combines errors from file reading and model parsing.
#[derive(Debug, thiserror::Error, derive_more::From)]
pub enum ParseError<'a> {
    #[error("model: {0}")]
    Model(ModelError<'a>),
    #[error("document: {0}")]
    Doc(DocError),
}

/// Failure while parsing (ZIP-) file and extracting json document.
#[derive(Debug, thiserror::Error, derive_more::From)]
pub enum DocError {
    #[error("stream or file was used that doesn't contain a document")]
    NoDocument,
    #[error("io error: {0}")]
    #[from(skip)]
    Io(#[from] Box<dyn std::error::Error>),
    #[error("invalid json document: {0}")]
    #[from(skip)]
    Json(#[from] serde_json::Error),
    #[error("file {0:?} read error: {1}")]
    #[from(skip)]
    FileRead(PathBuf, std::io::Error),
}
