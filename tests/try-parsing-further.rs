use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use scratch_test_model::ProjectDoc;

fn parse_in_folder(folder: &str) {
    let walker = walkdir::WalkDir::new(folder).into_iter();
    let files = walker.flatten().collect_vec();

    let errors: Vec<_> = files
        .into_par_iter()
        .filter(|file| {
            file.file_type().is_file()
                && file.path().extension().and_then(|s| s.to_str()) == Some("sb3")
        })
        .map(|file| {
            ProjectDoc::from_sb3_file(file.path())
                .map(|_| ())
                .map_err(|err| (file.path().to_owned(), format!("{err:#?}")))
                .err()
        })
        .flatten()
        .collect();

    if !errors.is_empty() {
        let count = errors.len();
        let out = errors
            .into_iter()
            .map(|(path, error)| format!("{path:?}:\n{error}\n\n"))
            .join("");
        panic!("Failed to parse {count} files:\n\n{out}");
    }
}

#[test]
fn try_parsing_further() {
    parse_in_folder("sb3/further/valid");
}

// As described in [https://github.com/Marcel-Scherzinger/scratch-test-model/issues/3]
// this currently doesn't work
//
// It's marked as `should_panic` so it's noticed if a future version of a dependency fixes this
#[test]
#[should_panic]
fn try_parsing_io_unsupported_archive() {
    parse_in_folder("sb3/further/unsupported-archive");
}
