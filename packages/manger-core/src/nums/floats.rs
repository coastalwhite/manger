use std::convert::TryInto;

use super::{Digit, OneOrMore, Sign};
use crate::{Consumable, ConsumeError, ConsumeErrorType, ConsumeSource};

#[derive(Debug, PartialEq)]
enum FpCategory {
    Infinity(Sign),
    NaN,
    Normal(i64, i64),
}

const INFINITY_TEXT: &str = "infinity";
const NAN_TEXT: &str = "nan";

impl Consumable for FpCategory {
    fn consume_from(source: &str) -> Result<(Self, &str), ConsumeError> {
        // NaN parsing
        if source.to_lowercase().starts_with(NAN_TEXT) {
            return Ok((FpCategory::NaN, &source[NAN_TEXT.len()..]));
        }

        // Infinity parsing
        if let Ok((sign, unconsumed)) = Sign::consume_from(source) {
            if unconsumed.to_lowercase().starts_with(INFINITY_TEXT) {
                return Ok((
                    FpCategory::Infinity(sign),
                    &unconsumed[INFINITY_TEXT.len()..],
                ));
            }

            // Normal Float parsing
            let (fst_int, unconsumed) = i64::consume_from(unconsumed)?;
            let unconsumed = unconsumed.consume_lit(&'.')?;
            let (snd_int, unconsumed) = i64::consume_from(unconsumed)?;

            Ok((FpCategory::Normal(fst_int, snd_int), unconsumed))
        } else {
            Err(ConsumeError::new_with(ConsumeErrorType::UnexpectedToken {
                index: 0,
                token: '_',
            }))
        }
    }
}

impl Consumable for f32 {
    fn consume_from(source: &str) -> Result<(Self, &str), ConsumeError> {
        let (fp_category, unconsumed) = FpCategory::consume_from(source)?;

        use FpCategory::*;
        Ok((
            match fp_category {
                NaN => f32::NAN,
                Infinity(sign) => match sign {
                    Sign::Negative => f32::NEG_INFINITY,
                    Sign::Positive => f32::INFINITY,
                },
                Normal(fst, snd) => {
                    use az::{OverFlowingAs};
                    let (wrapped_fst, overflowed) = fst.overflowing_as::<f32>();
                    let (wrapped_snd, overflowed) = snd.overflowing_as::<f32>();

                }
            },
            unconsumed,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fp_category_parse_nan() {
        assert_eq!(
            <FpCategory<f32>>::consume_from(NAN_TEXT),
            Ok((FpCategory::NaN, ""))
        );
        assert_eq!(
            <FpCategory<f32>>::consume_from("Nan"),
            Ok((FpCategory::NaN, ""))
        );
        assert_eq!(
            <FpCategory<f32>>::consume_from("NaN"),
            Ok((FpCategory::NaN, ""))
        );
        assert_eq!(
            <FpCategory<f32>>::consume_from("naN"),
            Ok((FpCategory::NaN, ""))
        );
        assert_eq!(
            <FpCategory<f32>>::consume_from("nAN"),
            Ok((FpCategory::NaN, ""))
        );
        assert!(<FpCategory<f32>>::consume_from("-nAN").is_err());
    }

    #[test]
    fn fp_category_parse_infinity() {
        assert_eq!(
            <FpCategory<f32>>::consume_from(INFINITY_TEXT),
            Ok((FpCategory::Infinity(Sign::Positive), ""))
        );
        assert_eq!(
            <FpCategory<f32>>::consume_from("Nan"),
            Ok((FpCategory::NaN, ""))
        );
        assert_eq!(
            <FpCategory<f32>>::consume_from("NaN"),
            Ok((FpCategory::NaN, ""))
        );
        assert_eq!(
            <FpCategory<f32>>::consume_from("naN"),
            Ok((FpCategory::NaN, ""))
        );
        assert_eq!(
            <FpCategory<f32>>::consume_from("nAN"),
            Ok((FpCategory::NaN, ""))
        );
        assert!(<FpCategory<f32>>::consume_from("-nAN").is_err());
    }
}
