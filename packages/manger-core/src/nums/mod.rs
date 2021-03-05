use crate::{Consumable, ConsumeError, ConsumeErrorType};

// Since we need to define the impl for Consumable in the same crate as the trait is defined in we
// have to simulate some of the behaviour of the default lib here

#[derive(Debug, PartialEq, Clone, Copy)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

impl Consumable for Digit {
    fn consume_from(source: &str) -> Result<(Self, &str), ConsumeError> {
        use Digit::*;

        Ok((
            match source.chars().next() {
                None => Err(ConsumeError::new_with(
                    ConsumeErrorType::InsufficientTokens { index: 0 },
                ))?,
                Some('1') => One,
                Some('2') => Two,
                Some('3') => Three,
                Some('4') => Four,
                Some('5') => Five,
                Some('6') => Six,
                Some('7') => Seven,
                Some('8') => Eight,
                Some('9') => Nine,
                Some('0') => Zero,
                Some(token) => Err(ConsumeError::new_with(ConsumeErrorType::UnexpectedToken {
                    index: 0,
                    token,
                }))?,
            },
            utf8_slice::from(source, 1),
        ))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Sign {
    Positive,
    Negative,
}

impl Consumable for Sign {
    fn consume_from(source: &str) -> Result<(Self, &str), ConsumeError> {
        use Sign::*;

        Ok(
            match source.chars().next() {
                Some('-') => (Negative, utf8_slice::from(source, 1)),
                Some('+') => (Positive, utf8_slice::from(source, 1)),
                _ => (Positive, source),
            }
        )
    }
}

macro_rules! sign_into_primitive {
    ($($primitive:ty),+) => {
        $(
            impl Into<$primitive> for Sign {
                fn into(self) -> $primitive {
                    match self {
                        Sign::Positive   => 1,
                        Sign::Negative   => -1,
                    }
                }
            }
        )+
    };
}

macro_rules! digit_into_primitive {
    ($($primitive:ty),+) => {
        $(
            impl Into<$primitive> for Digit {
                fn into(self) -> $primitive {
                    match self {
                        Digit::One   => 1,
                        Digit::Two   => 2,
                        Digit::Three => 3,
                        Digit::Four  => 4,
                        Digit::Five  => 5,
                        Digit::Six   => 6,
                        Digit::Seven => 7,
                        Digit::Eight => 8,
                        Digit::Nine  => 9,
                        Digit::Zero  => 0,
                    }
                }
            }
        )+
    };
}

sign_into_primitive!(i8, i16, i32, i64, i128, isize);
digit_into_primitive!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

struct OneOrMore<T>(T, Vec<T>);

impl<T> OneOrMore<T> {
    pub fn into_vec(self) -> Vec<T> {
        let OneOrMore(head, tail) = self;

        let mut vec = Vec::with_capacity(tail.len() + 1);

        vec.push(head);
        tail.into_iter().for_each(|item| vec.push(item));

        vec
    }
}

impl<T: Consumable> Consumable for OneOrMore<T> {
    fn consume_from(source: &str) -> Result<(Self, &str), ConsumeError> {
        let ((head, tail), unconsumed) = <(T, Vec<T>)>::consume_from(source)?;

        Ok((OneOrMore(head, tail), unconsumed))
    }
}

mod floats;
mod integers;