use syn::parse::{Parse, ParseStream, Result};
use quote::quote;

use crate::ToTokenstream;

#[derive(Debug)]
pub struct Literal(syn::Lit);

impl Parse for Literal {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.is_empty() {
            panic!("Expected Literal, came up empty handed.");
        }

        Ok(Literal(stream.parse::<syn::Lit>()?))
    }
}

impl ToTokenstream for Literal {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        let Literal(literal) = self;

        let qt = quote!{
            manger_core::ConsumeSource::mut_consume_lit(&mut unconsumed, &#literal)
                .map(|by| {
                    #[allow(unused_assignments)]
                    { offset += by };
                })
                .map_err( |err| err.offset(offset) )?;
        };

        qt.into()
    }
}
