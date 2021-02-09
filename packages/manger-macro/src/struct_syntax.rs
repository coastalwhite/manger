use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Token,
};

use quote::quote;

use crate::ToTokenstream;
use crate::specifier::Specifier;
use crate::sequence_item::SequenceItem;
use crate::mapping::Mapping;

#[derive(Debug)]
pub struct Struct {
    pub specifier: Specifier,
    pub sequence_items: Vec<SequenceItem>,
    pub mapping: Mapping,
}

impl Parse for Struct {
    fn parse(stream: ParseStream) -> Result<Self> {
        let specifier = stream.parse()?;

        let content;
        braced!(content in stream);

        let container_group;
        bracketed!(container_group in content);

        let sequence_items =
            <Punctuated<SequenceItem, Token![,]>>::parse_terminated(&container_group)?
                .into_iter()
                .collect();

        let mapping = if let Ok(_) = content.parse::<Token![;]>() {
            Mapping::parse(&content)?
        } else {
            Mapping::Unit
        };

        Ok(Struct {
            specifier,
            sequence_items,
            mapping,
        })
    }
}

impl ToTokenstream for Struct {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        let ident = &self.specifier.ident;
        let (impl_generics, type_generics, where_clause) = &self.specifier.generics.split_for_impl();

        let sequence_items: Vec<proc_macro2::TokenStream> = self
            .sequence_items
            .iter()
            .map(|seq_item| seq_item.to_tokenstream())
            .collect();

        let mapping: proc_macro2::TokenStream = self.mapping.to_tokenstream().into();

        quote! {
            impl #impl_generics manger_core::Consumable for #ident #type_generics
            #where_clause
            {
                fn consume_from(source: &str) -> Result<(Self, &str), manger_core::ConsumeError> {
                    let mut unconsumed = source;
                    let mut offset = 0;

                    #(#sequence_items)*

                    Ok(
                        (
                            #ident #mapping,
                            unconsumed
                        )
                    )
                }
            }
        }
    }
}
