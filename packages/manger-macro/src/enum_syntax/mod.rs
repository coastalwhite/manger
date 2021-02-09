use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Token,
};

use quote::quote;

use crate::{specifier::Specifier, ToTokenstream};
use variant::Variant;

mod variant;

#[derive(Debug)]
pub struct Enum {
    specifier: Specifier,
    variants: Vec<Variant>,
}

impl Parse for Enum {
    fn parse(stream: ParseStream) -> Result<Self> {
        let specifier = stream.parse()?;

        let content;
        braced!(content in stream);

        let variants = <Punctuated<Variant, Token![,]>>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(Enum {
            specifier,
            variants,
        })
    }
}

impl ToTokenstream for Enum {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        let ident = &self.specifier.ident;
        let (impl_generics, type_generics, where_clause) = &self.specifier.generics.split_for_impl();

        let variants: Vec<proc_macro2::TokenStream> = self
            .variants
            .iter()
            .map(|variant| variant.to_tokenstream(ident))
            .collect();

        quote! {
            impl #impl_generics manger_core::Consumable for #ident #type_generics
            #where_clause
            {
                fn consume_from(source: &str) -> Result<(Self, &str), manger_core::ConsumeError> {
                    #(#variants)*

                    Err(manger_core::ConsumeError::new())
                }
            }
        }
    }
}
