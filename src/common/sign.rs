use crate::consume_enum;

/// Enum that represents parsing a number sign.
///
/// Will consume into `Sign::Negative` for '-'.
/// Will consume into `Sign::Positive` for '+' or if neither '+' or '-' are found.
///
/// # Examples
///
/// ```
/// use manger::ConsumeSource;
/// use manger::common::Sign;
///
/// assert_eq!(
///     "42".consume::<Sign>?,
///     Sign::Positive
/// );
///     
/// assert_eq!(
///     "-1".consume::<Sign>?,
///     Sign::Positive
/// );
///
/// assert_eq!(
///     "+a".consume::<Sign>?,
///     Sign::Positive
/// );
///
/// # Ok(())
/// ```
#[derive(Debug, PartialEq)]
pub enum Sign {
    /// Consumed either a '+' or nothing.
    Positive,

    /// Consumed a '-'.
    Negative,
}

use crate::chars;
use crate::common;

#[derive(Debug, PartialEq)]
enum PositiveType {
    Plus,
    Empty,
}
consume_enum!(
    PositiveType {
        Plus => [
            : chars::Plus;
        ],
        Empty => [
            : common::Nothing;
        ]
    }
);

consume_enum!(
    Sign {
        Negative => [
            : chars::Hyphen;
        ],
        Positive => [
            : PositiveType;
        ]
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
