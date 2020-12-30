use crate::common::{Digit, OneOrMore, Sign};
use crate::{ConsumeError, ConsumeErrorType};

macro_rules! impl_consume_uint {
    ( $type: ty, $test_name:ident$(, $plus_maxvalue:literal )? ) => {
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

        #[test]
        fn $test_name() {
            use crate::ConsumeErrorType::*;
            use crate::{ ConsumeError, Consumable };

            for i in <$type>::MIN..(<$type>::MIN + 10) {
                assert_eq!(i, <$type>::consume_from(&format!("{}", i)).expect("MIN TEST FAILED").0);
            }

            for i in (<$type>::MAX - 10)..<$type>::MAX {
                assert_eq!(i, <$type>::consume_from(&format!("{}", i)).expect("MAX TEST FAILED").0);
            }

            assert_eq!(
                <$type>::consume_from("").unwrap_err(),
                ConsumeError::new_from(
                        vec![InsufficientTokens { index: 0 }; 10]
                    )
            );
            assert_eq!(
                <$type>::consume_from("-123").unwrap_err(),
                ConsumeError::new_from(
                    vec![UnexpectedToken { index: 0, token: '-' }; 10]
                )
            );
            $(
            assert_eq!(
                <$type>::consume_from($plus_maxvalue).unwrap_err(),
                ConsumeError::new_with(InvalidValue { index: 0 })
            );
            )?
        }
    };
}

macro_rules! impl_consume_int {
    ( $type: ty, $test_name:ident$(, $plus_maxvalue:literal, $min_minvalue:literal )? ) => {
        impl $crate::Consumable for $type {
            fn consume_from(s: &str) -> Result<(Self, &str), ConsumeError> {
                let (sign, unconsumed) = Sign::consume_from(s)?;
                let (digits, unconsumed) = OneOrMore::<Digit>::consume_from(unconsumed)?;

                let mut num: $type = 0;
                let normal = sign.normal::<$type>();

                for digit in digits.into_iter() {
                    let digit = normal * digit.value::<$type>();

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

        #[test]
            fn $test_name() {
                use crate::ConsumeErrorType::*;
                use crate::{ ConsumeError, Consumable };

                for i in <$type>::MIN..(<$type>::MIN + 10) {
                    assert_eq!(i, <$type>::consume_from(&format!("{}", i)).expect("MIN TEST FAILED").0);
                }

                for i in (-10)..10 {
                    assert_eq!(i, <$type>::consume_from(&format!("{}", i)).expect("AROUND 0 TEST FAILED").0);
                }

                for i in (<$type>::MAX - 10)..<$type>::MAX {
                    assert_eq!(i, <$type>::consume_from(&format!("{}", i)).expect("MAX TEST FAILED").0);
                }

                assert_eq!(
                    <$type>::consume_from("").unwrap_err(),
                    ConsumeError::new_from(
                        vec![InsufficientTokens { index: 0 }; 10]
                    )
                );
                assert_eq!(
                    <$type>::consume_from("a123").unwrap_err(),
                    ConsumeError::new_from(
                        vec![UnexpectedToken { index: 0, token: 'a' }; 10]
                    )
                );
                $(
                assert_eq!(
                    <$type>::consume_from($plus_maxvalue).unwrap_err(),
                    ConsumeError::new_with(InvalidValue { index: 0 })
                );
                assert_eq!(
                    <$type>::consume_from($min_minvalue).unwrap_err(),
                    ConsumeError::new_with(InvalidValue { index: 0 })
                );
                )?
            }
    };
}

impl_consume_uint!(u8, u8_consuming, "256");
impl_consume_uint!(u16, u16_consuming, "65536");
impl_consume_uint!(u32, u32_consuming, "4294967296");
impl_consume_uint!(u64, u64_consuming, "18446744073709551616");
impl_consume_uint!(
    u128,
    u128_consuming,
    "340282366920938463463374607431768211456"
);
impl_consume_uint!(usize, usize_consuming);

impl_consume_int!(i8, i8_consuming, "128", "-129");
impl_consume_int!(i16, i16_consuming, "32768", "-32769");
impl_consume_int!(i32, i32_consuming, "2147483648", "-2147483649");
impl_consume_int!(
    i64,
    i64_consuming,
    "9223372036854775808",
    "-9223372036854775809"
);
impl_consume_int!(
    i128,
    i128_consuming,
    "170141183460469231731687303715884105728",
    "-170141183460469231731687303715884105729"
);
impl_consume_int!(isize, isize_consuming);
