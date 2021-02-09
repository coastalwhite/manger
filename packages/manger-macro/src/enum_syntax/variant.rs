use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Token,
};

use quote::quote;

use crate::sequence_item::SequenceItem;
use crate::{mapping::Mapping, ToTokenstream};

#[derive(Debug)]
pub struct Variant {
    ident: syn::Ident,
    sequence_items: Vec<SequenceItem>,
    mapping: Mapping,
}

impl Parse for Variant {
    fn parse(stream: ParseStream) -> Result<Self> {
        let ident = stream.parse()?;

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

        Ok(Variant {
            ident,
            sequence_items,
            mapping,
        })
    }
}

impl Variant {
    pub fn to_tokenstream(&self, enum_name: &syn::Ident) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let sequence_items: Vec<proc_macro2::TokenStream> = self
            .sequence_items
            .iter()
            .map(|seq_item| seq_item.to_tokenstream())
            .collect();
        let mapping = self.mapping.to_tokenstream();

        quote! {
            if let Result::<(#enum_name, &str), manger_core::ConsumeError>::Ok(res) = (|| {
                let mut unconsumed = source;
                let mut offset = 0;

                #(#sequence_items)*

                return Ok(
                    (
                        #enum_name::#ident #mapping,
                        unconsumed
                    )
                );
            })() {
                return Ok(res);
            }
        }
    }
}
