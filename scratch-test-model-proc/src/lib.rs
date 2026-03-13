mod parse_impls;
mod valid_attributes;

use proc_macro2::TokenStream as TokenStream2;
use valid_attributes::{BlockLevelAttribute, my_attributes};

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

struct BlockKindEnumSpec {
    vis: syn::Visibility,
    name: syn::Ident,
    variants: Vec<BlockVariantSpec>,
    #[allow(unused)]
    default_location: Option<ParameterLocation>,
}
struct BlockVariantSpec {
    /// The opcode of the block, likely snakecase
    opcode: String,
    name: syn::Ident,
    parameters: Vec<BlockParameterSpec>,
    parse_via: Option<syn::Ident>,
}

enum VariantLevelAttributes {
    Opcode(syn::LitStr),
    ParseVia(syn::Ident),
}
enum FieldLevelAttributes {
    JsonName(syn::LitStr),
    Location(ParameterLocation),
    // Optional(bool),
}

struct BlockParameterSpec {
    /// The name the parameter uses in the JSON representation, likely uppercase
    json_name: String,
    location: ParameterLocation,
    // optional: bool,
    #[allow(unused)]
    vis: syn::Visibility,
    name: syn::Ident,
    #[allow(unused)]
    ty: syn::Type,
}

#[derive(Debug, Clone, Copy)]
enum ParameterLocation {
    Inputs,
    Fields,
}

impl quote::ToTokens for BlockKindEnumSpec {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let unit_name = syn::Ident::new(
            &format!("{}Unit", self.name),
            proc_macro2::Span::call_site(),
        );
        let name = &self.name;
        let variants = self
            .variants
            .iter()
            .map(|v| v.name.clone())
            .collect::<Vec<_>>();
        let opcodes: Vec<_> = self.variants.iter().map(|v| v.opcode.clone()).collect();
        let opcode_comments = opcodes.iter().map(|o| format!("`{o}` is the json-side opcode"));
        let opcode_names = opcodes.iter();
        let vis = &self.vis;


        // unit version
        tokens.extend(quote! {
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
            #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
            #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #vis enum #unit_name {
                #( 
                    #[doc = #opcode_comments]
                    #[cfg_attr(feature = "serde", serde(rename = #opcode_names))]
                    #variants
                ,)*
            }
            impl crate::_exports::AsOpcodeUnit for #name {
                type OpcodeUnit = #unit_name;

                fn opcode(&self) -> Self::OpcodeUnit {
                    match self {
                        #( Self::#variants{..} => Self::OpcodeUnit::#variants, )*
                    }
                }
            }
            impl #unit_name {
                /// Method with the same functionality as
                /// [`AsOpcodeName::opcode_name`](crate::_exports::AsOpcodeName::opcode_name),
                /// but this can be used in a `const`-context.
                pub const fn k_opcode_name(&self) -> &'static str {
                    match self {
                        #( Self::#variants => #opcodes, )*
                    }
                }
            }
            impl crate::_exports::AsOpcodeName for #unit_name {
                fn opcode_name(&self) -> &'static str {
                    match self {
                        #( Self::#variants => #opcodes, )*
                    }
                }
            }

            impl std::fmt::Display for #unit_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(crate::_exports::AsOpcodeName::opcode_name(&self))
                }
            }
        });

        let variants = self.variants.iter().map(|variant| {
            if let Some(parse_via_func) = variant.parse_via.as_ref() {
                quote! {
                    #parse_via_func(all_target_blocks, opcode, inputs, fields, mutation)
                }
            } else {

            let attribute_defs = variant.parameters.iter().map(|p| {
                let ident = &p.name;
                let key = &p.json_name;
                match p.location {
                    ParameterLocation::Inputs => quote! {
                        let #ident = crate::_exports::helper_attr_access(all_target_blocks, inputs, #key)?;
                    },
                    ParameterLocation::Fields => quote! {
                        let #ident = crate::_exports::helper_attr_access(all_target_blocks, fields, #key)?;
                    },
                }
            });
            let attribute_names = variant.parameters.iter().map(|p| &p.name);

            let var_name = &variant.name;

            quote! {
                #(#attribute_defs;)*
                Ok(Self::#var_name { #(#attribute_names,)* })
            }}
        });

        tokens.extend(quote!( impl crate::_exports::ParseJsonBlock for #name {
            fn parse_json_block<'a, 'b>(
                all_target_blocks: crate::_exports::JsonBlocks<'a>,
                opcode: &str,
                inputs: &'b crate::_exports::BlockProperties<'a>,
                fields: &'b crate::_exports::BlockProperties<'a>,
                mutation: Option<&'b crate::_exports::BlockProperties<'a>>
            ) -> Result<Self, crate::_exports::BlockKindError<'a>>
            where
                Self: Sized {
                match opcode {
                    #(#opcodes => {#variants})*
                    opcode => Err(crate::_exports::BlockKindError::UnknownOpcode(opcode.into()))
                }
            }
        } ));

        // do for all attributes
        //
        let attribute_types = self.variants.iter()
            .flat_map(|variant| variant.parameters.iter().map(|p| &p.ty)).map(|ty| {
                quote! { #ty: crate::_exports::DoForAttrs<'a, S> }
            });

        let variants = self.variants.iter().map(|variant| {
            let params = variant.parameters.iter().map(|param| {
                let name = &param.name;
                quote! { crate::_exports::DoForAttrs::<'a, S>::do_for_attrs(
                    #name,
                    inputs_that_are_passed_to_all_attributes, 
                    outputs_received_by_all_attributes,
                )?; 
                }
            });
            let names = variant.parameters.iter().map(|param| &param.name);
            let var_name = &variant.name;
            quote! {
                Self::#var_name{ #(#names),* } => { #(#params)* }
            }
        });

        tokens.extend(quote!(
            impl<'a, S: crate::_exports::DoForAttrsStrategy<'a>> crate::_exports::DoForAttrs<'a, S> for #name 
                where
                    #(#attribute_types),*
            {
                fn do_for_attrs(
                    &'a self,
                    inputs_that_are_passed_to_all_attributes: &S::Inputs,
                    outputs_received_by_all_attributes: &mut S::Outputs
                ) -> Result<(), S::Error> {
                    match self {
                        #(#variants)*
                    }
                    Ok(())
                }
            }
    ));
    }
}

#[proc_macro_derive(Block, attributes(block))]
pub fn derive_block(input: TokenStream) -> TokenStream {
    let data = parse_macro_input!(input as BlockKindEnumSpec);

    quote! {
        #data
    }
    .into()
}
