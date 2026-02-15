use syn::{Token, punctuated::Punctuated};

use crate::{FieldLevelAttributes, ParameterLocation, VariantLevelAttributes};

pub fn my_attributes<P: syn::parse::Parse>(
    attrs: Vec<syn::Attribute>,
) -> syn::Result<Vec<Punctuated<P, Token![,]>>> {
    attrs
        .into_iter()
        .flat_map(|a| match a.meta {
            syn::Meta::List(list)
                if list.path.is_ident("block") && matches!(a.style, syn::AttrStyle::Outer) =>
            {
                Some(list.parse_args_with(Punctuated::<P, Token![,]>::parse_terminated))
            }
            _ => None,
        })
        .collect::<Result<Vec<_>, _>>()
}

#[derive(Debug)]
pub enum BlockLevelAttribute {
    DefaultLocation(ParameterLocation),
}
impl syn::parse::Parse for BlockLevelAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = syn::Ident::parse(input)?;
        match key.to_string().as_str() {
            "default_location" => {
                let _: Token![=] = input.parse()?;
                input.parse().map(Self::DefaultLocation)
            }
            other => Err(input.error(format!("{other:?} is no valid attribute for a block enum"))),
        }
    }
}

impl syn::parse::Parse for VariantLevelAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = syn::Ident::parse(input)?;
        match key.to_string().as_str() {
            "opcode" => {
                let _: Token![=] = input.parse()?;
                input.parse().map(Self::Opcode)
            }
            "parse_via" => {
                let _: Token![=] = input.parse()?;
                input.parse().map(Self::ParseVia)
            }
            other => Err(input.error(format!(
                "{other:?} is no valid attribute for a block enum variant"
            ))),
        }
    }
}

impl syn::parse::Parse for FieldLevelAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = syn::Ident::parse(input)?;
        match key.to_string().as_str() {
            "location" => {
                let _: Token![=] = input.parse()?;
                input.parse().map(Self::Location)
            }
            "json_name" => {
                let _: Token![=] = input.parse()?;
                input.parse().map(Self::JsonName)
            }

            // "optional" => Ok(Self::Optional(true)),
            other => Err(input.error(format!(
                "{other:?} is no valid attribute for a block enum variant field"
            ))),
        }
    }
}
