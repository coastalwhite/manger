use syn::parse::{Parse, ParseStream, Result};

pub struct Literal(syn::Lit);

impl Parse for Literal {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.is_empty() {
            panic!("Expected Literal, came up empty handed.");
        }

        Ok(Literal(stream.parse::<syn::Lit>()?))
    }
}
