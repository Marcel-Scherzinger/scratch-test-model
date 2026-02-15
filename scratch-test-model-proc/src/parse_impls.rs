use convert_case::Casing;
use syn::{parse::Parse, spanned::Spanned};

use crate::{
    BlockKindEnumSpec, BlockLevelAttribute, BlockParameterSpec, BlockVariantSpec,
    FieldLevelAttributes, ParameterLocation, VariantLevelAttributes, my_attributes,
};

impl Parse for ParameterLocation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let location: syn::Ident = input.parse()?;
        let loc = location.to_string();
        if loc == "inputs" {
            Ok(Self::Inputs)
        } else if loc == "fields" {
            Ok(Self::Fields)
        } else {
            Err(input.error("location has to be inputs or fields"))
        }
    }
}

impl Parse for BlockKindEnumSpec {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = syn::ItemEnum::parse(input)?;
        let name = item.ident;

        let attrs = my_attributes::<BlockLevelAttribute>(item.attrs)?;
        let mut default_location = None;
        for a in attrs.into_iter().flatten() {
            let BlockLevelAttribute::DefaultLocation(loc) = a;
            default_location = Some(loc);
        }
        let variants = item
            .variants
            .into_iter()
            .map(|var| BlockVariantSpec::new(default_location.as_ref(), var))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            vis: item.vis,
            name,
            variants,
            default_location,
        })
    }
}
impl BlockParameterSpec {
    pub fn new(
        default_location: Option<&ParameterLocation>,
        field: syn::Field,
    ) -> syn::Result<Self> {
        let span = field.span();
        let ty = field.ty;
        let vis = field.vis;
        let name = field.ident.unwrap();

        let converter = convert_case::Converter::new()
            .remove_boundaries(&convert_case::Boundary::digits())
            .to_case(convert_case::Case::UpperSnake);
        let mut json_name = converter.convert(name.to_string());
        let mut location = default_location.copied();
        // let mut optional = false;

        let attrs = my_attributes::<FieldLevelAttributes>(field.attrs)?;

        for a in attrs.iter().flatten() {
            match a {
                FieldLevelAttributes::JsonName(j) => json_name = j.value(),
                FieldLevelAttributes::Location(l) => location = Some(*l),
                // FieldLevelAttributes::Optional(o) => optional = *o,
            }
        }
        let Some(location) = location else {
            return Err(syn::Error::new(
                span,
                "specify location or set default_location on type",
            ));
        };
        Ok(Self {
            json_name,
            location,
            // optional,
            vis,
            name,
            ty,
        })
    }
}
impl BlockVariantSpec {
    fn new(
        default_location: Option<&ParameterLocation>,
        variant: syn::Variant,
    ) -> syn::Result<Self> {
        let span = variant.span();
        let attrs = my_attributes::<VariantLevelAttributes>(variant.attrs)?;
        let name = variant.ident;

        let parameters = match variant.fields {
            syn::Fields::Unit => vec![],
            syn::Fields::Named(named) => named
                .named
                .into_iter()
                .map(|p| BlockParameterSpec::new(default_location, p))
                .collect::<Result<_, _>>()?,
            syn::Fields::Unnamed(_) => {
                return Err(syn::Error::new(
                    span,
                    "variants with unnamed fields are not supported",
                ));
            }
        };

        let mut opcode = name.to_string().to_case(convert_case::Case::Snake);
        let mut parse_via = None;
        for a in attrs.into_iter().flatten() {
            if let VariantLevelAttributes::Opcode(op) = a {
                opcode = op.value()
            } else if let VariantLevelAttributes::ParseVia(f) = a {
                parse_via = Some(f);
            }
        }

        Ok(Self {
            opcode,
            name,
            parameters,
            parse_via,
        })
    }
}
