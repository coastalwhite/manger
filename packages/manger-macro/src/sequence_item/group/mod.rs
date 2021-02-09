use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, parenthesized, Token};


use crate::sequence_item::SequenceItem;
use crate::ToTokenstream;

mod options;

/// A sequence item representing a group of sequence_items
///
/// The EBNF syntax is:
/// ```ebnf
/// group := options? '(' (SEQUENCE_ITEM ',')* SEQUENCE_ITEM ','? ')'
/// options := '{' (GROUP_OPTION ',')* GROUP_OPTION ','? '}'
/// ```
#[derive(Debug)]
pub struct Group {
    options: Vec<options::GroupOption>,
    sequence_items: Vec<SequenceItem>,
}

impl Parse for Group {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.is_empty() {
            panic!("Expected SequenceItem::Group, came up empty handed.");
        }

        // Parse the options if they are available
        let options = if stream.peek(syn::token::Brace) {
            let options_content;
            braced!(options_content in stream);
            <Punctuated<options::GroupOption, Token![,]>>::parse_terminated(&options_content)?
                .into_iter()
                .collect()
        } else {
            Vec::new()
        };

        // Parse the sequence items
        let content;
        parenthesized!(content in stream);
        let sequence_items = <Punctuated<SequenceItem, Token![,]>>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(Group {
            options,
            sequence_items,
        })
    }
}

impl ToTokenstream for Group {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        unimplemented!();

        //TODO: Options
        //TODO: Impl this
    }
}
