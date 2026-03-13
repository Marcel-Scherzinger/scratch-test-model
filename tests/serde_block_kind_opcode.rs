use scratch_test_model::blocks::{
    BlockKindUnit, CmpBlockKindUnit, ExprOrCmpBlockKindUnit, StmtBlockKindUnit,
};

#[test]
fn test_serde_block_kind_unit() {
    let x = StmtBlockKindUnit::LooksSay;
    let string = serde_json::to_string(&x).unwrap();
    assert_eq!(r#""looks_say""#, string.as_str());
    let b: BlockKindUnit = serde_json::from_str(&string).unwrap();
    assert_eq!(BlockKindUnit::Stmt(x), b);

    let x = StmtBlockKindUnit::PenSetPenColorParamTo;
    let string = serde_json::to_string(&x).unwrap();
    assert_eq!(r#""pen_setPenColorParamTo""#, string.as_str());
    let b: BlockKindUnit = serde_json::from_str(&string).unwrap();
    assert_eq!(BlockKindUnit::Stmt(x), b);

    let x = ExprOrCmpBlockKindUnit::Cmp(CmpBlockKindUnit::OperatorOr);
    let string = serde_json::to_string(&x).unwrap();
    assert_eq!(r#""operator_or""#, string.as_str());
    let b: CmpBlockKindUnit = serde_json::from_str(&string).unwrap();
    assert_eq!(CmpBlockKindUnit::OperatorOr, b);
}
