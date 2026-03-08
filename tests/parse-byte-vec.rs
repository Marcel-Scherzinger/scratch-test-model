#[test]
fn parse_byte_vec() {
    let vec_data: Vec<u8> = std::fs::read("sb3/all-predefined-blocks.sb3").unwrap();
    let mut cursor = std::io::Cursor::new(vec_data);
    let json = scratch_test_model::json_from_sb3_stream(&mut cursor, Some("all-predefined-blocks"))
        .unwrap();
    let _doc = scratch_test_model::ProjectDoc::from_json(&json).unwrap();
}
