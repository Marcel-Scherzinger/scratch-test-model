use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use scratch_test_model::{ProjectDoc, json_from_sb3_file};

fn parse_in_folder(folder: &str) {
    let walker = walkdir::WalkDir::new(folder).into_iter();
    let files = walker.flatten().collect_vec();

    let (oks, errors): (Vec<_>, Vec<_>) = files
        .into_par_iter()
        .filter(|file| {
            file.file_type().is_file()
                && file.path().extension().and_then(|s| s.to_str()) == Some("sb3")
        })
        .map(|file| {
            let json = json_from_sb3_file(file.path())
                .map_err(|err| (file.path().to_owned(), format!("{err:#?}")))?;
            ProjectDoc::from_json(&json)
                .map(|_| ())
                .map_err(|err| (file.path().to_owned(), format!("{err:#?}")))
        })
        .partition(Result::is_ok);
    let errors: Vec<_> = errors.into_iter().flat_map(Result::err).collect();

    if !errors.is_empty() {
        let count = errors.len();
        let out = errors
            .into_iter()
            .map(|(path, error)| format!("{path:?}:\n{error}\n\n"))
            .join("");
        panic!("Failed to parse {count} files:\n\n{out}");
    }
    if oks.is_empty() {
        panic!("No sb3 files found in folder: {folder:?}");
    }
}

#[test]
fn try_parsing_further() {
    parse_in_folder("sb3/further/valid");
}

// archives that failed to parse using other reading function (#3)
#[test]
fn try_parsing_archives() {
    parse_in_folder("sb3/archive/first.sb3");
    parse_in_folder("sb3/archive/second.sb3");
}
