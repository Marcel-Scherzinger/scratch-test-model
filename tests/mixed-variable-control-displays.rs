use scratch_test_model::ProjectDoc;

#[test]
fn parse_mixed_variable_control_displays_numeric() {
    if let Err(err) =
        ProjectDoc::from_sb3_file("sb3/mixed-variable-control-displays-number-values.sb3")
    {
        panic!("{err:#?}");
    }
}

#[test]
fn parse_mixed_variable_control_displays_boolean() {
    if let Err(err) =
        ProjectDoc::from_sb3_file("sb3/mixed-variable-control-displays-boolean-values.sb3")
    {
        panic!("{err:#?}");
    }
}
