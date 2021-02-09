//! Types for common structures within consuming.

use manger_core::{Consumable, ConsumeError, ConsumeErrorType};
use manger_macro::mangez;

/// End of stream of tokens.
///
/// Will succeed in consumation if the end of string has been reached. Will fail if it has not been
/// reached.
///
/// # Examples
///
/// ```
/// use manger::{consume_struct, Consumable};
/// use manger::common;
///
/// #[derive(PartialEq, Debug)]
/// struct EncasedInteger(i32);
/// consume_struct!(
///     EncasedInteger => [
///         > '(',
///         value: i32,
///         > ')',
///         : common::End;
///         (value)
///     ]
/// );
///
/// assert!(EncasedInteger::consume_from("(42)").is_ok());
/// assert!(EncasedInteger::consume_from("(42) some leftover tokens").is_err());
/// ```
#[derive(Debug, PartialEq)]
pub struct End;

impl Consumable for End {
    fn consume_from(source: &str) -> Result<(Self, &str), ConsumeError> {
        if source.is_empty() {
            Ok((End, ""))
        } else {
            Err(ConsumeError::new_with(ConsumeErrorType::UnexpectedToken {
                index: 0,
                token: source.chars().next().unwrap(),
            }))
        }
    }
}

/// A catch-all clause for consuming.
///
/// Most often used with Enums and with the `Either<L, R>` struct.
#[derive(Debug, PartialEq)]
pub struct CatchAll;

mangez!(
    CatchAll {
        [ "" ]
    }
);

/// Enum that represents parsing a number sign.
///
/// Will consume into `Sign::Negative` for '-'.
/// Will consume into `Sign::Positive` for '+' or if neither '+' or '-' are found.
///
/// # Examples
///
/// ```
/// use manger::Consumable;
/// use manger::common::Sign;
///
/// assert_eq!(
///     Sign::consume_from("42")?.0,
///     Sign::Positive
/// );
///     
/// assert_eq!(
///     Sign::consume_from("-1")?.0,
///     Sign::Negative
/// );
///
/// assert_eq!(
///     Sign::consume_from("+a")?.0,
///     Sign::Positive
/// );
/// # Ok::<(), manger::ConsumeError>(())
/// ```
#[derive(Debug, PartialEq)]
pub enum Sign {
    /// Consumed either a '+' or nothing.
    Positive,

    /// Consumed a '-'.
    Negative,
}

#[derive(Debug, PartialEq)]
enum PositiveType {
    Plus,
    Empty,
}
mangez!(
    PositiveType {
        Plus {
            [ : chars::Plus ]
        },
        Empty {
            [ : CatchAll ]
        }
    }
);

mangez!(
    Sign {
        Negative {
            ['-']
        },
        Positive {
            [: PositiveType]
        }
    }
);

impl Sign {
    /// Fetch the normalized value for a sign. This will `Positive` into `1` and `Negative` into
    /// `-1`.
    pub fn normal<'a, T: From<&'a Sign>>(&'a self) -> T {
        <T>::from(self)
    }

    /// Returns whether the `Sign` is of the `Positive` variant.
    pub fn is_positive(&self) -> bool {
        use Sign::*;

        match self {
            Positive => true,
            Negative => false,
        }
    }

    /// Returns whether the `Sign` is of the `Negative` variant.
    pub fn is_negative(&self) -> bool {
        use Sign::*;

        match self {
            Positive => false,
            Negative => true,
        }
    }
}

macro_rules! from_sign_int {
    ( $( $type:ty ),* ) => {
        $(
        impl From<&Sign> for $type {
            fn from(sign: &Sign) -> Self {
                use Sign::*;

                match sign {
                    Positive => 1,
                    Negative => -1,
                }
            }
        }
        )*
    };
}

macro_rules! from_sign_float {
    ( $( $type:ty ),* ) => {
        $(
        impl From<&Sign> for $type {
            fn from(sign: &Sign) -> Self {
                use Sign::*;

                match sign {
                    Positive => 1.0,
                    Negative => -1.0,
                }
            }
        }
        )*
    };
}

from_sign_int!(i8, i16, i32, i64, i128, isize);
from_sign_float!(f32, f64);

/// Struct representing a Whitespace utf-8 character.
///
/// Will consume all characters which return true on [`char::is_whitespace`].
#[derive(Debug, PartialEq)]
pub struct Whitespace;

mangez!(
    Whitespace {
        [
            : char { |token: char| token.is_whitespace() }
        ]
    }
);

mod digit;
pub use digit::Digit;

mod one_or_more;
pub use one_or_more::OneOrMore;

pub mod chars;
