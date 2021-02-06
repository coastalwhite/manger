use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn mangez(input: TokenStream) -> TokenStream {
    let sq_item = parse_macro_input!(input as sequence_item::SequenceItem);
    let sq_debug = format!("{:?}", sq_item);

    let qt = quote!{
        println!("{:?}", #sq_debug)
    };

    qt.into()
}

mod sequence_item;
