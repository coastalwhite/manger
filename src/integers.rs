use crate::chars::CharConsumeError;
use crate::standard::{Digit, Sign};
use crate::{ConsumeParsable, OneOrMore};

#[derive(Debug, PartialEq)]
pub enum IntegerConsumeError {
    Overflow,
    InvalidToken(usize, char),
    EmptyString,
}

impl Into<IntegerConsumeError> for CharConsumeError {
    fn into(self) -> IntegerConsumeError {
        match self {
            CharConsumeError::EmptyString => IntegerConsumeError::EmptyString,
            CharConsumeError::InvalidToken(i, c) => IntegerConsumeError::InvalidToken(i, c),
        }
    }
}

macro_rules! impl_consume_uint {
    ( $type: ty ) => {
        impl ConsumeParsable for $type {
            type ConsumeError = IntegerConsumeError;

            fn consume(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
                let ((Digit(head), tail), unconsumed) =
                    OneOrMore::<Digit>::consume(s).map_err(|either_err| {
                        match either_err {
                            either::Left(e) => e,
                            either::Right(e) => e,
                        }
                        .into()
                    })?;

                let mut num = <$type>::from(head);

                for Digit(digit) in tail {
                    let digit = <$type>::from(digit);

                    if num > (<$type>::MAX - digit) / 10 {
                        return Err(IntegerConsumeError::Overflow);
                    }

                    num *= 10;
                    num += digit;
                }

                Ok((num, unconsumed))
            }
        }
    };
}

macro_rules! impl_consume_int {
    ( $type: ty ) => {
        impl ConsumeParsable for $type {
            type ConsumeError = IntegerConsumeError;

            fn consume(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
                let (Sign(is_positive), unconsumed) = Sign::consume(s).unwrap();

                let ((Digit(head), tail), unconsumed) = OneOrMore::<Digit>::consume(unconsumed)
                    .map_err(|either_err| {
                        match either_err {
                            either::Left(e) => e,
                            either::Right(e) => e,
                        }
                        .into()
                    })?;

                let mut num = head as $type;
                if !is_positive {
                    num *= -1;
                }

                for Digit(digit) in tail {
                    let digit = digit as $type;

                    if is_positive {
                        if num > (<$type>::MAX - digit) / 10 {
                            return Err(IntegerConsumeError::Overflow);
                        }
                    } else {
                        if num < (<$type>::MIN + digit) / 10 {
                            return Err(IntegerConsumeError::Overflow);
                        }
                    }

                    num *= 10;
                    num += if is_positive { digit } else { -1 * digit };
                }

                Ok((num, unconsumed))
            }
        }
    };
}

impl_consume_uint!(u8);
impl_consume_uint!(u16);
impl_consume_uint!(u32);
impl_consume_uint!(u64);

impl_consume_int!(i8);
impl_consume_int!(i16);
impl_consume_int!(i32);
impl_consume_int!(i64);
impl_consume_int!(i128);

#[cfg(test)]
mod tests {
    use super::IntegerConsumeError::*;
    use crate::ConsumeParsable;

