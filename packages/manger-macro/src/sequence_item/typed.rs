use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::Token;

use crate::ToTokenstream;

/// The typed struct represents a typed sequence item
///
/// EBNF Syntax is as follows (with `IDENT`, `TYPE` and `EXPRBLOCK` the rust standard syntaxes):
/// ```ebnf
/// typed := IDENT? ':' TYPE EXPRBLOCK?
/// ```
#[derive(Debug)]
pub struct Typed {
    ident: Option<syn::Ident>,
    ty: syn::Path,
    filter: Option<syn::ExprBlock>,
}

impl Parse for Typed {
    fn parse(stream: ParseStream) -> Result<Self> {
        // Panic if the stream is empty. This should never happen.
        if stream.is_empty() {
            panic!("Expected Typed, came up empty handed.");
        }

        // Try to parse a identifier
        let ident = stream.parse::<syn::Ident>().ok();

        // Parse a colon
        stream.parse::<Token![:]>()?;

        // Parse a type
        let ty = stream.parse::<syn::Path>()?;

        // Try to parse a expression
        let expr_option = stream.parse::<syn::Expr>().ok();

        // Check that that expression either does not exist or is a block expression.
        let filter = if let Some(expr) = expr_option {
            if let syn::Expr::Block(block) = expr {
                Some(block)
            } else {
                // Found a different type of expression
                return Err(syn::parse::Error::new(
                    expr.span(),
                    "Expected a block expression. Found a different type of expression",
                ));
            }
        } else {
            // Nothing going on, didn't find a expression
            None
        };

        Ok(Typed { ident, ty, filter })
    }
}

impl ToTokenstream for Typed {
    fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        let ident_qt = self
            .ident
            .as_ref()
            .map_or(quote! {}, |ref ident| quote! { let #ident = });
        let type_qt = &self.ty;

        let filter_qt = self.filter.as_ref().map_or(quote! {}, |ref filter| {
            quote! {
                .and_then(
                    |(item, by)| {
                        if (#filter)(item) {
                            Ok((item, by))
                        } else {
                            Err(
                                manger_core::ConsumeError::new_with(
                                    manger_core::ConsumeErrorType::InvalidValue { index: offset }
                                )
                            )
                        }
                    }
                )
            }
        });

        quote! {
            #ident_qt
            manger_core::ConsumeSource::mut_consume_by::<#type_qt>(&mut unconsumed)
                #filter_qt
                .map(|(prop, by)| {
                    #[allow(unused_assignments)]
                    { offset += by };

                    prop
                })
                .map_err( |err| err.offset(offset) )?;
        }
    }
}
