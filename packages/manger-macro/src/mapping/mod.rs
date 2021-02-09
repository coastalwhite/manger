use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    Token,
};

use crate::ToTokenstream;

use named::NamedMapping;
use unnamed::UnnamedMapping;

mod named;
mod unnamed;

#[derive(Debug)]
pub enum Mapping {
    Unnamed(UnnamedMapping),
    Named(NamedMapping),
    Unit,
}

impl Parse for Mapping {
    fn parse(stream: ParseStream) -> Result<Self> {
        Ok(
            if stream.cursor().eof() || {
                let fork = stream.fork();
                fork.parse::<Token![;]>().is_ok() && fork.cursor().eof()
            } {
                Mapping::Unit
            } else if let Ok(mapping) = stream.parse() {
                Mapping::Named(mapping)
            } else if let Ok(mapping) = stream.parse() {
                Mapping::Unnamed(mapping)
            } else {
                Err(stream.error("Unrecognized token appended!"))?
            },
        )
    }
}

impl ToTokenstream for Mapping {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        use Mapping::*;

        match self {
            Unit => quote! { },
            Unnamed(unnamed) => unnamed.to_tokenstream(),
            Named(named) => named.to_tokenstream(),
        }
    }
}
