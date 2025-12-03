macro_rules! impl_string_from {
    ($({$($generics:ident),*})? $type: ty, $inter: ty) => {
        impl<'a, $($($generics)*)?> From<&'a str> for $type {
            fn from(val: &'a str) -> Self {
                let r: $inter = val.into();
                r.into()
            }
        }
        impl<'a, $($($generics)*)?> From<String> for $type {
            fn from(val: String) -> Self {
                let r: $inter = val.into();
                r.into()
            }
        }
    };
}
pub(crate) use impl_string_from;
