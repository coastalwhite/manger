use proc_macro::TokenStream;
use syn::parse::{ Parse, ParseStream, Result };
use syn::Token;

use quote::quote;

enum GroupFlag {
}

struct GroupFlags(Vec<GroupFlag>);

struct GroupItem {
    flags: GroupFlags,
    sequence_items: Vec<SequenceItem>,
    tuple_structure: Option<Vec<syn::Expr>>
}

enum SequenceItem {
    Literal(syn::Lit),
    Typed(Option<syn::Ident>, syn::Type),
    Group(GroupItem)
}

struct TypedSyntax {
    ident: Option<syn::Ident>,
    _colon_token: Token![:],
    ty: syn::Type
}

impl Parse for SequenceItem {
    fn parse(stream: ParseStream) -> Result<Self> {
        //TODO: Add group parsing        
        if stream.is_empty() {
            panic!("Expected SequenceItem, came up empty handed.");
        }

        // Try Literal
        if let Ok(lit) = stream.parse::<syn::Lit>() {
            return Ok(SequenceItem::Literal(lit));
        }

        // Try Typed
        let syntax = TypedSyntax {
            ident: stream.parse().ok(),
            _colon_token: stream.parse().unwrap(),
            ty: stream.parse().unwrap(),
        };
    
        Ok(SequenceItem::Typed(syntax.ident, syntax.ty))
    }
}

#[proc_macro]
pub fn sequence_item(input: TokenStream) -> TokenStream {
    let seq = syn::parse_macro_input!(input as SequenceItem);

    match seq {
        SequenceItem::Literal(lit) => quote! { format!("Literal {:?}", #lit) },
        SequenceItem::Typed(ident, ty) => {
            let ident_str = format!("{:?}", ident);
            let ty_str = format!("{:?}", ty);

            quote! { format!("Typed {}, {}", #ident_str, #ty_str) }
        }
        SequenceItem::Group(_g) => quote! { "Group" },
    }.into()
}
