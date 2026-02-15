use scratch_test_model::{ProjectDoc, json_from_sb3_file, json_from_sb3_stream};

#[test]
fn text2speech_extension_blocks() {
    let json_data = json_from_sb3_file("sb3/text2speech.sb3").unwrap();

    let res = ProjectDoc::from_json(&json_data);
    if let Err(err) = res {
        panic!("{err:#?}");
    }
}

#[test]
fn pen_extension_blocks() {
    let mut sb3_file =
        std::fs::File::open("sb3/pen-extension-blocks.sb3").expect("file to be present");

    let json_data = json_from_sb3_stream(
        &mut sb3_file,
        Some("sb3/pen-extension-blocks.sb3".to_string()),
    )
    .unwrap();

    let res = ProjectDoc::from_json(&json_data);
    if let Err(err) = res {
        panic!("{err:#?}");
    }

    /*
    // assert!(doc.invalid_blocks().next().is_some());

    /* if doc.unknown_blocks().next().is_some() {
            let unknown: HashMap<_, _> = doc.unknown_blocks().collect();
            let ids: BTreeSet<_> = unknown.values().collect();

            panic!("document shouldn't contain unknown blocks: {ids:#?}\n{unknown:#?}");
        }
    */
    use itertools::Itertools;

    let sorted_expected = PEN_BLOCKS.iter().sorted().cloned().unique().collect_vec();
    /*
        let sorted_found = doc
            .unsupported_blocks()
            .map(|(_, o)| o.clone())
            .unique()
            .sorted()
            .collect_vec();
    */

    for (a, b) in sorted_expected.iter().zip(&sorted_found) {
        assert_eq!(a, b);
    }

    assert_eq!(sorted_expected, sorted_found);

    assert_eq!(
        10,
        doc.unsupported_blocks().map(|(_, o)| o).unique().count()
    )
        */
}

/*
const PEN_BLOCKS: [UnsupportedBlockKind; 10] = [
    UnsupportedBlockKind::PenChangePenColorParamBy,
    UnsupportedBlockKind::PenChangePenSizeBy,
    UnsupportedBlockKind::PenClear,
    UnsupportedBlockKind::PenMenuColorParam,
    UnsupportedBlockKind::PenPenDown,
    UnsupportedBlockKind::PenPenUp,
    UnsupportedBlockKind::PenSetPenColorParamTo,
    UnsupportedBlockKind::PenSetPenColorToColor,
    UnsupportedBlockKind::PenSetPenSizeTo,
    UnsupportedBlockKind::PenStamp,
];
*/
