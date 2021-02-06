use syn::parse::{Parse, ParseStream, Result};
use syn::Token;

pub struct GroupOptions {
    ignore_inner_whitespace: true,
    ignore_outer_whitespace: tru
}

#[derive(Debug)]
pub enum GroupOption {
    IgnoreInnerWhitespace(bool),
    IgnoreOuterWhitespace(bool),
}

macro_rules! bool_option_arm {
    ($variant:ident, $stream:ident) => {
        {
            if $stream.parse::<Token![:]>().is_ok() {
                let literal = $stream.parse::<syn::Lit>()?;

                if let syn::Lit::Bool(bool_lit) = literal {
                    Ok(GroupOption::$variant(bool_lit.value))
                } else {
                    Err(
                        syn::parse::Error::new(
                            literal.span(),
                            "Expected a boolean literal. Found a different type of literal"
                        )
                    )
                }
            } else {
                Ok(GroupOption::$variant(true))
            }
        }
    };
}

impl Parse for GroupOption {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.is_empty() {
            panic!("Expected GroupOptions. Came up empty handed.");
        }

        let option_keyword = stream.parse::<syn::Ident>()?;
        match &option_keyword.to_string()[..] {
            "ignore_inner_whitespace" => bool_option_arm!(IgnoreInnerWhitespace, stream),
            "ignore_outer_whitespace" => bool_option_arm!(IgnoreOuterWhitespace, stream),
            _ => Err(syn::parse::Error::new(
                option_keyword.span(),
                "Unknown option",
            )),
        }
    }
}
