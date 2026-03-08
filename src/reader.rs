use crate::aux::JsonCtx;
use crate::error::ModelError;
use crate::scopes::{ProjectDoc, Target};

#[cfg(feature = "sb3")]
#[allow(unused)]
pub use sb3::{json_from_sb3_file, json_from_sb3_stream};

impl ProjectDoc {
    /// Parses a document from a json value
    pub fn from_json<'a>(doc: &'a serde_json::Value) -> Result<ProjectDoc, ModelError<'a>> {
        use crate::aux::WithJsonContextExt;
        let semver = doc["meta"]["semver"].as_str().map(svalue::ARc::from);
        let targets = doc["targets"]
            .as_array()
            .ok_or(ModelError::NoTargetsArray(doc))?;
        let targets: Result<Vec<Target>, JsonCtx<'a, _>> = targets
            .iter()
            .map(|x| Target::from_json.with_ctx(x))
            .collect();
        Ok(ProjectDoc {
            targets: targets?.into(),
            semver,
        })
    }
}

#[cfg(feature = "sb3")]
mod sb3 {
    use std::{
        io::{Read, Seek},
        path::Path,
    };

    use crate::{
        ProjectDoc,
        error::{DocError, ModelError},
    };

    fn read_json_from_file<R: Read + Seek, T: std::fmt::Debug>(
        reader: &mut R,
        tag: T,
    ) -> Result<serde_json::Value, DocError> {
        let mut archive = zip::ZipArchive::new(reader)?;
        for idx in 0..archive.len() {
            let file = match archive.by_index(idx) {
                Ok(ok) => ok,
                Err(err) => {
                    log::error!("Error encountered while reading sb3 {tag:?}: {err}");
                    return Err(DocError::Io(err.into()));
                }
            };
            if file.name().to_lowercase().ends_with(".json") {
                return Ok(serde_json::from_reader(file)?);
            }
        }
        Err(DocError::NoDocument)
    }

    pub fn json_from_sb3_stream<R: Read + Seek, T: std::fmt::Debug>(
        handle: &mut R,
        tag: Option<T>,
    ) -> Result<serde_json::Value, DocError> {
        let tag = tag.map(|s| format!("{s:?}")).unwrap_or_default();
        read_json_from_file(handle, tag)
    }
    pub fn json_from_sb3_file(path: impl AsRef<Path>) -> Result<serde_json::Value, DocError> {
        // wrapper is generic and will be duplicated for every concrete type
        // while this implementation can be used for all of them
        fn from_sb3_file_impl(path: &Path) -> Result<serde_json::Value, DocError> {
            let mut handle = std::fs::File::open(path)
                .map_err(|err| DocError::FileRead(path.to_path_buf(), err))?;
            read_json_from_file(&mut handle, path)
        }
        let path = path.as_ref();
        from_sb3_file_impl(path)
    }

    // TODO: think about public API

    #[ouroboros::self_referencing]
    pub struct InnerParseResult {
        value: serde_json::Value,
        #[borrows(value)]
        #[covariant]
        result: Result<ProjectDoc, ModelError<'this>>,
    }
    pub struct ParseResult(InnerParseResult);

