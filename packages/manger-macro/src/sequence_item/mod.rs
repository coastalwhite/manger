use syn::parse::{Parse, ParseStream, Result};

use group::Group;
use literal::Literal;
use typed::Typed;

use crate::ToTokenstream;

#[derive(Debug)]
pub enum SequenceItem {
    Literal(Literal),
    Typed(Typed),
    Group(Group),
}

pub mod group;
pub mod literal;
pub mod typed;

impl Parse for SequenceItem {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.is_empty() {
            panic!("Expected SequenceItem, came up empty handed.");
        }

        // Try Literal
        if let Ok(lit) = stream.parse::<Literal>() {
            return Ok(SequenceItem::Literal(lit));
        }

        // Try Typed
        if let Ok(typed) = stream.parse::<Typed>() {
            return Ok(SequenceItem::Typed(typed));
        }

        // Try Group
        if let Ok(group) = stream.parse::<Group>() {
            return Ok(SequenceItem::Group(group));
        }

        Err(stream.error("Expected either a literal, a typed item or a group. Found none of those"))
    }
}

impl ToTokenstream for SequenceItem {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        use SequenceItem::*;

        match self {
            Typed(typed) => typed.to_tokenstream(),
            Literal(lit) => lit.to_tokenstream(),
            Group(group) => group.to_tokenstream(),
        }
    }
}
