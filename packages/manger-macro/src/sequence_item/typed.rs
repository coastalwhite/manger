use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::Token;

/// The typed struct represents a typed sequence item
/// 
/// EBNF Syntax is as follows (with `IDENT`, `TYPE` and `EXPRBLOCK` the rust standard syntaxes):
/// ```ebnf
/// typed := IDENT? ':' TYPE EXPRBLOCK?
/// ```
#[derive(Debug)]
pub struct Typed {
    ident: Option<syn::Ident>,
    ty: syn::Type,
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
        let ty = stream.parse::<syn::Type>()?;

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
