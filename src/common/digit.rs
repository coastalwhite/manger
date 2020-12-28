use crate::consume_enum;
#[derive(Debug, PartialEq)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

consume_enum!(
    Digit {
        Zero => [ > '0'; ],
        One => [ > '1'; ],
        Two => [ > '2'; ],
        Three => [ > '3'; ],
        Four => [ > '4'; ],
        Five => [ > '5'; ],
        Six => [ > '6'; ],
        Seven => [ > '7'; ],
        Eight => [ > '8'; ],
        Nine => [ > '9'; ]
    }
);

impl Digit {
    pub fn value<'a, T: From<&'a Digit>>(&'a self) -> T {
        <T>::from(self)
    }
}

macro_rules! from_digit {
    ( $( $type:ty ),* ) => {
        $(
        impl From<&Digit> for $type {
            fn from(digit: &Digit) -> Self {
                use Digit::*;

                match digit {
                    Zero => 0,
                    One => 1,
                    Two => 2,
                    Three => 3,
                    Four => 4,
                    Five => 5,
                    Six => 6,
                    Seven => 7,
                    Eight => 8,
                    Nine => 9,
                }
            }
        }
        )*
    };
}

from_digit!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
