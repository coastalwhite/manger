use crate::consume_enum;

/// Enum representing a decimal digit.
///
/// # Examples
///
/// ```
/// use manger::Consumable;
/// use manger::common::Digit;
///
/// assert_eq!(
///     Digit::consume_from("432")?.0,
///     Digit::Four
/// );
///
/// assert_eq!(
///     Digit::consume_from("2")?.0,
///     Digit::Two
/// );
/// # Ok::<(), manger::ConsumeError>(())
/// ```
#[derive(Debug, PartialEq)]
pub enum Digit {
    /// Consuming found a '0'.
    Zero,

    /// Consuming found a '1'.
    One,

    /// Consuming found a '2'.
    Two,

    /// Consuming found a '3'.
    Three,

    /// Consuming found a '4'.
    Four,

    /// Consuming found a '5'.
    Five,

    /// Consuming found a '6'.
    Six,

    /// Consuming found a '7'.
    Seven,

    /// Consuming found a '8'.
    Eight,

    /// Consuming found a '9'.
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
    /// Get the value of the [`Digit`] as primitive type.
    ///
    /// # Examples
    ///
    /// ```
    /// use manger::common::Digit;
    ///
    /// assert_eq!(Digit::One.value::<u32>(), 1);
    /// assert_eq!(Digit::One.value::<f32>(), 1.0f32);
    /// assert_eq!(Digit::One.value::<usize>(), 1);
    /// ```
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
    ( @float $( $type:ty ),* ) => {
        $(
        impl From<&Digit> for $type {
            fn from(digit: &Digit) -> Self {
                use Digit::*;

                match digit {
                    Zero => 0.0,
                    One => 1.0,
                    Two => 2.0,
                    Three => 3.0,
                    Four => 4.0,
                    Five => 5.0,
                    Six => 6.0,
                    Seven => 7.0,
                    Eight => 8.0,
                    Nine => 9.0,
                }
            }
        }
        )*
    };
}

from_digit!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
from_digit!(@float f32, f64);
