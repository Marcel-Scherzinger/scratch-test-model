macro_rules! associate_ast_types {
    ($(
        $($t: ty)|+ => $ast: ty
    ),* $(,)?) => {
        $(
            $(
                impl $crate::ast::IsAssociatedWithAstDataType for $t {
                    type AstDataType = $ast;
                }
            )+
        )*
    };
}
pub(crate) use associate_ast_types;