    #[test]
    fn test_u8_consume_parse() {
        for i in 0..u8::MAX {
            assert_eq!(i, u8::consume(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_u8_consume_parse_errors() {
        assert_eq!(u8::consume("").unwrap_err(), EmptyString);
        assert_eq!(u8::consume("-123").unwrap_err(), InvalidToken(0, '-'));
        assert_eq!(u8::consume("256").unwrap_err(), Overflow);
    }

    #[test]
    fn test_u16_consume_parse() {
        for i in 0..200 {
            assert_eq!(i, u16::consume(&format!("{}", i)).unwrap().0);
        }

        for i in (u16::MAX - 200)..u16::MAX {
            assert_eq!(i, u16::consume(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_u16_consume_parse_errors() {
        assert_eq!(u16::consume("").unwrap_err(), EmptyString);
        assert_eq!(u16::consume("-123").unwrap_err(), InvalidToken(0, '-'));
        assert_eq!(u16::consume("65536").unwrap_err(), Overflow);
    }

    #[test]
    fn test_u32_consume_parse() {
        for i in 0..200 {
            assert_eq!(i, u32::consume(&format!("{}", i)).unwrap().0);
        }

        for i in (u32::MAX - 200)..u32::MAX {
            assert_eq!(i, u32::consume(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_u32_consume_parse_errors() {
        assert_eq!(u32::consume("").unwrap_err(), EmptyString);
        assert_eq!(u32::consume("-123").unwrap_err(), InvalidToken(0, '-'));
        assert_eq!(u32::consume("4294967296").unwrap_err(), Overflow);
    }

    #[test]
    fn test_u64_consume_parse() {
        for i in 0..200 {
            assert_eq!(i, u64::consume(&format!("{}", i)).unwrap().0);
        }

        for i in (u64::MAX - 200)..u64::MAX {
            assert_eq!(i, u64::consume(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_u64_consume_parse_errors() {
        assert_eq!(u64::consume("").unwrap_err(), EmptyString);
        assert_eq!(u64::consume("-123").unwrap_err(), InvalidToken(0, '-'));
        assert_eq!(u64::consume("18446744073709551616").unwrap_err(), Overflow);
    }

    #[test]
    fn test_i8_consume_parse() {
        for i in i8::MIN..i8::MAX {
            assert_eq!(i, i8::consume(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_i8_consume_parse_errors() {
        assert_eq!(i8::consume("").unwrap_err(), EmptyString);
        assert_eq!(i8::consume("a123").unwrap_err(), InvalidToken(0, 'a'));
        assert_eq!(i8::consume("128").unwrap_err(), Overflow);
        assert_eq!(i8::consume("-129").unwrap_err(), Overflow);
    }

    #[test]
    fn test_i16_consume_parse() {
        for i in i16::MIN..(i16::MIN + 200) {
            assert_eq!(i, i16::consume(&format!("{}", i)).unwrap().0);
        }

        for i in -100..100 {
            assert_eq!(i, i16::consume(&format!("{}", i)).unwrap().0);
        }

        for i in (i16::MAX - 200)..i16::MAX {
            assert_eq!(i, i16::consume(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_i16_consume_parse_errors() {
        assert_eq!(i16::consume("").unwrap_err(), EmptyString);
        assert_eq!(i16::consume("a123").unwrap_err(), InvalidToken(0, 'a'));
        assert_eq!(i16::consume("32768").unwrap_err(), Overflow);
        assert_eq!(i16::consume("-32769").unwrap_err(), Overflow);
    }

    #[test]
    fn test_i32_consume_parse() {
        for i in i32::MIN..(i32::MIN + 200) {
            assert_eq!(i, i32::consume(&format!("{}", i)).unwrap().0);
        }

        for i in -100..100 {
            assert_eq!(i, i32::consume(&format!("{}", i)).unwrap().0);
        }

        for i in (i32::MAX - 200)..i32::MAX {
            assert_eq!(i, i32::consume(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_i32_consume_parse_errors() {
        assert_eq!(i32::consume("").unwrap_err(), EmptyString);
        assert_eq!(i32::consume("a123").unwrap_err(), InvalidToken(0, 'a'));
        assert_eq!(i32::consume("2147483648").unwrap_err(), Overflow);
        assert_eq!(i32::consume("-2147483649").unwrap_err(), Overflow);
    }

    #[test]
    fn test_i64_consume_parse() {
        for i in i64::MIN..(i64::MIN + 200) {
            assert_eq!(i, i64::consume(&format!("{}", i)).unwrap().0);
        }

        for i in -100..100 {
            assert_eq!(i, i64::consume(&format!("{}", i)).unwrap().0);
        }

        for i in (i64::MAX - 200)..i64::MAX {
            assert_eq!(i, i64::consume(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_i64_consume_parse_errors() {
        assert_eq!(i64::consume("").unwrap_err(), EmptyString);
        assert_eq!(i64::consume("a123").unwrap_err(), InvalidToken(0, 'a'));
        assert_eq!(i64::consume("9223372036854775808").unwrap_err(), Overflow);
        assert_eq!(i64::consume("-9223372036854775809").unwrap_err(), Overflow);
    }

    #[test]
    fn test_i128_consume_parse() {
        for i in i128::MIN..(i128::MIN + 200) {
            assert_eq!(i, i128::consume(&format!("{}", i)).unwrap().0);
        }

        for i in -100..100 {
            assert_eq!(i, i128::consume(&format!("{}", i)).unwrap().0);
        }

        for i in (i128::MAX - 200)..i128::MAX {
            assert_eq!(i, i128::consume(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_i128_consume_parse_errors() {
        assert_eq!(i128::consume("").unwrap_err(), EmptyString);
        assert_eq!(i128::consume("a123").unwrap_err(), InvalidToken(0, 'a'));
        assert_eq!(
            i128::consume("170141183460469231731687303715884105728").unwrap_err(),
            Overflow
        );
        assert_eq!(
            i128::consume("-170141183460469231731687303715884105729").unwrap_err(),
            Overflow
        );
    }
}
