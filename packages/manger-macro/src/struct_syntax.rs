use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Token,
};

use quote::quote;

use crate::sequence_item::group::options::GroupOption;
use crate::mapping::Mapping;
use crate::sequence_item::SequenceItem;
use crate::specifier::Specifier;
use crate::ToTokenstream;

#[derive(Debug)]
pub struct Struct {
    pub specifier: Specifier,
    pub options: Vec<GroupOption>,
    pub sequence_items: Vec<SequenceItem>,
    pub mapping: Mapping,
}

impl Parse for Struct {
    fn parse(stream: ParseStream) -> Result<Self> {
        let specifier = stream.parse()?;

        let content;
        braced!(content in stream);

        // Parse the options if they are available
        let options = if stream.peek(syn::token::Brace) {
            let options_content;
            braced!(options_content in stream);
            <Punctuated<
                crate::sequence_item::group::options::GroupOption,
                Token![,]
            >>::parse_terminated(&options_content)?
                .into_iter()
                .collect()
        } else {
            Vec::new()
        };

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
            options,
            sequence_items,
            mapping,
        })
    }
}

impl ToTokenstream for Struct {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        let ident = &self.specifier.ident;
        let (impl_generics, type_generics, where_clause) =
            &self.specifier.generics.split_for_impl();

        let sequence_items: Vec<proc_macro2::TokenStream> = self
            .sequence_items
            .iter()
            .map(|seq_item| seq_item.to_tokenstream())
            .collect();

        let (head, tail) = sequence_items.split_at(1);

        let mapping: proc_macro2::TokenStream = self.mapping.to_tokenstream().into();

        quote! {
            impl #impl_generics manger_core::Consumable for #ident #type_generics
            #where_clause
            {
                fn consume_from(source: &str) -> Result<(Self, &str), manger_core::ConsumeError> {
                    let mut unconsumed = source;
                    let mut offset = 0;

                    #(#head)*
                    #(
                        let mut index = 0;
                        for c in unconsumed.chars() {
                            if !c.is_whitespace() {
                                break;
                            }
                            index += 1;
                        }
                        unconsumed = utf8_slice(unconsumed, index);
                        #tail
                    )*

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
