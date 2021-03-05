use syn::parse::{Parse, ParseStream, Result};

#[derive(Debug)]
pub struct Specifier {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
}

impl Parse for Specifier {
    fn parse(stream: ParseStream) -> Result<Self> {
        Ok(Specifier {
            ident: stream.parse()?,
            generics: stream.parse()?,
        })
    }
}
