//! Types for common structures within consuming.

#[doc(inline)]
pub use one_or_more::OneOrMore;

#[doc(inline)]
pub use sign::Sign;

#[doc(inline)]
pub use catch_all::CatchAll;

#[doc(inline)]
pub use digit::Digit;

#[doc(inline)]
pub use whitespace::Whitespace;

mod catch_all;
mod digit;
mod one_or_more;
mod sign;
mod whitespace;
