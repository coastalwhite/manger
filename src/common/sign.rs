use crate::consume_enum;

#[derive(Debug, PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

use crate::chars;
use crate::common;
use either::Either;
consume_enum!(
    Sign {
        Negative => [
            : chars::Hyphen;
        ],
        Positive => [
            : Either<common::Nothing, chars::Plus>;
        ]
    }
);

impl Sign {
    pub fn normal<'a, T: From<&'a Sign>>(&'a self) -> T {
        <T>::from(self)
    }

    pub fn is_positive(&self) -> bool {
        use Sign::*;

        match self {
            Positive => true,
            Negative => false,
        }
    }

    pub fn is_negative(&self) -> bool {
        !self.is_positive()
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
