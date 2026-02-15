//! Model to parse the [`*.sb3` file format](https://en.scratch-wiki.info/wiki/Scratch_File_Format)
//! of the [Scratch](https://scratch.mit.edu/) block-oriented programming language
//!
//! (I want to note that I was unable to find the above link to the format specification
//! when I was developing this so I reverse-engineered the format from example files.
//! Luckily, it looks like I correctly understood the meaning of the components.
//! This is the second version of the parser that supports all standard blocks
//! and some extensions even blocks that aren't supported by the interpreter.)
//!
//!
//! The program components extracted from a `*.sb3` file are represented by the
//! [`ProjectDoc`] type which offers different methods for parsing a file or
//! a sequence of bytes representing the file's content.
//! The simplest is to first parse the document as json as this value
//! will be borrowed internally.
//!
//! ```no_run
//! # use scratch_test_model::{ProjectDoc, json_from_sb3_file};
//! let json = json_from_sb3_file("/path/to/file.sb3").unwrap();
//! let doc = ProjectDoc::from_json(&json);
//! println!("{doc:#?}");
//! ```
//!
//! # Steps
//!
//! ## JSON extraction
//!
//! A [Scratch file](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Project_Files)
//! is a ZIP file with images, sounds, ... and a `project.json`
//! that contains all information about the program structure.
//!
//! If the used decompression algorithm is unable to read the file
//! or if you want to provide the `project.json` file directly
//! you can use [`ProjectDoc::from_json`] which can be called with
//! an already parsed JSON document.
//!
//! (See [`ProjectDoc`] for details.)
//!
//! (The next sections will explain how all program blocks are
//! extracted from an entered JSON document
//! – regardless how it was obtained in this section.)
//!
//! ## Targets
//!
//! Each document contains some meta information and – most
//! importantly – a `targets` field containing a list of "targets".
//! A [target](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Targets)
//! is a sprite or the background and contains different data.
//! (The sprites have _local_ variables and the background's
//! variables and lists are seen as _global_.)
//!
//! The relevant information stored in the modeled target:
//! - [`TargetVariables`]: all variables of the target (local for sprites and global for the stage)
//! - [`TargetLists`]: all lists of the target (local for sprites and global for the stage)
//! - [`TargetBlocks`]: collection of all statement, expression, comparison, event, ... blocks of the program
//! - [`TargetProcedures`]: generated from the blocks.
//!   (The information about a procedure is split
//!   across multiple blocks in a weird format.
//!   This combines this data per procedure)
//!
//! (See [`Target`] for details.)

#[allow(unused)]
use scopes::{Target, TargetBlocks, TargetLists, TargetProcedures, TargetVariables};

pub mod attrs;
mod aux;
pub mod blocks;
pub mod error;
mod reader;
pub mod scopes;

pub use reader::json_from_sb3_file;
pub use reader::json_from_sb3_stream;
pub use scopes::ProjectDoc;
pub type Id = svalue::ARc<str>;

/// everything that the proc-macro needs to reference is here
#[allow(unused)]
mod _exports {
    pub use crate::aux::AttrLocation;
    pub(crate) use crate::aux::errors::BlockKindError;
    pub use crate::aux::opcode_trait::{AsOpcodeName, AsOpcodeUnit};
    pub(crate) use crate::aux::parse_attr::helper_attr_access;
    pub(crate) use crate::aux::parse_block::{BlockProperties, JsonBlocks, ParseJsonBlock};
}
