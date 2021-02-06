use syn::parse::{Parse, ParseStream, Result};

use typed::Typed;
use group::Group;

#[derive(Debug)]
pub enum SequenceItem {
    Literal(syn::Lit),
    Typed(Typed),
    Group(Group),
}

mod group;
mod typed;

impl Parse for SequenceItem {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.is_empty() {
            panic!("Expected SequenceItem, came up empty handed.");
        }

        // Try Literal
        if let Ok(lit) = stream.parse::<syn::Lit>() {
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
