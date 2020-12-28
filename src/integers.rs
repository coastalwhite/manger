use crate::common::{Digit, OneOrMore, Sign};
use crate::{ConsumeError, ConsumeErrorType};

macro_rules! impl_consume_uint {
    ( $type: ty ) => {
        impl $crate::Consumable for $type {
            fn consume_from(s: &str) -> Result<(Self, &str), ConsumeError> {
                let (digits, unconsumed) = OneOrMore::<Digit>::consume_from(s)?;

                let mut num: $type = 0;

                for digit in digits.into_iter() {
                    let digit = digit.value();

                    num = num
                        .checked_mul(10)
                        .and_then(|num| num.checked_add(digit))
                        .ok_or(ConsumeError::new_with(ConsumeErrorType::InvalidValue {
                            index: 0,
                        }))?;
                }

                Ok((num, unconsumed))
            }
        }
    };
}

macro_rules! impl_consume_int {
    ( $type: ty ) => {
        impl $crate::Consumable for $type {
            fn consume_from(s: &str) -> Result<(Self, &str), ConsumeError> {
                let (sign, unconsumed) = Sign::consume_from(s)?;
                let (digits, unconsumed) = OneOrMore::<Digit>::consume_from(unconsumed)?;

                let mut num: $type = 0;

                for digit in digits.into_iter() {
                    let digit = digit.value();

                    num = num
                        .checked_mul(10)
                        .and_then(|num| num.checked_add(digit))
                        .ok_or(ConsumeError::new_with(ConsumeErrorType::InvalidValue {
                            index: 0,
                        }))?;
                }

                Ok((num * sign.normal::<$type>(), unconsumed))
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
    use crate::error::ConsumeError;
    use crate::error::ConsumeErrorType::*;
    use crate::Consumable;

    #[test]
    fn test_u8_consume_parse() {
        for i in 0..u8::MAX {
            assert_eq!(i, u8::consume_from(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_u8_consume_parse_errors() {
        assert_eq!(
            u8::consume_from("").unwrap_err(),
            ConsumeError::new_with(InsufficientTokens { index: 0 })
        );
        assert_eq!(
            u8::consume_from("-123").unwrap_err(),
            ConsumeError::new_with(UnexpectedToken {
                index: 0,
                token: '-'
            })
        );
        assert_eq!(
            u8::consume_from("256").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
    }

    #[test]
    fn test_u16_consume_parse() {
        for i in 0..200 {
            assert_eq!(i, u16::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in (u16::MAX - 200)..u16::MAX {
            assert_eq!(i, u16::consume_from(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_u16_consume_parse_errors() {
        assert_eq!(
            u16::consume_from("").unwrap_err(),
            ConsumeError::new_with(InsufficientTokens { index: 0 })
        );
        assert_eq!(
            u16::consume_from("-123").unwrap_err(),
            ConsumeError::new_with(UnexpectedToken {
                index: 0,
                token: '-'
            })
        );
        assert_eq!(
            u16::consume_from("65536").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
    }

    #[test]
    fn test_u32_consume_parse() {
        for i in 0..200 {
            assert_eq!(i, u32::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in (u32::MAX - 200)..u32::MAX {
            assert_eq!(i, u32::consume_from(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_u32_consume_parse_errors() {
        assert_eq!(
            u32::consume_from("").unwrap_err(),
            ConsumeError::new_with(InsufficientTokens { index: 0 })
        );
        assert_eq!(
            u32::consume_from("-123").unwrap_err(),
            ConsumeError::new_with(UnexpectedToken {
                index: 0,
                token: '-'
            })
        );
        assert_eq!(
            u32::consume_from("4294967296").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
    }

    #[test]
    fn test_u64_consume_parse() {
        for i in 0..200 {
            assert_eq!(i, u64::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in (u64::MAX - 200)..u64::MAX {
            assert_eq!(i, u64::consume_from(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_u64_consume_parse_errors() {
        assert_eq!(
            u64::consume_from("").unwrap_err(),
            ConsumeError::new_with(InsufficientTokens { index: 0 })
        );
        assert_eq!(
            u64::consume_from("-123").unwrap_err(),
            ConsumeError::new_with(UnexpectedToken {
                index: 0,
                token: '-'
            })
        );
        assert_eq!(
            u64::consume_from("18446744073709551616").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
    }

    #[test]
    fn test_i8_consume_parse() {
        for i in i8::MIN..i8::MAX {
            assert_eq!(i, i8::consume_from(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_i8_consume_parse_errors() {
        assert_eq!(
            i8::consume_from("").unwrap_err(),
            ConsumeError::new_with(InsufficientTokens { index: 0 })
        );
        assert_eq!(
            i8::consume_from("a123").unwrap_err(),
            ConsumeError::new_with(UnexpectedToken {
                index: 0,
                token: 'a'
            })
        );
        assert_eq!(
            i8::consume_from("128").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
        assert_eq!(
            i8::consume_from("-129").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
    }

    #[test]
    fn test_i16_consume_parse() {
        for i in i16::MIN..(i16::MIN + 200) {
            assert_eq!(i, i16::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in -100..100 {
            assert_eq!(i, i16::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in (i16::MAX - 200)..i16::MAX {
            assert_eq!(i, i16::consume_from(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_i16_consume_parse_errors() {
        assert_eq!(
            i16::consume_from("").unwrap_err(),
            ConsumeError::new_with(InsufficientTokens { index: 0 })
        );
        assert_eq!(
            i16::consume_from("a123").unwrap_err(),
            ConsumeError::new_with(UnexpectedToken {
                index: 0,
                token: 'a'
            })
        );
        assert_eq!(
            i16::consume_from("32768").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
        assert_eq!(
            i16::consume_from("-32769").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
    }

    #[test]
    fn test_i32_consume_parse() {
        for i in i32::MIN..(i32::MIN + 200) {
            assert_eq!(i, i32::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in -100..100 {
            assert_eq!(i, i32::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in (i32::MAX - 200)..i32::MAX {
            assert_eq!(i, i32::consume_from(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_i32_consume_parse_errors() {
        assert_eq!(
            i32::consume_from("").unwrap_err(),
            ConsumeError::new_with(InsufficientTokens { index: 0 })
        );
        assert_eq!(
            i32::consume_from("a123").unwrap_err(),
            ConsumeError::new_with(UnexpectedToken {
                index: 0,
                token: 'a'
            })
        );
        assert_eq!(
            i32::consume_from("2147483648").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
        assert_eq!(
            i32::consume_from("-2147483649").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
    }

    #[test]
    fn test_i64_consume_parse() {
        for i in i64::MIN..(i64::MIN + 200) {
            assert_eq!(i, i64::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in -100..100 {
            assert_eq!(i, i64::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in (i64::MAX - 200)..i64::MAX {
            assert_eq!(i, i64::consume_from(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_i64_consume_parse_errors() {
        assert_eq!(
            i64::consume_from("").unwrap_err(),
            ConsumeError::new_with(InsufficientTokens { index: 0 })
        );
        assert_eq!(
            i64::consume_from("a123").unwrap_err(),
            ConsumeError::new_with(UnexpectedToken {
                index: 0,
                token: 'a'
            })
        );
        assert_eq!(
            i64::consume_from("9223372036854775808").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
        assert_eq!(
            i64::consume_from("-9223372036854775809").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
    }

    #[test]
    fn test_i128_consume_parse() {
        for i in i128::MIN..(i128::MIN + 200) {
            assert_eq!(i, i128::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in -100..100 {
            assert_eq!(i, i128::consume_from(&format!("{}", i)).unwrap().0);
        }

        for i in (i128::MAX - 200)..i128::MAX {
            assert_eq!(i, i128::consume_from(&format!("{}", i)).unwrap().0);
        }
    }

    #[test]
    fn test_i128_consume_parse_errors() {
        assert_eq!(
            i128::consume_from("").unwrap_err(),
            ConsumeError::new_with(InsufficientTokens { index: 0 })
        );
        assert_eq!(
            i128::consume_from("a123").unwrap_err(),
            ConsumeError::new_with(UnexpectedToken {
                index: 0,
                token: 'a'
            })
        );
        assert_eq!(
            i128::consume_from("170141183460469231731687303715884105728").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
        assert_eq!(
            i128::consume_from("-170141183460469231731687303715884105729").unwrap_err(),
            ConsumeError::new_with(InvalidValue { index: 0 })
        );
    }
}
