use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Token,
};

use crate::ToTokenstream;

#[derive(Debug)]
pub struct KeyValue {
    key: syn::Ident,
    expr: Option<syn::Expr>,
}

#[derive(Debug)]
pub struct NamedMapping {
    props: Vec<KeyValue>,
}

impl Parse for KeyValue {
    fn parse(stream: ParseStream) -> Result<Self> {
        let key = stream.parse::<syn::Ident>()?;

        let expr = if let Ok(_) = stream.parse::<Token![:]>() {
            Some(stream.parse::<syn::Expr>()?)
        } else {
            None
        };

        Ok(KeyValue { key, expr })
    }
}

impl Parse for NamedMapping {
    fn parse(stream: ParseStream) -> Result<Self> {
        let content;
        braced!(content in stream);

        let props = <Punctuated<KeyValue, Token![,]>>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(NamedMapping { props })
    }
}

impl ToTokenstream for KeyValue {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        let ident = &self.key;
        let expr = self.expr.as_ref();

        quote! { #ident: #expr }
    }
}
impl ToTokenstream for NamedMapping {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        let props: Vec<proc_macro2::TokenStream> = self
            .props
            .iter()
            .map(|prop| prop.to_tokenstream().into())
            .collect();

        quote! { { #(#props),* } }
    }
}
