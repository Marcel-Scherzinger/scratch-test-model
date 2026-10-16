//! > **Note**
//! > This project was merged into [github:marcel-scherzinger/bast3st-scratch](https://github.com/marcel-scherzinger/bast3st-scratch)

pub use smodel::{Id, ProjectDoc, attrs, blocks, error, scopes};

#[cfg(feature = "sb3")]
pub use smodel::json_from_sb3_file;
#[cfg(feature = "sb3")]
pub use smodel::json_from_sb3_stream;