    #[allow(unused)]
    impl ParseResult {
        pub fn json(&self) -> &serde_json::Value {
            self.0.borrow_value()
        }
        pub fn result(&self) -> &Result<ProjectDoc, ModelError<'_>> {
            self.0.borrow_result()
        }
        pub fn ok(&self) -> Option<&ProjectDoc> {
            self.result().as_ref().ok()
        }
        pub fn err(&self) -> Option<&ModelError<'_>> {
            self.result().as_ref().err()
        }
    }

    pub type ReadResult = Result<ParseResult, DocError>;

    impl ProjectDoc {
        // TODO: think about public API
        pub(crate) fn from_owned_json(doc: serde_json::Value) -> ParseResult {
            let i = InnerParseResultBuilder {
                value: doc,
                result_builder: |doc| ProjectDoc::from_json(doc),
            }
            .build();
            ParseResult(i)
        }
        #[allow(unused)]
        // TODO: think about public API
        /// Read a sb3 file by [`Path`]
        ///
        /// This is a convenience method that will:
        ///
        /// 1. Open the provided path
        ///     - `Err(`[`DocError::FileRead`]`)` in case of IO error
        /// 2. Extract `project.json`
        ///     - `Err(`[`DocError::Io`]`)` in case of IO error while reading the zip file
        ///     - `Err(`[`DocError::NoDocument`]`)` if file list ends before
        ///       `project.json` was found
        /// 3. Parse `project.json`
        ///     - `Err(`[`DocError::Json`]`)` if the `project.json` fails to parse
        /// 4. Read the information to generate a valid document
        ///     - `Ok(...(`[`ModelError`]`)...)` if the read data is represents
        ///       a semantically invalid program.
        ///     - `Ok(...(`[`ProjectDoc`]`)...)` if everythinf is valid
        ///
        ///    The real inner `Result` can be borrowed via [`ParseResult::result`]
        ///
        ///
        pub(crate) fn from_sb3_file<'a>(path: impl AsRef<Path> + 'a) -> ReadResult {
            // wrapper is generic and will be duplicated for every concrete type
            // while this implementation can be used for all of them
            fn from_sb3_file_impl(path: &Path) -> ReadResult {
                let mut handle = std::fs::File::open(path)
                    .map_err(|err| DocError::FileRead(path.to_path_buf(), err))?;
                loop {
                    match zip::read::read_zipfile_from_stream(&mut handle) {
                        Ok(Some(file)) => {
                            if file.name().to_lowercase().ends_with(".json") {
                                let value: serde_json::Value = serde_json::from_reader(file)?;
                                return Ok(ProjectDoc::from_owned_json(value));
                            }
                        }
                        Ok(None) => Err(DocError::NoDocument)?,
                        Err(e) => {
                            log::error!("Error encountered while reading sb3 {path:?}: {e:?}");
                            Err(DocError::Io(e.into()))?
                        }
                    }
                }
            }
            let path = path.as_ref();
            from_sb3_file_impl(path)
        }
        /// Read sb3 data as a stream from an instance of [`Read`]
        /// with a suffix string used for context in error logs
        ///
        /// The suffix should be empty or start with a space
        /// (that's why this method is private and should be used only
        /// by functions of this type) and will be used to give additional
        /// context to the log messages for errors.
        /// This could for example be a file name.
        ///
        /// This is a convenience method that will:
        ///
        /// 1. Extract `project.json`
        ///     - `Err(`[`DocError::Io`]`)` in case of IO error while reading the zip file
        ///     - `Err(`[`DocError::NoDocument`]`)` if file list ends before
        ///       `project.json` was found
        /// 2. Parse `project.json`
        ///     - `Err(`[`DocError::Json`]`)` if the `project.json` fails to parse
        /// 3. Read the information to generate a valid document
        ///     - `Ok(...(`[`ModelError`]`)...)` if the read data is represents
        ///       a semantically invalid program.
        ///     - `Ok(...(`[`ProjectDoc`]`)...)` if everythinf is valid
        ///
        ///    The real inner `Result` can be borrowed via [`ParseResult::result`]
        ///
        fn _inner_from_sb3_stream<R: Read>(handle: &mut R, suffix: &str) -> ReadResult {
            loop {
                match zip::read::read_zipfile_from_stream(handle) {
                    Ok(Some(file)) => {
                        if file.name().to_lowercase().ends_with(".json") {
                            let value: serde_json::Value = serde_json::from_reader(file)?;
                            return Ok(Self::from_owned_json(value));
                        }
                    }
                    Ok(None) => Err(DocError::NoDocument)?,
                    Err(e) => {
                        log::error!("Error encountered while reading sb3{suffix}: {e:?}");
                        Err(DocError::Io(e.into()))?
                    }
                }
            }
        }
        #[allow(unused)]
        // TODO: think about public API
        /// Read sb3 data as a stream from an instance of [`Read`]
        /// where a tag with extra context for log messages is provided
        ///
        /// This is a convenience method that will:
        ///
        /// 1. Extract `project.json`
        ///     - `Err(`[`DocError::Io`]`)` in case of IO error while reading the zip file
        ///     - `Err(`[`DocError::NoDocument`]`)` if file list ends before
        ///       `project.json` was found
        /// 2. Parse `project.json`
        ///     - `Err(`[`DocError::Json`]`)` if the `project.json` fails to parse
        /// 3. Read the information to generate a valid document
        ///     - `Ok(...(`[`ModelError`]`)...)` if the read data is represents
        ///       a semantically invalid program.
        ///     - `Ok(...(`[`ProjectDoc`]`)...)` if everythinf is valid
        ///
        ///    The real inner `Result` can be borrowed via [`ParseResult::result`]
        ///
        pub(crate) fn from_tagged_sb3_stream<R: Read, T: std::fmt::Display>(
            handle: &mut R,
            tag: &T,
        ) -> ReadResult {
            Self::_inner_from_sb3_stream(handle, &format!(" {tag}"))
        }
        #[allow(unused)]
        // TODO: think about public API
        /// Read sb3 data as a stream from an instance of [`Read`]
        ///
        /// (If you want to tag this stream with a filename
        /// use [`Self::from_tagged_sb3_stream`])
        ///
        /// This is a convenience method that will:
        ///
        /// 1. Extract `project.json`
        ///     - `Err(`[`DocError::Io`]`)` in case of IO error while reading the zip file
        ///     - `Err(`[`DocError::NoDocument`]`)` if file list ends before
        ///       `project.json` was found
        /// 2. Parse `project.json`
        ///     - `Err(`[`DocError::Json`]`)` if the `project.json` fails to parse
        /// 3. Read the information to generate a valid document
        ///     - `Ok(...(`[`ModelError`]`)...)` if the read data is represents
        ///       a semantically invalid program.
        ///     - `Ok(...(`[`ProjectDoc`]`)...)` if everythinf is valid
        ///
        ///    The real inner `Result` can be borrowed via [`ParseResult::result`]
        ///
        pub(crate) fn from_sb3_stream<R: Read>(handle: &mut R) -> ReadResult {
            Self::_inner_from_sb3_stream(handle, "")
        }
    }
}
