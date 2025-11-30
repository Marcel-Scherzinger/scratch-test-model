use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use scratch_test_model::ProjectDoc;

#[test]
fn try_parsing_further() {
    let walker = walkdir::WalkDir::new("sb3/further").into_iter();
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
