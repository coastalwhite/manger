use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parse_macro_input};

mod enum_syntax;
mod mapping;
mod sequence_item;
mod specifier;
mod struct_syntax;

use enum_syntax::Enum;
use specifier::Specifier;
use struct_syntax::Struct;

#[proc_macro]
pub fn mangez_debug(input: TokenStream) -> TokenStream {
    let consume_syntax = parse_macro_input!(input as ConsumeSyntax);
    let syntax_str = format!("{:#?}", consume_syntax.to_tokenstream());

    let qt = quote! {
        #syntax_str
    };

    qt.into()
}

#[proc_macro]
pub fn mangez_tree(input: TokenStream) -> TokenStream {
    let consume_syntax = parse_macro_input!(input as ConsumeSyntax);
    let syntax_str = format!("{:#?}", consume_syntax);

    let qt = quote! {
        #syntax_str
    };

    qt.into()
}

#[proc_macro]
pub fn mangez(input: TokenStream) -> TokenStream {
    let consume_syntax = parse_macro_input!(input as ConsumeSyntax);
    consume_syntax.to_tokenstream().into()
}

trait ToTokenstream {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream;
}

#[derive(Debug)]
enum ConsumeSyntax {
    Struct(Struct),
    Enum(Enum),
}

impl Parse for ConsumeSyntax {
    fn parse(stream: ParseStream) -> Result<Self> {
        let check_stream = stream.fork();

        check_stream.parse::<Specifier>()?;
        let content;
        braced!(content in check_stream);
        Ok(
            if content.parse::<syn::Ident>().is_ok() && content.peek(syn::token::Brace) {
                ConsumeSyntax::Enum(stream.parse()?)
            } else {
                ConsumeSyntax::Struct(stream.parse()?)
            },
        )
    }
}

impl ToTokenstream for ConsumeSyntax {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        match self {
            ConsumeSyntax::Struct(s) => s.to_tokenstream(),
            ConsumeSyntax::Enum(e) => e.to_tokenstream(),
        }
    }
}
