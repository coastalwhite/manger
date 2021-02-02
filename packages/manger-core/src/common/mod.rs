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

#[doc(inline)]
pub use end::End;

mod catch_all;
mod digit;
mod end;
mod one_or_more;
mod sign;
mod whitespace;
