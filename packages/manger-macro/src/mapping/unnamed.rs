use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Token,
};

use crate::ToTokenstream;

#[derive(Debug)]
pub struct UnnamedMapping {
    exprs: Vec<syn::Expr>,
}

impl Parse for UnnamedMapping {
    fn parse(stream: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in stream);

        let exprs = <Punctuated<syn::Expr, Token![,]>>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(UnnamedMapping { exprs })
    }
}

impl ToTokenstream for UnnamedMapping {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        let exprs =  &self.exprs;

        quote! {
            ( #(#exprs),* )
        }
    }
}
