use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, parenthesized, Token};

use super::SequenceItem;

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
        let options_content;
        let options = if let Ok(_) = (|ref mut cnt| Ok(braced![cnt in stream]))(options_content) {
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
