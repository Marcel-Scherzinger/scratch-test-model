use scratch_test_model::{ProjectDoc, json_from_sb3_file};

#[test]
fn parse_mixed_variable_control_displays_numeric() {
    let json = json_from_sb3_file("sb3/mixed-variable-control-displays-number-values.sb3").unwrap();
    if let Err(err) = ProjectDoc::from_json(&json) {
        panic!("{err:#?}");
    }
}

#[test]
fn parse_mixed_variable_control_displays_boolean() {
    let json =
        json_from_sb3_file("sb3/mixed-variable-control-displays-boolean-values.sb3").unwrap();
    if let Err(err) = ProjectDoc::from_json(&json) {
        panic!("{err:#?}");
    }
}
